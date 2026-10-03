//! UI control density is a read-only view over the retained animated curve.
use super::*;

unsafe extern "C" {
    fn eg_control_axis_read(lines: *const f32, size: i32, count: i32,
        easing: f32, distance: f32, positions: *mut f32, references: *mut f32, capacity: i32) -> i32;
    #[cfg(test)]
    fn eg_control_axis_drag(lines: *mut f32, pins: *const u8, size: i32,
        reference: f32, delta: f32, elastic: *const EgElasticParams) -> i32;
    fn eg_control_axis_drag_live(lines: *mut f32,pins: *const u8,size: i32,
        reference: f32,target: f32,elastic: *const EgElasticParams,
        render: *const EgRenderParams,columns: i32) -> i32;
    fn eg_control_axis_reflow(lines: *const f32,size: i32,count: i32,easing: f32,
        distance: f32,refs: *mut f32,capacity: i32) -> i32;
    fn eg_control_axis_read_layout(lines: *const f32,size: i32,normalized: *const f32,
        count: i32,easing: f32,distance: f32,positions: *mut f32,refs: *mut f32,capacity: i32) -> i32;
}

// Query a source reference against the already evaluated immutable axis using
// the existing core's exact viewer/WarpMath inversion, including easing.
pub(crate) fn position_at(lines:&[f32],source:f32,easing:f32,distance:f32)->Result<f32,ae::Error> {
    if !(3..=MAX_GUIDES+2).contains(&lines.len()) || !source.is_finite() || !(0.0..=1.0).contains(&source) {
        return Err(ae::Error::BadCallbackParameter);
    }
    let q=if source==0.0 || source==1.0 {0.5}else{source};
    let normalized=[0.0,q,1.0];let mut positions=[0.0;3];let mut refs=[0.0;3];
    // SAFETY: immutable validated dimensions, stack-owned nonaliasing outputs;
    // the bounded core call does not retain any input/output pointer.
    let rc=unsafe{eg_control_axis_read_layout(lines.as_ptr(),lines.len() as i32,
        normalized.as_ptr(),3,easing,distance,positions.as_mut_ptr(),refs.as_mut_ptr(),3)};
    if rc!=0{return Err(ae::Error::BadCallbackParameter);}
    Ok(positions[if source==0.0 {0}else if source==1.0 {2}else{1}])
}

pub(crate) fn reflow_axis(lines: &[f32],count: usize,easing: f32,distance: f32)
    -> Result<Vec<f32>,ae::Error> {
    if !(1..=MAX_GUIDES).contains(&count) || !(3..=MAX_GUIDES+2).contains(&lines.len()) {
        return Err(ae::Error::BadCallbackParameter);
    }
    let mut refs=vec![0.0;count+2];
    // SAFETY: checked slice dimensions, bounded owned output and immutable input.
    let rc=unsafe {eg_control_axis_reflow(lines.as_ptr(),lines.len() as i32,count as i32,
        easing,distance,refs.as_mut_ptr(),refs.len() as i32)};
    if rc==0 {Ok(refs)} else {Err(ae::Error::BadCallbackParameter)}
}

fn sample_layout(lines: &[f32],normalized: &[f32],easing: f32,distance: f32)
    -> Result<(Vec<f32>,Vec<f32>),ae::Error> {
    if normalized.is_empty() || !control_layout::State::valid_axis(normalized) ||
        !(3..=MAX_GUIDES+2).contains(&lines.len()) {return Err(ae::Error::BadCallbackParameter);}
    let mut positions=vec![0.0;normalized.len()];let mut refs=positions.clone();
    // SAFETY: all arrays are bounded to52 elements and remain valid; outputs do
    // not alias input or each other. C++ validates before publishing either.
    let rc=unsafe {eg_control_axis_read_layout(lines.as_ptr(),lines.len() as i32,
        normalized.as_ptr(),normalized.len() as i32,easing,distance,positions.as_mut_ptr(),
        refs.as_mut_ptr(),positions.len() as i32)};
    if rc==0 {Ok((positions,refs))} else {Err(ae::Error::BadCallbackParameter)}
}

pub(crate) struct View {
    pub grid: GridArb,
    pub column_refs: Vec<f32>,
    pub row_refs: Vec<f32>,
}

pub(crate) fn drag_live(lines: &mut [f32],pins: &[u8],reference: f32,target: f32,
                       elastic: &EgElasticParams,render: &EgRenderParams,columns: bool)
    -> Result<(),ae::Error> {
    if !(3..=MAX_GUIDES+2).contains(&lines.len()) || pins.len()!=lines.len() {
        return Err(ae::Error::BadCallbackParameter);
    }
    // SAFETY: bounded slices and complete render inputs live throughout this
    // synchronous call. The core publishes only the final validated edited axis.
    let rc=unsafe {eg_control_axis_drag_live(lines.as_mut_ptr(),pins.as_ptr(),lines.len() as i32,
        reference,target,elastic,render,i32::from(columns))};
    if rc==0 {Ok(())} else {Err(ae::Error::BadCallbackParameter)}
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
    let layout=params.get(Params::ControlLayout)?.as_arbitrary()?.value::<control_layout::State>()?;
    if !layout.valid() {return Err(ae::Error::BadCallbackParameter);}
    let mut displayed=view(&evaluated,counts,p.stretch_easing,p.easing_distance)?;
    apply_layout(&mut displayed,&evaluated,counts,&layout,p.stretch_easing,p.easing_distance)?;
    Ok(displayed)
}

#[cfg(test)]
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

// Scripted setValue does not deliver UserChangedParam. Resolve mismatched
// densities read-only here; the first real deformation drag freezes references.
pub(crate) fn apply_layout(displayed: &mut View,evaluated: &GridArb,counts: (usize,usize),
    layout: &control_layout::State,easing: f32,distance: f32) -> Result<(),ae::Error> {
    for column in [true,false] {
        let (lines,refs,count,retained)=if column {
            (&evaluated.column_lines,&layout.columns,counts.0,evaluated.columns as usize)
        } else {(&evaluated.row_lines,&layout.rows,counts.1,evaluated.rows as usize)};
        let fallback;
        let selected=if refs.len()==count+2 {refs.as_slice()}
        else if count!=retained {fallback=reflow_axis(lines,count,easing,distance)?;&fallback}
        else {continue;};
        let sampled=sample_layout(lines,selected,easing,distance)?;
        if column {(displayed.grid.column_lines,displayed.column_refs)=sampled;}
        else {(displayed.grid.row_lines,displayed.row_refs)=sampled;}
    }
    Ok(())
}
