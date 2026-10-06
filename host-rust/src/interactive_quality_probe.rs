//! Nondefault, read-only census. Counts callback quality/resolution, not pixels.
//! Render workers only touch atomics. UI gesture boundaries write at most eight
//! small records to a new private temporary file; no host/frame request is added.
use super::ae;
use std::cell::RefCell;
use std::io::Write;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{Instant, SystemTime, UNIX_EPOCH};

const BINS: usize = 18;
struct Census([AtomicU64; BINS]);
impl Census {
    const fn new() -> Self { Self([const { AtomicU64::new(0) }; BINS]) }
    fn add(&self, slot: usize) { self.0[slot].fetch_add(1, Ordering::Relaxed); }
    fn read(&self) -> [u64; BINS] { std::array::from_fn(|i| self.0[i].load(Ordering::Relaxed)) }
}
static COUNTS: Census = Census::new();
fn scale_class(x: f32, y: f32) -> usize {
    if x == 1.0 && y == 1.0 { 0 }
    else if x.is_finite() && y.is_finite() && x > 0.0 && y > 0.0 && x <= 1.0 && y <= 1.0 { 1 }
    else { 2 }
}
fn slot(stage: usize, low: bool, scale: usize) -> Option<usize> {
    if stage >= 3 || scale >= 3 { return None; }
    Some(stage * 6 + usize::from(low) * 3 + scale)
}
pub(crate) fn observe(input: &ae::InData, stage: usize) {
    // These input fields are documented for frame selectors in SDK25.6.
    let low = match input.quality() {
        ae::pf::Quality::Lo => true,
        ae::pf::Quality::Hi => false,
        ae::pf::Quality::DrawingAudio => return,
    };
    let scale = scale_class(f32::from(input.downsample_x()), f32::from(input.downsample_y()));
    if let Some(i) = slot(stage, low, scale) { COUNTS.add(i); }
}
struct Gesture { owner: i32, window: i32, index: usize, started: Instant, before: [u64; BINS] }
#[derive(Default)]
struct Ui { gesture: Option<Gesture>, file: Option<std::fs::File>, written: usize }
thread_local! { static UI: RefCell<Ui> = RefCell::new(Ui::default()); }
pub(crate) fn begin(owner: i32, window: i32, index: usize) {
    if index >= 4 { return; }
    UI.with(|ui| {
        let mut ui = ui.borrow_mut();
        if ui.written >= 8 || ui.gesture.is_some() { return; }
        ui.gesture = Some(Gesture { owner, window, index, started: Instant::now(), before: COUNTS.read() });
    });
}
fn delta(before: [u64; BINS], after: [u64; BINS]) -> [u64; BINS] {
    std::array::from_fn(|i| after[i].wrapping_sub(before[i]))
}
fn private_file() -> std::io::Result<std::fs::File> {
    let nonce = SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_nanos();
    let path = std::env::temp_dir().join(format!("FSTR-interactive-quality-{}-{nonce}.jsonl", std::process::id()));
    let mut options = std::fs::OpenOptions::new();
    options.write(true).create_new(true);
    #[cfg(unix)] {
        use std::os::unix::fs::OpenOptionsExt;
        options.mode(0o600);
    }
    // create_new rejects an existing file or symlink. No fixed path is truncated.
    options.open(path)
}
pub(crate) fn end(reason: &'static str) {
    // Called only from existing UI callbacks. Census includes any other effect
    // rendered during this interval: use an owned one-effect fixture for inference.
    UI.with(|ui| {
        let mut ui = ui.borrow_mut();
        let Some(g) = ui.gesture.take() else { return; };
        let counts = delta(g.before, COUNTS.read());
        ui.written += 1;
        if ui.file.is_none() { ui.file = private_file().ok(); }
        if let Some(file) = ui.file.as_mut() {
            let record = format!(
                "{{\"schema\":1,\"build\":\"{}\",\"pid\":{},\"owner\":{},\"window\":{},\"corner\":{},\"reason\":\"{}\",\"milliseconds\":{},\"counts\":{:?}}}\n",
                super::build_identity::DIAGNOSTIC.split('\r').find_map(|s| s.strip_prefix("ElasticGridBuildID=")).unwrap_or("unknown"),
                std::process::id(), g.owner, g.window, g.index, reason, g.started.elapsed().as_millis(), counts);
            let _ = file.write_all(record.as_bytes());
        }
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn distinguishes_quality_from_resolution_and_rejects_unknown_scale() {
        assert_eq!(scale_class(1.0, 1.0), 0);
        assert_eq!(scale_class(0.5, 1.0), 1);
        assert_eq!(scale_class(1.0, 0.25), 1);
        for v in [0.0, -1.0, 2.0, f32::NAN, f32::INFINITY] { assert_eq!(scale_class(v, 1.0), 2); }
        let mut bins = Vec::new();
        for stage in 0..3 { for low in [false, true] { for scale in 0..3 { bins.push(slot(stage, low, scale).unwrap()); } } }
        assert_eq!(bins, (0..BINS).collect::<Vec<_>>());
        assert!(slot(3, false, 0).is_none()); assert!(slot(0, false, 3).is_none());
        assert!(slot(usize::MAX, true, usize::MAX).is_none());
    }
    #[test]
    fn concurrent_census_has_no_reset_race_and_handles_counter_wrap() {
        let counts = Census::new();
        let before = counts.read();
        std::thread::scope(|scope| { for i in 0..BINS { let counts = &counts; scope.spawn(move || { for _ in 0..1000 { counts.add(i); } }); } });
        assert_eq!(delta(before, counts.read()), [1000; BINS]);
        assert_eq!(delta([u64::MAX; BINS], [0; BINS]), [1; BINS]);
    }
}
