use super::*;

const HIT_SLOP: f32 = 9.0;
const DRAG_NONE: isize = 0;
const DRAG_COLUMNS: isize = 1;
const DRAG_ROWS: isize = 2;

fn overlay_color(in_data: &ae::InData) -> ae::drawbot::ColorRgba {
    let fallback = ae::drawbot::ColorRgba {
        red: 0.85,
        green: 0.85,
        blue: 0.85,
        alpha: 0.9,
    };
    if in_data.is_premiere() {
        return fallback;
    }
    ae::pf::suites::EffectCustomUIOverlayTheme::new()
        .and_then(|suite| suite.preferred_foreground_color())
        .unwrap_or(fallback)
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
        let d = point_segment_distance(mouse.h as f32, mouse.v as f32, a, b);
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
    let color = overlay_color(in_data);
    let pen = supplier.new_pen(&color, 1.0)?;
    let width = in_data.width().max(1) as f32;
    let height = in_data.height().max(1) as f32;

    // Border keeps the overlay visually tied to the source layer even when the
    // layer itself is scaled/rotated in a Comp viewer.
    let p00 = layer_to_frame(in_data, event, 0.0, 0.0)?;
    let p10 = layer_to_frame(in_data, event, width, 0.0)?;
    let p11 = layer_to_frame(in_data, event, width, height)?;
    let p01 = layer_to_frame(in_data, event, 0.0, height)?;
    draw_segment(&supplier, &surface, &pen, p00, p10)?;
    draw_segment(&supplier, &surface, &pen, p10, p11)?;
    draw_segment(&supplier, &surface, &pen, p11, p01)?;
    draw_segment(&supplier, &surface, &pen, p01, p00)?;

    let draw_handles = grid.column_lines.len().max(grid.row_lines.len()) <= 34;
    for i in 1..grid.column_lines.len().saturating_sub(1) {
        let x = grid.column_lines[i] * width;
        let a = layer_to_frame(in_data, event, x, 0.0)?;
        let b = layer_to_frame(in_data, event, x, height)?;
        draw_segment(&supplier, &surface, &pen, a, b)?;
        if draw_handles {
            let mid = ae::drawbot::RectF32 {
                left: (a.x + b.x) * 0.5 - 2.5,
                top: (a.y + b.y) * 0.5 - 2.5,
                width: 5.0,
                height: 5.0,
            };
            surface.paint_rect(&color, &mid)?;
        }
    }
    for i in 1..grid.row_lines.len().saturating_sub(1) {
        let y = grid.row_lines[i] * height;
        let a = layer_to_frame(in_data, event, 0.0, y)?;
        let b = layer_to_frame(in_data, event, width, y)?;
        draw_segment(&supplier, &surface, &pen, a, b)?;
        if draw_handles {
            let mid = ae::drawbot::RectF32 {
                left: (a.x + b.x) * 0.5 - 2.5,
                top: (a.y + b.y) * 0.5 - 2.5,
                width: 5.0,
                height: 5.0,
            };
            surface.paint_rect(&color, &mid)?;
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
    let label = format!("{} × {} — SMARTFX-SNAPSHOT", grid.columns, grid.rows);
    let origin = ae::drawbot::PointF32 {
        x: frame.left as f32 + 5.0,
        y: frame.top as f32 + 4.0,
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
        event.set_event_out_flags(ae::EventOutFlags::HANDLED_EVENT | ae::EventOutFlags::UPDATE_NOW);
    }
    Ok(())
}

pub fn drag(
    in_data: &ae::InData,
    params: &mut ae::Parameters<Params>,
    event: &mut ae::EventExtra,
) -> Result<(), ae::Error> {
    let axis = event.continue_refcon(0);
    let index = event.continue_refcon(1) as usize;
    if axis == DRAG_NONE || (axis != DRAG_COLUMNS && axis != DRAG_ROWS) {
        return Ok(());
    }

    let (layer_x, layer_y) = frame_to_layer(in_data, event, event.screen_point())?;
    let width = in_data.width().max(1) as f32;
    let height = in_data.height().max(1) as f32;
    let mut grid = grid_snapshot(params)?;
    let elastic = elastic_params(params)?;

    let rc = if axis == DRAG_COLUMNS {
        if index == 0 || index + 1 >= grid.column_lines.len() {
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
    let grid = grid_snapshot(params)?;
    if hit_test(in_data, &grid, event, event.screen_point())?.is_some() {
        event.set_cursor(ae::CursorType::Crosshairs);
        event.set_event_out_flags(ae::EventOutFlags::HANDLED_EVENT);
    }
    Ok(())
}
