#include <wx/wx.h>
#include "wxd_utils.h"
#include <cstring>   // For strncpy, strlen
#include <algorithm> // For std::min
#include <cstdlib>   // For strdup

namespace wxd_cpp_utils {

size_t
copy_wxstring_to_buffer(const wxString& str, char* buffer, size_t buffer_len)
{
    wxScopedCharBuffer utf8_buf = str.ToUTF8();
    size_t source_len = utf8_buf.length(); // Length of the UTF-8 string, excluding null terminator

    if (buffer && buffer_len > 0) {
        // Number of bytes to copy, leaving space for null terminator
        size_t copy_len = std::min(source_len, buffer_len - 1);

        // Using memcpy is often safer with wxScopedCharBuffer as data() might not be null-terminated by itself
        // if the original string contained embedded nulls (though unlikely for UI strings).
        // wxScopedCharBuffer::data() returns a const char*, so we might need a cast if buffer is not const,
        // or just copy it using a loop or memcpy if available.
        // However, wxScopedCharBuffer ensures it's null-terminated if it was converted from wxString.
        memcpy(buffer, utf8_buf.data(), copy_len);
        buffer[copy_len] = '\0'; // Ensure null termination
    }

    return source_len; // Return the original length of the UTF-8 string (excluding its null terminator)
}

}

// --- Colour Conversion Helper Implementations ---

wxColour
wxdColourToWxColour(unsigned long wxd_colour_val)
{
    unsigned char r = (wxd_colour_val >> 24) & 0xFF;
    unsigned char g = (wxd_colour_val >> 16) & 0xFF;
    unsigned char b = (wxd_colour_val >> 8) & 0xFF;
    unsigned char a = wxd_colour_val & 0xFF;
    return wxColour(r, g, b, a);
}

unsigned long
wxColourToWxdColour(const wxColour& wx_colour_obj)
{
    return (static_cast<unsigned long>(wx_colour_obj.Red()) << 24) |
           (static_cast<unsigned long>(wx_colour_obj.Green()) << 16) |
           (static_cast<unsigned long>(wx_colour_obj.Blue()) << 8) |
           static_cast<unsigned long>(wx_colour_obj.Alpha());
}