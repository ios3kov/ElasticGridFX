put('host-rust/src/detail_map.rs', [
    (0,0,r'''//! Count-independent material-coordinate refinement. Empty = exact identity.
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
'''),
])
put('host-rust/src/first_application_tests.rs', [
    (4,1,r'''    initial_identity_values(grid,1,0.0,0.0,0.5)
'''),
    (62,2,r'''        assert!(!initial_identity_values(&grid,1,value,0.0,0.5));
        assert!(!initial_identity_values(&grid,1,0.0,value,0.5));
'''),
    (66,1,r'''        assert!(!initial_identity_values(&grid,1,0.0,0.0,value));
'''),
    (68,4,r'''    for mode in [0,2,3] {assert!(!initial_identity_values(&grid,mode,0.0,0.0,0.5));}
'''),
])
put('host-rust/src/guide_density.rs', [
    (0,0,r'''//! Control density is independent of the stored base warp and detail field.
//! Count changes only derive a view; intentional drags edit geometry.
//! Hidden original shape data remains when guides are removed from the view.
use super::*;

#[derive(Clone, Copy, Debug)]
struct Guide {
    numerator: usize,
    denominator: usize,
}

fn guides(stored: usize, visible: usize) -> Result<Vec<Guide>, ae::Error> {
    if !(1..=MAX_GUIDES).contains(&stored) || !(1..=MAX_GUIDES).contains(&visible) {
        return Err(ae::Error::BadCallbackParameter);
    }
    let cells = stored + 1;
    let mut result = Vec::with_capacity(visible + 2);
    result.push(Guide { numerator: 0, denominator: cells });
    if visible < stored {
        // Keep an evenly spaced subset of original anchors; no curve data is
        // deleted. Integer quantiles are deterministic and cannot duplicate.
        for i in 1..=visible {
            let index = (2 * i * cells + visible + 1) / (2 * (visible + 1));
            result.push(Guide { numerator: index, denominator: cells });
        }
    } else {
        let extra = visible - stored;
        let rounded = |i: usize| (2 * i * extra + cells) / (2 * cells);
        for gap in 0..cells {
            let added = rounded(gap + 1) - rounded(gap);
            for j in 1..=added {
                result.push(Guide {
                    numerator: gap * (added + 1) + j,
                    denominator: cells * (added + 1),
                });
            }
            if gap + 1 < cells {
                result.push(Guide { numerator: gap + 1, denominator: cells });
            }
        }
    }
    result.push(Guide { numerator: cells, denominator: cells });
    Ok(result)
}

/// This is the ONLY canonical-grid read for rendering. No visible count input.
pub(crate) fn render_grid(grid: &GridArb) -> Result<GridArb, ae::Error> {
    if !grid.is_valid() { return Err(ae::Error::BadCallbackParameter); }
    Ok(grid.clone())
}

#[derive(Clone, Copy)]
pub(crate) struct Mapping { pub easing:f32, pub distance:f32 }
impl Default for Mapping { fn default()->Self {Self {easing:0.0,distance:0.25}} }
fn axis_coordinates(axis:&[f32],queries:&[f32],mapping:Mapping,forward:bool)->Result<Vec<f32>,ae::Error> {
    if queries.is_empty() || queries.len()>52 {return Err(ae::Error::BadCallbackParameter);}
    let mut values=vec![0.0;queries.len()];
    // Actual CPU mapping, also used by the renderer. All input/output storage
    // is owned and lives across the synchronous call; no pointer is retained.
    let code=unsafe {eg_axis_coordinates(axis.as_ptr(),axis.len() as i32,queries.as_ptr(),
        values.as_mut_ptr(),queries.len() as i32,mapping.easing,mapping.distance,i32::from(forward))};
    if code!=0 {return Err(ae::Error::BadCallbackParameter);}
    Ok(values)
}
fn view_axis(axis:&[f32],detail:&[f32],visible:usize,mapping:Mapping)->Result<Vec<f32>,ae::Error> {
    let refs=guides(axis.len()-2,visible)?;
    let queries:Vec<f32>=refs.iter().map(|r|detail_map::sample(detail,r.numerator as f64/r.denominator as f64)).collect();
    axis_coordinates(axis,&queries,mapping,true)
}
/// Temporary viewer data, never passed to a renderer or saved as a key.
pub(crate) fn view_grid_mapped(grid:&GridArb,columns:usize,rows:usize,mapping:Mapping)->Result<GridArb,ae::Error> {
    if !grid.is_valid() {return Err(ae::Error::BadCallbackParameter);}
    let mut view=GridArb::uniform(columns,rows);
    view.column_lines=view_axis(&grid.column_lines,&grid.column_detail,columns,mapping)?;
    view.row_lines=view_axis(&grid.row_lines,&grid.row_detail,rows,mapping)?;
    Ok(view)
}
#[cfg(test)]
fn view_grid(grid:&GridArb,columns:usize,rows:usize)->Result<GridArb,ae::Error> {
    view_grid_mapped(grid,columns,rows,Mapping::default())
}
pub(crate) struct Drag {
    pub columns:bool,pub visible:usize,pub index:usize,pub target:f32,pub mapping:Mapping,
}
/// An intentional guide drag edits geometry, not the count. Extra handles have
/// independent local degrees of freedom in the fixed detail field. Existing
/// untouched, same-density guides retain the historical drag algorithm.
pub(crate) fn drag_control(grid:&mut GridArb,evaluated:&GridArb,request:Drag,settings:&EgElasticParams)
    ->Result<bool,ae::Error> {
    if !grid.is_valid() || !evaluated.is_valid() || !request.target.is_finite()
        || !settings.tension_radius.is_finite() || !settings.elasticity_strength.is_finite()
        || !settings.min_spacing.is_finite() {return Err(ae::Error::BadCallbackParameter);}
    let (lines,pins,detail,base)=if request.columns {
        (&mut grid.column_lines,&mut grid.column_pins,&mut grid.column_detail,&evaluated.column_lines)
    } else {(&mut grid.row_lines,&mut grid.row_pins,&mut grid.row_detail,&evaluated.row_lines)};
    if base.len()!=lines.len() {return Err(ae::Error::BadCallbackParameter);}
    let refs=guides(lines.len()-2,request.visible)?;
    if request.index==0 || request.index+1>=refs.len() {return Err(ae::Error::BadCallbackParameter);}
    let reference=refs[request.index];
    let q=reference.numerator as f64/reference.denominator as f64;
    let current=view_axis(base,detail,request.visible,request.mapping)?[request.index];
    if current.to_bits()==request.target.to_bits() || settings.elasticity_strength<=0.0 {return Ok(false);}
    if detail.is_empty() && request.visible==lines.len()-2 {
        let target=if current.to_bits()==lines[request.index].to_bits() {request.target}
            else {lines[request.index]+(request.target-current)};
        let before=lines.clone();
        let code=unsafe {eg_drag_axis(lines.as_mut_ptr(),pins.as_mut_ptr(),lines.len() as i32,
                                     request.index as i32,target,settings)};
        if code!=0 {return Err(ae::Error::BadCallbackParameter);}
        return Ok(lines.iter().zip(before).any(|(a,b)|a.to_bits()!=b.to_bits()));
    }
    let target=axis_coordinates(base,&[request.target],request.mapping,false)?[0];
    detail_map::move_control(detail,q,target,request.visible,settings)
}

#[cfg(test)]
#[path = "guide_density_tests.rs"]
mod tests;
'''),
])
