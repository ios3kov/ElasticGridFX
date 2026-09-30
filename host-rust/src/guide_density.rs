//! Control density is independent of the stored base warp and detail field.
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

#[cfg(test)]
#[path = "detail_contract_tests.rs"]
mod contracts;
