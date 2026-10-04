//! Independent native UI; no provider adapter or activation result yet.
use super::*;

unsafe extern "C" {
    fn eg_show_license_window(version: *const std::ffi::c_char) -> bool;
}

pub(crate) fn show() -> Result<(), ae::Error> {
    // Headless/aerender must never show activation or About UI.
    if ae::pf::suites::App::new()?.is_render_engine()? {
        return Ok(());
    }
    const VERSION: &std::ffi::CStr =
        match std::ffi::CStr::from_bytes_with_nul(concat!(env!("CARGO_PKG_VERSION"), "\0").as_bytes()) {
            Ok(version) => version,
            Err(_) => panic!("Invalid package version"),
        };
    // SAFETY: NUL-terminated static string; synchronous native UI consumes it
    // during the call. The bridge rejects non-main-thread calls and catches exceptions.
    if unsafe { eg_show_license_window(VERSION.as_ptr()) } {
        Ok(())
    } else {
        Err(ae::Error::Generic)
    }
}
