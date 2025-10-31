#include <wx/wxprec.h>
#include <wx/wx.h>
#include "../include/wxdragon.h"
#include "wxd_utils.h"  // For wxd_cpp_utils::to_wxPoint, to_wxSize if used, and string conversions.
#include <wx/fontdlg.h> // wxFontDialog

extern "C" {

// --- wxFontData Implementation ---

wxd_FontData_t*
wxd_FontData_Create(void)
{
    wxFontData* data = new wxFontData();
    return reinterpret_cast<wxd_FontData_t*>(data);
}

void
wxd_FontData_Destroy(wxd_FontData_t* self)
{
    if (!self)
        return;
    wxFontData* data = reinterpret_cast<wxFontData*>(self);
    delete data;
}

void
wxd_FontData_EnableEffects(wxd_FontData_t* self, bool enable)
{
    if (!self)
        return;
    wxFontData* data = reinterpret_cast<wxFontData*>(self);
    data->EnableEffects(enable);
}

bool
wxd_FontData_GetEnableEffects(wxd_FontData_t* self)
{
    if (!self)
        return false;
    wxFontData* data = reinterpret_cast<wxFontData*>(self);
    return data->GetEnableEffects();
}

void
wxd_FontData_SetInitialFont(wxd_FontData_t* self, const wxd_Font_t* font)
{
    if (!self || !font)
        return;
    wxFontData* data = reinterpret_cast<wxFontData*>(self);
    wxFont* wxfont = reinterpret_cast<wxFont*>(const_cast<wxd_Font_t*>(font));
    data->SetInitialFont(*wxfont);
}

// --- wxFont Implementation ---

wxd_Font_t*
wxd_Font_Create(void)
{
    wxFont* font = new wxFont(wxNullFont);
    return reinterpret_cast<wxd_Font_t*>(font);
}

void
wxd_Font_Destroy(wxd_Font_t* self)
{
    if (!self)
        return;
    wxFont* font = reinterpret_cast<wxFont*>(self);
    delete font;
}

int
wxd_Font_GetPointSize(wxd_Font_t* self)
{
    if (!self)
        return 0;
    wxFont* font = reinterpret_cast<wxFont*>(self);
    return font->GetPointSize();
}

int
wxd_Font_GetFamily(wxd_Font_t* self)
{
    if (!self)
        return 0;
    wxFont* font = reinterpret_cast<wxFont*>(self);
    return font->GetFamily();
}

int
wxd_Font_GetStyle(wxd_Font_t* self)
{
    if (!self)
        return 0;
    wxFont* font = reinterpret_cast<wxFont*>(self);
    return font->GetStyle();
}

int
wxd_Font_GetWeight(wxd_Font_t* self)
{
    if (!self)
        return 0;
    wxFont* font = reinterpret_cast<wxFont*>(self);
    return font->GetWeight();
}

void
wxd_Font_SetWeight(wxd_Font_t* self, int weight)
{
    if (!self)
        return;
    wxFont* font = reinterpret_cast<wxFont*>(self);
    font->SetWeight(static_cast<wxFontWeight>(weight));
}

bool
wxd_Font_GetUnderlined(wxd_Font_t* self)
{
    if (!self)
        return false;
    wxFont* font = reinterpret_cast<wxFont*>(self);
    return font->GetUnderlined();
}

int
wxd_Font_GetFaceName(wxd_Font_t* self, char* buffer, size_t buffer_len)
{
    if (!self)
        return -1;
    wxFont* font = reinterpret_cast<wxFont*>(self);
    wxString faceName = font->GetFaceName();
    return (int)wxd_cpp_utils::copy_wxstring_to_buffer(faceName, buffer, buffer_len);
}

bool
wxd_Font_IsOk(wxd_Font_t* self)
{
    if (!self)
        return false;
    wxFont* font = reinterpret_cast<wxFont*>(self);
    return font->IsOk();
}

WXD_EXPORTED bool
wxd_Font_AddPrivateFont(const char* font_file_path)
{
    if (!font_file_path)
        return false;
#if wxUSE_PRIVATE_FONTS
    return wxFont::AddPrivateFont(WXD_STR_TO_WX_STRING_UTF8_NULL_OK(font_file_path));
#else
    return false;
#endif
}

WXD_EXPORTED wxd_Font_t*
wxd_Font_CreateEx(int point_size, int family, int style, int weight, bool underlined,
                  const char* face_name)
{
    wxString wx_face_name = WXD_STR_TO_WX_STRING_UTF8_NULL_OK(face_name);
    // wxFONTFAMILY_DEFAULT, wxFONTSTYLE_NORMAL, wxFONTWEIGHT_NORMAL are wxWidgets enums.
    // The C API uses ints; they should correspond to these enum values.
    // Example: family might be an int like 70 for wxFONTFAMILY_DEFAULT.
    // Need to cast these ints to the respective wx enums if they are not directly compatible.
    // For now, assuming direct cast is okay if the C-side constants match wx values.
    wxFont* font = new wxFont(point_size, static_cast<wxFontFamily>(family),
                              static_cast<wxFontStyle>(style), static_cast<wxFontWeight>(weight),
                              underlined, wx_face_name);
    if (font && font->IsOk()) {
        return (wxd_Font_t*)font;
    }
    else {
        delete font; // Delete if not Ok
        return NULL;
    }
}

void
wxd_Font_SetPointSize(wxd_Font_t* self, int point_size)
{
    if (!self)
        return;
    wxFont* font = reinterpret_cast<wxFont*>(self);
    font->SetPointSize(point_size);
}

void
wxd_Font_SetFamily(wxd_Font_t* self, int family)
{
    if (!self)
        return;
    wxFont* font = reinterpret_cast<wxFont*>(self);
    font->SetFamily(static_cast<wxFontFamily>(family));
}

void
wxd_Font_SetStyle(wxd_Font_t* self, int style)
{
    if (!self)
        return;
    wxFont* font = reinterpret_cast<wxFont*>(self);
    font->SetStyle(static_cast<wxFontStyle>(style));
}

void
wxd_Font_SetUnderlined(wxd_Font_t* self, bool underlined)
{
    if (!self)
        return;
    wxFont* font = reinterpret_cast<wxFont*>(self);
    font->SetUnderlined(underlined);
}

// --- wxFontDialog Implementation ---

wxd_FontDialog_t*
wxd_FontDialog_Create(wxd_Window_t* parent, const char* title, wxd_FontData_t* data)
{
    wxWindow* parentWin = reinterpret_cast<wxWindow*>(parent);
    wxFontData* fontData = nullptr;

    if (data) {
        fontData = reinterpret_cast<wxFontData*>(data);
    }
    else {
        // If no data provided, create a default one
        fontData = new wxFontData();
    }

    wxFontDialog* dialog = new wxFontDialog(parentWin, *fontData);

    // Set title if provided
    if (title && *title) {
        dialog->SetTitle(WXD_STR_TO_WX_STRING_UTF8_NULL_OK(title));
    }

    // If we created our own fontData, we need to delete it
    if (!data) {
        delete fontData;
    }

    return reinterpret_cast<wxd_FontDialog_t*>(dialog);
}

wxd_FontData_t*
wxd_FontDialog_GetFontData(wxd_FontDialog_t* self)
{
    if (!self)
        return nullptr;

    wxFontDialog* dialog = reinterpret_cast<wxFontDialog*>(self);
    // Note: This returns a reference to the internal wxFontData, not a new instance
    // The pointer will be valid as long as the dialog exists
    return reinterpret_cast<wxd_FontData_t*>(&dialog->GetFontData());
}

wxd_Font_t*
wxd_FontDialog_GetFont(wxd_FontDialog_t* self)
{
    if (!self)
        return nullptr;

    wxFontDialog* dialog = reinterpret_cast<wxFontDialog*>(self);
    wxFont font = dialog->GetFontData().GetChosenFont();

    if (!font.IsOk()) {
        return nullptr;
    }

    // Create a new wxFont object to return (will be owned by Rust)
    wxFont* newFont = new wxFont(font);
    return reinterpret_cast<wxd_Font_t*>(newFont);
}

} // extern "C"