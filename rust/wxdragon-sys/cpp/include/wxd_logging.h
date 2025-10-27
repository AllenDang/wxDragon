#ifndef WXD_LOGGING_H
#define WXD_LOGGING_H

#include <stdarg.h>

#ifdef __cplusplus
extern "C" {
#endif

// Helper function to log messages from C/C++ into Rust's log system,
// this function is implemented in Rust.
// - `level` is an integer representing log level (e.g., 1=Error, 2=Warn, 3=Info, 4=Debug, 5=Trace)
// - `msg` is a NUL-terminated C string pointer. If null, the call logs a warning and returns.
void
wxd_rust_log(int level, const char* msg);

// Provides a printf-style logging interface for C/C++.
// - `level` is an integer representing log level (e.g., 1=Error, 2=Warn, 3=Info, 4=Debug, 5=Trace)
// - `fmt` is a NUL-terminated C format string, followed by variable arguments.
void
wxd_log_printf(int level, const char* fmt, ...);

// Provides a vprintf-style logging interface for C/C++.
// - `level` is an integer representing log level (e.g., 1=Error, 2=Warn, 3=Info, 4=Debug, 5=Trace)
// - `fmt` is a NUL-terminated C format string
// If you already have a va_list available, this version can be used directly
void
wxd_log_vprintf(int level, const char* fmt, va_list ap);

// Convenience macros: Only Error and Warn include [file:line].
// 1=Error, 2=Warn, 3=Info, 4=Debug, 5=Trace

// Error/Warn: include file:line
#define WXD_LOG_ERROR(msg) wxd_log_printf(1, "[%s:%d] %s", __FILE__, __LINE__, (msg))
#define WXD_LOG_WARN(msg)  wxd_log_printf(2, "[%s:%d] %s", __FILE__, __LINE__, (msg))

#define WXD_LOG_ERRORF(fmt, ...) wxd_log_printf(1, "[%s:%d] " fmt, __FILE__, __LINE__, __VA_ARGS__)
#define WXD_LOG_WARNF(fmt, ...)  wxd_log_printf(2, "[%s:%d] " fmt, __FILE__, __LINE__, __VA_ARGS__)

// Info/Debug/Trace: plain messages (no file:line)
#define WXD_LOG_INFO(msg)  wxd_log_printf(3, "%s", (msg))
#define WXD_LOG_DEBUG(msg) wxd_log_printf(4, "%s", (msg))
#define WXD_LOG_TRACE(msg) wxd_log_printf(5, "%s", (msg))

#define WXD_LOG_INFOF(fmt, ...)  wxd_log_printf(3, (fmt), __VA_ARGS__)
#define WXD_LOG_DEBUGF(fmt, ...) wxd_log_printf(4, (fmt), __VA_ARGS__)
#define WXD_LOG_TRACEF(fmt, ...) wxd_log_printf(5, (fmt), __VA_ARGS__)

#ifdef __cplusplus
}
#endif

#endif /* WXD_LOGGING_H */
