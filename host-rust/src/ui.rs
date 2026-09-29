use super::*;

const HIT_SLOP: f32 = 9.0;
const GRIP_LENGTH: f32 = 48.0;
const GUIDE_GAP: f32 = 24.0;
const DRAG_NONE: isize = 0;
const DRAG_COLUMNS: isize = 1;
const DRAG_ROWS: isize = 2;

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

fn overlay_color(value: f32) -> ae::drawbot::ColorRgba {
    ae::drawbot::ColorRgba { red: value, green: value, blue: value, alpha: 1.0 }
}

type Segment = (ae::drawbot::PointF32, ae::drawbot::PointF32);

// Layout in frame coordinates, so grips keep their size under layer zoom and
// follow rotated guides. Short/dense guides retain a plain line.
fn guide_segments(a: ae::drawbot::PointF32, b: ae::drawbot::PointF32,
                  split: bool, handles: bool) -> (Vec<Segment>, Vec<Segment>) {
    let length = ((b.x - a.x).powi(2) + (b.y - a.y).powi(2)).sqrt();
    if !length.is_finite() || length <= 1.0e-3 {
        return (Vec::new(), Vec::new());
    }
    if !handles || length < GRIP_LENGTH + GUIDE_GAP + 8.0 {
        return (vec![(a, b)], Vec::new());
    }
    let point = |offset: f32| ae::drawbot::PointF32 {
        x: (a.x + b.x) * 0.5 + (b.x - a.x) * offset / length,
        y: (a.y + b.y) * 0.5 + (b.y - a.y) * offset / length,
    };
    if split {
        let gap = GUIDE_GAP * 0.5;
        (vec![(a, point(-gap)), (point(gap), b)],
         vec![(point(-gap - GRIP_LENGTH * 0.5), point(-gap)),
              (point(gap), point(gap + GRIP_LENGTH * 0.5))])
    } else {
        (vec![(a, b)], vec![(point(-GRIP_LENGTH * 0.5), point(GRIP_LENGTH * 0.5))])
    }
}

fn layer_to_frame(
    in_data: &ae::InData,
    event: &ae::EventExtra,
    x: f32,
    y: f32,
) -> Result<ae::drawbot::PointF32, ae::Error> {
    let mut p = ae::sys::PF_FixedPoint {
        x: ae::Fixed::from(x).as_fixed(),
        y: ae::Fixed::from(y).as_fixed(),
    };
    if event.window_type() == ae::WindowType::Comp {
        event.callbacks().layer_to_comp(in_data.current_time(), in_data.time_scale(), &mut p)?;
    }
    event.callbacks().source_to_frame(&mut p)?;
    Ok(ae::drawbot::PointF32 {
        x: ae::Fixed::from_fixed(p.x).as_f32(),
        y: ae::Fixed::from_fixed(p.y).as_f32(),
    })
}

fn frame_to_layer(
    in_data: &ae::InData,
    event: &ae::EventExtra,
    p: ae::Point,
) -> Result<(f32, f32), ae::Error> {
    let mut fixed = ae::sys::PF_FixedPoint {
        x: ae::Fixed::from_int(p.h).as_fixed(),
        y: ae::Fixed::from_int(p.v).as_fixed(),
    };
    event.callbacks().frame_to_source(&mut fixed)?;
    if event.window_type() == ae::WindowType::Comp {
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
    event: &ae::EventExtra,
    mouse: ae::Point,
) -> Result<Option<(isize, usize)>, ae::Error> {
    if event.window_type() != ae::WindowType::Comp && event.window_type() != ae::WindowType::Layer {
        return Ok(None);
    }
    let width = in_data.width().max(1) as f32;
    let height = in_data.height().max(1) as f32;
    let mut best: Option<(f32, isize, usize)> = None;

    for i in 1..grid.column_lines.len().saturating_sub(1) {
        if grid.column_pins.get(i).copied().unwrap_or(0) != 0 {
            continue;
        }
        let x = grid.column_lines[i] * width;
        let a = layer_to_frame(in_data, event, x, 0.0)?;
        let b = layer_to_frame(in_data, event, x, height)?;
        let (segments, _) = guide_segments(a, b, true,
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
        let a = layer_to_frame(in_data, event, 0.0, y)?;
        let b = layer_to_frame(in_data, event, width, y)?;
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
    let mut path = supplier.new_path()?;
    path.move_to(a.x, a.y)?;
    path.line_to(b.x, b.y)?;
    surface.stroke_path(pen, &path)?;
    Ok(())
}

fn draw_viewer(
    in_data: &ae::InData,
    params: &ae::Parameters<Params>,
    event: &mut ae::EventExtra,
) -> Result<(), ae::Error> {
    if event.in_flags().contains(ae::EventInFlags::DONT_DRAW) {
        return Ok(());
    }
    let grid = grid_snapshot(params)?;
    let drawbot = event.context_handle().drawing_reference()?;
    let supplier = drawbot.supplier()?;
    let surface = drawbot.surface()?;
    // DRAWBOT exposes strokes, not a supported destination-invert operation.
    // Opaque black/white strokes remain distinct even at middle gray.
    let color = overlay_color(1.0);
    let dark = overlay_color(0.0);
    let pen = supplier.new_pen(&color, 1.0)?;
    let outline = supplier.new_pen(&dark, 3.0)?;
    let grip = supplier.new_pen(&color, 3.0)?;
    let grip_outline = supplier.new_pen(&dark, 5.0)?;
    let stroke = |a, b| -> Result<(), ae::Error> {
        draw_segment(&supplier, &surface, &outline, a, b)?;
        draw_segment(&supplier, &surface, &pen, a, b)
    };
    let width = in_data.width().max(1) as f32;
    let height = in_data.height().max(1) as f32;

    // Border keeps the overlay visually tied to the source layer even when the
    // layer itself is scaled/rotated in a Comp viewer.
    let p00 = layer_to_frame(in_data, event, 0.0, 0.0)?;
    let p10 = layer_to_frame(in_data, event, width, 0.0)?;
    let p11 = layer_to_frame(in_data, event, width, height)?;
    let p01 = layer_to_frame(in_data, event, 0.0, height)?;
    stroke(p00, p10)?;
    stroke(p10, p11)?;
    stroke(p11, p01)?;
    stroke(p01, p00)?;

    let draw_handles = grid.column_lines.len().max(grid.row_lines.len()) <= 34;
    for i in 1..grid.column_lines.len().saturating_sub(1) {
        let x = grid.column_lines[i] * width;
        let a = layer_to_frame(in_data, event, x, 0.0)?;
        let b = layer_to_frame(in_data, event, x, height)?;
        let (lines, grips) = guide_segments(a, b, true,
            draw_handles && grid.column_pins.get(i).copied().unwrap_or(0) == 0);
        for (a, b) in lines { stroke(a, b)?; }
        for (a, b) in grips {
            draw_segment(&supplier, &surface, &grip_outline, a, b)?;
            draw_segment(&supplier, &surface, &grip, a, b)?;
        }
    }
    for i in 1..grid.row_lines.len().saturating_sub(1) {
        let y = grid.row_lines[i] * height;
        let a = layer_to_frame(in_data, event, 0.0, y)?;
        let b = layer_to_frame(in_data, event, width, y)?;
        let (lines, grips) = guide_segments(a, b, false,
            draw_handles && grid.row_pins.get(i).copied().unwrap_or(0) == 0);
        for (a, b) in lines { stroke(a, b)?; }
        for (a, b) in grips {
            draw_segment(&supplier, &surface, &grip_outline, a, b)?;
            draw_segment(&supplier, &surface, &grip, a, b)?;
        }
    }

    event.set_event_out_flags(ae::EventOutFlags::HANDLED_EVENT);
    Ok(())
}

fn draw_effect_control(
    params: &ae::Parameters<Params>,
    event: &mut ae::EventExtra,
) -> Result<(), ae::Error> {
    if event.effect_area() != ae::EffectArea::Control
        || params.index(Params::GridState) != Some(event.param_index())
    {
        return Ok(());
    }

    let grid = grid_snapshot(params)?;
    let drawbot = event.context_handle().drawing_reference()?;
    let supplier = drawbot.supplier()?;
    let surface = drawbot.surface()?;
    let frame = event.current_frame();
    let font = supplier.new_default_font(supplier.default_font_size()?)?;
    let brush = supplier.new_brush(&ae::drawbot::ColorRgba {
        red: 0.85,
        green: 0.85,
        blue: 0.85,
        alpha: 1.0,
    })?;
    let raw_id = build_identity::BUILD_ID.strip_prefix("EGFX-").unwrap_or(build_identity::BUILD_ID);
    let short_len = raw_id.len().min(12);
    let label = format!("{} × {}   EGFX-{}", grid.columns, grid.rows, &raw_id[..short_len]);
    let origin = ae::drawbot::PointF32 {
        x: frame.left as f32 + 6.0,
        y: frame.top as f32 + 9.0,
    };
    surface.draw_string(
        &brush,
        &font,
        &label,
        &origin,
        ae::drawbot::TextAlignment::Left,
        ae::drawbot::TextTruncation::End,
        frame.width() as f32 - 10.0,
    )?;
    event.set_event_out_flags(ae::EventOutFlags::HANDLED_EVENT);
    Ok(())
}

pub fn draw(
    in_data: &ae::InData,
    params: &mut ae::Parameters<Params>,
    event: &mut ae::EventExtra,
) -> Result<(), ae::Error> {
    match event.window_type() {
        ae::WindowType::Comp | ae::WindowType::Layer => draw_viewer(in_data, params, event),
        ae::WindowType::Effect => draw_effect_control(params, event),
    }
}

pub fn click(
    in_data: &ae::InData,
    params: &mut ae::Parameters<Params>,
    event: &mut ae::EventExtra,
) -> Result<(), ae::Error> {
    if event.window_type() != ae::WindowType::Comp && event.window_type() != ae::WindowType::Layer {
        return Ok(());
    }
    let grid = grid_snapshot(params)?;
    if let Some((axis, index)) = hit_test(in_data, &grid, event, event.screen_point())? {
        event.set_continue_refcon(0, axis as _);
        event.set_continue_refcon(1, index as _);
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
    let result = drag_inner(in_data, params, event);
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
    if axis == DRAG_NONE || (axis != DRAG_COLUMNS && axis != DRAG_ROWS) {
        event.set_send_drag(false);
        return Ok(());
    }
    set_drag_cursor(true);

    let (layer_x, layer_y) = frame_to_layer(in_data, event, event.screen_point())?;
    let width = in_data.width().max(1) as f32;
    let height = in_data.height().max(1) as f32;
    let mut grid = grid_snapshot(params)?;
    let elastic = elastic_params(params)?;

    let rc = if axis == DRAG_COLUMNS {
        if index == 0 || index + 1 >= grid.column_lines.len() {
            event.set_send_drag(false);
            return Ok(());
        }
        let target = layer_x / width;
        unsafe {
            eg_drag_axis(
                grid.column_lines.as_mut_ptr(),
                grid.column_pins.as_mut_ptr(),
                grid.column_lines.len() as i32,
                index as i32,
                target,
                &elastic,
            )
        }
    } else {
        if index == 0 || index + 1 >= grid.row_lines.len() {
            event.set_send_drag(false);
            return Ok(());
        }
        let target = layer_y / height;
        unsafe {
            eg_drag_axis(
                grid.row_lines.as_mut_ptr(),
                grid.row_pins.as_mut_ptr(),
                grid.row_lines.len() as i32,
                index as i32,
                target,
                &elastic,
            )
        }
    };

    if rc == 0 {
        params.get_mut(Params::GridState)?.as_arbitrary_mut()?.set_value(grid)?;
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
        let grid = grid_snapshot(params)?;
        if hit_test(in_data, &grid, event, event.screen_point())?.is_none() {
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
    use super::*;

    #[test]
    fn split_grips_and_gap_follow_reference() {
        let a = ae::drawbot::PointF32 { x: 0.0, y: 0.0 };
        let b = ae::drawbot::PointF32 { x: 0.0, y: 200.0 };
        let (lines, grips) = guide_segments(a, b, true, true);
        assert_eq!(lines.len(), 2);
        assert_eq!(grips.len(), 2);
        assert_eq!((lines[0].1.y, lines[1].0.y), (88.0, 112.0));
        assert_eq!((grips[0].0.y, grips[1].1.y), (64.0, 136.0));
        assert!(lines.iter().all(|&(a,b)| point_segment_distance(0.0,100.0,a,b) > HIT_SLOP));
    }

    #[test]
    fn horizontal_grip_rotates_with_guide_and_keeps_frame_length() {
        let a = ae::drawbot::PointF32 { x: 0.0, y: 0.0 };
        let b = ae::drawbot::PointF32 { x: 120.0, y: 160.0 };
        let (_, grips) = guide_segments(a, b, false, true);
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
            let (lines, grips) = guide_segments(a, ae::drawbot::PointF32 {x:length,y:0.0}, true, handles);
            assert!(grips.is_empty());
            assert_eq!(lines.len(), usize::from(length > 0.0));
        }
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
