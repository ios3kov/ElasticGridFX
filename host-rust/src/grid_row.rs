//! Topic-only arbitrary parameter UI. Never draws into rendered image pixels.
use super::*;
use ae::drawbot::{ColorRgba, PointF32, RectF32, TextAlignment, TextTruncation};

fn button(frame: ae::Rect) -> Option<RectF32> {
    let width = (frame.right - frame.left) as f32;
    let height = (frame.bottom - frame.top) as f32;
    // Reserve the left side for Grid Positions and AE's native animation UI.
    if width < 190.0 || height < 12.0 { return None; }
    Some(RectF32 { left: frame.right as f32 - 66.0, top: frame.top as f32 + 1.0,
        width: 60.0, height: (height - 2.0).min(24.0) })
}
fn contains(rect: RectF32, point: ae::Point) -> bool {
    let (x, y) = (point.h as f32, point.v as f32);
    x >= rect.left && x < rect.left + rect.width &&
        y >= rect.top && y < rect.top + rect.height
}

pub(crate) fn draw(event: &mut ae::EventExtra) -> Result<(), ae::Error> {
    if event.effect_area() != ae::EffectArea::Title { return Ok(()); }
    let frame = event.current_frame();
    let Some(rect) = button(frame) else { return Ok(()); };
    let drawbot = event.context_handle().drawing_reference()?;
    let supplier = drawbot.supplier()?;
    let surface = drawbot.surface()?;
    let size = supplier.default_font_size()?;
    let font = supplier.new_default_font(size)?;
    let text = supplier.new_brush(&ColorRgba { red: 0.85, green: 0.85, blue: 0.85, alpha: 1.0 })?;
    let background = supplier.new_brush(&ColorRgba { red: 0.22, green: 0.22, blue: 0.22, alpha: 1.0 })?;
    let mut path = supplier.new_path()?;
    path.add_rounded_rect(&rect, 4.0)?;
    surface.fill_path(&background, &path, ae::drawbot::FillType::Winding)?;
    // Preserve the parameter title when servicing its custom topic drawing.
    surface.draw_string(&text, &font, "Grid Positions", &PointF32 {
        x: frame.left as f32 + 2.0, y: rect.top + (rect.height + size) * 0.5 - 2.0,
    }, TextAlignment::Left, TextTruncation::End, rect.left - frame.left as f32 - 6.0)?;
    surface.draw_string(&text, &font, "Reset", &PointF32 {
        x: rect.left + rect.width * 0.5, y: rect.top + (rect.height + size) * 0.5 - 2.0,
    }, TextAlignment::Center, TextTruncation::None, rect.width)?;
    event.set_event_out_flags(ae::EventOutFlags::HANDLED_EVENT);
    Ok(())
}

pub(crate) fn click(params: &mut ae::Parameters<Params>, event: &mut ae::EventExtra) -> Result<(), ae::Error> {
    if event.effect_area() != ae::EffectArea::Title { return Ok(()); }
    if button(event.current_frame()).is_some_and(|rect| contains(rect, event.screen_point())) {
        reset_grid::apply(params)?;
        event.set_send_drag(false);
        event.set_event_out_flags(ae::EventOutFlags::HANDLED_EVENT |
            ae::EventOutFlags::ALWAYS_UPDATE | ae::EventOutFlags::UPDATE_NOW);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn reset_hit_area_preserves_title_and_refuses_outside_points() {
        let frame = ae::Rect { left: 40, top: 50, right: 340, bottom: 72 };
        let rect = button(frame).unwrap();
        assert!(rect.left >= frame.left as f32 + 120.0);
        assert!(contains(rect, ae::Point { h: 300, v: 60 }));
        for point in [ae::Point { h: 50, v: 60 }, ae::Point { h: 300, v: 40 },
            ae::Point { h: 334, v: 60 }, ae::Point { h: 300, v: 71 }] {
            assert!(!contains(rect, point));
        }
        assert!(button(ae::Rect { right: 180, ..frame }).is_none());
    }
}
