// #include "wx/wxprec.h"
// #ifndef WX_PRECOMP
// #include "wx/wx.h"
// #endif

#include "../include/wxdragon.h"
#include "wxd_logging.h"

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stddef.h>

// helper function: format with va_list and send to wxd_rust_log
static void
wxd_log_vformat_and_send(int level, const char* fmt, va_list ap)
{
    if (!fmt) {
        wxd_rust_log(level, "(null)");
        return;
    }

#if defined(_MSC_VER)
    // copy va_list and compute required size first, so that original ap is not consumed
    va_list ap_copy;
    va_copy(ap_copy, ap);
    int required = _vscprintf(fmt, ap_copy);
    va_end(ap_copy);
    if (required < 0) {
        wxd_rust_log(level, "(format error)");
        return;
    }
    // +1 for NUL
    size_t bufsize = (size_t)required + 1;
    char* buf = (char*)malloc(bufsize);
    if (!buf) {
        wxd_rust_log(level, "(alloc error)");
        return;
    }
    // use original ap to write
    int written = vsnprintf_s(buf, bufsize, _TRUNCATE, fmt, ap);
    if (written < 0) {
        free(buf);
        wxd_rust_log(level, "(vsnprintf error)");
        return;
    }
#else
    // copy va_list and compute required size first, so that original ap is not consumed
    va_list ap_copy;
    va_copy(ap_copy, ap);
    int required = vsnprintf(NULL, 0, fmt, ap_copy);
    va_end(ap_copy);
    if (required < 0) {
        wxd_rust_log(level, "(format error)");
        return;
    }
    size_t bufsize = (size_t)required + 1;
    char* buf = (char*)malloc(bufsize);
    if (!buf) {
        wxd_rust_log(level, "(alloc error)");
        return;
    }
    int written = vsnprintf(buf, bufsize, fmt, ap);
    if (written < 0) {
        free(buf);
        wxd_rust_log(level, "(vsnprintf error)");
        return;
    }
#endif

    // call the Rust side fixed signature function
    wxd_rust_log(level, buf);
    free(buf);
}

extern "C" void
wxd_log_vprintf(int level, const char* fmt, va_list ap)
{
    // copy va_list to avoid affecting the caller's va_list
    va_list ap_copy;
    va_copy(ap_copy, ap);
    wxd_log_vformat_and_send(level, fmt, ap_copy);
    va_end(ap_copy);
}

extern "C" void
wxd_log_printf(int level, const char* fmt, ...)
{
    va_list ap;
    va_start(ap, fmt);
    wxd_log_vformat_and_send(level, fmt, ap);
    va_end(ap);
}
