//! Final UI gesture only. An animated Point must change at the current time,
//! rather than offsetting its entire track through PF CHANGED_VALUE.
use super::*;

// EndAddKeyframes commits the time and value as ONE undoable operation.
// Separate InsertKeyframe/SetKeyframeValue left a new key behind after one Undo
// in the exact AE25.6 Dev173 trial, despite the outer Utility undo group.
trait KeyHost {
    fn add(&mut self) -> Result<i32, ae::Error>;
    fn write(&mut self, index: i32) -> Result<(), ae::Error>;
    fn end(&mut self, commit: bool) -> Result<(), ae::Error>;
}
fn commit<H: KeyHost>(host: &mut H) -> Result<(), ae::Error> {
    let prepared = host.add().and_then(|index| host.write(index));
    let ended = host.end(prepared.is_ok());
    prepared.and(ended)
}

// The pinned Rust wrapper ends batches in Drop and discards EndAddKeyframes
// errors. Use the SDK table directly so failure cannot be reported as success.
// All pointers/handles are callback-local; validate functions before starting.
struct Batch<'a> {
    suite: &'a ae::sys::AEGP_KeyframeSuite5,
    handle: Option<ae::sys::AEGP_AddKeyframesInfoH>,
    stream: ae::sys::AEGP_StreamRefH,
    time: ae::sys::A_Time,
    target: ae::sys::AEGP_StreamValue2,
}
fn checked(code: ae::sys::A_Err) -> Result<(), ae::Error> {
    if code == 0 {
        Ok(())
    } else {
        Err(ae::Error::from(code))
    }
}
impl KeyHost for Batch<'_> {
    fn add(&mut self) -> Result<i32, ae::Error> {
        let mut index = 0;
        checked(unsafe {
            (self
                .suite
                .AEGP_AddKeyframes
                .ok_or(ae::Error::MissingSuite)?)(
                self.handle.ok_or(ae::Error::BadCallbackParameter)?,
                ae::aegp::TimeMode::CompTime.into(),
                &self.time,
                &mut index,
            )
        })?;
        Ok(index)
    }
    fn write(&mut self, index: i32) -> Result<(), ae::Error> {
        debug_assert_eq!(self.target.streamH, self.stream);
        checked(unsafe {
            (self
                .suite
                .AEGP_SetAddKeyframe
                .ok_or(ae::Error::MissingSuite)?)(
                self.handle.ok_or(ae::Error::BadCallbackParameter)?,
                index,
                &self.target,
            )
        })
    }
    fn end(&mut self, commit: bool) -> Result<(), ae::Error> {
        let end = self
            .suite
            .AEGP_EndAddKeyframes
            .ok_or(ae::Error::MissingSuite)?;
        let handle = self.handle.take().ok_or(ae::Error::BadCallbackParameter)?;
        checked(unsafe { end(commit.into(), handle) })
    }
}
impl Drop for Batch<'_> {
    fn drop(&mut self) {
        // Only unwinding/early construction exit reaches this path. Never
        // publish an incomplete value; the normal end path reports its error.
        if let (Some(handle), Some(end)) = (self.handle.take(), self.suite.AEGP_EndAddKeyframes) {
            unsafe {
                end(0, handle);
            }
        }
    }
}
fn atomic_key(
    input: &ae::InData,
    stream: &ae::aegp::StreamReferenceHandle,
    time: ae::Time,
    target: ae::aegp::StreamValue,
) -> Result<(), ae::Error> {
    let basic = input.pica_basic_suite_ptr();
    if basic.is_null() {
        return Err(ae::Error::MissingSuite);
    }
    let basic = unsafe { &*basic };
    let acquire = basic.AcquireSuite.ok_or(ae::Error::MissingSuite)?;
    let release = basic.ReleaseSuite.ok_or(ae::Error::MissingSuite)?;
    let name = ae::sys::kAEGPKeyframeSuite.as_ptr().cast();
    let version = ae::sys::kAEGPKeyframeSuiteVersion5 as i32;
    let mut ptr: *const c_void = std::ptr::null();
    checked(unsafe { acquire(name, version, &mut ptr) })?;
    let result = (|| {
        if ptr.is_null() {
            return Err(ae::Error::MissingSuite);
        }
        let suite = unsafe { &*ptr.cast::<ae::sys::AEGP_KeyframeSuite5>() };
        let start = suite
            .AEGP_StartAddKeyframes
            .ok_or(ae::Error::MissingSuite)?;
        suite.AEGP_AddKeyframes.ok_or(ae::Error::MissingSuite)?;
        suite.AEGP_SetAddKeyframe.ok_or(ae::Error::MissingSuite)?;
        suite.AEGP_EndAddKeyframes.ok_or(ae::Error::MissingSuite)?;
        let mut handle = std::ptr::null_mut();
        checked(unsafe { start(stream.as_ptr(), &mut handle) })?;
        if handle.is_null() {
            return Err(ae::Error::BadCallbackParameter);
        }
        let mut batch = Batch {
            suite,
            handle: Some(handle),
            stream: stream.as_ptr(),
            time: time.into(),
            target: ae::sys::AEGP_StreamValue2 {
                streamH: stream.as_ptr(),
                val: target.to_sys(),
            },
        };
        commit(&mut batch)
    })();
    let released = checked(unsafe { release(name, version) });
    result.and(released)
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
        atomic_key(input, &stream, time, target)?;
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
        saved: Vec<(i32, i32)>,
        pending: Vec<(i32, i32)>,
        existing: bool,
        fail: u8,
        ends: Vec<bool>,
        undo: Option<Vec<(i32, i32)>>,
    }
    impl KeyHost for Host {
        fn add(&mut self) -> Result<i32, ae::Error> {
            if self.fail == 1 {
                return Err(ae::Error::Generic);
            }
            if !self.existing {
                self.pending.insert(1, (5, 0));
            }
            Ok(1)
        }
        fn write(&mut self, i: i32) -> Result<(), ae::Error> {
            if self.fail == 2 {
                return Err(ae::Error::Generic);
            }
            self.pending[i as usize].1 = 99;
            Ok(())
        }
        fn end(&mut self, commit: bool) -> Result<(), ae::Error> {
            self.ends.push(commit);
            if self.fail == 3 {
                return Err(ae::Error::Generic);
            }
            if commit {
                self.undo = Some(self.saved.clone());
                self.saved = self.pending.clone();
            }
            Ok(())
        }
    }
    fn host(existing: bool, fail: u8) -> Host {
        let saved = vec![(0, 12), (10, 24), (20, 36)];
        Host {
            pending: saved.clone(),
            saved,
            existing,
            fail,
            ends: Vec::new(),
            undo: None,
        }
    }
    #[test]
    fn existing_key_changes_only_itself_and_preserves_other_times_and_values() {
        let mut h = host(true, 0);
        commit(&mut h).unwrap();
        assert_eq!(h.saved, vec![(0, 12), (10, 99), (20, 36)]);
        assert_eq!(h.ends, [true]);
    }
    #[test]
    fn new_current_key_has_one_undo_restoring_the_entire_original_track() {
        let mut h = host(false, 0);
        let original = h.saved.clone();
        commit(&mut h).unwrap();
        assert_eq!(h.saved, vec![(0, 12), (5, 99), (10, 24), (20, 36)]);
        h.saved = h.undo.take().unwrap();
        assert_eq!(h.saved, original);
    }
    #[test]
    fn failed_time_or_value_preparation_discards_the_batch_without_a_saved_key() {
        for existing in [false, true] {
            for fail in [1, 2] {
                let mut h = host(existing, fail);
                let original = h.saved.clone();
                assert!(commit(&mut h).is_err());
                assert_eq!(h.saved, original);
                assert!(h.undo.is_none());
                assert_eq!(h.ends, [false]);
            }
        }
    }
    #[test]
    fn end_failure_is_not_reported_as_success() {
        let mut h = host(false, 3);
        assert!(commit(&mut h).is_err());
        assert_eq!(h.ends, [true]);
    }
}
