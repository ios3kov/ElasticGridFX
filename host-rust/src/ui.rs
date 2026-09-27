use super::*;

const HIT_SLOP: f32 = 10.0;
const DRAG_NONE: isize = 0;
const DRAG_POINT: isize = 1;

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

fn point_distance(px: f32, py: f32, p: ae::drawbot::PointF32) -> f32 {
    ((px - p.x).powi(2) + (py - p.y).powi(2)).sqrt()
}

fn node_pinned(grid: &GridArb, index: usize) -> bool {
    grid.column_pins.get(index).copied().unwrap_or(1) != 0
        || grid.row_pins.get(index).copied().unwrap_or(1) != 0
}

fn hit_test(
    in_data: &ae::InData,
    grid: &GridArb,
    event: &ae::EventExtra,
    mouse: ae::Point,
) -> Result<Option<usize>, ae::Error> {
    if event.window_type() != ae::WindowType::Comp && event.window_type() != ae::WindowType::Layer {
        return Ok(None);
    }
    let width = in_data.width().max(1) as f32;
    let height = in_data.height().max(1) as f32;
    let columns = grid.columns as usize;
    let rows = grid.rows as usize;
    let mut best: Option<(f32, usize)> = None;

    for row in 1..rows {
        for column in 1..columns {
            let index = grid.node_index(column, row);
            if node_pinned(grid, index) {
                continue;
            }
            let (x, y) = grid.point(column, row);
            let p = layer_to_frame(in_data, event, x * width, y * height)?;
            let d = point_distance(mouse.h as f32, mouse.v as f32, p);
            if d <= HIT_SLOP && best.map(|v| d < v.0).unwrap_or(true) {
                best = Some((d, index));
            }
        }
    }
    Ok(best.map(|(_, index)| index))
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
    let columns = grid.columns as usize;
    let rows = grid.rows as usize;

    let frame_point = |column: usize, row: usize| -> Result<ae::drawbot::PointF32, ae::Error> {
        let (x, y) = grid.point(column, row);
        layer_to_frame(in_data, event, x * width, y * height)
    };

    for row in 0..=rows {
        for column in 0..columns {
            draw_segment(
                &supplier,
                &surface,
                &pen,
                frame_point(column, row)?,
                frame_point(column + 1, row)?,
            )?;
        }
    }
    for column in 0..=columns {
        for row in 0..rows {
            draw_segment(
                &supplier,
                &surface,
                &pen,
                frame_point(column, row)?,
                frame_point(column, row + 1)?,
            )?;
        }
    }

    if columns.max(rows) <= 32 {
        for row in 1..rows {
            for column in 1..columns {
                let index = grid.node_index(column, row);
                if node_pinned(&grid, index) {
                    continue;
                }
                let p = frame_point(column, row)?;
                let handle = ae::drawbot::RectF32 {
                    left: p.x - 3.0,
                    top: p.y - 3.0,
                    width: 6.0,
                    height: 6.0,
                };
                surface.paint_rect(&color, &handle)?;
            }
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
    let label = format!("{} × {} — drag mesh points in Viewer", grid.columns, grid.rows);
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

fn falloff_weight(profile: i32, t: f32) -> f32 {
    let x = t.clamp(0.0, 1.0);
    match profile {
        1 => 1.0 - x,
        3 => (-4.5 * x * x).exp(),
        4 => (x * std::f32::consts::FRAC_PI_2).cos().max(0.0),
        _ => {
            let s = x * x * (3.0 - 2.0 * x);
            1.0 - s
        }
    }
}

fn move_mesh_point(
    grid: &mut GridArb,
    index: usize,
    target_x: f32,
    target_y: f32,
    elastic: &EgElasticParams,
) -> bool {
    if !grid.is_valid() || index >= grid.column_lines.len() || node_pinned(grid, index) {
        return false;
    }

    let original = grid.clone();
    let columns = grid.columns as usize;
    let rows = grid.rows as usize;
    let stride = columns + 1;
    let selected_row = index / stride;
    let selected_column = index % stride;
    if selected_column == 0 || selected_column >= columns || selected_row == 0 || selected_row >= rows {
        return false;
    }

    let old_x = original.column_lines[index];
    let old_y = original.row_lines[index];
    let dx = target_x.clamp(0.0, 1.0) - old_x;
    let dy = target_y.clamp(0.0, 1.0) - old_y;
    let radius = elastic.tension_radius.max(0.0);
    let strength = elastic.elasticity_strength.clamp(0.0, 2.0);

    for row in 1..rows {
        for column in 1..columns {
            let i = row * stride + column;
            if node_pinned(&original, i) {
                continue;
            }
            let dc = column as f32 - selected_column as f32;
            let dr = row as f32 - selected_row as f32;
            let distance = (dc * dc + dr * dr).sqrt();
            let weight = if i == index {
                1.0
            } else if radius <= 0.0 || distance > radius {
                0.0
            } else {
                falloff_weight(elastic.falloff, distance / radius) * strength
            };
            grid.column_lines[i] = original.column_lines[i] + dx * weight;
            grid.row_lines[i] = original.row_lines[i] + dy * weight;
        }
    }

    // Keep the mesh non-folding: each row must remain left-to-right and each
    // column top-to-bottom. This preserves a stable inverse warp under drag.
    let spacing_x = elastic.min_spacing.clamp(0.0, 0.45 / columns.max(1) as f32);
    let spacing_y = elastic.min_spacing.clamp(0.0, 0.45 / rows.max(1) as f32);
    for _ in 0..4 {
        for row in 1..rows {
            for column in 1..columns {
                let i = row * stride + column;
                if node_pinned(grid, i) { continue; }
                let left = row * stride + column - 1;
                let right = row * stride + column + 1;
                let lo = grid.column_lines[left] + spacing_x;
                let hi = grid.column_lines[right] - spacing_x;
                if lo <= hi {
                    grid.column_lines[i] = grid.column_lines[i].clamp(lo, hi);
                }
            }
        }
        for column in 1..columns {
            for row in 1..rows {
                let i = row * stride + column;
                if node_pinned(grid, i) { continue; }
                let up = (row - 1) * stride + column;
                let down = (row + 1) * stride + column;
                let lo = grid.row_lines[up] + spacing_y;
                let hi = grid.row_lines[down] - spacing_y;
                if lo <= hi {
                    grid.row_lines[i] = grid.row_lines[i].clamp(lo, hi);
                }
            }
        }
    }

    if grid.is_valid() {
        true
    } else {
        *grid = original;
        false
    }
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
    if let Some(index) = hit_test(in_data, &grid, event, event.screen_point())? {
        event.set_continue_refcon(0, DRAG_POINT as _);
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
    if event.continue_refcon(0) != DRAG_POINT {
        return Ok(());
    }
    let index = event.continue_refcon(1) as usize;
    let (layer_x, layer_y) = frame_to_layer(in_data, event, event.screen_point())?;
    let width = in_data.width().max(1) as f32;
    let height = in_data.height().max(1) as f32;
    let mut grid = grid_snapshot(params)?;
    let elastic = elastic_params(params)?;

    if move_mesh_point(
        &mut grid,
        index,
        layer_x / width,
        layer_y / height,
        &elastic,
    ) {
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
