//! Count-independent material-coordinate refinement. Empty = exact identity.
use super::*;
pub(crate) const SAMPLES: usize = 257;

pub(crate) fn valid(values: &[f32]) -> bool {
    values.is_empty() || (values.len() == SAMPLES && values[0] == 0.0
        && values[SAMPLES-1] == 1.0 && values.iter().all(|v| v.is_finite())
        && values.windows(2).all(|p| p[0] < p[1]))
}
fn identity() -> Vec<f32> { (0..SAMPLES).map(|i| i as f32 / (SAMPLES-1) as f32).collect() }
pub(crate) fn sample(values: &[f32], position: f64) -> f32 {
    let p = position.clamp(0.0, 1.0);
    if values.is_empty() { return p as f32; }
    let x = p * (SAMPLES-1) as f64;
    let left = (x.floor() as usize).min(SAMPLES-1);
    let right = (left+1).min(SAMPLES-1);
    (f64::from(values[left]) + (f64::from(values[right])-f64::from(values[left])) * (x-left as f64)) as f32
}
pub(crate) fn interpolate(a: &[f32], b: &[f32], time: f64) -> Vec<f32> {
    if time <= 0.0 { return a.to_vec(); }
    if time >= 1.0 { return b.to_vec(); }
    if a.is_empty() && b.is_empty() { return Vec::new(); }
    (0..SAMPLES).map(|i| {
        let q = i as f64 / (SAMPLES-1) as f64;
        let x = f64::from(sample(a, q)); let y = f64::from(sample(b, q));
        (x + (y-x)*time) as f32
    }).collect()
}
fn weight(distance: f64, radius: f64, kind: i32) -> f64 {
    if distance == 0.0 { return 1.0; }
    if radius <= 0.0 || distance >= radius { return 0.0; }
    let t = distance/radius;
    match kind { 2 => (-4.0*t*t).exp(), 3 => 1.0-t, _ => 1.0-t*t*(3.0-2.0*t) }
}
// No mutation until a finite, monotone result has been validated. The current
// UI count chooses only the intentional edit's radius, never the stored basis.
pub(crate) fn move_control(values: &mut Vec<f32>, position: f64, target: f32,
                          visible: usize, settings: &EgElasticParams) -> Result<bool, ae::Error> {
    if !valid(values) || !position.is_finite() || position <= 0.0 || position >= 1.0
        || !target.is_finite() || !(1..=50).contains(&visible)
        || !settings.tension_radius.is_finite() || !settings.elasticity_strength.is_finite()
        || !settings.min_spacing.is_finite() { return Err(ae::Error::BadCallbackParameter); }
    let old_value = sample(values, position);
    let strength = f64::from(settings.elasticity_strength).clamp(0.0, 2.0);
    let delta = (f64::from(target)-f64::from(old_value))*strength;
    if delta == 0.0 { return Ok(false); }
    let before = if values.is_empty() { identity() } else { values.clone() };
    let cells = (SAMPLES-1) as f64;
    let radius = f64::from(settings.tension_radius).clamp(0.0,20.0)/(visible+1) as f64;
    let mut weights: Vec<f64> = (0..SAMPLES).map(|i| {
        if i == 0 || i+1 == SAMPLES { 0.0 }
        else { weight((i as f64/cells-position).abs(), radius, settings.falloff) }
    }).collect();
    let x = position*cells; let left = x.floor() as usize; let right = left+1; let t = x-left as f64;
    let mut response = (1.0-t)*weights[left]+t*weights[right];
    if response <= 1.0e-12 {
        if left > 0 { weights[left] = 1.0-t; }
        if right+1 < SAMPLES { weights[right] = t; }
        response = (1.0-t)*weights[left]+t*weights[right];
    }
    if response <= 0.0 { return Err(ae::Error::BadCallbackParameter); }
    let displacement = delta/response;
    let floor = (f64::from(settings.min_spacing).clamp(0.0,0.25)*(visible+1) as f64/cells)
        .clamp(1.0e-6, 0.5/cells);
    let mut limit = 1.0_f64;
    for i in 0..SAMPLES-1 {
        let gap = f64::from(before[i+1])-f64::from(before[i]);
        let change = displacement*(weights[i+1]-weights[i]);
        if change < 0.0 { limit = limit.min(((gap-floor.min(gap)) / -change).max(0.0)); }
    }
    if limit < 1.0 { limit *= 1.0-1.0e-6; }
    let mut candidate = before.clone(); let mut valid_result = false;
    for _ in 0..32 {
        for i in 1..SAMPLES-1 { candidate[i] = (f64::from(before[i])+displacement*weights[i]*limit) as f32; }
        if valid(&candidate) { valid_result = true; break; }
        limit *= 0.5;
    }
    if !valid_result { return Err(ae::Error::BadCallbackParameter); }
    if candidate.iter().zip(&before).all(|(a,b)| a.to_bits()==b.to_bits()) { return Ok(false); }
    *values = candidate;
    Ok(true)
}

#[repr(C)]
pub(crate) struct Maps { pub columns:*const f32, pub column_count:i32, pub rows:*const f32, pub row_count:i32 }
impl Maps {
    pub fn from_grid(grid:&GridArb) -> Self {
        Self { columns:grid.column_detail.as_ptr(),column_count:grid.column_detail.len() as i32,
            rows:grid.row_detail.as_ptr(),row_count:grid.row_detail.len() as i32 }
    }
}
