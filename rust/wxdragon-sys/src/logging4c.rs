//! C-callable logging helpers.
//!
//! This module exposes simple extern "C" functions that C/C++ code can call
//! to forward log messages into Rust's `log` ecosystem.

fn map_level(level: i32) -> log::Level {
    match level {
        1 => log::Level::Error,
        2 => log::Level::Warn,
        3 => log::Level::Info,
        4 => log::Level::Debug,
        5 => log::Level::Trace,
        _ => log::Level::Info,
    }
}

/// General logging entrypoint for C/C++ with explicit level.
///
/// Safety and behavior:
/// - `level` must be a valid log level integer (1=Error, 2=Warn, 3=Info, 4=Debug, 5=Trace).
/// - `msg` is a NUL-terminated C string pointer. If null, the call logs a warning and returns.
/// - Converts non-UTF8 lossily; never panics.
#[unsafe(no_mangle)]
pub extern "C" fn wxd_rust_log(level: i32, msg: *const std::os::raw::c_char) {
    let c_cpp = "C/C++";
    if msg.is_null() {
        log::warn!("{c_cpp}: wxd_rust_log called with null pointer (level: {level:?})");
        return;
    }

    let cstr = unsafe { std::ffi::CStr::from_ptr(msg) };
    let text = cstr.to_string_lossy();
    match map_level(level) {
        log::Level::Error => log::error!("{c_cpp}: {text}"),
        log::Level::Warn => log::warn!("{c_cpp}: {text}"),
        log::Level::Info => log::info!("{c_cpp}: {text}"),
        log::Level::Debug => log::debug!("{c_cpp}: {text}"),
        log::Level::Trace => log::trace!("{c_cpp}: {text}"),
    }
}
