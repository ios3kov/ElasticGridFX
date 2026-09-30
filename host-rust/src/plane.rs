//! Four-corner plane: owned render snapshot and UI geometry from the same core.
use super::*;
use std::ptr::NonNull;

pub(crate) const CORNERS: [Params; 4] = [Params::PlaneTopLeft, Params::PlaneTopRight,
    Params::PlaneBottomRight, Params::PlaneBottomLeft];

pub(crate) fn update_ui(params: &ae::Parameters<Params>) -> Result<(), ae::Error> {
    let three_d=layer_is_3d(params,false)?;
    let enabled=!three_d && params.get(Params::PlaneMode)?.as_popup()?.value()==2;
    let mut mode=(*params.get(Params::PlaneMode)?).clone();
    mode.set_ui_flag(ae::ParamUIFlags::DISABLED,three_d);
    // Keep the serialized 2D selection/keyframes untouched. Both stored
    // ordinals display the effective plane while the entire selector is locked.
    mode.as_popup_mut()?.set_options(if three_d {&["Layer Plane (3D)","Layer Plane (3D)"]}
        else {&["Layer Plane","Four Corners"]});
    mode.update_param_ui()?;
    for id in CORNERS.into_iter().chain([Params::ResetPlane]) {
        let current=params.get(id)?;
        let mut definition=(*current).clone();
        definition.set_ui_flag(ae::ParamUIFlags::DISABLED,!enabled);
        definition.update_param_ui()?;
    }
    Ok(())
}
fn layer_is_3d(params:&ae::Parameters<Params>,checkout:bool)->Result<bool,ae::Error>{
    #[cfg(fstr_binding_probe)]
    {binding_probe::layer_is_3d(params,checkout)}
    #[cfg(not(fstr_binding_probe))]
    {let _=(params,checkout);Ok(false)}
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
}
impl State {
    pub fn corner_controls(&self) -> Option<[f64; 8]> {
        if self.editable_corners { self.corners } else { None }
    }
    pub fn read(params: &ae::Parameters<Params>, in_data: &ae::InData, checkout: bool, frame_context: bool) -> Result<Self, ae::Error> {
        let mode = if checkout { checked_popup(params, Params::PlaneMode)? }
                   else { params.get(Params::PlaneMode)?.as_popup()?.value() };
        if mode == 1 || layer_is_3d(params,checkout)? {
            #[cfg(fstr_binding_probe)]
            if let Some(derived)=binding_probe::sampled_plane(in_data,params,checkout,frame_context)? {return Ok(derived);}
            return Ok(Self::default());
        }
        if mode != 2 { return Err(ae::Error::BadCallbackParameter); }
        let mut corners = [0.0; 8];
        // pre_effect_source_origin is valid only in frame selectors, not UI.
        let origin = if frame_context {in_data.pre_effect_source_origin()} else {ae::Point {h:0,v:0}};
        for (i, id) in CORNERS.into_iter().enumerate() {
            let value = if checkout {params.checkout(id)?.as_point()?.float_value()?}
                        else {params.get(id)?.as_point()?.float_value()?};
            // AE already scales points. Remove buffer expansion once to match
            // the logical layer canvas used by the existing SmartFX path.
            corners[2*i] = value.x - origin.h as f64;
            corners[2*i+1] = value.y - origin.v as f64;
        }
        #[cfg(fstr_binding_probe)]
        if let Some(base)=binding_probe::sampled_plane(in_data,params,checkout,frame_context)? {
            let basis=base.corners.ok_or(ae::Error::BadCallbackParameter)?;
            let (width,height)=if frame_context {rendered_canvas(*in_data)}
                else {(in_data.width(),in_data.height())};
            let mapped=if Geometry::new(&basis).is_none() {basis}
                else {project_parameters(&basis,&corners,width as f64,height as f64)
                    .ok_or(ae::Error::BadCallbackParameter)?};
            return Ok(Self {corners:Some(mapped),editable_corners:true,comp_space:true,
                parameter_basis:Some(basis)});
        }
        Ok(Self {corners: Some(corners), editable_corners: true,comp_space:false,parameter_basis:None})
    }
    pub fn geometry(&self) -> Option<Geometry> {
        self.corners.and_then(|corners| Geometry::new(&corners))
    }
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
    #[cfg(test)]
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

pub(crate) fn render(input: Option<&ae::Layer>, output: &mut ae::Layer,
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
    let mut report = Report::default();
    let rc = unsafe {eg_render_plane_region(&src, &dst, output.bit_depth() as i32,
                         &frame, &mut report, p.quality - 1, p.edge_mode - 1)};
    // Invalid geometry is exact pass-through. UI diagnoses it, never render.
    match rc {
        0 => Ok(()), 5 => Err(ae::Error::InterruptCancel),
        1 | 2 | 4 => Err(ae::Error::BadCallbackParameter),
        _ => Err(ae::Error::InternalStructDamaged),
    }
}
