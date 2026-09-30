//! Fit public point controls to source boundaries, never to raster indices.
use super::*;

pub(crate) fn points(width: i32, height: i32) -> Result<[(f32, f32); 4], ae::Error> {
    // PointDef::set_value uses signed 16.16. Refuse invalid/unrepresentable
    // dimensions instead of silently clamping or collapsing the fitted quad.
    if !(1..=32767).contains(&width) || !(1..=32767).contains(&height) {
        return Err(ae::Error::BadCallbackParameter);
    }
    let (w, h) = (width as f32, height as f32);
    Ok([(0.0, 0.0), (w, 0.0), (w, h), (0.0, h)])
}

fn changes(current: &[(f64, f64); 4], target: &[(f32, f32); 4]) -> [bool; 4] {
    std::array::from_fn(|i| current[i] != (f64::from(target[i].0), f64::from(target[i].1)))
}

// USER_CHANGED_PARAM only. Point edits retain AE's existing key/Undo policy.
// A repeated, already-fitted command must not create a new identical point key.
pub(crate) fn apply(input: &ae::InData, params: &mut ae::Parameters<Params>) -> Result<bool, ae::Error> {
    let target = points(input.width(), input.height())?;
    let mut current = [(0.0, 0.0); 4];
    // Validate/read every point before making the first write.
    for (i, id) in plane::CORNERS.into_iter().enumerate() {
        let value = params.get(id)?.as_point()?.float_value()?;
        current[i] = (value.x, value.y);
    }
    let changed = changes(&current, &target);
    for (i, id) in plane::CORNERS.into_iter().enumerate() {
        if changed[i] {
            let mut param = params.get_mut(id)?;
            param.as_point_mut()?.set_value(target[i]);
            param.set_value_changed();
        }
    }
    Ok(changed.into_iter().any(|value| value))
}

#[cfg(test)]
#[path = "fit_layer_tests.rs"]
mod tests;
