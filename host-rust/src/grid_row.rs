//! Topic-only arbitrary parameter UI. Never draws into rendered image pixels.
use super::*;
use ae::drawbot::{ColorRgba, PointF32, RectF32, TextAlignment, TextTruncation};

fn button(frame: ae::Rect, offset: i32) -> Option<RectF32> {
    // AE supplies the value-column offset; follow its divider when the panel
    // is resized rather than anchoring Reset to the panel's right edge.
    let left = frame.left as f32 + offset as f32;
    let available = frame.right as f32 - left - 6.0;
    let height = (frame.bottom - frame.top) as f32;
    if offset < 120 || available < 60.0 || height < 12.0 { return None; }
    Some(RectF32 { left, top: frame.top as f32 + 1.0,
        width: available.min(130.0), height: (height - 2.0).min(24.0) })
}
fn contains(rect: RectF32, point: ae::Point) -> bool {
    let (x, y) = (point.h as f32, point.v as f32);
    x >= rect.left && x < rect.left + rect.width &&
        y >= rect.top && y < rect.top + rect.height
}

pub(crate) fn draw(event: &mut ae::EventExtra) -> Result<(), ae::Error> {
    if event.effect_area() != ae::EffectArea::Title { return Ok(()); }
    let frame = event.param_title_frame();
    let Some(rect) = button(frame, event.horiz_offset()) else { return Ok(()); };
    let drawbot = event.context_handle().drawing_reference()?;
    let supplier = drawbot.supplier()?;
    let surface = drawbot.surface()?;
    let size = supplier.default_font_size()?;
    let font = supplier.new_default_font(size)?;
    let app = ae::pf::suites::App::new()?;
    let color = |kind| -> Result<ColorRgba, ae::Error> {
        let c = app.color(kind)?;
        Ok(ColorRgba { red: c.red as f32 / 65535.0, green: c.green as f32 / 65535.0,
            blue: c.blue as f32 / 65535.0, alpha: 1.0 })
    };
    let text = supplier.new_brush(&color(ae::AppColorType::ButtonText)?)?;
    let background = supplier.new_brush(&color(ae::AppColorType::ButtonFill)?)?;
    let mut path = supplier.new_path()?;
    path.add_rounded_rect(&rect, rect.height * 0.5)?;
    surface.fill_path(&background, &path, ae::drawbot::FillType::Winding)?;
    // AE already draws the title and stopwatch even for topic-only UI.
    // Draw only Reset, leaving the native caption/animation hit area intact.
    surface.draw_string(&text, &font, "Reset", &PointF32 {
        x: rect.left + rect.width * 0.5, y: rect.top + (rect.height + size) * 0.5 - 2.0,
    }, TextAlignment::Center, TextTruncation::None, rect.width)?;
    event.set_event_out_flags(ae::EventOutFlags::HANDLED_EVENT);
    Ok(())
}

pub(crate) fn click(event: &mut ae::EventExtra) -> Result<(), ae::Error> {
    if event.effect_area() != ae::EffectArea::Title { return Ok(()); }
    if button(event.param_title_frame(), event.horiz_offset())
        .is_some_and(|rect| contains(rect, event.screen_point())) {
        // PF_CHANGE_VALUE is valid in DRAG, not DO_CLICK. Arm the button and
        // commit on release inside its bounds, allowing drag-out cancellation.
        event.set_continue_refcon(0, 1);
        event.set_send_drag(true);
        event.set_event_out_flags(ae::EventOutFlags::HANDLED_EVENT);
    }
    Ok(())
}

pub(crate) fn drag(params: &mut ae::Parameters<Params>, event: &mut ae::EventExtra) -> Result<(), ae::Error> {
    if event.continue_refcon(0) != 1 { return Ok(()); }
    event.set_event_out_flags(ae::EventOutFlags::HANDLED_EVENT);
    if event.last_time() {
        event.set_continue_refcon(0, 0);
        event.set_send_drag(false);
        if button(event.param_title_frame(), event.horiz_offset())
            .is_some_and(|rect| contains(rect, event.screen_point())) && reset_grid::apply(params)? {
            event.set_event_out_flags(ae::EventOutFlags::HANDLED_EVENT |
                ae::EventOutFlags::ALWAYS_UPDATE | ae::EventOutFlags::UPDATE_NOW);
        }
    } else { event.set_send_drag(true); }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn reset_hit_area_preserves_title_and_refuses_outside_points() {
        let frame = ae::Rect { left: 40, top: 50, right: 340, bottom: 72 };
        let rect = button(frame, 180).unwrap();
        assert!(rect.left >= frame.left as f32 + 180.0);
        assert!(contains(rect, ae::Point { h: 300, v: 60 }));
        for point in [ae::Point { h: 50, v: 60 }, ae::Point { h: 300, v: 40 },
            ae::Point { h: 334, v: 60 }, ae::Point { h: 300, v: 71 }] {
            assert!(!contains(rect, point));
        }
        assert!(button(ae::Rect { right: 180, ..frame }, 180).is_none());
    }
}
