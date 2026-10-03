//! Topic-only arbitrary parameter UI. Never draws into rendered image pixels.
use super::*;
use ae::drawbot::{ColorRgba, PointF32, RectF32, TextAlignment, TextTruncation};

// Bounded test-only observations. Never enabled in the default product build;
// records geometry/dispatch only, without project names or arbitrary payloads.
#[cfg(feature = "render-diagnostics")]
pub(crate) fn observe(event: &mut ae::EventExtra) {
    use std::io::Write;
    use std::sync::{Mutex, OnceLock};
    if event.window_type() != ae::WindowType::Effect { return; }
    let label = match event.event() {
        ae::Event::Click(_) => "click", ae::Event::Drag(_) => "drag", _ => return,
    };
    static LOG: OnceLock<Option<Mutex<(std::fs::File, usize)>>> = OnceLock::new();
    let log = LOG.get_or_init(|| {
        let p = std::env::temp_dir().join(format!("egfx-grid-events-{}.txt", std::process::id()));
        std::fs::OpenOptions::new().write(true).create_new(true).open(p).ok().map(|f| Mutex::new((f,0)))
    });
    if let Some(log) = log {
        if let Ok(mut entry) = log.lock() {
            if entry.1 >= 40 { return; }
            entry.1 += 1;
            let point = event.screen_point();
            let state: [isize;4] = std::array::from_fn(|i| event.continue_refcon(i));
            let _ = writeln!(entry.0, "{} point={:?} send={} last={} state={:?}",
                label,point,event.send_drag(),event.last_time(),state);
            if label == "click" {
                let _ = writeln!(entry.0,"index={} area={:?} title={:?} current={:?}",
                    event.param_index(),event.effect_area(),event.param_title_frame(),event.current_frame());
            }
            let _ = entry.0.flush();
        }
    }
}

fn button(frame: ae::Rect) -> Option<RectF32> {
    // ECW divides the row at its center, with the native value control inset
    // by 16 logical units. horiz_offset is not initialized for this arbitrary
    // topic in AE 25.6 and must never be used as a coordinate.
    let width = (frame.right - frame.left) as f32;
    let left = frame.left as f32 + width * 0.5 + 16.0;
    let available = frame.right as f32 - left - 5.0;
    let height = (frame.bottom - frame.top) as f32;
    // A narrow ECW still has a usable native value column. Size to that
    // column; a fixed row-width cutoff incorrectly hid Reset on resize.
    if available < 60.0 || height < 12.0 { return None; }
    Some(RectF32 { left, top: frame.top as f32,
        width: available.floor().min(130.0), height: (height - 1.0).min(16.0) })
}
fn contains(rect: RectF32, point: ae::Point) -> bool {
    let (x, y) = (point.h as f32, point.v as f32);
    x >= rect.left && x < rect.left + rect.width &&
        y >= rect.top && y < rect.top + rect.height
}

pub(crate) fn draw(event: &mut ae::EventExtra) -> Result<(), ae::Error> {
    if event.effect_area() != ae::EffectArea::Title { return Ok(()); }
    let frame = event.param_title_frame();
    #[cfg(feature = "render-diagnostics")]
    {
        static LOG: std::sync::Once = std::sync::Once::new();
        LOG.call_once(|| {
            use std::io::Write;
            let p = std::env::temp_dir().join(format!("egfx-grid-layout-{}.txt", std::process::id()));
            if let Ok(mut file) = std::fs::OpenOptions::new().write(true).create_new(true).open(p) {
                let _ = write!(file, "title={:?} current={:?} offset={}", frame, event.current_frame(), event.horiz_offset());
            }
        });
    }
    let Some(rect) = button(frame) else { return Ok(()); };
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

// A drag continuation has its own event data. Capture the hit rectangle at
// DO_CLICK instead of reading effect_win fields which the SDK only guarantees
// for DO_CLICK/DRAW/ADJUST_CURSOR. Four integers, no retained host pointers.
const RESET_GESTURE: isize = 0x45474658;
fn capture(rect: RectF32) -> [isize; 4] {
    [RESET_GESTURE, rect.left.to_bits() as isize, rect.top.to_bits() as isize,
        ((rect.width as u32) << 16 | rect.height as u32) as isize]
}
fn restore(state: [isize; 4]) -> Option<RectF32> {
    if state[0] != RESET_GESTURE || state[1] < 0 || state[1] as u64 > u32::MAX as u64 ||
        state[2] < 0 || state[2] as u64 > u32::MAX as u64 ||
        state[3] <= 0 || state[3] as u64 > u32::MAX as u64 { return None; }
    let rect = RectF32 { left: f32::from_bits(state[1] as u32),
        top: f32::from_bits(state[2] as u32), width: ((state[3] as u32) >> 16) as f32,
        height: ((state[3] as u32) & 0xffff) as f32 };
    if !rect.left.is_finite() || !rect.top.is_finite() ||
        !(1.0..=130.0).contains(&rect.width) || !(1.0..=16.0).contains(&rect.height) { return None; }
    Some(rect)
}
pub(crate) fn click(event: &mut ae::EventExtra) -> Result<(), ae::Error> {
    if event.effect_area() != ae::EffectArea::Title { return Ok(()); }
    if let Some(rect) = button(event.param_title_frame()).filter(|r| contains(*r, event.screen_point())) {
        // PF_CHANGE_VALUE is valid in DRAG, not DO_CLICK. Arm the button and
        // commit on release inside its bounds, allowing drag-out cancellation.
        for (index, value) in capture(rect).into_iter().enumerate() { event.set_continue_refcon(index, value); }
        event.set_send_drag(true);
        event.set_event_out_flags(ae::EventOutFlags::HANDLED_EVENT);
    }
    Ok(())
}

pub(crate) fn drag(params: &mut ae::Parameters<Params>, event: &mut ae::EventExtra) -> Result<(), ae::Error> {
    let Some(rect) = restore(std::array::from_fn(|i| event.continue_refcon(i))) else { return Ok(()); };
    event.set_event_out_flags(ae::EventOutFlags::HANDLED_EVENT);
    if event.last_time() {
        event.set_continue_refcon(0, 0);
        event.set_send_drag(false);
        if contains(rect, event.screen_point()) && reset_grid::apply(params)? {
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
        let rect = button(frame).unwrap();
        assert!(rect.left >= frame.left as f32 + 166.0);
        assert!(contains(rect, ae::Point { h: 300, v: 60 }));
        for point in [ae::Point { h: 50, v: 60 }, ae::Point { h: 300, v: 40 },
            ae::Point { h: 340, v: 60 }, ae::Point { h: 300, v: 71 }] {
            assert!(!contains(rect, point));
        }
        assert!(button(ae::Rect { right: 180, ..frame }).is_none());
    }

    #[test]
    fn reset_remains_visible_when_panel_shrinks_below_old_cutoff() {
        for width in [220, 240, 260, 299, 300, 400, 648] {
            let frame = ae::Rect { left: 17, top: 153, right: 17 + width, bottom: 170 };
            let rect = button(frame).expect("usable value column must retain Reset");
            assert_eq!(rect.left, 17.0 + width as f32 * 0.5 + 16.0);
            assert!(rect.width >= 60.0 && rect.width <= 130.0);
            assert!(rect.left + rect.width <= frame.right as f32 - 5.0);
            assert_eq!(rect.height, 16.0);
            let restored = restore(capture(rect)).unwrap();
            assert!(contains(restored, ae::Point { h: (rect.left + rect.width * 0.5) as _,
                v: 160 }));
        }
    }

    #[test]
    fn reset_continuation_keeps_original_bounds_and_rejects_foreign_state() {
        let rect = RectF32 { left: -20.5, top: 300.0, width: 130.0, height: 16.0 };
        let state = capture(rect);
        let retained = restore(state).unwrap();
        assert_eq!(retained.left, rect.left); assert_eq!(retained.top, rect.top);
        assert!(contains(retained, ae::Point { h: 0, v: 310 }));
        assert!(!contains(retained, ae::Point { h: 110, v: 310 }));
        assert!(!contains(retained, ae::Point { h: 0, v: 316 }));
        for bad in [[0;4], [1, state[1], state[2], state[3]],
            [RESET_GESTURE, f32::NAN.to_bits() as isize, state[2], state[3]],
            [RESET_GESTURE, state[1], state[2], (131 << 16) | 16]] {
            assert!(restore(bad).is_none());
        }
    }
}
