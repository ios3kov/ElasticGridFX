//! UI control density is a read-only view over the retained animated curve.
use super::*;

unsafe extern "C" {
    fn eg_control_axis_read(lines: *const f32, size: i32, count: i32,
        easing: f32, distance: f32, positions: *mut f32, references: *mut f32, capacity: i32) -> i32;
    fn eg_control_axis_drag(lines: *mut f32, pins: *const u8, size: i32,
        reference: f32, delta: f32, elastic: *const EgElasticParams) -> i32;
}

pub(crate) struct View {
    pub grid: GridArb,
    pub column_refs: Vec<f32>,
    pub row_refs: Vec<f32>,
}

fn sample(lines: &[f32], count: usize, easing: f32, distance: f32)
    -> Result<(Vec<f32>, Vec<f32>), ae::Error> {
    if !(1..=MAX_GUIDES).contains(&count) || !(3..=MAX_GUIDES+2).contains(&lines.len()) {
        return Err(ae::Error::BadCallbackParameter);
    }
    let mut positions = vec![0.0; count+2];
    let mut references = vec![0.0; count+2];
    // SAFETY: both output arrays have the advertised capacity; input is immutable
    // and lives throughout the call. The C++ boundary contains all exceptions.
    let rc = unsafe { eg_control_axis_read(lines.as_ptr(), lines.len() as i32,
        count as i32, easing, distance, positions.as_mut_ptr(), references.as_mut_ptr(),
        positions.len() as i32) };
    if rc != 0 { return Err(ae::Error::BadCallbackParameter); }
    Ok((positions, references))
}

pub(crate) fn view(evaluated: &GridArb, counts: (usize, usize), easing: f32, distance: f32)
    -> Result<View, ae::Error> {
    let (column_lines,column_refs) = sample(&evaluated.column_lines,counts.0,easing,distance)?;
    let (row_lines,row_refs) = sample(&evaluated.row_lines,counts.1,easing,distance)?;
    let mut grid = GridArb {columns:counts.0 as u16,rows:counts.1 as u16,
        column_lines,row_lines,column_pins:Vec::new(),row_pins:Vec::new()};
    grid.canonicalize_pins();
    Ok(View {grid,column_refs,row_refs})
}

pub(crate) fn read(in_data: &ae::InData, params: &mut ae::Parameters<Params>)
    -> Result<View, ae::Error> {
    let saved = grid_snapshot(params)?;
    let p = evaluated_params(params,*in_data,&saved)?;
    let mut evaluated = saved.clone();
    (evaluated.column_lines,evaluated.row_lines) = plane::evaluated_axes(&p)?;
    let counts = (
        params.get(Params::Columns)?.as_slider()?.value().clamp(1,MAX_GUIDES as i32) as usize,
        params.get(Params::Rows)?.as_slider()?.value().clamp(1,MAX_GUIDES as i32) as usize);
    view(&evaluated,counts,p.stretch_easing,p.easing_distance)
}

pub(crate) fn drag(lines: &mut [f32], pins: &[u8], reference: f32, delta: f32,
                  elastic: &EgElasticParams) -> Result<(), ae::Error> {
    if lines.len()!=pins.len() || !(3..=MAX_GUIDES+2).contains(&lines.len()) {
        return Err(ae::Error::BadCallbackParameter);
    }
    // SAFETY: matching typed slices, no aliases; data remain owned by the caller.
    let rc = unsafe { eg_control_axis_drag(lines.as_mut_ptr(),pins.as_ptr(),lines.len() as i32,
        reference,delta,elastic) };
    if rc == 0 {Ok(())} else {Err(ae::Error::BadCallbackParameter)}
}

#[cfg(test)]
#[path = "control_grid_tests.rs"]
mod tests;
