#ifndef WXD_UTILS_H
#define WXD_UTILS_H

#include <wx/gdicmn.h>            // For wxPoint, wxSize, wxDefaultPosition, wxDefaultSize
#include <wx/string.h>            // For wxString
#include <wx/window.h>            // For wxWindow and FromDIP/ToDIP functions
#include "../include/wxd_types.h" // For wxd_Point, wxd_Size (CHANGED from wxdragon.h)
#include <wx/colour.h>            // For wxColour type

#ifdef __cplusplus
// Helper macro to convert const char* to wxString, handling nulls and UTF-8
#define WXD_STR_TO_WX_STRING_UTF8_NULL_OK(input_text) \
    wxString::FromUTF8(input_text ? input_text : "")
// Helper macro for getting wxString result into C buffer
// Note: Relies on wxd_cpp_utils::copy_wxstring_to_buffer being declared below or already visible
#define GET_WX_STRING_RESULT(wx_str_expr, c_buffer, c_buf_len) \
    wxd_cpp_utils::copy_wxstring_to_buffer(wx_str_expr, c_buffer, c_buf_len)
#endif

// This function must be declared for C linkage if called from C or bindgen expects C linkage.
// It is typically implemented in a .cpp file.
#ifdef __cplusplus
extern "C" {
#endif

// Free a string that was allocated by Rust (from CString::into_raw())
// This function is implemented in Rust and must be used instead of free() for Rust-allocated strings
void
wxd_Variant_Free_Rust_String(char* str);

#ifdef __cplusplus
} // extern "C"
#endif

namespace wxd_cpp_utils {

// Inline helper function to convert wxd_Point to wxPoint
// This function automatically applies FromDIP() to ensure DPI-aware positioning
inline wxPoint
to_wx(const wxd_Point& p)
{
    if (p.x == -1 && p.y == -1) { // Common convention for default pos
        return wxDefaultPosition;
    }
    // Apply FromDIP to convert device-independent pixels to physical pixels
    // This ensures consistent positioning across different DPI settings and build configurations
    return wxWindow::FromDIP(wxPoint(p.x, p.y), nullptr);
}

// Inline helper function to convert wxd_Size to wxSize
// This function automatically applies FromDIP() to ensure DPI-aware sizing
inline wxSize
to_wx(const wxd_Size& s)
{
    if (s.width == -1 && s.height == -1) { // Common convention for default size
        return wxDefaultSize;
    }
    // Apply FromDIP to convert device-independent pixels to physical pixels
    // This ensures consistent sizing across different DPI settings and build configurations
    return wxWindow::FromDIP(wxSize(s.width, s.height), nullptr);
}

/**
 * @brief Copies a wxString to a C char buffer, ensuring null termination.
 *
 * Converts the wxString to UTF-8 before copying.
 * If the buffer is too small, the string will be truncated, but still null-terminated.
 *
 * @param str The source wxString.
 * @param buffer The destination char buffer.
 * @param buffer_len The total size of the destination buffer (including space for null terminator).
 * @return The number of bytes that would be written if the buffer was large enough 
 *         (excluding the null terminator), similar to snprintf. This is the length of str.ToUTF8().
 */
size_t
copy_wxstring_to_buffer(const wxString& str, char* buffer, size_t buffer_len);

}

// Helper to convert wxd_Colour_t representation (unsigned long RGBA) to wxColour
wxColour
wxdColourToWxColour(unsigned long wxd_colour_val);

// Helper to convert wxColour to wxd_Colour_t representation (unsigned long RGBA)
unsigned long
wxColourToWxdColour(const wxColour& wx_colour_obj);

#endif // WXD_UTILS_H