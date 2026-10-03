//! Per-instance initialization receipt. No geometry, parameter data or host handles.
//! CompletelyGeneral uses a synchronous main-thread, process-private TLS request;
//! foreign callbacks outside that scope are no-ops and no extra pointer is read.
use super::*;
use std::cell::RefCell;

const CURRENT: u16 = 1;
const MAGIC: &[u8; 8] = b"EGFXBND1";
#[derive(Default)]
pub(crate) struct State { generation: u16 }
impl State {
    pub fn flatten(&self) -> (u16, Vec<u8>) {
        let mut bytes = MAGIC.to_vec();
        bytes.extend(self.generation.to_le_bytes());
        (1, bytes)
    }
    pub fn unflatten(version: u16, bytes: &[u8]) -> Result<Self, ae::Error> {
        // Previous production used () and stored no sequence payload.
        if version == 0 && bytes.is_empty() { return Ok(Self::default()); }
        if version != 1 || bytes.len() != 10 || &bytes[..8] != MAGIC {
            return Err(ae::Error::InvalidParms);
        }
        let generation = u16::from_le_bytes([bytes[8], bytes[9]]);
        if generation > CURRENT { return Err(ae::Error::InvalidParms); }
        Ok(Self { generation })
    }
    pub fn service(&mut self) -> Result<(), ae::Error> {
        REQUEST.with(|slot| {
            let mut slot = slot.borrow_mut();
            let Some(request) = slot.as_mut() else { return Ok(()); };
            if request.response.is_some() { return Err(ae::Error::BadCallbackParameter); }
            if let Some(mark) = request.mark { self.generation = if mark { CURRENT } else { 0 }; }
            request.response = Some(self.generation);
            Ok(())
        })
    }
}
struct Request { mark: Option<bool>, response: Option<u16> }
thread_local! { static REQUEST: RefCell<Option<Request>> = const { RefCell::new(None) }; }
#[cfg(any(test, fstr_auto_binding))]
struct ClearRequest;
#[cfg(any(test, fstr_auto_binding))]
impl Drop for ClearRequest {
    fn drop(&mut self) { REQUEST.with(|slot| { slot.replace(None); }); }
}

#[cfg(fstr_auto_binding)]
pub fn query(id: ae::aegp::PluginId, effect: ae::aegp::EffectRefHandle, mark: Option<bool>) -> Result<bool, ae::Error> {
    if !lifecycle_probe::main_thread() { return Err(ae::Error::BadCallbackParameter); }
    // Caller has validated/reacquired the exact owned effect on the UI thread.
    let effects = ae::aegp::suites::Effect::new()?;
    REQUEST.with(|slot| {
        if slot.borrow().is_some() { return Err(ae::Error::BadCallbackParameter); }
        slot.replace(Some(Request { mark, response: None }));
        Ok(())
    })?;
    let _clear = ClearRequest;
    effects.effect_call_generic::<()>(effect, id, ae::Time { value: 0, scale: 1 },
        &ae::Command::CompletelyGeneral, None)?;
    let generation = REQUEST.with(|slot| slot.borrow().as_ref().and_then(|r| r.response))
        .ok_or(ae::Error::BadCallbackParameter)?;
    if generation > CURRENT || (mark.is_some_and(|m| generation != if m { CURRENT } else { 0 })) {
        return Err(ae::Error::BadCallbackParameter);
    }
    Ok(generation == CURRENT)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn legacy_empty_and_receipt_roundtrip_reject_foreign_or_future_data() {
        let legacy = State::unflatten(0, &[]).unwrap();
        assert_eq!(legacy.generation, 0);
        let state = State { generation: CURRENT };
        let (v, bytes) = state.flatten();
        assert_eq!(State::unflatten(v, &bytes).unwrap().generation, CURRENT);
        assert!(State::unflatten(0, b"foreign").is_err());
        assert!(State::unflatten(2, &bytes).is_err());
        for end in 0..bytes.len() { assert!(State::unflatten(v, &bytes[..end]).is_err()); }
        let mut future = bytes.clone(); future[8] = 2;
        assert!(State::unflatten(v, &future).is_err());
        let mut foreign = bytes; foreign[0] ^= 1;
        assert!(State::unflatten(v, &foreign).is_err());
    }
    #[test]
    fn scoped_receipt_mark_is_instance_local_and_foreign_callbacks_are_noops() {
        let mut a = State::default(); let b = State::default();
        a.service().unwrap(); assert_eq!(a.generation, 0);
        REQUEST.with(|slot| slot.replace(Some(Request { mark: Some(true), response: None })));
        { let _clear = ClearRequest; a.service().unwrap(); assert!(a.service().is_err()); }
        assert!(REQUEST.with(|slot| slot.borrow().is_none()));
        assert_eq!(a.generation, CURRENT); assert_eq!(b.generation, 0);
        REQUEST.with(|slot| slot.replace(Some(Request { mark: None, response: None })));
        { let _clear = ClearRequest; a.service().unwrap();
          assert_eq!(REQUEST.with(|slot| slot.borrow().as_ref().unwrap().response), Some(CURRENT)); }
        assert_eq!(State::unflatten(1, &a.flatten().1).unwrap().generation, CURRENT);
    }
}
