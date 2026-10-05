//! UI-only corner magnifier. AE owns asynchronous requests and cancels stale frames.
//! No render callback, saved parameter, OS capture, or retained host pixel pointer.
use super::*;
const SIZE:usize=129;
const RADIUS:f32=64.0;
const ZOOM:f32=3.0;
const PURPOSE:u32=0x4653544c;
#[derive(Clone,Copy)]
struct Gesture {owner:i32,window:i32,index:usize,native:bool}
#[derive(Clone,Copy)]
struct Observation {owner:i32,window:i32,time:i32,scale:u32,corners:[f64;8]}
thread_local! {static PREVIOUS:std::cell::Cell<Option<Observation>>=const{std::cell::Cell::new(None)};}
thread_local! {static ACTIVE:std::cell::Cell<Option<Gesture>>=const{std::cell::Cell::new(None)};}
pub(crate) fn clear(){ACTIVE.set(None);PREVIOUS.set(None);}
// PF effect_ref changes across AE native Point preview callbacks. Retain only
// the documented stream ID, disposing every acquired handle in this callback.
fn owner(input:&ae::InData,id:Option<ae::aegp::PluginId>)->Result<i32,ae::Error>{
    let id=id.ok_or(ae::Error::BadCallbackParameter)?;
    let interface=ae::aegp::suites::PFInterface::new()?;
    let effects=ae::aegp::suites::Effect::new()?;
    let effect=interface.new_effect_for_effect(input.effect_ref(),id)?;
    let result=(||{
        let streams=ae::aegp::suites::Stream::new()?;
        let stream=streams.new_effect_stream_by_index(effect,id,1)?;
        streams.unique_stream_id(&stream)
    })();
    let disposed=effects.dispose_effect(effect);
    match (result,disposed){(Ok(v),Ok(()))=>Ok(v),(Err(e),_)|(_,Err(e))=>Err(e)}
}
pub(crate) fn begin(input:&ae::InData,event:&ae::EventExtra,index:usize,id:Option<ae::aegp::PluginId>){
    if let Ok(owner)=owner(input,id){if index<4 {ACTIVE.set(Some(Gesture{owner,
        window:ui::event_window_code(event),index,native:false}));}}
}
fn native_button_down()->bool{
    #[cfg(any(target_os="macos",target_os="windows"))]{
        unsafe extern "C"{fn eg_loupe_button_down()->bool;}
        // Read-only native UI state. Never called from a render worker.
        unsafe{eg_loupe_button_down()}
    }
    #[cfg(not(any(target_os="macos",target_os="windows")))]{false}
}
pub(crate) fn native_change(input:&ae::InData,index:usize,id:Option<ae::aegp::PluginId>){
    if index<4&&native_button_down(){if let Ok(owner)=owner(input,id){
        ACTIVE.set(Some(Gesture{owner,window:0,index,native:true}));
    }}
}
fn changed_corner(a:&[f64;8],b:&[f64;8])->Option<usize>{
    let mut found=None;
    for i in 0..4 {if a[2*i]!=b[2*i]||a[2*i+1]!=b[2*i+1]{
        if found.is_some(){return None;}found=Some(i);
    }}found
}
pub(crate) fn observe(input:&ae::InData,event:&ae::EventExtra,corners:[f64;8],id:Option<ae::aegp::PluginId>){
    let Ok(owner)=owner(input,id) else{clear();return;};
    let now=Observation{owner,window:ui::event_window_code(event),
        time:input.current_time(),scale:input.time_scale(),corners};
    #[cfg(all(feature="preview-overlay-probe",target_os="macos"))]{
        unsafe extern "C"{fn eg_loupe_button_probe()->i32;}
        let flags=unsafe{eg_loupe_button_probe()};
        super::preview_overlay_probe::loupe(input,event,3,flags as isize,false);
        let bits=PREVIOUS.get().map(|old|{
            i32::from(old.owner==now.owner) | (i32::from(old.window==now.window)<<1) |
            (i32::from(old.time==now.time&&old.scale==now.scale)<<2) |
            (changed_corner(&old.corners,&now.corners).map(|i| (i as i32+1)<<3).unwrap_or(0))
        }).unwrap_or(-1);
        super::preview_overlay_probe::loupe(input,event,4,bits as isize,false);
    }
    if let Some(old)=PREVIOUS.get(){
        if old.owner==now.owner&&old.window==now.window&&old.time==now.time&&old.scale==now.scale&&native_button_down(){
            if let Some(index)=changed_corner(&old.corners,&now.corners){
                // Covers live native Point previews even if supervision is
                // deferred until mouse-up. Ignore scrubbing and whole-plane moves.
                if ACTIVE.get().is_none(){native_change(input,index,id);}
            }
        }
    }
    PREVIOUS.set(Some(now));
}

pub(crate) fn active(input:&ae::InData,event:&ae::EventExtra,id:Option<ae::aegp::PluginId>)->Option<usize>{
    let g=ACTIVE.get()?;
    if g.native&&!native_button_down(){clear();return None;}
    (g.owner==owner(input,id).ok()?&&(g.window==0||g.window==ui::event_window_code(event))).then_some(g.index)
}
fn source(event:&ae::EventExtra,x:f32,y:f32)->Result<(f32,f32),ae::Error>{
    if !x.is_finite()||!y.is_finite()||x.abs()>32767.0||y.abs()>32767.0{return Err(ae::Error::BadCallbackParameter);}
    let mut p=ae::sys::PF_FixedPoint{x:ae::Fixed::from(x).as_fixed(),y:ae::Fixed::from(y).as_fixed()};
    event.callbacks().frame_to_source(&mut p)?;
    Ok((ae::Fixed::from_fixed(p.x).as_f32(),ae::Fixed::from_fixed(p.y).as_f32()))
}
// Constant-size opaque interior, transparent exterior. Target has black/white
// edges so it stays visible on light/dark images without obscuring the exact point.
fn raster(mut sample:impl FnMut(f32,f32)->Option<[u8;4]>)->Vec<u8>{
    let mut out=vec![0;SIZE*SIZE*4];
    for y in 0..SIZE {for x in 0..SIZE {
        let dx=x as f32-RADIUS;let dy=y as f32-RADIUS;let distance=dx.hypot(dy);
        if distance>RADIUS {continue;}
        let checker=if (x/8+y/8)%2==0 {64_u8}else{88};
        let mut rgb=[checker;3];
        if let Some(p)=sample(dx/ZOOM,dy/ZOOM){
            // Host U8 is ARGB premultiplied. Composite on the lens checkerboard.
            for c in 0..3 {rgb[c]=(u16::from(p[c+1])+u16::from(checker)*(255-u16::from(p[0]))/255).min(255) as u8;}
        }
        let target=dx.abs().max(dy.abs())<=12.0&&(dx.abs()<=1.0||dy.abs()<=1.0)&&dx.abs().max(dy.abs())>=3.0;
        let target_edge=dx.abs().max(dy.abs())<=13.0&&(dx.abs()<=2.0||dy.abs()<=2.0)&&dx.abs().max(dy.abs())>=2.0;
        if distance>=RADIUS-2.0||target_edge {rgb=[0;3];}
        if (RADIUS-4.0..RADIUS-2.0).contains(&distance)||target {rgb=[255;3];}
        let i=(y*SIZE+x)*4;out[i]=255;out[i+1..i+4].copy_from_slice(&rgb);
    }}out
}
pub(crate) fn draw(input:&ae::InData,event:&mut ae::EventExtra,supplier:&ae::drawbot::Supplier,
    surface:&ae::drawbot::Surface,center:ae::drawbot::PointF32,id:Option<ae::aegp::PluginId>)->Result<(),ae::Error>{
    source(event,center.x,center.y)?;
    let pixels=frame_pixels(input,event,center,id)?;
    let image=supplier.new_image_from_buffer(SIZE,SIZE,SIZE*4,ae::drawbot::PixelLayout::Argb32Straight,&pixels)?;
    surface.draw_image(&image,&ae::drawbot::PointF32{x:center.x-RADIUS,y:center.y-RADIUS},1.0)
}
#[derive(Clone,Copy,PartialEq,Eq)]
struct FrameKey{owner:i32,window:i32,time:i32,scale:u32,stamp:[i8;4]}
struct Frame{key:FrameKey,width:usize,height:usize,region:ae::Rect,pixels:Vec<u8>}
thread_local!{
    static FRAME:std::cell::RefCell<Option<Frame>>=const{std::cell::RefCell::new(None)};
    static VIEW:std::cell::Cell<Option<(i32,i32)>>=const{std::cell::Cell::new(None)};
}
pub(crate) fn close(){clear();FRAME.with_borrow_mut(|f|*f=None);VIEW.set(None);}
// SDK25.6 AE_EffectUI.h:419 explicitly limits reserved_job_manageP to
// Effect pane custom UI. Never ask a Comp/Layer context for an AsyncManager.
pub(crate) fn prepare_frame(input:&ae::InData,event:&mut ae::EventExtra,id:Option<ae::aegp::PluginId>)->Result<(),ae::Error>{
    if event.window_type()!=ae::WindowType::Effect{return Ok(());}
    let owner=owner(input,id)?;let id=id.ok_or(ae::Error::BadCallbackParameter)?;
    let window=VIEW.get().filter(|v|v.0==owner).map(|v|v.1).unwrap_or(ae::sys::PF_Window_COMP);
    let render=ae::aegp::suites::Render::new()?;
    let key=FrameKey{owner,window,time:input.current_time(),scale:input.time_scale(),stamp:render.current_timestamp()?.a};
    if FRAME.with_borrow(|f|f.as_ref().is_some_and(|f|f.key==key)){return Ok(());}
    let interface=ae::aegp::suites::PFInterface::new()?;
    let layers=ae::aegp::suites::Layer::new()?;
    let layer=interface.effect_layer(input.effect_ref())?;
    let time=interface.convert_effect_to_comp_time(input.effect_ref(),input.current_time(),input.time_scale())?;
    let manager=ae::pf::suites::EffectCustomUI::new()?.context_async_manager(input.as_ptr(),*event)?;
    let receipt=if window==ae::sys::PF_Window_COMP{
        let comp=layers.layer_parent_comp(layer)?;
        let item=ae::aegp::suites::Comp::new()?.item_from_comp(comp)?;
        let options=ae::aegp::RenderOptions::from_item(item,id)?;
        options.set_time(time)?;options.set_world_type(ae::aegp::WorldType::U8)?;
        manager.checkout_or_render_item_frame_async_manager(PURPOSE,options.handle())?
    }else{
        let options=ae::aegp::LayerRenderOptions::from_layer(layer,id)?;
        options.set_time(time)?;options.set_world_type(ae::aegp::WorldType::U8)?;
        manager.checkout_or_render_layer_frame_async_manager(PURPOSE,options.handle())?
    };
    if receipt.is_null(){return Ok(());}
    let result=(||{
        let world=render.receipt_world(receipt)?;let worlds=ae::aegp::suites::World::new()?;
        let (w,h)=worlds.size(world)?;let stride=worlds.row_bytes(world)?;
        if w<=0||h<=0{return Err(ae::Error::BadCallbackParameter);}
        let (w,h)=(w as usize,h as usize);
        let size=w.checked_mul(h).and_then(|v|v.checked_mul(4)).filter(|v|*v<=64*1024*1024).ok_or(ae::Error::BadCallbackParameter)?;
        if stride<w*4||stride.checked_mul(h).is_none_or(|v|v>isize::MAX as usize){return Err(ae::Error::BadCallbackParameter);}
        let ptr=worlds.base_addr8(world)?.cast::<u8>();if ptr.is_null(){return Err(ae::Error::BadCallbackParameter);}
        let region=render.rendered_region(receipt)?;let mut pixels=vec![0;size];
        for y in 0..h{
            // Bounded row copy while receipt is checked out; no host pointer survives.
            let row=unsafe{std::slice::from_raw_parts(ptr.add(y*stride),w*4)};
            pixels[y*w*4..(y+1)*w*4].copy_from_slice(row);
        }
        Ok(Frame{key,width:w,height:h,region,pixels})
    })();
    let checked=render.checkin_frame(receipt);
    match (result,checked){(Ok(frame),Ok(()))=>{
        FRAME.with_borrow_mut(|f|*f=Some(frame));
        // Only a new cached timestamp requests repaint; avoids a DRAW feedback loop.
        ae::pf::suites::AdvApp::new()?.refresh_all_windows()
    },(Err(e),_)|(_,Err(e))=>Err(e)}
}
fn frame_pixels(input:&ae::InData,event:&mut ae::EventExtra,center:ae::drawbot::PointF32,id:Option<ae::aegp::PluginId>)->Result<Vec<u8>,ae::Error>{
    let owner=owner(input,id)?;let window=ui::event_window_code(event);VIEW.set(Some((owner,window)));
    let c=source(event,center.x,center.y)?;
    let xp=source(event,center.x+1.0,center.y)?;let yp=source(event,center.x,center.y+1.0)?;
    FRAME.with_borrow(|frame|{
        let f=frame.as_ref().filter(|f|f.key.owner==owner&&f.key.window==window&&f.key.time==input.current_time()&&f.key.scale==input.time_scale()).ok_or(ae::Error::BadCallbackParameter)?;
        Ok(raster(|dx,dy|{
            let x=(c.0+dx*(xp.0-c.0)+dy*(yp.0-c.0)).round() as i32;
            let y=(c.1+dx*(xp.1-c.1)+dy*(yp.1-c.1)).round() as i32;
            if x<0||y<0||x>=f.width as i32||y>=f.height as i32||x<f.region.left||y<f.region.top||x>=f.region.right||y>=f.region.bottom{return None;}
            let i=(y as usize*f.width+x as usize)*4;
            Some([f.pixels[i],f.pixels[i+1],f.pixels[i+2],f.pixels[i+3]])
        }))
    })
}
#[cfg(test)] mod tests{
    use super::*;
    #[test]fn circle_center_and_target_preserve_precise_sampling(){
        let p=raster(|x,y|Some([255,(100.+x) as u8,(100.+y) as u8,20]));
        let at=|x:usize,y:usize|&p[(y*SIZE+x)*4..(y*SIZE+x+1)*4];
        assert_eq!(at(64,64),[255,100,100,20]);assert_eq!(at(0,0),[0,0,0,0]);
        assert_eq!(at(64,70),[255,255,255,255]);assert_eq!(at(63,63),[255,99,99,20]);
    }
    #[test]fn native_point_observation_distinguishes_one_corner_from_plane_motion(){
        let a=[0.,0.,10.,0.,10.,10.,0.,10.];let mut b=a;
        assert_eq!(changed_corner(&a,&b),None);b[0]=1.;b[1]=2.;assert_eq!(changed_corner(&a,&b),Some(0));
        b[2]=11.;assert_eq!(changed_corner(&a,&b),None);
    }
    #[test]fn transparent_pixels_use_checker_and_no_frame_is_retained(){
        assert_eq!(raster(|_,_|Some([0,0,0,0])),raster(|_,_|None));
        ACTIVE.set(Some(Gesture{owner:123,window:1,index:2,native:false}));clear();assert!(ACTIVE.get().is_none());
    }
}
