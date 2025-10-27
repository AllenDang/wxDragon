#include <wx/wxprec.h>
#include <wx/wx.h>

#include "../include/wxdragon.h" // Main header for WXD_EXPORTED, types, and wxd_pickers.h
#include "wxd_utils.h"           // For utility macros, if any are needed

#include <wx/fontpicker.h> // For wxFontPickerCtrl
#include <wx/font.h>       // For wxFont

// External declaration for wxd_Font_Destroy if not in wxdragon.h (it should be for proper resource management)
// extern "C" void wxd_Font_Destroy(wxd_Font_t* font); // Assuming wxd_Font_Destroy is defined elsewhere

// --- FontPickerCtrl ---
WXD_EXPORTED wxd_FontPickerCtrl_t*
wxd_FontPickerCtrl_Create(wxd_Window_t* parent, wxd_Id id,
                          const wxd_Font_t* initial_font, // Can be NULL
                          wxd_Point pos, wxd_Size size, wxd_Style_t style)
{
    wxFont font_to_use = initial_font ? *((wxFont*)initial_font) : wxNullFont;

    return (wxd_FontPickerCtrl_t*)new wxFontPickerCtrl((wxWindow*)parent, id, font_to_use,
                                                       wxPoint(pos.x, pos.y),
                                                       wxSize(size.width, size.height), style,
                                                       wxDefaultValidator, wxFontPickerCtrlNameStr);
}

WXD_EXPORTED wxd_Font_t*
wxd_FontPickerCtrl_GetSelectedFont(wxd_FontPickerCtrl_t* self)
{
    if (!self)
        return NULL;
    wxFont selected_font = ((wxFontPickerCtrl*)self)->GetSelectedFont();
    if (!selected_font.IsOk()) {
        return NULL; // Return NULL if the font is not valid
    }
    // Rust will own this new wxFont instance and must call wxd_Font_Destroy
    return (wxd_Font_t*)new wxFont(selected_font);
}

WXD_EXPORTED void
wxd_FontPickerCtrl_SetSelectedFont(wxd_FontPickerCtrl_t* self, const wxd_Font_t* font)
{
    if (!self)
        return;
    if (!font) {
        // wxWidgets documentation is unclear if SetSelectedFont accepts wxNullFont directly
        // or if we should pass a default font or do nothing. For now, do nothing if null.
        // It might be better to pass wxNullFont if that's valid for wxFontPickerCtrl.
        // Let's assume for now it can take a wxNullFont if font is null.
        ((wxFontPickerCtrl*)self)->SetSelectedFont(wxNullFont);
        return;
    }
    ((wxFontPickerCtrl*)self)->SetSelectedFont(*((wxFont*)font));
}