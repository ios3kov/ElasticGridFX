//! Automatic native-plane dependencies in dedicated append-only hidden streams.
use super::*;
use binding_transaction::{Host, Snapshot, Outcome};
const NAMES: [&str;5] = ["__FSTR Probe TL", "__FSTR Probe TR", "__FSTR Probe BR", "__FSTR Probe BL", "__FSTR Plane Kind"];

pub fn layer_is_3d(params:&ae::Parameters<Params>,checkout:bool)->Result<bool,ae::Error>{
    let kind=if checkout {checked_float(params,Params::ResearchPlaneKind)?}
        else {params.get(Params::ResearchPlaneKind)?.as_float_slider()?.value()};
    match kind {0.0|1.0=>Ok(false),2.0|3.0=>Ok(true),_=>Err(ae::Error::BadCallbackParameter)}
}

// Shared UI/render snapshot. No AEGP calls: all dependencies are PF parameters.
// Native 3D text is distinct from raster footage's layer-local effect input.
pub fn sampled_plane(in_data:&ae::InData,params:&ae::Parameters<Params>,checkout:bool,frame_context:bool)
    ->Result<Option<plane::State>,ae::Error>{
    let kind=if checkout {checked_float(params,Params::ResearchPlaneKind)?}
        else {params.get(Params::ResearchPlaneKind)?.as_float_slider()?.value()};
    // 0 pending; 1 2D layer-local; 2 3D text comp-space; 3 3D raster layer-local.
    // The marker is installed last and checked out with the four points.
    // AE may request a neutral first frame before the deferred binding runs.
    // Only an exact initial identity can render without knowing the layer plane.
    // Do not turn an unknown/deformed plane into a successful legacy render.
    let initial_identity = kind == 0.0 && frame_context && cfg!(fstr_auto_binding)
        && pending_frame_identity(params, checkout)?;
    if !resolve_comp_space_kind(kind,frame_context,cfg!(fstr_auto_binding),initial_identity)? {
        return Ok(None);
    }
    let par=in_data.pixel_aspect_ratio();
    if par.num<=0 || i64::from(par.num)!=i64::from(par.den) {
        return Err(ae::Error::BadCallbackParameter);
    }
    let mut corners=[0.0;8];
    let origin=if frame_context {in_data.pre_effect_source_origin()} else {ae::Point {h:0,v:0}};
    for (i,id) in [Params::ResearchPlaneTL,Params::ResearchPlaneTR,
        Params::ResearchPlaneBR,Params::ResearchPlaneBL].into_iter().enumerate(){
        let p=if checkout {params.checkout(id)?.as_point()?.float_value()?}
            else {params.get(id)?.as_point()?.float_value()?};
        corners[2*i]=p.x-origin.h as f64;corners[2*i+1]=p.y-origin.v as f64;
    }
    // Degenerate geometry follows the core's exact pass-through contract.
    // No public corner controls are exposed for an automatically derived plane.
    Ok(Some(plane::State {corners:Some(corners),editable_corners:false,comp_space:true,parameter_basis:None}))
}

// Pending initialization may hide the UI plane, but must not silently use the
// legacy screen-space renderer. Noninteractive callers receive an SDK error;
// no UI, AEGP calls or writes are performed here. Legacy research stays opt-in.
fn comp_space_kind(kind:f64,frame_context:bool,automatic:bool)->Result<bool,ae::Error>{
    match kind {
        0.0 if automatic && frame_context=>Err(ae::Error::BadCallbackParameter),
        0.0|1.0|3.0=>Ok(false),
        2.0=>Ok(true),
        _=>Err(ae::Error::BadCallbackParameter),
    }
}

// Keep the original rejecting policy for every unproven pending frame. The
// exception is a mathematically identical image, not a "ready" binding state.
fn resolve_comp_space_kind(kind:f64,frame_context:bool,automatic:bool,initial_identity:bool)
    ->Result<bool,ae::Error>{
    if kind == 0.0 && frame_context && automatic && initial_identity {
        return Ok(false);
    }
    comp_space_kind(kind,frame_context,automatic)
}

fn initial_identity_values(grid:&GridArb,topology:(i32,i32),mode:i32,
                           wave:f64,easing:f64,min_spacing:f64)->bool {
    // Deliberately narrower than every possible identity grid. No approximate
    // equality, resizes, sanitization, or fallback from malformed saved state.
    // Both legacy0 and new hidden100 smoothing defaults have byte-exact
    // neutral dense/sparse production FFI proof at8/16/32bpc. No other value
    // expands eligibility for a pending unknown layer plane.
    if topology != (4,4) || mode != 1 || wave.to_bits() != 0.0f64.to_bits()
        || ![0.0f64.to_bits(),100.0f64.to_bits()].contains(&easing.to_bits())
        || min_spacing.to_bits() != 0.5f64.to_bits() {
        return false;
    }
    let initial=GridArb::default();
    grid == &initial
        && grid.column_lines.iter().zip(&initial.column_lines)
            .all(|(a,b)|a.to_bits()==b.to_bits())
        && grid.row_lines.iter().zip(&initial.row_lines)
            .all(|(a,b)|a.to_bits()==b.to_bits())
}

fn pending_frame_identity(params:&ae::Parameters<Params>,checkout:bool)->Result<bool,ae::Error>{
    // SmartPreRender has no valid ordinary params array. All dependencies used
    // in this decision must be checked out just like the owned render snapshot.
    let float=|id| ->Result<f64,ae::Error>{
        if checkout {checked_float(params,id)} else {Ok(params.get(id)?.as_float_slider()?.value())}
    };
    let mode=if checkout {checked_popup(params,Params::PlaneMode)?}
        else {params.get(Params::PlaneMode)?.as_popup()?.value()};
    let wave=float(Params::WaveAmplitude)?;
    let easing=float(Params::StretchEasing)?;
    let spacing=float(Params::MinSpacing)?;
    // Density controls are not render dependencies. Prove identity using only
    // the original stored lattice; do not sanitize or resize invalid data.
    let matches=|grid:&GridArb|initial_identity_values(
        grid,(i32::from(grid.columns),i32::from(grid.rows)),mode,wave,easing,spacing);
    if checkout {
        let checked=params.checkout(Params::GridState)?;
        let value=checked.as_arbitrary()?.value::<GridArb>()?;
        Ok(matches(&value))
    } else {
        let param=params.get(Params::GridState)?;
        let value=param.as_arbitrary()?.value::<GridArb>()?;
        Ok(matches(&value))
    }
}

pub fn add_params(params:&mut ae::Parameters<Params>)->Result<(),ae::Error>{
    for (id,name) in [Params::ResearchPlaneTL,Params::ResearchPlaneTR,
        Params::ResearchPlaneBR,Params::ResearchPlaneBL].into_iter().zip(NAMES) {
        params.add_with_flags(id,name,ae::PointDef::setup(|f| {
            f.set_default((0.0,0.0));f.set_restrict_bounds(false);
        }),ae::ParamFlag::empty(),ae::ParamUIFlags::INVISIBLE)?;
    }
    params.add_with_flags(Params::ResearchPlaneKind,NAMES[4],ae::FloatSliderDef::setup(|f| {
        setup_float(f,(0.0,3.0),(0.0,3.0),0.0,0,false);
    }),ae::ParamFlag::empty(),ae::ParamUIFlags::INVISIBLE)?;
    Ok(())
}
fn expressions()->Vec<String>{
    let mut values=legacy_expressions();
    values[4]="// FSTR native plane v2\nvar k=1;if(thisLayer.transform.position.value.length===3){k=3;try{var t=thisLayer.text.sourceText.value;k=2;}catch(e){}}\nk;".into();
    values
}
fn legacy_expressions()->Vec<String>{
    let mut values:Vec<String>=[(false,false),(true,false),(true,true),(false,true)].map(|(right,bottom)|format!(
        "// FSTR research plane v1\nvar r=thisLayer.sourceRectAtTime(time,false);\nvar p=thisLayer.toComp([r.left{},r.top{},0]);\n[p[0],p[1]];",
        if right {"+r.width"} else {""},if bottom {"+r.height"} else {""})).into();
    values.push("// FSTR research plane v1\nvar k=1;try{var t=thisLayer.text.sourceText.value;if(thisLayer.transform.position.value.length===3)k=2;}catch(e){}\nk;".into());
    values
}
fn err(e:ae::Error)->String{format!("{e:?}")}
// UI grouping changes stream positions. Resolve unique invariant hidden names
// and then validate each stream's name/type again immediately before access.
fn hidden_indices(names: &[String]) -> Result<[i32; 5], String> {
    let mut result=[-1; 5];
    for (index, name) in names.iter().enumerate() {
        if let Some(slot)=NAMES.iter().position(|expected| name == expected) {
            if result[slot] != -1 {return Err("Duplicate hidden stream".into());}
            result[slot]=index as i32;
        }
    }
    if result.iter().any(|index| *index < 0) {return Err("Missing hidden stream".into());}
    Ok(result)
}

#[cfg(test)]
mod schema_tests {
    use super::*;
    #[test]
    fn hidden_streams_survive_ui_insertions_and_refuse_ambiguous_schema() {
        for offset in [24, 27, 31] {
            let mut names=vec!["UI control".to_string(); offset];
            names.extend(NAMES.map(str::to_string));
            assert_eq!(hidden_indices(&names).unwrap(),std::array::from_fn(|i|(offset+i) as i32));
            names.push(NAMES[0].into());
            assert!(hidden_indices(&names).is_err());
            names.pop(); names.pop();
            assert!(hidden_indices(&names).is_err());
        }
    }
}

struct Adapter {
    id:ae::aegp::PluginId,
    effect:ae::aegp::EffectRefHandle,
    layer:ae::aegp::LayerHandle,
    basic:*const ae::sys::SPBasicSuite,
    indices: [i32; 5],
    stream_count: i32,
}
impl Adapter {
    fn new(id:ae::aegp::PluginId,effect:ae::aegp::EffectRefHandle,layer:ae::aegp::LayerHandle,basic:*const ae::sys::SPBasicSuite)->Result<Self,String>{
        let suite=ae::aegp::suites::Stream::new().map_err(err)?;
        let stream_count=suite.effect_num_param_streams(effect).map_err(err)?;
        if !(5..=128).contains(&stream_count) {return Err("Invalid effect schema size".into());}
        let mut names=vec![String::new()];
        // AE effect parameter streams start at 1; index 0 is the input layer.
        for index in 1..stream_count {
            let stream=suite.new_effect_stream_by_index(effect,id,index).map_err(err)?;
            names.push(suite.stream_name(&stream,id,true).map_err(err)?);
        }
        let indices=hidden_indices(&names)?;
        let mut host=Self{id,effect,layer,basic,indices,stream_count};
        host.validate_target()?;
        Ok(host)
    }
    fn stream(&mut self,i:usize)->Result<ae::aegp::StreamReferenceHandle,String>{
        self.validate_target()?;
        if i>=5 {return Err("Invalid hidden stream".into());}
        let suite=ae::aegp::suites::Stream::new().map_err(err)?;
        let stream=suite.new_effect_stream_by_index(self.effect,self.id,self.indices[i]).map_err(err)?;
        if suite.stream_name(&stream,self.id,true).map_err(err)?!=NAMES[i] ||
            suite.stream_type(&stream).map_err(err)?!=if i==4 {ae::aegp::StreamType::OneD} else {ae::aegp::StreamType::TwoDSpatial} {
            return Err("Hidden schema mismatch".into());
        }
        Ok(stream)
    }
}
impl Host for Adapter {
    fn validate_target(&mut self)->Result<(),String>{
        let flags=ae::aegp::suites::Layer::new().map_err(err)?.layer_flags(self.layer).map_err(err)?;
        if flags.contains(ae::aegp::LayerFlags::LOCKED) {return Err("Locked target".into());}
        let effects=ae::aegp::suites::Effect::new().map_err(err)?;
        let key=effects.installed_key_from_layer_effect(self.effect).map_err(err)?;
        if effects.effect_match_name(key).map_err(err)?!="com.elasticgrid.fx.warp" ||
            ae::aegp::suites::Stream::new().map_err(err)?.effect_num_param_streams(self.effect).map_err(err)?!=self.stream_count {
            return Err("Wrong effect schema".into());
        }
        Ok(())
    }
    fn read(&mut self,i:usize)->Result<Snapshot,String>{
        let stream=self.stream(i)?;let suite=ae::aegp::suites::Stream::new().map_err(err)?;
        let keys=ae::aegp::suites::Keyframe::new().map_err(err)?.stream_num_kfs(&stream).map_err(err)?;
        if keys<0 {return Err("Hidden stream cannot animate".into());}
        Ok(Snapshot {expression:read_expression(self.basic,self.id,&stream)?,
            enabled:suite.expression_state(&stream,self.id).map_err(err)?,keys:keys as usize})
    }
    fn begin_undo(&mut self)->Result<(),String>{ae::aegp::suites::Utility::new().map_err(err)?.start_undo_group("FSTR research binding").map_err(err)}
    fn end_undo(&mut self)->Result<(),String>{ae::aegp::suites::Utility::new().map_err(err)?.end_undo_group().map_err(err)}
    fn expression(&mut self,i:usize,value:&str)->Result<(),String>{
        let stream=self.stream(i)?;ae::aegp::suites::Stream::new().map_err(err)?.set_expression_string(&stream,self.id,value).map_err(err)
    }
    fn enable(&mut self,i:usize,value:bool)->Result<(),String>{
        let stream=self.stream(i)?;ae::aegp::suites::Stream::new().map_err(err)?.set_expression_state(&stream,self.id,value).map_err(err)
    }
}
// Caller has already restricted the operation to the owned fixture's newly
// added second effect, on the main thread, with a valid scoped suite context.
#[cfg(not(fstr_auto_binding))]
pub fn run(id:ae::aegp::PluginId,effect:ae::aegp::EffectRefHandle,layer:ae::aegp::LayerHandle,basic:*const ae::sys::SPBasicSuite)->Result<(),String>{
    let mut host=Adapter::new(id,effect,layer,basic)?;let e=expressions();
    let first=binding_transaction::install(&mut host,&e).map_err(|x|format!("{x:?}"))?;
    if first!=Outcome::Installed {return Err("Expected pristine research target".into());}
    let repeat=binding_transaction::install(&mut host,&e).map_err(|x|format!("{x:?}"))?;
    if repeat!=Outcome::AlreadyInstalled {return Err("Idempotency failed".into());}
    Ok(())
}

#[cfg(fstr_auto_binding)]
pub fn bind(id:ae::aegp::PluginId,effect:ae::aegp::EffectRefHandle,layer:ae::aegp::LayerHandle,basic:*const ae::sys::SPBasicSuite)->Result<Outcome,String>{
    binding_transaction::install_or_upgrade(&mut Adapter::new(id,effect,layer,basic)?,&expressions(),&legacy_expressions()).map_err(|e|format!("{e:?}"))
}

// The wrapper locks even a null GetExpression result. Query explicitly and
// never lock/free a null handle; a successful null result means no expression.
fn read_expression(basic:*const ae::sys::SPBasicSuite,id:ae::aegp::PluginId,
    stream:&ae::aegp::StreamReferenceHandle)->Result<String,String>{
    if basic.is_null() {return Err("Missing basic suite".into());}
    let mut ptr: *const c_void=std::ptr::null();
    let name=ae::sys::kAEGPStreamSuite.as_ptr().cast();
    let version=ae::sys::kAEGPStreamSuiteVersion6 as i32;
    let basic=unsafe {&*basic};
    let acquire=basic.AcquireSuite.ok_or("No AcquireSuite")?;
    let release=basic.ReleaseSuite.ok_or("No ReleaseSuite")?;
    let code=unsafe {acquire(name,version,&mut ptr)};
    if code!=0 {return Err(format!("Acquire StreamSuite: {code}"));}
    let result=(|| {
        if ptr.is_null() {return Err("Null StreamSuite".into());}
        let suite=unsafe {&*ptr.cast::<ae::sys::AEGP_StreamSuite6>()};
        let get=suite.AEGP_GetExpression.ok_or("No GetExpression")?;
        let mut handle=std::ptr::null_mut();
        let code=unsafe {get(id,stream.as_ptr(),&mut handle)};
        if code!=0 {return Err(format!("GetExpression: {code}"));}
        if handle.is_null() {return Ok(String::new());}
        let memory=ae::aegp::suites::Memory::new().map_err(err)?;
        let read=(|| {
            let bytes=memory.mem_handle_size(handle).map_err(err)?;
            if !(2..=1024*1024).contains(&bytes) || bytes%2!=0 {return Err("Invalid expression size".into());}
            let data=memory.lock_mem_handle(handle).map_err(err)?;
            let decoded=if data.is_null() {Err("Null expression data".into())} else {
                let units=unsafe {std::slice::from_raw_parts(data.cast::<u16>(),bytes/2)};
                match units.iter().position(|v|*v==0) {
                    Some(end)=>String::from_utf16(&units[..end]).map_err(|_|"Invalid expression UTF16".into()),
                    None=>Err("Unterminated expression".into()),
                }
            };
            let unlock=memory.unlock_mem_handle(handle).map_err(err);
            match (decoded,unlock) {(Ok(s),Ok(()))=>Ok(s),(Err(e),_)|(_,Err(e))=>Err(e)}
        })();
        let free=memory.free_mem_handle(handle).map_err(err);
        match (read,free) {(Ok(s),Ok(()))=>Ok(s),(Err(e),_)|(_,Err(e))=>Err(e)}
    })();
    let code=unsafe {release(name,version)};
    if code!=0 {return Err(format!("Release StreamSuite: {code}"));}
    result
}

#[cfg(test)] mod tests {
    use super::*;
    #[test] fn pending_binding_never_renders_legacy_in_automatic_mode(){
        assert!(comp_space_kind(0.0,true,true).is_err());
        assert_eq!(comp_space_kind(0.0,false,true),Ok(false));
        assert_eq!(comp_space_kind(0.0,true,false),Ok(false));
        for frame in [false,true] {
            assert_eq!(comp_space_kind(1.0,frame,true),Ok(false));
            assert_eq!(comp_space_kind(2.0,frame,true),Ok(true));
            assert_eq!(comp_space_kind(3.0,frame,true),Ok(false));
            for invalid in [-1.0,0.5,4.0,f64::NAN,f64::INFINITY] {
                assert!(comp_space_kind(invalid,frame,true).is_err());
            }
        }
    }
    use std::sync::{OnceLock,atomic::{AtomicUsize,Ordering}};
    static RELEASES:AtomicUsize=AtomicUsize::new(0);
    unsafe extern "C" fn empty(_:i32,_:ae::sys::AEGP_StreamRefH,out:*mut ae::sys::AEGP_MemHandle)->i32 {
        unsafe {*out=std::ptr::null_mut();} 0
    }
    unsafe extern "C" fn acquire(_: *const std::ffi::c_char,version:i32,out:*mut *const c_void)->i32 {
        if version!=ae::sys::kAEGPStreamSuiteVersion6 as i32 {return 1;}
        static SUITE:OnceLock<ae::sys::AEGP_StreamSuite6>=OnceLock::new();
        let suite=SUITE.get_or_init(|| {
            // ABI struct contains only nullable function pointers.
            let mut s:ae::sys::AEGP_StreamSuite6=unsafe {std::mem::zeroed()};
            s.AEGP_GetExpression=Some(empty);s
        });
        unsafe {*out=(suite as *const ae::sys::AEGP_StreamSuite6).cast();} 0
    }
    unsafe extern "C" fn release(_: *const std::ffi::c_char,_:i32)->i32 {RELEASES.fetch_add(1,Ordering::SeqCst);0}
    #[test] fn successful_null_expression_never_acquires_memory_or_locks_null(){
        let mut basic:ae::sys::SPBasicSuite=unsafe {std::mem::zeroed()};
        basic.AcquireSuite=Some(acquire);basic.ReleaseSuite=Some(release);
        let stream=ae::aegp::StreamReferenceHandle::from_raw(std::ptr::null_mut());
        assert!(read_expression(std::ptr::null(),1,&stream).is_err());
        assert_eq!(read_expression(&basic,1,&stream),Ok(String::new()));
        assert_eq!(RELEASES.load(Ordering::SeqCst),1);
    }
}

#[cfg(test)]
#[path = "first_application_tests.rs"]
mod first_application_tests;
