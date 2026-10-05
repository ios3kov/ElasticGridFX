//! Current-time reset through the host-owned supervised parameter transaction.
use super::*;

fn target(columns: i32, rows: i32) -> Result<GridArb, ae::Error> {
    if !(1..=MAX_GUIDES as i32).contains(&columns) ||
        !(1..=MAX_GUIDES as i32).contains(&rows) {
        return Err(ae::Error::BadCallbackParameter);
    }
    Ok(GridArb::uniform(columns as usize, rows as usize))
}

// USER_CHANGED_PARAM or native UI DRAG release only. Change only the current GridState value. AE owns
// current-time key insertion/replacement and Undo; never enumerate/delete keys
// or switch time-varying state. A neutral repeated reset is a no-op.
pub(crate) fn apply(params: &mut ae::Parameters<Params>) -> Result<bool, ae::Error> {
    let current = params.get(Params::GridState)?.as_arbitrary()?.value::<GridArb>()?;
    if !current.is_valid() { return Err(ae::Error::BadCallbackParameter); }
    // Viewer density does not own the retained animated lattice.
    let neutral = target(i32::from(current.columns), i32::from(current.rows))?;
    let changed = *params.get(Params::GridState)?.as_arbitrary()?.value::<GridArb>()? != neutral;
    if changed {
        params.get_mut(Params::GridState)?.as_arbitrary_mut()?.set_value(neutral)?;
    }
    Ok(changed)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn reset_target_matches_current_topology_and_neutral_output() {
        for (columns, rows) in [(1, 1), (4, 7), (50, 50)] {
            let state = target(columns, rows).unwrap();
            assert_eq!(state.columns, columns as u16);
            assert_eq!(state.rows, rows as u16);
            for axis in [&state.column_lines, &state.row_lines] {
                assert_eq!(axis[0], 0.0);
                assert_eq!(*axis.last().unwrap(), 1.0);
                for (i, point) in axis.iter().enumerate() {
                    assert_eq!(*point, i as f32 / (axis.len() - 1) as f32);
                }
            }
        }
    }
    #[test]
    fn invalid_topology_is_refused_before_parameter_write() {
        for (columns, rows) in [(0, 4), (4, -1), (51, 4), (4, 51)] {
            assert!(target(columns, rows).is_err());
        }
    }
}
