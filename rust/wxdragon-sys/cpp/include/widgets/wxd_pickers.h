#ifndef WXD_PICKERS_H
#define WXD_PICKERS_H

#include "../wxdragon.h"

// --- ColourPickerCtrl ---
WXD_EXPORTED wxd_ColourPickerCtrl_t*
wxd_ColourPickerCtrl_Create(wxd_Window_t* parent, wxd_Id id, wxd_Colour_t initial_colour,
                            wxd_Point pos, wxd_Size size, wxd_Style_t style);

WXD_EXPORTED wxd_Colour_t
wxd_ColourPickerCtrl_GetColour(wxd_ColourPickerCtrl_t* self);

WXD_EXPORTED void
wxd_ColourPickerCtrl_SetColour(wxd_ColourPickerCtrl_t* self, wxd_Colour_t colour);

// --- DatePickerCtrl ---
WXD_EXPORTED wxd_DatePickerCtrl_t*
wxd_DatePickerCtrl_Create(wxd_Window_t* parent, wxd_Id id, const wxd_DateTime_t* dt, wxd_Point pos,
                          wxd_Size size, wxd_Style_t style);

WXD_EXPORTED wxd_DateTime_t*
wxd_DatePickerCtrl_GetValue(wxd_DatePickerCtrl_t* self);

WXD_EXPORTED void
wxd_DatePickerCtrl_SetValue(wxd_DatePickerCtrl_t* self, const wxd_DateTime_t* dt);

WXD_EXPORTED bool
wxd_DatePickerCtrl_GetRange(wxd_DatePickerCtrl_t* self, wxd_DateTime_t** dt1, wxd_DateTime_t** dt2);

WXD_EXPORTED void
wxd_DatePickerCtrl_SetRange(wxd_DatePickerCtrl_t* self, const wxd_DateTime_t* dt1,
                            const wxd_DateTime_t* dt2);

// --- TimePickerCtrl ---
WXD_EXPORTED wxd_TimePickerCtrl_t*
wxd_TimePickerCtrl_Create(wxd_Window_t* parent, wxd_Id id, const wxd_DateTime_t* dt, wxd_Point pos,
                          wxd_Size size, wxd_Style_t style);

WXD_EXPORTED wxd_DateTime_t*
wxd_TimePickerCtrl_GetValue(wxd_TimePickerCtrl_t* self);

WXD_EXPORTED void
wxd_TimePickerCtrl_SetValue(wxd_TimePickerCtrl_t* self, const wxd_DateTime_t* dt);

// --- FilePickerCtrl ---
WXD_EXPORTED wxd_FilePickerCtrl_t*
wxd_FilePickerCtrl_Create(wxd_Window_t* parent, wxd_Id id, const char* message,
                          const char* wildcard, const char* path, wxd_Point pos, wxd_Size size,
                          wxd_Style_t style);

WXD_EXPORTED const char*
wxd_FilePickerCtrl_GetPath(wxd_FilePickerCtrl_t* self);

WXD_EXPORTED void
wxd_FilePickerCtrl_SetPath(wxd_FilePickerCtrl_t* self, const char* path);

// --- DirPickerCtrl ---
WXD_EXPORTED wxd_DirPickerCtrl_t*
wxd_DirPickerCtrl_Create(
    wxd_Window_t* parent, wxd_Id id,
    const char* message, // wxWidgets calls this 'message' but it's the label for the dialog
    const char* path,    // Initial path
    wxd_Point pos, wxd_Size size, wxd_Style_t style);

/**
 * @brief Get the current path from the DirPickerCtrl
 * Returns the required UTF-8 byte length (excluding the null terminator), if any error, -1 returned.
 * If buffer is not null and buffer_len > 0, copies up to buffer_len - 1 bytes and null-terminates.
 * If the current path is longer than buffer_len - 1, it will be truncated.
 */
WXD_EXPORTED int
wxd_DirPickerCtrl_GetPath(const wxd_DirPickerCtrl_t* self, char* buffer, size_t buffer_len);

WXD_EXPORTED void
wxd_DirPickerCtrl_SetPath(wxd_DirPickerCtrl_t* self, const char* path);

// --- FontPickerCtrl ---
WXD_EXPORTED wxd_FontPickerCtrl_t*
wxd_FontPickerCtrl_Create(wxd_Window_t* parent, wxd_Id id,
                          const wxd_Font_t* initial_font, // Can be NULL for default
                          wxd_Point pos, wxd_Size size, wxd_Style_t style);

WXD_EXPORTED wxd_Font_t*
wxd_FontPickerCtrl_GetSelectedFont(wxd_FontPickerCtrl_t* self);

WXD_EXPORTED void
wxd_FontPickerCtrl_SetSelectedFont(wxd_FontPickerCtrl_t* self, const wxd_Font_t* font);

#endif // WXD_PICKERS_H