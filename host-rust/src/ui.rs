use super::*;

const HIT_SLOP: f32 = 9.0;
const GRIP_LENGTH: f32 = 48.0;
const GUIDE_GAP: f32 = 12.0;
const DRAG_NONE: ae::sys::A_intptr_t = 0;
const DRAG_COLUMNS: ae::sys::A_intptr_t = 1;
const DRAG_ROWS: ae::sys::A_intptr_t = 2;
const DRAG_CORNER: ae::sys::A_intptr_t = 3;

struct ViewPlane {
    state: plane::State,
    geometry: Option<plane::Geometry>,
    width: f32,
    height: f32,
    projection: Option<ui_projection::Projection>,
    projection_unavailable: bool,
    comp_space: bool,
}
impl ViewPlane {
    fn needs_layer_conversion(&self, window: ae::WindowType) -> bool {
        !self.comp_space && self.projection.is_none() && window==ae::WindowType::Comp
    }
    fn read(in_data: &ae::InData, params: &ae::Parameters<Params>, event: &ae::EventExtra) -> Result<Self, ae::Error> {
        let state = plane::State::read(params, in_data, false, false)?;
        let comp_space=state.comp_space;
        let geometry = state.geometry();
        let (projection,projection_unavailable)=if comp_space {
            // Comp-space text overlay is not yet supported in the Layer viewer.
            // Never use that viewer's layer-local source_to_frame on this quad.
            (None,event.window_type()!=ae::WindowType::Comp)
        } else {match ui_projection::read(in_data,event) {
            Ok(value)=>(value,false), Err(_)=>(None,true),
        }};
        Ok(Self {state, geometry, width: in_data.width().max(1) as f32,
            height: in_data.height().max(1) as f32, projection, projection_unavailable,comp_space})
    }
    fn invalid(&self) -> bool { self.state.corners.is_some() && self.geometry.is_none() }
    fn local(&self, x: f32, y: f32) -> Option<(f32, f32)> {
        if let Some(geometry) = &self.geometry {
            geometry.map(true, x as f64, y as f64).map(|(x,y)| (x as f32,y as f32))
        } else if self.invalid() { None }
        else { Some((x / self.width, y / self.height)) }
    }
}

fn grid_to_frame(in_data: &ae::InData, event: &ae::EventExtra, plane: &ViewPlane,
                 x: f32, y: f32) -> Result<ae::drawbot::PointF32, ae::Error> {
    let (x,y) = if let Some(geometry) = &plane.geometry {
        let (x,y) = geometry.map(false, (x/plane.width) as f64, (y/plane.height) as f64)
            .ok_or(ae::Error::BadCallbackParameter)?;
        (x as f32,y as f32)
    } else {(x,y)};
    layer_to_frame(in_data,event,plane,x,y)
}


thread_local! {
    static GUIDE_DRAGGING: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
}

fn hand_cursor(dragging: bool) -> ae::CursorType {
    #[cfg(target_os = "macos")]
    {
        unsafe extern "C" { fn eg_set_hand_cursor(dragging: bool) -> bool; }
        // The shim checks the UI thread before touching AppKit.
        if unsafe { eg_set_hand_cursor(dragging) } {
            return ae::CursorType::Custom;
        }
    }
    // Built-in fallback on other hosts; exact closed-hand appearance is Mac-only.
    if dragging { ae::CursorType::Pan } else { ae::CursorType::Hand }
}

fn set_drag_cursor(dragging: bool) {
    GUIDE_DRAGGING.set(dragging);
    let cursor = hand_cursor(dragging);
    // AppKit already set the custom cursor. CUSTOM belongs in AdjustCursor's
    // response; do not pass sentinel values to the host's PF_SetCursor suite.
    if cursor == ae::CursorType::Custom {
        return;
    }
    if let Ok(app) = ae::pf::suites::App::new() {
        let _ = app.set_cursor(cursor);
    }
}

pub fn release_cursor() {
    GUIDE_DRAGGING.set(false);
    // Leave the selected tool's cursor to AE. PF_SetCursor(NONE) is invalid
    // on AE 25.6; forcing Arrow here also overrides text/pen/rotation tools.
}

// PF_ContextH is valid only for this owning UI callback; never retain it.
pub(crate) fn event_window_code(event:&ae::EventExtra)->i32 {
    let handle=event.as_ref().contextH;
    if handle.is_null(){return ae::sys::PF_Window_NONE;}
    // SAFETY: the host owns the callback's handle; null is checked at both levels.
    let context=unsafe{*handle};
    if context.is_null(){ae::sys::PF_Window_NONE}else{unsafe{(*context).w_type}}
}
pub(crate) fn known_window(window:i32)->bool {
    [ae::sys::PF_Window_COMP,ae::sys::PF_Window_LAYER,ae::sys::PF_Window_EFFECT].contains(&window)
}

fn overlay_color(value: f32) -> ae::drawbot::ColorRgba {
    ae::drawbot::ColorRgba { red: value, green: value, blue: value, alpha: 1.0 }
}

type Segment = (ae::drawbot::PointF32, ae::drawbot::PointF32);

// Layout in frame coordinates, so grips keep their size under layer zoom and
// follow rotated guides. Cut locations come from the current grid, not a fixed
// midpoint. The same clipped geometry is used for drawing and picking.
fn guide_segments(a: ae::drawbot::PointF32, b: ae::drawbot::PointF32,
                  crossings: &[ae::drawbot::PointF32], handles: bool) -> (Vec<Segment>, Vec<Segment>) {
    let length = ((b.x - a.x).powi(2) + (b.y - a.y).powi(2)).sqrt();
    if !length.is_finite() || length <= 1.0e-3 {
        return (Vec::new(), Vec::new());
    }
    let point = |offset: f32| ae::drawbot::PointF32 {
        x: a.x + (b.x - a.x) * offset / length,
        y: a.y + (b.y - a.y) * offset / length,
    };
    let mut cuts: Vec<f32> = crossings.iter().map(|p|
        ((p.x - a.x) * (b.x - a.x) + (p.y - a.y) * (b.y - a.y)) / length)
        .filter(|s| s.is_finite() && *s > 0.0 && *s < length).collect();
    cuts.sort_by(f32::total_cmp);
    let mut spans = Vec::with_capacity(cuts.len() + 1);
    let mut start = 0.0_f32;
    for cut in cuts {
        let end = (cut - GUIDE_GAP * 0.5).max(0.0);
        if end > start { spans.push((start, end)); }
        start = start.max((cut + GUIDE_GAP * 0.5).min(length));
    }
    if start < length { spans.push((start, length)); }
    let lines = spans.iter().map(|&(lo, hi)| (point(lo), point(hi))).collect();
    let grips = if handles && length >= GRIP_LENGTH + GUIDE_GAP + 8.0 {
        spans.iter().filter_map(|&(lo, hi)| {
            let lo = lo.max((length - GRIP_LENGTH) * 0.5);
            let hi = hi.min((length + GRIP_LENGTH) * 0.5);
            (hi > lo).then(|| (point(lo), point(hi)))
        }).collect()
    } else { Vec::new() };
    (lines, grips)
}

fn column_crossings(in_data: &ae::InData, event: &ae::EventExtra,
                    plane: &ViewPlane, grid: &GridArb, x: f32) -> Result<Vec<ae::drawbot::PointF32>, ae::Error> {
    grid.row_lines.iter().skip(1).take(grid.row_lines.len().saturating_sub(2))
        .map(|y| grid_to_frame(in_data, event, plane, x, y * in_data.height().max(1) as f32))
        .collect()
}

fn layer_to_frame(
    in_data: &ae::InData,
    event: &ae::EventExtra,
    plane: &ViewPlane,
    x: f32,
    y: f32,
) -> Result<ae::drawbot::PointF32, ae::Error> {
    let unavailable=ae::drawbot::PointF32 {x:f32::NAN,y:f32::NAN};
    if plane.projection_unavailable {return Ok(unavailable);}
    // AE's legacy UI conversion uses 16.16. Never saturate oversized corners
    // into a false on-screen position. Such points remain editable in the ECP.
    if !x.is_finite() || !y.is_finite() || x.abs()>32767.0 || y.abs()>32767.0 {
        return Ok(unavailable);
    }
    let (x,y)=if let Some(projection)=&plane.projection {
        let Some((x,y))=projection.forward(x as f64,y as f64) else {return Ok(unavailable);};
        if x.abs()>32767.0||y.abs()>32767.0 {return Ok(unavailable);}
        (x as f32,y as f32)
    } else {(x,y)};
    let mut p = ae::sys::PF_FixedPoint {
        x: ae::Fixed::from(x).as_fixed(),
        y: ae::Fixed::from(y).as_fixed(),
    };
    if plane.needs_layer_conversion(event.window_type()) &&
        event.callbacks().layer_to_comp(in_data.current_time(), in_data.time_scale(), &mut p).is_err() {
        return Ok(unavailable);
    }
    if event.callbacks().source_to_frame(&mut p).is_err() {return Ok(unavailable);}
    Ok(ae::drawbot::PointF32 {
        x: ae::Fixed::from_fixed(p.x).as_f32(),
        y: ae::Fixed::from_fixed(p.y).as_f32(),
    })
}

fn frame_to_layer(
    in_data: &ae::InData,
    event: &ae::EventExtra,
    plane: &ViewPlane,
    p: ae::Point,
) -> Result<(f32, f32), ae::Error> {
    if plane.projection_unavailable {return Err(ae::Error::BadCallbackParameter);}
    let mut fixed = ae::sys::PF_FixedPoint {
        x: ae::Fixed::from_int(p.h).as_fixed(),
        y: ae::Fixed::from_int(p.v).as_fixed(),
    };
    event.callbacks().frame_to_source(&mut fixed)?;
    if let Some(projection)=&plane.projection {
        return projection.backward(ae::Fixed::from_fixed(fixed.x).as_f32() as f64,
                                   ae::Fixed::from_fixed(fixed.y).as_f32() as f64)
            .map(|(x,y)|(x as f32,y as f32)).ok_or(ae::Error::BadCallbackParameter);
    }
    if plane.needs_layer_conversion(event.window_type()) {
        event.callbacks().comp_to_layer(in_data.current_time(), in_data.time_scale(), &mut fixed)?;
    }
    Ok((
        ae::Fixed::from_fixed(fixed.x).as_f32(),
        ae::Fixed::from_fixed(fixed.y).as_f32(),
    ))
}

fn point_segment_distance(px: f32, py: f32, a: ae::drawbot::PointF32, b: ae::drawbot::PointF32) -> f32 {
    let vx = b.x - a.x;
    let vy = b.y - a.y;
    let wx = px - a.x;
    let wy = py - a.y;
    let vv = vx * vx + vy * vy;
    if vv <= 1.0e-6 {
        return ((px - a.x).powi(2) + (py - a.y).powi(2)).sqrt();
    }
    let t = ((wx * vx + wy * vy) / vv).clamp(0.0, 1.0);
    let qx = a.x + t * vx;
    let qy = a.y + t * vy;
    ((px - qx).powi(2) + (py - qy).powi(2)).sqrt()
}

fn hit_test(
    in_data: &ae::InData,
    grid: &GridArb,
    plane: &ViewPlane,
    event: &ae::EventExtra,
    mouse: ae::Point,
) -> Result<Option<(ae::sys::A_intptr_t, usize)>, ae::Error> {
    if event.window_type() != ae::WindowType::Comp && event.window_type() != ae::WindowType::Layer {
        return Ok(None);
    }
    let width = in_data.width().max(1) as f32;
    let height = in_data.height().max(1) as f32;
    let mut best: Option<(f32, ae::sys::A_intptr_t, usize)> = None;

    if let Some(corners) = plane.state.corner_controls() {
        for i in 0..4 {
            let p = layer_to_frame(in_data,event,plane,corners[2*i] as f32,corners[2*i+1] as f32)?;
            let d = ((mouse.h as f32-p.x).powi(2)+(mouse.v as f32-p.y).powi(2)).sqrt();
            if d <= HIT_SLOP && best.map(|v| d<v.0).unwrap_or(true) {best=Some((d,DRAG_CORNER,i));}
        }
        if best.is_some() {return Ok(best.map(|(_,axis,index)| (axis,index)));}
    }
    if plane.invalid() {return Ok(None);}

    for i in 1..grid.column_lines.len().saturating_sub(1) {
        if grid.column_pins.get(i).copied().unwrap_or(0) != 0 {
            continue;
        }
        let x = grid.column_lines[i] * width;
        let a = grid_to_frame(in_data, event, plane, x, 0.0)?;
        let b = grid_to_frame(in_data, event, plane, x, height)?;
        let crossings = column_crossings(in_data, event, plane, grid, x)?;
        let (segments, _) = guide_segments(a, b, &crossings,
            grid.column_lines.len().max(grid.row_lines.len()) <= 34);
        let d = segments.into_iter().map(|(a, b)|
            point_segment_distance(mouse.h as f32, mouse.v as f32, a, b))
            .fold(f32::INFINITY, f32::min);
        if d <= HIT_SLOP && best.map(|v| d < v.0).unwrap_or(true) {
            best = Some((d, DRAG_COLUMNS, i));
        }
    }
    for i in 1..grid.row_lines.len().saturating_sub(1) {
        if grid.row_pins.get(i).copied().unwrap_or(0) != 0 {
            continue;
        }
        let y = grid.row_lines[i] * height;
        let a = grid_to_frame(in_data, event, plane, 0.0, y)?;
        let b = grid_to_frame(in_data, event, plane, width, y)?;
        let d = point_segment_distance(mouse.h as f32, mouse.v as f32, a, b);
        if d <= HIT_SLOP && best.map(|v| d < v.0).unwrap_or(true) {
            best = Some((d, DRAG_ROWS, i));
        }
    }
    Ok(best.map(|(_, axis, index)| (axis, index)))
}

fn draw_segment(
    supplier: &ae::drawbot::Supplier,
    surface: &ae::drawbot::Surface,
    pen: &ae::drawbot::Pen,
    a: ae::drawbot::PointF32,
    b: ae::drawbot::PointF32,
) -> Result<(), ae::Error> {
    if !a.x.is_finite() || !a.y.is_finite() || !b.x.is_finite() || !b.y.is_finite() {return Ok(());}
    let mut path = supplier.new_path()?;
    path.move_to(a.x, a.y)?;
    path.line_to(b.x, b.y)?;
    surface.stroke_path(pen, &path)?;
    Ok(())
}

// Range annotations use only this callback's current evaluator and transform.
fn draw_range(in_data:&ae::InData,params:&mut ae::Parameters<Params>,event:&ae::EventExtra,
    plane:&ViewPlane,supplier:&ae::drawbot::Supplier,surface:&ae::drawbot::Surface)->Result<(),ae::Error> {
    let saved=grid_snapshot(params)?;let anchor=range_feedback::anchor(event,&saved);
    let p=evaluated_params(params,*in_data,&saved)?;
    let (columns,rows)=plane::evaluated_axes(&p)?;
    let axis=if anchor.column {&columns}else{&rows};
    let Some((lo,hi))=range_feedback::bounds(anchor,p.tension_radius,axis.len()) else{return Ok(());};
    let color=ae::drawbot::ColorRgba{red:0.2,green:0.7,blue:1.0,alpha:1.0};
    let pen=supplier.new_pen(&color,0.8)?;let cap=supplier.new_pen(&color,2.5)?;
    let width=in_data.width().max(1) as f32;let height=in_data.height().max(1) as f32;
    for source in [lo,hi,anchor.source] {
        let position=control_grid::position_at(axis,source,p.stretch_easing,p.easing_distance)?;
        let (a,b)=if anchor.column {
            (grid_to_frame(in_data,event,plane,position*width,0.0)?,
             grid_to_frame(in_data,event,plane,position*width,height)?)
        }else{
            (grid_to_frame(in_data,event,plane,0.0,position*height)?,
             grid_to_frame(in_data,event,plane,width,position*height)?)
        };
        if source==anchor.source {
            // A short edge cap denotes a reference, not another draggable guide.
            let length=((b.x-a.x).powi(2)+(b.y-a.y).powi(2)).sqrt();
            if length.is_finite() && length>1e-3 {
                let fraction=(8.0/length).min(1.0);
                draw_segment(supplier,surface,&cap,a,ae::drawbot::PointF32{
                    x:a.x+(b.x-a.x)*fraction,y:a.y+(b.y-a.y)*fraction})?;
            }
        }else{draw_segment(supplier,surface,&pen,a,b)?;}
    }Ok(())
}

fn draw_viewer(
    in_data: &ae::InData,
    params: &mut ae::Parameters<Params>,
    event: &mut ae::EventExtra,
) -> Result<(), ae::Error> {
    if event.in_flags().contains(ae::EventInFlags::DONT_DRAW) {
        return Ok(());
    }
    let plane = ViewPlane::read(in_data, params, event)?;
    if plane.projection_unavailable {
        event.set_event_out_flags(ae::EventOutFlags::HANDLED_EVENT);
        return Ok(());
    }
    let controls = control_grid::read(in_data,params)?;
    let grid = controls.grid;
    let drawbot = event.context_handle().drawing_reference()?;
    let supplier = drawbot.supplier()?;
    let surface = drawbot.surface()?;
    // DRAWBOT exposes strokes, not a supported destination-invert operation.
    // Opaque black/white strokes remain distinct even at middle gray.
    let color = overlay_color(1.0);
    let dark = overlay_color(0.0);
    let pen = supplier.new_pen(&color, 0.5)?;
    let outline = supplier.new_pen(&dark, 1.5)?;
    let grip = supplier.new_pen(&color, 1.5)?;
    let grip_outline = supplier.new_pen(&dark, 3.0)?;
    let stroke = |a, b| -> Result<(), ae::Error> {
        draw_segment(&supplier, &surface, &outline, a, b)?;
        draw_segment(&supplier, &surface, &pen, a, b)
    };
    let width = in_data.width().max(1) as f32;
    let height = in_data.height().max(1) as f32;

    if let Some(corners) = plane.state.corner_controls() {
        for i in 0..4 {
            let p=layer_to_frame(in_data,event,&plane,corners[2*i] as f32,corners[2*i+1] as f32)?;
            // Four visible, frame-sized corner grips, including invalid quads so
            // the user can repair them. Native point controls remain available.
            let pts=[(p.x-5.0,p.y-5.0),(p.x+5.0,p.y-5.0),(p.x+5.0,p.y+5.0),(p.x-5.0,p.y+5.0)];
            for j in 0..4 {
                stroke(ae::drawbot::PointF32{x:pts[j].0,y:pts[j].1},
                    ae::drawbot::PointF32{x:pts[(j+1)%4].0,y:pts[(j+1)%4].1})?;
            }
        }
    }
    if plane.invalid() {
        event.set_event_out_flags(ae::EventOutFlags::HANDLED_EVENT);
        return Ok(());
    }

    // Border keeps the overlay visually tied to the source layer even when the
    // layer itself is scaled/rotated in a Comp viewer.
    let p00 = grid_to_frame(in_data, event, &plane, 0.0, 0.0)?;
    let p10 = grid_to_frame(in_data, event, &plane, width, 0.0)?;
    let p11 = grid_to_frame(in_data, event, &plane, width, height)?;
    let p01 = grid_to_frame(in_data, event, &plane, 0.0, height)?;
    stroke(p00, p10)?;
    stroke(p10, p11)?;
    stroke(p11, p01)?;
    stroke(p01, p00)?;

    let draw_handles = grid.column_lines.len().max(grid.row_lines.len()) <= 34;
    for i in 1..grid.column_lines.len().saturating_sub(1) {
        let x = grid.column_lines[i] * width;
        let a = grid_to_frame(in_data, event, &plane, x, 0.0)?;
        let b = grid_to_frame(in_data, event, &plane, x, height)?;
        let crossings = column_crossings(in_data, event, &plane, &grid, x)?;
        let (lines, grips) = guide_segments(a, b, &crossings,
            draw_handles && grid.column_pins.get(i).copied().unwrap_or(0) == 0);
        for (a, b) in lines { stroke(a, b)?; }
        for (a, b) in grips {
            draw_segment(&supplier, &surface, &grip_outline, a, b)?;
            draw_segment(&supplier, &surface, &grip, a, b)?;
        }
    }
    for i in 1..grid.row_lines.len().saturating_sub(1) {
        let y = grid.row_lines[i] * height;
        let a = grid_to_frame(in_data, event, &plane, 0.0, y)?;
        let b = grid_to_frame(in_data, event, &plane, width, y)?;
        let (lines, grips) = guide_segments(a, b, &[],
            draw_handles && grid.row_pins.get(i).copied().unwrap_or(0) == 0);
        for (a, b) in lines { stroke(a, b)?; }
        for (a, b) in grips {
            draw_segment(&supplier, &surface, &grip_outline, a, b)?;
            draw_segment(&supplier, &surface, &grip, a, b)?;
        }
    }

    draw_range(in_data,params,event,&plane,&supplier,&surface)?;
    event.set_event_out_flags(ae::EventOutFlags::HANDLED_EVENT);
    Ok(())
}

fn draw_effect_control(
    _in_data: &ae::InData,
    params: &ae::Parameters<Params>,
    event: &mut ae::EventExtra,
) -> Result<(), ae::Error> {
    if params.index(Params::GridState) == Some(event.param_index()) {
        grid_row::draw(event)?;
    } else if params.index(Params::VersionRow)==Some(event.param_index()) {
        version_row::draw(event)?;
    }
    Ok(())
}

pub fn draw(
    in_data: &ae::InData,
    params: &mut ae::Parameters<Params>,
    event: &mut ae::EventExtra,
) -> Result<(), ae::Error> {
    match event.window_type() {
        ae::WindowType::Comp | ae::WindowType::Layer => draw_viewer(in_data, params, event),
        ae::WindowType::Effect => draw_effect_control(in_data, params, event),
    }
}

pub fn click(
    in_data: &ae::InData,
    params: &mut ae::Parameters<Params>,
    event: &mut ae::EventExtra,
) -> Result<(), ae::Error> {
    if event.window_type() == ae::WindowType::Effect {
        if params.index(Params::GridState) == Some(event.param_index()) {
            grid_row::click(event)?;
        }
        return Ok(());
    }
    if event.window_type() != ae::WindowType::Comp && event.window_type() != ae::WindowType::Layer {
        return Ok(());
    }
    let plane=ViewPlane::read(in_data,params,event)?;
    let controls=control_grid::read(in_data,params)?;
    let grid=&controls.grid;
    let hit=hit_test(in_data, grid, &plane, event, event.screen_point())?;
    #[cfg(feature="preview-overlay-probe")]
    super::preview_overlay_probe::interaction(in_data,event,
        hit.map(|(axis,index)|(axis,index as isize)).unwrap_or((-1,-1)),false);
    if let Some((axis, index)) = hit {
        if axis==DRAG_COLUMNS || axis==DRAG_ROWS {
            let saved=grid_snapshot(params)?;
            let (refs,side)=if axis==DRAG_COLUMNS {(&controls.column_refs,saved.column_lines.len())}
                else{(&controls.row_refs,saved.row_lines.len())};
            let anchor=range_feedback::Anchor{column:axis==DRAG_COLUMNS,source:refs[index]/(side-1) as f32};
            range_feedback::write(event,range_feedback::record(&saved,anchor));
        }
        event.set_continue_refcon(0, axis as _);
        event.set_continue_refcon(1, index as _);
        event.set_continue_refcon(2, grid.columns as _);
        event.set_continue_refcon(3, grid.rows as _);
        event.set_send_drag(true);
        set_drag_cursor(true);
        event.set_event_out_flags(ae::EventOutFlags::HANDLED_EVENT | ae::EventOutFlags::UPDATE_NOW);
    }
    Ok(())
}

pub fn drag(
    in_data: &ae::InData,
    params: &mut ae::Parameters<Params>,
    event: &mut ae::EventExtra,
) -> Result<(), ae::Error> {
    if event.window_type() == ae::WindowType::Effect {
        return grid_row::drag(params, event);
    }
    let result = drag_inner(in_data, params, event);
    #[cfg(feature="preview-overlay-probe")]
    super::preview_overlay_probe::interaction(in_data,event,
        (event.continue_refcon(0),event.continue_refcon(1)),result.is_err());
    if result.is_err() || event.last_time() || !event.send_drag() {
        event.set_continue_refcon(0, DRAG_NONE as _);
        event.set_send_drag(false);
        release_cursor();
    }
    result
}

fn drag_inner(
    in_data: &ae::InData,
    params: &mut ae::Parameters<Params>,
    event: &mut ae::EventExtra,
) -> Result<(), ae::Error> {
    let axis = event.continue_refcon(0);
    let index = event.continue_refcon(1) as usize;
    if axis == DRAG_NONE || (axis != DRAG_COLUMNS && axis != DRAG_ROWS && axis != DRAG_CORNER) {
        event.set_send_drag(false);
        return Ok(());
    }
    set_drag_cursor(true);

    let plane=ViewPlane::read(in_data,params,event)?;
    let Ok((layer_x, layer_y)) = frame_to_layer(in_data, event, &plane, event.screen_point()) else {
        event.set_send_drag(false);return Ok(());
    };
    if axis == DRAG_CORNER {
        if index>=4 || plane.state.corner_controls().is_none() {event.set_send_drag(false);return Ok(());}
        let (layer_x,layer_y)=if let Some(basis)=plane.state.parameter_basis {
            let Some((x,y))=plane::parameter_point(&basis,layer_x as f64,layer_y as f64,
                plane.width as f64,plane.height as f64) else {event.set_send_drag(false);return Ok(());};
            (x as f32,y as f32)
        } else {(layer_x,layer_y)};
        let mut param=params.get_mut(plane::CORNERS[index])?;
        param.as_point_mut()?.set_value((layer_x,layer_y));
        param.set_value_changed();
        event.set_event_out_flags(ae::EventOutFlags::HANDLED_EVENT | ae::EventOutFlags::ALWAYS_UPDATE | ae::EventOutFlags::UPDATE_NOW);
        event.set_send_drag(!event.last_time());
        return Ok(());
    }
    let Some((local_x,local_y))=plane.local(layer_x,layer_y) else {
        event.set_send_drag(false);return Ok(());
    };
    let mut grid = grid_snapshot(params)?;
    let before = grid.clone();
    let elastic = elastic_params(params)?;
    let displayed = control_grid::read(in_data,params)?;
    let render = evaluated_params(params,*in_data,&grid)?;
    // A reentrant density change must not redirect an in-flight drag to another handle.
    if event.continue_refcon(2) != displayed.grid.columns as ae::sys::A_intptr_t ||
        event.continue_refcon(3) != displayed.grid.rows as ae::sys::A_intptr_t {
        event.set_send_drag(false); return Ok(());
    }
    let (positions,refs,lines,pins,target) = if axis == DRAG_COLUMNS {
        (&displayed.grid.column_lines,&displayed.column_refs,
         &mut grid.column_lines,&grid.column_pins,local_x)
    } else {
        (&displayed.grid.row_lines,&displayed.row_refs,
         &mut grid.row_lines,&grid.row_pins,local_y)
    };
    if index == 0 || index+1 >= positions.len() {
        event.set_send_drag(false); return Ok(());
    }
    let anchor=range_feedback::Anchor{column:axis==DRAG_COLUMNS,source:refs[index]/(lines.len()-1) as f32};
    control_grid::drag_live(lines,pins,refs[index],target,&elastic,&render,axis==DRAG_COLUMNS)?;
    // Only a real deformation edit may write the animated arbitrary parameter.
    if grid != before {
        control_layout::freeze_for_drag(in_data,params,axis==DRAG_COLUMNS)?;
        let marker=range_feedback::record(&grid,anchor);
        params.get_mut(Params::GridState)?.as_arbitrary_mut()?.set_value(grid)?;
        range_feedback::write(event,marker);
        event.set_event_out_flags(
            ae::EventOutFlags::HANDLED_EVENT
                | ae::EventOutFlags::ALWAYS_UPDATE
                | ae::EventOutFlags::UPDATE_NOW,
        );
    }

    if event.last_time() {
        event.set_continue_refcon(0, DRAG_NONE as _);
        event.set_continue_refcon(1, 0);
        event.set_send_drag(false);
    } else {
        event.set_send_drag(true);
    }
    Ok(())
}

pub fn adjust_cursor(
    in_data: &ae::InData,
    params: &mut ae::Parameters<Params>,
    event: &mut ae::EventExtra,
) -> Result<(), ae::Error> {
    if event.window_type() != ae::WindowType::Comp && event.window_type() != ae::WindowType::Layer {
        return Ok(());
    }
    let dragging = GUIDE_DRAGGING.get();
    if !dragging {
        let plane=ViewPlane::read(in_data,params,event)?;
        let controls=control_grid::read(in_data,params)?;
    let grid=&controls.grid;
        if hit_test(in_data, grid, &plane, event, event.screen_point())?.is_none() {
            // Documented AdjustCursor handoff; never call PF_SetCursor(NONE).
            // Do not mark handled, so AE can use the currently selected tool.
            return Ok(());
        }
    }
    event.set_cursor(hand_cursor(dragging));
    event.set_event_out_flags(ae::EventOutFlags::HANDLED_EVENT);
    Ok(())
}

#[cfg(test)]
mod cursor_tests {
    #[test]
    fn only_known_callback_windows_enter_wrapper_conversion() {
        for w in [ae::sys::PF_Window_COMP,ae::sys::PF_Window_LAYER,ae::sys::PF_Window_EFFECT] {
            assert!(super::known_window(w));
        }
        for w in [ae::sys::PF_Window_NONE,ae::sys::PF_Window_PREVIEW,99] {
            assert!(!super::known_window(w));
        }
    }
    #[test]
    fn projected_text_quad_picking_uses_comp_coordinates_once() {
        use super::*;
        // Recorded native AE Y30 quad. This tests geometry, not host callbacks.
        let state=plane::State {corners:Some([
            193.126991294881,144.064876060526,474.465671864278,124.959806473857,
            474.465671864278,621.680710828136,193.126991294881,558.293851707426
        ]),editable_corners:false,comp_space:true,parameter_basis:None};
        let p=ViewPlane {geometry:state.geometry(),state,width:640.0,height:480.0,
            projection:None,projection_unavailable:false,comp_space:true};
        assert!(!p.needs_layer_conversion(ae::WindowType::Comp));
        assert!(p.state.corner_controls().is_none());
        for x in [0.0,0.25,0.5,1.0] {for y in [0.0,0.4,1.0] {
            let q=p.geometry.as_ref().unwrap().map(false,x,y).unwrap();
            let local=p.local(q.0 as f32,q.1 as f32).unwrap();
            assert!((local.0 as f64-x).abs()<1e-6);
            assert!((local.1 as f64-y).abs()<1e-6);
        }}
        let mut ordinary=p;
        ordinary.comp_space=false;
        assert!(ordinary.needs_layer_conversion(ae::WindowType::Comp));
        assert!(!ordinary.needs_layer_conversion(ae::WindowType::Layer));
    }
    use super::*;

    #[test]
    fn split_grips_and_gap_follow_reference() {
        let a = ae::drawbot::PointF32 { x: 0.0, y: 0.0 };
        let b = ae::drawbot::PointF32 { x: 0.0, y: 200.0 };
        let (lines, grips) = guide_segments(a, b, &[ae::drawbot::PointF32 {x:0.0,y:100.0}], true);
        assert_eq!(lines.len(), 2);
        assert_eq!(grips.len(), 2);
        assert_eq!((lines[0].1.y, lines[1].0.y), (94.0, 106.0));
        assert_eq!((grips[0].0.y, grips[1].1.y), (76.0, 124.0));
        assert!(lines.iter().all(|&(a,b)| point_segment_distance(0.0,100.0,a,b) >= GUIDE_GAP * 0.5));
    }

    #[test]
    fn horizontal_grip_rotates_with_guide_and_keeps_frame_length() {
        let a = ae::drawbot::PointF32 { x: 0.0, y: 0.0 };
        let b = ae::drawbot::PointF32 { x: 120.0, y: 160.0 };
        let (_, grips) = guide_segments(a, b, &[], true);
        assert_eq!(grips.len(), 1);
        let (g0, g1) = grips[0];
        assert!(((g1.x-g0.x).hypot(g1.y-g0.y) - GRIP_LENGTH).abs() < 0.001);
        assert!(point_segment_distance(g0.x,g0.y,a,b) < 0.001);
        assert!(point_segment_distance(g1.x,g1.y,a,b) < 0.001);
        assert!(point_segment_distance(60.0,80.0,a,b) < HIT_SLOP);
        assert!(point_segment_distance(100.0,0.0,a,b) > HIT_SLOP);
    }

    #[test]
    fn short_dense_and_degenerate_guides_have_no_grips() {
        let a = ae::drawbot::PointF32 { x: 0.0, y: 0.0 };
        for (length, handles) in [(20.0,true), (200.0,false), (0.0,true)] {
            let (lines, grips) = guide_segments(a, ae::drawbot::PointF32 {x:length,y:0.0}, &[], handles);
            assert!(grips.is_empty());
            assert_eq!(lines.len(), usize::from(length > 0.0));
        }
    }

    #[test]
    fn gaps_follow_moving_intersections_without_a_phantom_center_gap() {
        let p = |y| ae::drawbot::PointF32 { x: 0.0, y };
        for middle in [40.0, 80.0, 140.0] {
            let (lines, _) = guide_segments(p(0.0), p(200.0), &[p(20.0), p(middle), p(180.0)], false);
            assert_eq!(lines.len(), 4);
            assert_eq!(lines[1].1.y, middle - 6.0);
            assert_eq!(lines[2].0.y, middle + 6.0);
            assert!(lines.iter().any(|&(a,b)| a.y <= 100.0 && b.y >= 100.0));
        }
    }

    #[test]
    fn overlapping_gaps_merge_and_rotated_crossings_project_correctly() {
        let p = |s: f32| ae::drawbot::PointF32 { x: s * 0.6, y: s * 0.8 };
        let (lines, _) = guide_segments(p(0.0), p(200.0), &[p(101.0), p(99.0), p(99.0)], true);
        assert_eq!(lines.len(), 2);
        assert!((lines[0].1.x - p(93.0).x).abs() < 0.001);
        assert!((lines[1].0.y - p(107.0).y).abs() < 0.001);
    }

    #[test]
    fn drag_cursor_state_starts_idle_and_can_be_cleared_without_host() {
        GUIDE_DRAGGING.set(false);
        assert!(!GUIDE_DRAGGING.get());
        GUIDE_DRAGGING.set(true);
        assert!(GUIDE_DRAGGING.get());
        GUIDE_DRAGGING.set(false);
        assert!(!GUIDE_DRAGGING.get());
    }
}
