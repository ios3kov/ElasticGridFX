//! Final UI gesture only. An animated Point must change at the current time,
//! rather than offsetting its entire track through PF CHANGED_VALUE.
use super::*;

trait KeyHost {
    fn begin(&mut self) -> Result<(), ae::Error>;
    fn insert(&mut self) -> Result<i32, ae::Error>;
    fn write(&mut self, index: i32) -> Result<(), ae::Error>;
    fn count(&mut self) -> Result<i32, ae::Error>;
    fn remove(&mut self, index: i32) -> Result<(), ae::Error>;
    fn end(&mut self) -> Result<(), ae::Error>;
}
fn commit<H: KeyHost>(host: &mut H, before: i32) -> Result<(), ae::Error> {
    host.begin()?;
    let changed = (|| {
        let index = host.insert()?;
        if let Err(error) = host.write(index) {
            // InsertKeyframe leaves an existing key untouched. If it added a
            // new key and setting its value failed, remove only that new key.
            if host.count()? > before {
                host.remove(index)?;
            }
            return Err(error);
        }
        Ok(())
    })();
    let ended = host.end();
    changed.and(ended)
}
struct Adapter<'a> {
    keys: &'a ae::aegp::suites::Keyframe,
    utility: &'a ae::aegp::suites::Utility,
    stream: &'a ae::aegp::StreamReferenceHandle,
    time: ae::Time,
    target: ae::aegp::StreamValue,
}
impl KeyHost for Adapter<'_> {
    fn begin(&mut self) -> Result<(), ae::Error> {
        self.utility.start_undo_group("FSTR Stretch corner")
    }
    fn insert(&mut self) -> Result<i32, ae::Error> {
        self.keys
            .insert_keyframe(self.stream, ae::aegp::TimeMode::CompTime, self.time)
    }
    fn write(&mut self, index: i32) -> Result<(), ae::Error> {
        self.keys
            .set_keyframe_value(self.stream, index, self.target)
    }
    fn count(&mut self) -> Result<i32, ae::Error> {
        self.keys.stream_num_kfs(self.stream)
    }
    fn remove(&mut self, index: i32) -> Result<(), ae::Error> {
        self.keys.delete_keyframe(self.stream, index)
    }
    fn end(&mut self) -> Result<(), ae::Error> {
        self.utility.end_undo_group()
    }
}

// Returns true only when an animated stream was committed here. Static Points
// continue through the already-verified normal PF change/Undo path. No suite,
// stream/effect handle or value is retained beyond this callback.
pub(crate) fn animated(
    input: &ae::InData,
    params: &ae::Parameters<Params>,
    id: Option<ae::aegp::PluginId>,
    index: usize,
    target: (i32, i32),
) -> Result<bool, ae::Error> {
    let id = id.ok_or(ae::Error::BadCallbackParameter)?;
    let param = *plane::CORNERS
        .get(index)
        .ok_or(ae::Error::BadCallbackParameter)?;
    let param_index = params.index(param).ok_or(ae::Error::BadCallbackParameter)? as i32;
    params.get(param)?.as_point()?;
    let interface = ae::aegp::suites::PFInterface::new()?;
    let effects = ae::aegp::suites::Effect::new()?;
    let streams = ae::aegp::suites::Stream::new()?;
    let keys = ae::aegp::suites::Keyframe::new()?;
    let utility = ae::aegp::suites::Utility::new()?;
    let effect = interface.new_effect_for_effect(input.effect_ref(), id)?;
    let result = (|| {
        let stream = streams.new_effect_stream_by_index(effect, id, param_index)?;
        if streams.stream_type(&stream)? != ae::aegp::StreamType::TwoDSpatial {
            return Err(ae::Error::BadCallbackParameter);
        }
        let count = keys.stream_num_kfs(&stream)?;
        if count < 0 {
            return Err(ae::Error::BadCallbackParameter);
        }
        if count == 0 {
            return Ok(false);
        }
        let time = interface.convert_effect_to_comp_time(
            input.effect_ref(),
            input.current_time(),
            input.time_scale(),
        )?;
        let target = ae::aegp::StreamValue::TwoDSpatial {
            x: f64::from(target.0) / 65536.0,
            y: f64::from(target.1) / 65536.0,
        };
        let mut adapter = Adapter {
            keys: &keys,
            utility: &utility,
            stream: &stream,
            time,
            target,
        };
        commit(&mut adapter, count)?;
        Ok(true)
    })();
    let disposed = effects.dispose_effect(effect);
    match (result, disposed) {
        (Ok(value), Ok(())) => Ok(value),
        (Err(error), _) | (_, Err(error)) => Err(error),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    struct Host {
        keys: Vec<(i32, i32)>,
        existing: bool,
        fail: u8,
        begins: usize,
        ends: usize,
    }
    impl KeyHost for Host {
        fn begin(&mut self) -> Result<(), ae::Error> {
            if self.fail == 1 {
                return Err(ae::Error::BadCallbackParameter);
            }
            self.begins += 1;
            Ok(())
        }
        fn insert(&mut self) -> Result<i32, ae::Error> {
            if self.fail == 2 {
                return Err(ae::Error::BadCallbackParameter);
            }
            if !self.existing {
                self.keys.insert(1, (5, 0));
            }
            Ok(1)
        }
        fn write(&mut self, i: i32) -> Result<(), ae::Error> {
            if self.fail == 3 {
                return Err(ae::Error::BadCallbackParameter);
            }
            self.keys[i as usize].1 = 99;
            Ok(())
        }
        fn count(&mut self) -> Result<i32, ae::Error> {
            Ok(self.keys.len() as i32)
        }
        fn remove(&mut self, i: i32) -> Result<(), ae::Error> {
            self.keys.remove(i as usize);
            Ok(())
        }
        fn end(&mut self) -> Result<(), ae::Error> {
            self.ends += 1;
            if self.fail == 4 {
                Err(ae::Error::BadCallbackParameter)
            } else {
                Ok(())
            }
        }
    }
    fn host(existing: bool, fail: u8) -> Host {
        Host {
            keys: vec![(0, 12), (10, 24), (20, 36)],
            existing,
            fail,
            begins: 0,
            ends: 0,
        }
    }
    #[test]
    fn existing_key_changes_only_itself_and_preserves_other_times_and_values() {
        let mut h = host(true, 0);
        commit(&mut h, 3).unwrap();
        assert_eq!(h.keys, vec![(0, 12), (10, 99), (20, 36)]);
        assert_eq!((h.begins, h.ends), (1, 1));
    }
    #[test]
    fn new_current_key_keeps_every_existing_key() {
        let mut h = host(false, 0);
        commit(&mut h, 3).unwrap();
        assert_eq!(h.keys, vec![(0, 12), (5, 99), (10, 24), (20, 36)]);
    }
    #[test]
    fn failed_write_removes_only_a_newly_inserted_key_and_balances_undo() {
        for existing in [false, true] {
            let mut h = host(existing, 3);
            let original = h.keys.clone();
            assert!(commit(&mut h, 3).is_err());
            assert_eq!(h.keys, original);
            assert_eq!((h.begins, h.ends), (1, 1));
        }
    }
    #[test]
    fn undo_start_insert_and_close_failures_are_not_reported_as_success() {
        for failure in [1, 2, 4] {
            let mut h = host(true, failure);
            assert!(commit(&mut h, 3).is_err());
            assert_eq!(
                (h.begins, h.ends),
                if failure == 1 { (0, 0) } else { (1, 1) }
            );
        }
    }
}
