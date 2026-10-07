//! Four-corner plane: owned render snapshot and UI geometry from the same core.
use super::*;
use std::ptr::NonNull;

// Existing ordinals are serialized. Append new modes, never reorder old ones.
pub(crate) const DISPLAY_MODE_OPTIONS:[&str;4]=["Comp mode","Layer mode","Surface mode","Perspective"];
// This permutation is its own inverse. Stored ordinals never change.
pub(crate) fn display_mode_value(value:i32)->Result<i32,ae::Error>{
    match value{1|4=>Ok(value),2=>Ok(3),3=>Ok(2),_=>Err(ae::Error::BadCallbackParameter)}
}
pub(crate) const MODE_OPTIONS: [&str;4]=["Comp mode","Surface mode","Layer mode","Perspective"];
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub(crate) enum Mode {Comp,Flat,Layer,Perspective}
impl Mode {
    pub fn from_value(value:i32)->Result<Self,ae::Error>{
        match value {1=>Ok(Self::Comp),2=>Ok(Self::Flat),3=>Ok(Self::Layer),
            4=>Ok(Self::Perspective),_=>Err(ae::Error::BadCallbackParameter)}
    }
}
#[derive(Clone,Copy,Debug,Default,PartialEq,Eq)]
pub(crate) enum RenderKind {#[default] Region,Layer,Perspective,Comp}

pub(crate) const CORNERS: [Params; 4] = [Params::PlaneTopLeft, Params::PlaneTopRight,
    Params::PlaneBottomRight, Params::PlaneBottomLeft];

// UI callbacks only. Do not evaluate the hidden expression during
// UPDATE_PARAMS_UI (recursive checkout is forbidden there), or use its
// potentially stale PF snapshot to decide the current layer switch state.
fn ui_layer_is_3d(input: &ae::InData) -> Result<bool, ae::Error> {
    #[cfg(target_os="macos")]
    {
        unsafe extern "C" { fn pthread_main_np() -> i32; }
        if unsafe { pthread_main_np() } == 0 { return Err(ae::Error::BadCallbackParameter); }
    }
    let layer=ae::aegp::suites::PFInterface::new()?.effect_layer(input.effect_ref())?;
    Ok(ae::aegp::suites::Layer::new()?.layer_flags(layer)?
        .contains(ae::aegp::LayerFlags::LAYER_IS_3D))
}

fn ui_disabled(three_d:bool, mode:i32)->(bool,bool) {
    (false, !((mode==2 && !three_d) || mode==4))
}

// PF events permit DISABLED changes, not popup definition reconstruction.
// External layer switches cause a draw without necessarily UPDATE_PARAMS_UI.
pub(crate) fn sync_event_ui(input:&ae::InData,params:&ae::Parameters<Params>,claim_corners:bool)->Result<(),ae::Error>{
    let three_d=match ui_layer_is_3d(input){
        Ok(v)=>v,Err(ae::Error::BadCallbackParameter)=>return Ok(()),Err(e)=>return Err(e),
    };
    let (mode_disabled,corners_disabled)=ui_disabled(three_d,params.get(Params::PlaneMode)?.as_popup()?.value());
    for id in [Params::ModeSelector].into_iter().chain(CORNERS).chain([Params::ResetPlane]) {
        let current=params.get(id)?;
        #[cfg(feature="corner-topic-probe")]
        let claim_corners=claim_corners || id==Params::PlaneTopLeft;
        let disabled=if id==Params::ModeSelector {mode_disabled}
            else {corners_disabled || (claim_corners && CORNERS.contains(&id))};
        if current.ui_flags().contains(ae::ParamUIFlags::DISABLED)!=disabled {
            let mut definition=(*current).clone();
            definition.set_ui_flag(ae::ParamUIFlags::DISABLED,disabled);
            definition.update_param_ui()?;
        }
    }
    Ok(())
}

pub(crate) fn update_ui(input: &ae::InData, params: &ae::Parameters<Params>,plugin_id:Option<ae::aegp::PluginId>) -> Result<(), ae::Error> {
    // During effect construction AE may not yet have an owning layer for the
    // UI instance. Leave the setup defaults until the next host UI callback.
    let three_d=match ui_layer_is_3d(input) {
        Ok(value)=>value,
        Err(ae::Error::BadCallbackParameter)=>return Ok(()),
        Err(error)=>return Err(error),
    };
    let (mode_disabled,corners_disabled)=ui_disabled(three_d,params.get(Params::PlaneMode)?.as_popup()?.value());
    let current=params.get(Params::ModeSelector)?;
    // SDK25.6 Supervisor: change only a CONTROL_ONLY popup copy through
    // UpdateParamUI. No data stream or CHANGED_VALUE on this UI-only control.
    let stored=params.get(Params::PlaneMode)?.as_popup()?.value();
    let desired=display_mode_value(stored)?;
    if popup_ui_changed(current.as_popup()?.value(),current.ui_flags().contains(ae::ParamUIFlags::DISABLED),desired,mode_disabled) {
        let mut mode=(*current).clone();
        mode.set_ui_flag(ae::ParamUIFlags::DISABLED,mode_disabled);
        mode.as_popup_mut()?.set_value(desired);
        mode.update_param_ui()?;
    }
    edge::update_ui(params)?;
    let _=plugin_id;
    #[cfg(feature="corner-ownership")]
    let claim=corner_ownership::claimed_ui(input,params,plugin_id);
    #[cfg(not(feature="corner-ownership"))]
    let claim=false;
    for id in CORNERS.into_iter().chain([Params::ResetPlane]) {
        let corners_disabled=corners_disabled || (claim && CORNERS.contains(&id));
        #[cfg(feature="corner-topic-probe")]
        let corners_disabled=corners_disabled || id==Params::PlaneTopLeft;
        let current=params.get(id)?;
        if current.ui_flags().contains(ae::ParamUIFlags::DISABLED)!=corners_disabled {
            let mut definition=(*current).clone();
            definition.set_ui_flag(ae::ParamUIFlags::DISABLED,corners_disabled);
            definition.update_param_ui()?;
        }
    }
    Ok(())
}

#[cfg(test)] mod ui_state_tests {
    use super::ui_disabled;
    #[test] fn display_order_roundtrips_every_saved_mode_without_reinterpretation(){
        for value in 1..=4 {assert_eq!(super::display_mode_value(super::display_mode_value(value).unwrap()),Ok(value));}
        assert_eq!(super::DISPLAY_MODE_OPTIONS[super::display_mode_value(2).unwrap() as usize-1],"Surface mode");
        assert_eq!(super::DISPLAY_MODE_OPTIONS[super::display_mode_value(3).unwrap() as usize-1],"Layer mode");
        assert!(super::display_mode_value(0).is_err());
    }
    #[test] fn four_mode_corner_editing_respects_2d_and_3d() {
        for mode in 1..=4 {
            assert_eq!(ui_disabled(true,mode),(false,mode!=4));
            assert_eq!(ui_disabled(false,mode),(false,mode!=2 && mode!=4));
        }
    }
}
fn layer_is_3d(reads:&param_reads::Reads<'_,'_>)->Result<bool,ae::Error>{
    #[cfg(fstr_binding_probe)]
    {binding_probe::layer_is_3d(reads)}
    #[cfg(not(fstr_binding_probe))]
    {let _=reads;Ok(false)}
}
#[derive(Clone, Debug, Default)]
pub(crate) struct State {
    pub corners: Option<[f64; 8]>,
    // Geometry alone does not grant permission to edit public corner parameters.
    // Automatically derived regions must keep this false.
    pub editable_corners: bool,
    pub comp_space: bool,
    // Native text Four Corners uses the existing effect-canvas coordinates as
    // a normalized domain on the layer plane. Retain the unedited basis for
    // inverse corner picking, including repair of a degenerate user quad.
    pub parameter_basis: Option<[f64;8]>,
    pub render_kind: RenderKind,
    // Immutable unedited source basis for Corner Pin; no resampled intermediate.
    pub source_corners: Option<[f64;8]>,
}
impl State {
    pub fn corner_controls(&self) -> Option<[f64; 8]> {
        if self.editable_corners { self.corners } else { None }
    }
    pub fn read(params: &ae::Parameters<Params>, in_data: &ae::InData, checkout: bool, frame_context: bool) -> Result<Self, ae::Error> {
        Self::read_values(&param_reads::Reads::new(params,checkout,None),in_data,frame_context)
    }
    pub(crate) fn read_values(reads:&param_reads::Reads<'_,'_>,in_data:&ae::InData,frame_context:bool)->Result<Self,ae::Error>{
        let mode=reads.popup(Params::PlaneMode)?;
        let mode=Mode::from_value(mode)?;
        if mode==Mode::Layer {
            #[cfg(fstr_binding_probe)]
            {return binding_probe::sampled_layer_plane(in_data,reads,frame_context)
                .map(|state|state.unwrap_or_default());}
            #[cfg(not(fstr_binding_probe))]
            {return Ok(Self::default());}
        }
        if mode == Mode::Comp || (mode==Mode::Flat && layer_is_3d(reads)?) {
            #[cfg(fstr_binding_probe)]
            if let Some(derived)=if mode==Mode::Comp {binding_probe::sampled_comp_plane(in_data,reads,frame_context)?} else {binding_probe::sampled_plane(in_data,reads,frame_context)?} {return Ok(derived);}
            return Ok(Self::default());
        }
        let perspective=mode==Mode::Perspective;
        let mut corners = [0.0; 8];
        // pre_effect_source_origin is valid only in frame selectors, not UI.
        let origin = if frame_context {in_data.pre_effect_source_origin()} else {ae::Point {h:0,v:0}};
        for (i, id) in CORNERS.into_iter().enumerate() {
            let value=reads.point(id)?;
            // AE already scales points. Remove buffer expansion once to match
            // the logical layer canvas used by the existing SmartFX path.
            corners[2*i] = value.0 - origin.h as f64;
            corners[2*i+1] = value.1 - origin.v as f64;
        }
        #[cfg(fstr_binding_probe)]
        if let Some(base)=binding_probe::sampled_plane(in_data,reads,frame_context)? {
            let basis=base.corners.ok_or(ae::Error::BadCallbackParameter)?;
            let (width,height)=if frame_context {rendered_canvas(*in_data)}
                else {(in_data.width(),in_data.height())};
            let mapped=if Geometry::new(&basis).is_none() {basis}
                else {project_parameters(&basis,&corners,width as f64,height as f64)
                    .ok_or(ae::Error::BadCallbackParameter)?};
            return Ok(Self {corners:Some(mapped),editable_corners:true,comp_space:true,
                parameter_basis:Some(basis),render_kind:if perspective {RenderKind::Perspective} else {RenderKind::Region},
                source_corners:perspective.then_some(basis)});
        }
        let (sx,sy)=if frame_context {(f64::from(f32::from(in_data.downsample_x())),
            f64::from(f32::from(in_data.downsample_y())))} else {(1.0,1.0)};
        let source=source_rectangle(f64::from(in_data.width())*sx,f64::from(in_data.height())*sy,
            f64::from(origin.h),f64::from(origin.v))?;
        Ok(Self {corners: Some(corners), editable_corners: true,comp_space:false,parameter_basis:None,
            render_kind:if perspective {RenderKind::Perspective} else {RenderKind::Region},
            source_corners:perspective.then_some(source)})
    }
    pub fn geometry(&self) -> Option<Geometry> {
        self.corners.and_then(|corners| Geometry::new(&corners))
    }
}

fn source_rectangle(width:f64,height:f64,origin_x:f64,origin_y:f64)->Result<[f64;8],ae::Error>{
    if ![width,height,origin_x,origin_y].iter().all(|v|v.is_finite()) || width<=0.0 || height<=0.0 {
        return Err(ae::Error::BadCallbackParameter);
    }
    Ok([-origin_x,-origin_y,width-origin_x,-origin_y,
        width-origin_x,height-origin_y,-origin_x,height-origin_y])
}

fn project_parameters(basis:&[f64;8],points:&[f64;8],width:f64,height:f64)->Option<[f64;8]>{
    if width<=0.0 || height<=0.0 {return None;}
    let geometry=Geometry::new(basis)?;
    let mut projected=[0.0;8];
    for i in 0..4 {
        let p=geometry.map(false,points[2*i]/width,points[2*i+1]/height)?;
        projected[2*i]=p.0;projected[2*i+1]=p.1;
    }
    Some(projected)
}

pub(crate) fn parameter_point(basis:&[f64;8],x:f64,y:f64,width:f64,height:f64)->Option<(f64,f64)>{
    let uv=Geometry::new(basis)?.map(true,x,y)?;
    Some((uv.0*width,uv.1*height))
}

#[cfg(test)] mod parameter_tests {
    use super::*;
    #[test] fn projected_public_corners_and_inverse_drag_share_basis(){
        let basis=[100.0,80.0,430.0,50.0,510.0,400.0,60.0,360.0];
        let full=[0.0,0.0,640.0,0.0,640.0,480.0,0.0,480.0];
        let actual=project_parameters(&basis,&full,640.0,480.0).unwrap();
        for (a,b) in actual.iter().zip(basis) {assert!((a-b).abs()<1e-8);}
        let custom=[60.0,30.0,500.0,65.0,570.0,390.0,90.0,440.0];
        let projected=project_parameters(&basis,&custom,640.0,480.0).unwrap();
        for i in 0..4 {
            let p=parameter_point(&basis,projected[2*i],projected[2*i+1],640.0,480.0).unwrap();
            assert!((p.0-custom[2*i]).abs()<1e-8 && (p.1-custom[2*i+1]).abs()<1e-8);
        }
        let half_basis=basis.map(|v|v/2.0);
        let half=project_parameters(&half_basis,&custom.map(|v|v/2.0),320.0,240.0).unwrap();
        for (a,b) in half.iter().zip(projected) {assert!((a-b/2.0).abs()<1e-8);}
        assert!(project_parameters(&[0.0;8],&full,640.0,480.0).is_none());
        assert!(project_parameters(&basis,&full,0.0,480.0).is_none());
    }
}

pub(crate) struct Geometry(NonNull<c_void>);
impl Geometry {
    pub fn new(corners: &[f64; 8]) -> Option<Self> {
        NonNull::new(unsafe {eg_plane_geometry_create(corners.as_ptr())}).map(Self)
    }
    pub fn map(&self, inverse: bool, x: f64, y: f64) -> Option<(f64, f64)> {
        let mut output = [0.0; 2];
        let rc = unsafe {eg_plane_geometry_map(self.0.as_ptr(), i32::from(inverse), x, y, output.as_mut_ptr())};
        (rc == 0).then_some((output[0], output[1]))
    }
}
impl Drop for Geometry {
    fn drop(&mut self) { unsafe {eg_plane_geometry_destroy(self.0.as_ptr())}; }
}

#[repr(C)]
#[derive(Clone,Copy)]
pub(crate) struct Frame {
    pub corners: [f64; 8], pub columns: *const f32, pub rows: *const f32,
    pub column_count: i32, pub row_count: i32,
    pub surface_units_x: f64, pub surface_units_y: f64,
    pub canvas_width: i32, pub canvas_height: i32,
    pub source_x: i32, pub source_y: i32, pub output_x: i32, pub output_y: i32,
    pub easing: f32, pub easing_distance: f32,
    pub abort_fn: Option<unsafe extern "C" fn(*mut c_void) -> i32>, pub abort_refcon: *mut c_void,
}
#[repr(C)]
#[derive(Clone,Copy)]
pub(crate) struct Image { pub pixels: *mut c_void, pub row_bytes: isize, pub width: i32, pub height: i32 }
#[repr(C)]
#[derive(Default)]
pub(crate) struct Report { pub invalid_plane: i32, pub reserved: i32, pub outside_pixels: u64, pub invalid_projection_pixels: u64 }
unsafe extern "C" {
    fn eg_plane_geometry_create(corners: *const f64) -> *mut c_void;
    fn eg_plane_geometry_destroy(geometry: *mut c_void);
    fn eg_plane_geometry_map(geometry: *const c_void, inverse: i32, x: f64, y: f64, output: *mut f64) -> i32;
    fn eg_evaluate_grid(params: *const EgRenderParams, columns: *mut f32, column_capacity: i32,
                        rows: *mut f32, row_capacity: i32) -> i32;
    #[cfg(test)]
    pub(crate) fn eg_render_plane(src: *const Image, dst: *const Image, depth: i32,
                       frame: *const Frame, report: *mut Report) -> i32;
    #[cfg(test)]
    pub(crate) fn eg_render_plane_sampled(src: *const Image, dst: *const Image, depth: i32,
                       frame: *const Frame, report: *mut Report, quality: i32, edge: i32) -> i32;
    #[cfg(test)]
    pub(crate) fn eg_render_plane_projected(src: *const Image, dst: *const Image, depth: i32,
                       frame: *const Frame, report: *mut Report, quality: i32, edge: i32,
                       source_extent_x: f64, source_extent_y: f64) -> i32;
    pub(crate) fn eg_render_plane_region(src: *const Image, dst: *const Image, depth: i32,
                       frame: *const Frame, report: *mut Report, quality: i32, edge: i32) -> i32;
    pub(crate) fn eg_render_plane_comp(src: *const Image, dst: *const Image, depth: i32,
                       frame: *const Frame, report: *mut Report, quality: i32, edge: i32) -> i32;
    pub(crate) fn eg_render_plane_layer(src: *const Image, dst: *const Image, depth: i32,
                       frame: *const Frame, report: *mut Report, quality: i32, edge: i32) -> i32;
    pub(crate) fn eg_render_plane_between(src: *const Image, dst: *const Image, depth: i32,
                       frame: *const Frame, report: *mut Report, quality: i32, edge: i32,
                       source_corners: *const f64) -> i32;
}

pub(crate) fn evaluated_axes(p: &EgRenderParams) -> Result<(Vec<f32>, Vec<f32>), ae::Error> {
    if !(1..=50).contains(&p.columns) || !(1..=50).contains(&p.rows) {
        return Err(ae::Error::BadCallbackParameter);
    }
    let mut columns = vec![0.0; p.columns as usize + 2];
    let mut rows = vec![0.0; p.rows as usize + 2];
    let rc = unsafe {eg_evaluate_grid(p, columns.as_mut_ptr(), columns.len() as i32,
                                     rows.as_mut_ptr(), rows.len() as i32)};
    if rc != 0 { return Err(ae::Error::BadCallbackParameter); }
    Ok((columns, rows))
}

pub(crate) fn render(in_data: &ae::InData, input: Option<&ae::Layer>, output: &mut ae::Layer,
                     p: &EgRenderParams, state: &State) -> Result<(), ae::Error> {
    let corners = state.corners.ok_or(ae::Error::BadCallbackParameter)?;
    if input.is_some_and(|layer| layer.bit_depth() != output.bit_depth()) {
        return Err(ae::Error::BadCallbackParameter);
    }
    let (columns, rows) = evaluated_axes(p)?;
    let frame = Frame {corners, columns: columns.as_ptr(), rows: rows.as_ptr(),
        column_count: columns.len() as i32, row_count: rows.len() as i32,
        surface_units_x: 1.0, surface_units_y: 1.0,
        canvas_width: p.canvas_width, canvas_height: p.canvas_height,
        source_x: p.input_origin_x, source_y: p.input_origin_y,
        output_x: p.output_origin_x, output_y: p.output_origin_y,
        easing: p.stretch_easing, easing_distance: p.easing_distance,
        abort_fn: p.abort_fn, abort_refcon: p.abort_refcon};
    let src = if let Some(layer) = input {
        Image {pixels: unsafe {layer.data_ptr()}.cast_mut().cast(), row_bytes: layer.row_bytes(),
               width: layer.width() as i32, height: layer.height() as i32}
    } else {Image {pixels: std::ptr::null_mut(), row_bytes: 0, width: 0, height: 0}};
    let dst = Image {pixels: unsafe {output.data_ptr_mut()}.cast(), row_bytes: output.row_bytes(),
                     width: output.width() as i32, height: output.height() as i32};
    if row_tiles::eligible(&src,&dst,output.bit_depth() as i32,&frame,state) {
        // SAFETY: eligible checked layout/separation; worlds and evaluated axes
        // are retained until the synchronous AE iterator has joined all tasks.
        let job=unsafe {row_tiles::Job::new(src,dst,output.bit_depth() as i32,frame,
            (p.quality-1,p.edge_mode-1),state)};
        return job.run(|| {
            if p.abort_fn.is_some_and(|abort| unsafe {abort(p.abort_refcon)}!=0) {
                Err(ae::Error::InterruptCancel)
            } else {Ok(())}
        }, |count,callback| in_data.utils().iterate_generic(count,|_,i,_| callback(i)));
    }
    let mut report = Report::default();
    let rc=dispatch_render(&src,&dst,output.bit_depth() as i32,&frame,&mut report,
        (p.quality-1,p.edge_mode-1),state);
    // Invalid geometry is exact pass-through. UI diagnoses it, never render.
    render_result(rc)
}

fn render_result(rc:i32)->Result<(),ae::Error> {
    match rc {
        0 => Ok(()), 5 => Err(ae::Error::InterruptCancel),
        1 | 2 | 4 => Err(ae::Error::BadCallbackParameter),
        _ => Err(ae::Error::InternalStructDamaged),
    }
}

#[path="plane_row_tiles.rs"]
mod row_tiles;

// One shared dispatch for legacy/SmartFX and both platforms. Geometry invalidity
// retains the existing original-image fallback, including Perspective.
pub(crate) fn dispatch_render(src:&Image,dst:&Image,depth:i32,frame:&Frame,report:&mut Report,
                   sampling:(i32,i32),state:&State)->i32 {
    let (quality,edge)=sampling;
    // SAFETY: callers own the typed layer/frame/axis allocations for this
    // synchronous call. The C ABI validates all dimensions, strides and modes.
    unsafe {
        if state.render_kind==RenderKind::Comp {
            eg_render_plane_comp(src,dst,depth,frame,report,quality,edge)
        } else if state.render_kind==RenderKind::Perspective && state.geometry().is_some() {
            let Some(source)=state.source_corners else {return 1;};
            eg_render_plane_between(src,dst,depth,frame,report,quality,edge,source.as_ptr())
        } else if state.render_kind==RenderKind::Layer || (state.comp_space && !state.editable_corners) {
            eg_render_plane_layer(src,dst,depth,frame,report,quality,edge)
        } else {eg_render_plane_region(src,dst,depth,frame,report,quality,edge)}
    }
}

#[cfg(test)]
#[path="four_modes_tests.rs"]
mod four_modes_tests;
