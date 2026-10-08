//! UI-only corner magnifier. AE owns asynchronous requests and cancels stale frames.
//! No render callback, saved parameter, OS capture, or retained host pixel pointer.
use super::*;
const SIZE:usize=129;
const RADIUS:f32=64.0;
const ZOOM:f32=3.0;
const PURPOSE:u32=0x4653544c;
#[cfg(feature="loupe-image-probe")]
thread_local!{static IMAGE_RECORDED:std::cell::Cell<bool>=const{std::cell::Cell::new(false)};}
#[derive(Clone,Copy)]
struct Gesture {owner:i32,window:i32,index:usize,native:bool}
#[derive(Clone,Copy)]
struct Observation {owner:i32,window:i32,time:i32,scale:u32,corners:[f64;8]}
#[derive(Clone,Copy)]
struct Hover {owner:i32,window:i32,time:i32,scale:u32,index:usize,pointer:(f64,f64)}
thread_local! {static HOVER:std::cell::Cell<Option<Hover>>=const{std::cell::Cell::new(None)};}
thread_local! {static PREVIOUS:std::cell::Cell<Option<Observation>>=const{std::cell::Cell::new(None)};}
thread_local! {static ACTIVE:std::cell::Cell<Option<Gesture>>=const{std::cell::Cell::new(None)};}
pub(crate) fn clear(){
    #[cfg(feature="interactive-quality-probe")]super::interactive_quality_probe::end("clear");
    #[cfg(feature="preview-overlay-probe")]super::preview_overlay_probe::transition(0,probe_state());
    ACTIVE.set(None);PREVIOUS.set(None);HOVER.set(None);GESTURE_FRAME.set(None);
}
#[cfg(feature="preview-overlay-probe")]
pub(crate) fn probe_state()->isize{
    isize::from(native_button_down())|((ACTIVE.get().is_some() as isize)<<1)|
        ((HOVER.get().is_some() as isize)<<2)|((GESTURE_FRAME.get().is_some() as isize)<<3)|
        ((GESTURE_FRAME.get().is_some_and(|r|r.finished) as isize)<<4)
}
fn finish_native(button_down:bool){
    if !button_down&&ACTIVE.get().is_some_and(|g|g.native){
        #[cfg(feature="interactive-quality-probe")]super::interactive_quality_probe::end("release");
        #[cfg(feature="preview-overlay-probe")]super::preview_overlay_probe::transition(1,probe_state());
        // A stationary release does not imply leaving the hit-tested corner.
        // Preserve its scoped pointer anchor so a second press needs no motion.
        ACTIVE.set(None);PREVIOUS.set(None);GESTURE_FRAME.set(None);
    }
}
fn start_native(g:Gesture){
    #[cfg(feature="interactive-quality-probe")]super::interactive_quality_probe::begin(g.owner,g.window,g.index);
    #[cfg(feature="preview-overlay-probe")]super::preview_overlay_probe::transition(2,probe_state()|((g.index as isize)<<8));
    if !ACTIVE.get().is_some_and(|old|old.owner==g.owner&&old.index==g.index){
        #[cfg(feature="loupe-image-probe")]IMAGE_RECORDED.set(false);
        GESTURE_FRAME.set(None);FRAME.with_borrow_mut(|frame|*frame=None);
    }
    ACTIVE.set(Some(g));
}
fn native_pointer()->Option<(f64,f64)>{
    #[cfg(any(target_os="macos",target_os="windows"))]{
        unsafe extern "C"{fn eg_loupe_pointer(x:*mut f64,y:*mut f64)->bool;}
        let (mut x,mut y)=(0.,0.);
        if unsafe{eg_loupe_pointer(&mut x,&mut y)}&&x.is_finite()&&y.is_finite(){return Some((x,y));}
    }
    None
}
pub(crate) fn hover(input:&ae::InData,event:&ae::EventExtra,index:Option<usize>,id:Option<ae::aegp::PluginId>)->bool{
    #[cfg(feature="preview-overlay-probe")]super::preview_overlay_probe::transition(3,probe_state()|((index.map(|v|v as isize+1).unwrap_or(0))<<8));
    // A press can request one viewer update without writing any Point value.
    // Never arm from a drag, or read the DRAW union as mouse coordinates.
    if native_button_down(){
        if ACTIVE.get().is_some(){return false;}
        if let (Ok(owner),Some(pointer))=(owner(input,id),native_pointer()){
            let now=Observation{owner,window:ui::event_window_code(event),time:input.current_time(),
                scale:input.time_scale(),corners:[0.;8]};
            if start_hover(now,pointer){return true;}
        }
        return false;
    }
    finish_native(false);
    HOVER.set(index.filter(|&i|i<4).and_then(|index|Some(Hover{owner:owner(input,id).ok()?,
        window:ui::event_window_code(event),time:input.current_time(),scale:input.time_scale(),
        index,pointer:native_pointer()?})));
    false
}
fn start_hover(now:Observation,pointer:(f64,f64))->bool{
    #[cfg(feature="preview-overlay-probe")]{
        let mask=HOVER.get().map(|h|1|((h.owner==now.owner) as isize)<<1|
            ((h.window==now.window) as isize)<<2|((h.time==now.time) as isize)<<3|
            ((h.scale==now.scale) as isize)<<4|((pressed_hover(h,now,pointer).is_some()) as isize)<<5).unwrap_or(0)
            |((ACTIVE.get().is_some() as isize)<<6);
        super::preview_overlay_probe::transition(4,mask);
    }
    if ACTIVE.get().is_some(){return false;}
    let Some(index)=HOVER.get().and_then(|h|pressed_hover(h,now,pointer))else{return false;};
    start_native(Gesture{owner:now.owner,window:now.window,index,native:true});
    true
}
fn track_hover(now:Observation,index:usize,pointer:(f64,f64)){
    if ACTIVE.get().is_some_and(|g|g.native&&g.owner==now.owner&&g.index==index&&
        (g.window==0||g.window==now.window)){
        HOVER.set(Some(Hover{owner:now.owner,window:now.window,time:now.time,scale:now.scale,index,pointer}));
    }
}
fn pressed_hover(h:Hover,now:Observation,pointer:(f64,f64))->Option<usize>{
    (h.owner==now.owner&&h.window==now.window&&h.time==now.time&&h.scale==now.scale&&
        (h.pointer.0-pointer.0).abs()<=2.0&&(h.pointer.1-pointer.1).abs()<=2.0).then_some(h.index)
}
// PF effect_ref changes across AE native Point preview callbacks. Retain only
// the documented stream ID, disposing every acquired handle in this callback.
pub(crate) fn owner(input:&ae::InData,id:Option<ae::aegp::PluginId>)->Result<i32,ae::Error>{
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
    if let Ok(owner)=owner(input,id){if index<4 {
        #[cfg(feature="loupe-image-probe")]IMAGE_RECORDED.set(false);
        // The previous frame can have the same time/owner but older deformation.
        // Do not display it while this gesture's async image is pending.
        FRAME.with_borrow_mut(|frame|*frame=None);
        GESTURE_FRAME.set(None);ACTIVE.set(Some(Gesture{owner,
        window:ui::event_window_code(event),index,native:false}));}}
    #[cfg(feature="interactive-quality-probe")]
    if let Some(g)=ACTIVE.get(){super::interactive_quality_probe::begin(g.owner,g.window,g.index);}
}
pub(crate) fn native_button_down()->bool{
    #[cfg(any(target_os="macos",target_os="windows"))]{
        unsafe extern "C"{fn eg_loupe_button_down()->bool;}
        // Read-only native UI state. Never called from a render worker.
        unsafe{eg_loupe_button_down()}
    }
    #[cfg(not(any(target_os="macos",target_os="windows")))]{false}
}
pub(crate) fn native_change(input:&ae::InData,index:usize,id:Option<ae::aegp::PluginId>){
    if index<4&&native_button_down(){if let Ok(owner)=owner(input,id){
        start_native(Gesture{owner,window:0,index,native:true});
    }}
}
fn changed_corner(a:&[f64;8],b:&[f64;8])->Option<usize>{
    let mut found=None;
    for i in 0..4 {if a[2*i]!=b[2*i]||a[2*i+1]!=b[2*i+1]{
        if found.is_some(){return None;}found=Some(i);
    }}found
}
pub(crate) fn observe(input:&ae::InData,event:&ae::EventExtra,corners:[f64;8],id:Option<ae::aegp::PluginId>){
    finish_native(native_button_down());
    let Ok(owner)=owner(input,id) else{clear();return;};
    let now=Observation{owner,window:ui::event_window_code(event),
        time:input.current_time(),scale:input.time_scale(),corners};
    if ACTIVE.get().is_none()&&native_button_down(){
        if let Some(pointer)=native_pointer(){start_hover(now,pointer);}
    }
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
                if let Some(pointer)=native_pointer(){track_hover(now,index,pointer);}
            }
        }
    }
    PREVIOUS.set(Some(now));
}

pub(crate) fn active(input:&ae::InData,event:&ae::EventExtra,id:Option<ae::aegp::PluginId>)->Option<usize>{
    let g=ACTIVE.get()?;
    if g.native&&!native_button_down(){finish_native(false);return None;}
    (g.owner==owner(input,id).ok()?&&(g.window==0||g.window==ui::event_window_code(event))).then_some(g.index)
}
#[cfg(feature="deferred-corner-probe")]
pub(crate) fn policy_context(input:&ae::InData,event:&ae::EventExtra,id:Option<ae::aegp::PluginId>)
    ->Option<(i32,bool)> {
    // UI only, scoped by the same stable stream ID as the existing loupe.
    let scoped_owner=owner(input,id).ok()?;
    let active=ACTIVE.get().is_some_and(|g|g.owner==scoped_owner&&
        (g.window==0||g.window==ui::event_window_code(event)))&&native_button_down();
    Some((scoped_owner,active))
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
    surface:&ae::drawbot::Surface,center:ae::drawbot::PointF32,id:Option<ae::aegp::PluginId>,
    #[cfg(feature="loupe-upstream-probe")] coordinates:Result<[(f32,f32);3],ae::Error>,
)->Result<(),ae::Error>{
    source(event,center.x,center.y)?;
    let mut pixels=frame_pixels(input,event,center,id,
        #[cfg(feature="loupe-upstream-probe")] coordinates?,
    )?;
    #[cfg(feature="loupe-image-probe")]{
        let mut lo=255u8;let mut hi=0u8;let mut count=0usize;
        for y in 40..89 {for x in 40..89 {
            let dx=x as i32-64;let dy=y as i32-64;
            if dx.abs()<=3||dy.abs()<=3 {continue;}
            let i=(y*SIZE+x)*4;
            for value in &pixels[i+1..i+4]{lo=lo.min(*value);hi=hi.max(*value);}
            count+=1;
        }}
        super::loupe_image_probe::record(3,&[lo as f64,hi as f64,count as f64,
            supplier.supports_pixel_layout_argb()? as u8 as f64,
            supplier.prefers_pixel_layout_argb()? as u8 as f64,
            supplier.supports_pixel_layout_bgra()? as u8 as f64,
            supplier.prefers_pixel_layout_bgra()? as u8 as f64]);
    }
    #[cfg(feature="resource-census-probe")]
    let _bitmap_census=super::resource_census::Token::new(super::resource_census::Kind::UiBitmap,pixels.len() as u64);
    // SDK DrawbotSuite.h requires consulting the supplier preference. Source
    // pixels remain ARGB; convert only the small UI bitmap, never a host world.
    let bgra=supplier.supports_pixel_layout_bgra()?&&
        (supplier.prefers_pixel_layout_bgra()?||!supplier.supports_pixel_layout_argb()?);
    let layout=image_layout(&mut pixels,bgra);
    let image=supplier.new_image_from_buffer(SIZE,SIZE,SIZE*4,layout,&pixels)?;
    #[cfg(feature="resource-census-probe")]
    let _image_census=super::resource_census::Token::new(super::resource_census::Kind::DrawImage,(SIZE*SIZE*4) as u64);
    surface.draw_image(&image,&ae::drawbot::PointF32{x:center.x-RADIUS,y:center.y-RADIUS},1.0)
}
fn image_layout(pixels:&mut [u8],bgra:bool)->ae::drawbot::PixelLayout{
    if bgra {
        for pixel in pixels.chunks_exact_mut(4){pixel.reverse();}
        ae::drawbot::PixelLayout::Bgra32Straight
    }else{ae::drawbot::PixelLayout::Argb32Straight}
}
#[derive(Clone,Copy,PartialEq,Eq)]
struct FrameKey{owner:i32,window:i32,time:i32,scale:u32,stamp:[i8;4]}
// One immutable request per mouse gesture. Window refreshes and host timestamps
// cannot rearm completed work; only a new press creates another request.
#[derive(Clone,Copy)]
struct GestureFrame {key:FrameKey,finished:bool}
fn gesture_request(state:&mut Option<GestureFrame>,proposed:FrameKey)->Option<FrameKey>{
    let r=state.get_or_insert(GestureFrame{key:proposed,finished:false});
    (!r.finished).then_some(r.key)
}
fn gesture_finished(){GESTURE_FRAME.set(GESTURE_FRAME.get().map(|r|GestureFrame{finished:true,..r}));}
struct Frame{
    #[cfg(feature="resource-census-probe")]
    _census:Option<super::resource_census::Token>,
    key:FrameKey,width:usize,height:usize,region:ae::Rect,pixels:Vec<u8>}
// Bounded owner history; switching between multiple effect rows must not
// rearm a warm request at every playback time. Full history fails closed for
// idle preparation while active corner gestures remain available.
const WARM_OWNERS:usize=32;
type WarmRequests=[Option<(i32,i32,u32,bool)>;WARM_OWNERS];
const EMPTY_WARM:WarmRequests=[None;WARM_OWNERS];
thread_local!{
    static GESTURE_FRAME:std::cell::Cell<Option<GestureFrame>>=const{std::cell::Cell::new(None)};
    static FRAME:std::cell::RefCell<Option<Frame>>=const{std::cell::RefCell::new(None)};
    static VIEW:std::cell::Cell<Option<(i32,i32)>>=const{std::cell::Cell::new(None)};
    static WARM:std::cell::Cell<WarmRequests>=const{std::cell::Cell::new(EMPTY_WARM)};
}
pub(crate) fn close(){clear();FRAME.with_borrow_mut(|f|*f=None);VIEW.set(None);WARM.set(EMPTY_WARM);}
// Permit one initial preview frame per owner, then require a corner gesture.
// An unfinished warm request can be polled at that same time; advancing the
// timeline must not replace it with requests for every playback frame.
fn request_allowed(warm:&mut WarmRequests,owner:i32,time:i32,scale:u32,
                   gesture:bool,cached_owner:bool)->bool {
    if gesture { return true; }
    if cached_owner { return false; }
    if let Some(key)=warm.iter().flatten().find(|key|key.0==owner) {
        return !key.3 && key.1==time && key.2==scale;
    }
    let Some(slot)=warm.iter_mut().find(|slot|slot.is_none()) else{return false;};
    *slot=Some((owner,time,scale,false));true
}
fn warm_completed(warm:&mut WarmRequests,owner:i32){
    if let Some(key)=warm.iter_mut().flatten().find(|key|key.0==owner){key.3=true;}
}
// SDK25.6 AE_EffectUI.h:419 explicitly limits reserved_job_manageP to
// Effect pane custom UI. Never ask a Comp/Layer context for an AsyncManager.
pub(crate) fn prepare_frame(input:&ae::InData,event:&mut ae::EventExtra,id:Option<ae::aegp::PluginId>)->Result<(),ae::Error>{
    finish_native(native_button_down());
    let result=prepare_frame_inner(input,event,id);
    // Do not turn a failed checkout/copy/refresh into a per-DRAW retry loop.
    if result.is_err() {
        #[cfg(feature="resource-census-probe")]
        super::resource_census::preparation_error();
        if let Ok(owner)=owner(input,id) {
            if GESTURE_FRAME.get().is_some_and(|r|owner==r.key.owner){gesture_finished();}
            // A failed initial request is terminal too; do not repeat a host
            // alert on every Effect Controls DRAW at the same time.
            let mut warm=WARM.get();warm_completed(&mut warm,owner);WARM.set(warm);
        }
    }
    result
}
fn prepare_frame_inner(input:&ae::InData,event:&mut ae::EventExtra,id:Option<ae::aegp::PluginId>)->Result<(),ae::Error>{
    // Diagnostic156 isolates all secondary loupe renders (including warm work)
    // from the unchanged effect renderer. Never enabled in ordinary candidates.
    if cfg!(feature="loupe-source-disabled-probe"){return Ok(());}
    if event.window_type()!=ae::WindowType::Effect{return Ok(());}
    let owner=owner(input,id)?;
    let gesture=ACTIVE.get().is_some_and(|g|g.owner==owner)&&native_button_down();
    let cached_owner=FRAME.with_borrow(|frame|frame.as_ref().is_some_and(|f|f.key.owner==owner));
    let mut warm=WARM.get();
    let allowed=request_allowed(&mut warm,owner,input.current_time(),input.time_scale(),gesture,cached_owner);
    WARM.set(warm);
    #[cfg(feature="preview-overlay-probe")]super::preview_overlay_probe::transition(5,probe_state()|((allowed as isize)<<8));
    if !allowed { return Ok(()); }
    let id=id.ok_or(ae::Error::BadCallbackParameter)?;
    let window=VIEW.get().filter(|v|v.0==owner).map(|v|v.1).unwrap_or(ae::sys::PF_Window_COMP);
    let render=ae::aegp::suites::Render::new()?;
    if gesture&&GESTURE_FRAME.get().is_some_and(|r|r.finished){return Ok(());}
    let proposed=FrameKey{owner,window,time:input.current_time(),scale:input.time_scale(),stamp:render.current_timestamp()?.a};
    let key=if gesture {
        let mut state=GESTURE_FRAME.get();let key=gesture_request(&mut state,proposed);
        GESTURE_FRAME.set(state);let Some(key)=key else{return Ok(());};key
    }else{proposed};
    if FRAME.with_borrow(|f|f.as_ref().is_some_and(|f|f.key==key)){
        if gesture{gesture_finished();}return Ok(());
    }
    let interface=ae::aegp::suites::PFInterface::new()?;
    let layers=ae::aegp::suites::Layer::new()?;
    let layer=interface.effect_layer(input.effect_ref())?;
    let time=interface.convert_effect_to_comp_time(input.effect_ref(),key.time,key.scale)?;
    let manager=ae::pf::suites::EffectCustomUI::new()?.context_async_manager(input.as_ptr(),*event)?;
    #[cfg(feature="resource-census-probe")]
    super::resource_census::request(gesture);
    let receipt=if cfg!(feature="loupe-upstream-probe") {
        // SDK LayerRenderOptionsSuite2: own layer before this effect. This is a
        // bounded diagnostic prerequisite, NOT the full-composition solution.
        let effects=ae::aegp::suites::Effect::new()?;
        let effect=interface.new_effect_for_effect(input.effect_ref(),id)?;
        let options=(|| {
            let suite=ae::aegp::suites::LayerRenderOptions::new()?;
            Ok::<_,ae::Error>(ae::aegp::LayerRenderOptions::from_handle(
                suite.new_from_upstream_of_effect(effect,id)?,false))
        })();
        let disposed=effects.dispose_effect(effect);
        let options=match (options,disposed) {
            (Ok(options),Ok(()))=>options,
            (Err(e),_)|(_,Err(e))=>return Err(e),
        };
        options.set_time(time)?;options.set_world_type(ae::aegp::WorldType::U8)?;
        options.set_downsample_factor(1,1)?;
        options.set_matte_mode(ae::aegp::MatteMode::Straight)?;
        manager.checkout_or_render_layer_frame_async_manager(PURPOSE,options.handle())?
    }else if window==ae::sys::PF_Window_COMP{
        // Product L3: the evaluated composition, including FSTR and visible
        // background layers. Capture once; canonical corner values stay frozen
        // until mouse-up. Never substitute the diagnostic upstream-only layer.
        let comp=layers.layer_parent_comp(layer)?;
        let item=ae::aegp::suites::Comp::new()?.item_from_comp(comp)?;
        let options=ae::aegp::RenderOptions::from_item(item,id)?;
        options.set_time(time)?;options.set_world_type(ae::aegp::WorldType::U8)?;
        options.set_downsample_factor(1,1)?;
        options.set_channel_order(ae::aegp::ChannelOrder::Argb)?;
        options.set_matte_mode(ae::aegp::MatteMode::Straight)?;
        manager.checkout_or_render_item_frame_async_manager(PURPOSE,options.handle())?
    }else{
        let options=ae::aegp::LayerRenderOptions::from_layer(layer,id)?;
        options.set_time(time)?;options.set_world_type(ae::aegp::WorldType::U8)?;
        options.set_downsample_factor(1,1)?;
        options.set_matte_mode(ae::aegp::MatteMode::Straight)?;
        manager.checkout_or_render_layer_frame_async_manager(PURPOSE,options.handle())?
    };
    #[cfg(feature="resource-census-probe")]
    super::resource_census::receipt(!receipt.is_null());
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
        // Every request explicitly uses straight alpha. Normalize only our
        // owned copy for the existing checkerboard compositing, never host data.
        premultiply_straight(&mut pixels);
        Ok(Frame{
            #[cfg(feature="resource-census-probe")]
            _census:Some(super::resource_census::Token::new(super::resource_census::Kind::Loupe,size as u64)),
            key,width:w,height:h,region,pixels})
    })();
    let checked=render.checkin_frame(receipt);
    #[cfg(feature="resource-census-probe")]
    super::resource_census::checkin(true,checked.is_ok());
    // A terminal receipt finishes both warm and gesture work, including errors.
    if gesture{gesture_finished();}
    let mut warm=WARM.get();warm_completed(&mut warm,owner);WARM.set(warm);
    match (result,checked){(Ok(frame),Ok(()))=>{
        #[cfg(feature="loupe-image-probe")]{
            let (min,max)=frame.pixels.chunks_exact(4).fold((255u8,0u8),|(lo,hi),p|
                (lo.min(p[1]).min(p[2]).min(p[3]),hi.max(p[1]).max(p[2]).max(p[3])));
            super::loupe_image_probe::record(1,&[frame.width as f64,frame.height as f64,
                frame.region.left as f64,frame.region.top as f64,frame.region.right as f64,frame.region.bottom as f64,
                min as f64,max as f64,window as f64]);
        }
        FRAME.with_borrow_mut(|f|*f=Some(frame));
        // Background warm completion must not trigger a global redraw/render loop.
        if gesture { ae::pf::suites::AdvApp::new()?.refresh_all_windows()?; }
        Ok(())
    },(Err(e),_)|(_,Err(e))=>Err(e)}
}
fn premultiply_straight(pixels:&mut [u8]) {
    for p in pixels.chunks_exact_mut(4) {
        let alpha=u16::from(p[0]);
        for c in &mut p[1..] {*c=((u16::from(*c)*alpha+127)/255) as u8;}
    }
}

fn frame_pixels(input:&ae::InData,event:&mut ae::EventExtra,center:ae::drawbot::PointF32,id:Option<ae::aegp::PluginId>,
    #[cfg(feature="loupe-upstream-probe")] coordinates:[(f32,f32);3],
)->Result<Vec<u8>,ae::Error>{
    let owner=owner(input,id)?;let window=ui::event_window_code(event);VIEW.set(Some((owner,window)));
    #[cfg(feature="loupe-upstream-probe")]let [c,xp,yp]=coordinates;
    #[cfg(not(feature="loupe-upstream-probe"))]
    let (c,xp,yp)=(source(event,center.x,center.y)?,
        source(event,center.x+1.0,center.y)?,source(event,center.x,center.y+1.0)?);
    FRAME.with_borrow(|frame|{
        let f=frame.as_ref().filter(|f|f.key.owner==owner&&f.key.window==window&&f.key.time==input.current_time()&&f.key.scale==input.time_scale()).ok_or(ae::Error::BadCallbackParameter)?;
        #[cfg(feature="loupe-image-probe")]let mut valid=0usize;
        #[cfg(feature="loupe-image-probe")]let (mut sample_min,mut sample_max)=(255u8,0u8);
        let pixels=raster(|dx,dy|{
            let x=(c.0+dx*(xp.0-c.0)+dy*(yp.0-c.0)).round() as i32;
            let y=(c.1+dx*(xp.1-c.1)+dy*(yp.1-c.1)).round() as i32;
            if x<0||y<0||x>=f.width as i32||y>=f.height as i32||x<f.region.left||y<f.region.top||x>=f.region.right||y>=f.region.bottom{return None;}
            let i=(y as usize*f.width+x as usize)*4;
            #[cfg(feature="loupe-image-probe")]{valid+=1;
                for value in &f.pixels[i+1..i+4]{sample_min=sample_min.min(*value);sample_max=sample_max.max(*value);}
            }
            Some([f.pixels[i],f.pixels[i+1],f.pixels[i+2],f.pixels[i+3]])
        });
        #[cfg(feature="loupe-image-probe")]
        if !IMAGE_RECORDED.replace(true){
            super::loupe_image_probe::record(2,&[center.x as f64,center.y as f64,c.0 as f64,c.1 as f64,
                xp.0 as f64,xp.1 as f64,yp.0 as f64,yp.1 as f64,valid as f64,
                f.width as f64,f.height as f64,sample_min as f64,sample_max as f64]);
        }
        Ok(pixels)
    })
}
#[cfg(test)] mod tests{
    use super::*;
    fn request_key(stamp:i8)->FrameKey{FrameKey{owner:7,window:1,time:10,scale:25,stamp:[stamp;4]}}
    #[test]fn same_time_new_press_drops_previous_pixels_but_held_callbacks_keep_current_frame(){
        close();let g=Gesture{owner:7,window:1,index:0,native:true};
        start_native(g);
        FRAME.with_borrow_mut(|f|*f=Some(Frame{
            #[cfg(feature="resource-census-probe")]_census:None,
            key:request_key(0),width:1,height:1,
            region:ae::Rect{left:0,top:0,right:1,bottom:1},pixels:vec![255,99,0,0]}));
        start_native(g);assert!(FRAME.with_borrow(|f|f.is_some()));
        finish_native(false);start_native(g);
        assert!(FRAME.with_borrow(|f|f.is_none()));close();
    }
    #[test]fn held_press_cannot_requeue_after_completion_or_host_timestamp_changes(){
        let first=request_key(0);let mut state=None;
        assert!(gesture_request(&mut state,first)==Some(first));
        for stamp in 1..100{assert!(gesture_request(&mut state,request_key(stamp))==Some(first));}
        state.as_mut().unwrap().finished=true;
        for stamp in 0..100{assert!(gesture_request(&mut state,request_key(stamp)).is_none());}
    }
    #[test]fn release_and_same_corner_second_press_start_independent_requests(){
        clear();start_native(Gesture{owner:7,window:1,index:2,native:true});
        GESTURE_FRAME.set(Some(GestureFrame{key:request_key(0),finished:true}));
        start_native(Gesture{owner:7,window:0,index:2,native:true});
        assert!(GESTURE_FRAME.get().unwrap().finished); // supervision cannot rearm
        finish_native(false);assert!(ACTIVE.get().is_none());assert!(GESTURE_FRAME.get().is_none());
        start_native(Gesture{owner:7,window:1,index:2,native:true});
        let mut state=GESTURE_FRAME.get();assert!(gesture_request(&mut state,request_key(1)).is_some());
        clear();
    }
    #[test]fn failed_or_terminal_gesture_work_cannot_restart_until_release(){
        clear();GESTURE_FRAME.set(Some(GestureFrame{key:request_key(0),finished:false}));
        gesture_finished();let mut state=GESTURE_FRAME.get();
        assert!(gesture_request(&mut state,request_key(1)).is_none());
        clear();assert!(GESTURE_FRAME.get().is_none());
    }
    #[test]fn multiple_effects_cannot_rearm_warm_requests_on_each_playback_frame(){
        let mut warm=EMPTY_WARM;
        assert!(request_allowed(&mut warm,7,0,25,false,false));
        assert!(request_allowed(&mut warm,8,0,25,false,false));
        for time in 1..10000 {
            for owner in [7,8] {assert!(!request_allowed(&mut warm,owner,time,25,false,false));}
        }
    }
    #[test]fn warm_history_is_bounded_and_completed_owners_cannot_requeue(){
        let mut warm=EMPTY_WARM;
        for owner in 0..WARM_OWNERS as i32 {assert!(request_allowed(&mut warm,owner,0,25,false,false));}
        assert!(!request_allowed(&mut warm,100,0,25,false,false));
        assert!(request_allowed(&mut warm,100,9,25,true,false));
        warm_completed(&mut warm,0);
        assert!(!request_allowed(&mut warm,0,0,25,false,false));
        assert!(request_allowed(&mut warm,0,0,25,true,false));
        assert!(request_allowed(&mut warm,1,0,25,false,false));
    }
    #[test]fn playback_does_not_queue_loupe_frames_after_initial_warm(){
        let mut warm=EMPTY_WARM;
        assert!(request_allowed(&mut warm,7,0,25,false,false));
        assert!(request_allowed(&mut warm,7,0,25,false,false));
        for time in 1..10000 {assert!(!request_allowed(&mut warm,7,time,25,false,false));}
        for time in 0..10000 {assert!(!request_allowed(&mut warm,7,time,25,false,true));}
        assert!(request_allowed(&mut warm,7,91,25,true,true));
        assert!(!request_allowed(&mut warm,7,92,25,false,true));
        assert!(request_allowed(&mut warm,8,92,25,false,false));
        assert!(!request_allowed(&mut warm,8,93,25,false,false));
    }
    #[test]fn straight_frame_alpha_is_normalized_before_lens_compositing(){
        let mut pixels=[0,255,128,64,128,255,128,64,255,17,93,201];
        premultiply_straight(&mut pixels);
        assert_eq!(pixels,[0,0,0,0,128,128,64,32,255,17,93,201]);
        let premul=[pixels[4],pixels[5],pixels[6],pixels[7]];
        let bitmap=raster(|_,_|Some(premul));
        let i=(70*SIZE+70)*4;
        // Checker at (70,70) is64; alpha128 contributes31 of background.
        assert_eq!(&bitmap[i..i+4],&[255,159,95,63]);
    }
    #[test]fn supplier_bgra_conversion_preserves_rgb_alpha_and_transparent_exterior(){
        let argb=raster(|_,_|Some([255,17,93,201]));
        let mut bgra=argb.clone();
        assert!(matches!(image_layout(&mut bgra,true),ae::drawbot::PixelLayout::Bgra32Straight));
        for (a,b) in argb.chunks_exact(4).zip(bgra.chunks_exact(4)){
            assert_eq!([a[0],a[1],a[2],a[3]],[b[3],b[2],b[1],b[0]]);
        }
        let mut unchanged=argb.clone();
        assert!(matches!(image_layout(&mut unchanged,false),ae::drawbot::PixelLayout::Argb32Straight));
        assert_eq!(argb,unchanged);
    }
    #[test]fn circle_center_and_target_preserve_precise_sampling(){
        let p=raster(|x,y|Some([255,(100.+x) as u8,(100.+y) as u8,20]));
        let at=|x:usize,y:usize|&p[(y*SIZE+x)*4..(y*SIZE+x+1)*4];
        assert_eq!(at(64,64),[255,100,100,20]);assert_eq!(at(0,0),[0,0,0,0]);
        assert_eq!(at(64,70),[255,255,255,255]);assert_eq!(at(63,63),[255,99,99,20]);
    }
    #[test]fn moved_native_corner_retains_its_new_pointer_anchor_after_release(){
        clear();let pointer=(148.,460.);
        let now=Observation{owner:7,window:1,time:10,scale:25,corners:[0.;8]};
        start_native(Gesture{owner:7,window:0,index:2,native:true});
        track_hover(now,1,(99.,99.));assert!(HOVER.get().is_none());
        track_hover(now,2,pointer);finish_native(false);
        assert!(!start_hover(now,(123.,456.)));assert!(start_hover(now,pointer));
        clear();assert!(!start_hover(now,pointer));
    }
    #[test]fn stationary_repeated_press_needs_no_new_hover_or_point_motion(){
        clear();let pointer=(123.,456.);
        let h=Hover{owner:7,window:1,time:10,scale:25,index:2,pointer};
        let now=Observation{owner:7,window:1,time:10,scale:25,corners:[0.;8]};
        HOVER.set(Some(h));
        for _ in 0..3{
            assert!(start_hover(now,pointer));
            assert!(!start_hover(now,pointer)); // a held press does not start twice
            GESTURE_FRAME.set(Some(GestureFrame{key:request_key(0),finished:true}));
            finish_native(false);
            assert!(ACTIVE.get().is_none());assert!(GESTURE_FRAME.get().is_none());
        }
        clear();assert!(!start_hover(now,pointer));
    }
    #[test]fn stationary_corner_press_is_qualified_without_parameter_motion(){
        let h=Hover{owner:7,window:1,time:10,scale:25,index:2,pointer:(123.,456.)};
        let n=Observation{owner:7,window:1,time:10,scale:25,corners:[0.;8]};
        assert_eq!(pressed_hover(h,n,(123.,456.)),Some(2));
        assert_eq!(pressed_hover(h,n,(125.,454.)),Some(2));
        assert_eq!(pressed_hover(h,n,(126.,456.)),None);
        assert_eq!(pressed_hover(h,Observation{owner:8,..n},h.pointer),None);
        assert_eq!(pressed_hover(h,Observation{window:2,..n},h.pointer),None);
        assert_eq!(pressed_hover(h,Observation{time:11,..n},h.pointer),None);
        assert_eq!(pressed_hover(h,Observation{scale:50,..n},h.pointer),None);
        HOVER.set(Some(h));clear();assert!(HOVER.get().is_none());
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
