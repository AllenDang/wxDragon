//! Helper for the functions called by the C++ code.

use std::panic::{AssertUnwindSafe, catch_unwind};

/// Runs `callback`, returning `fallback` if it panics.
///
/// Every function called from C++ must use this, as a panic can't unwind
/// through the C++ frames and aborts the whole process instead, losing any
/// unsaved work of the application, which is a harsh punishment for e.g. an
/// out of range index in a data view model callback.
///
/// `context` is only used in the message logged when a panic does happen.
pub(crate) fn guard_ffi_callback<R>(context: &str, fallback: R, callback: impl FnOnce() -> R) -> R {
    match catch_unwind(AssertUnwindSafe(callback)) {
        Ok(value) => value,
        Err(_) => {
            log::error!("Panic in {context} called from C++, ignoring it.");
            fallback
        }
    }
}
