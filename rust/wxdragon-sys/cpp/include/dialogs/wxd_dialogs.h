#ifndef WXD_DIALOGS_H
#define WXD_DIALOGS_H

#include "../wxd_types.h"

// --- Dialog (Base) ---
WXD_EXPORTED wxd_Dialog_t* wxd_Dialog_Create(wxd_Window_t* parent, const char* title, wxd_Style_t style, int x, int y, int width, int height);
WXD_EXPORTED int wxd_Dialog_ShowModal(wxd_Dialog_t* self);
WXD_EXPORTED void wxd_Dialog_EndModal(wxd_Dialog_t* self, int retCode);

// --- MessageDialog ---
WXD_EXPORTED wxd_MessageDialog_t* wxd_MessageDialog_Create(wxd_Window_t* parent, const char* message, const char* caption, wxd_Style_t style);

// --- FileDialog ---
WXD_EXPORTED wxd_ArrayString_t* wxd_ArrayString_Create(); // Helper for paths/filenames
WXD_EXPORTED void wxd_ArrayString_Free(wxd_ArrayString_t* self);
WXD_EXPORTED int wxd_ArrayString_GetCount(wxd_ArrayString_t* self);
WXD_EXPORTED int wxd_ArrayString_GetString(wxd_ArrayString_t* self, int index, char* buffer, int bufLen);
WXD_EXPORTED bool wxd_ArrayString_Add(wxd_ArrayString_t* self, const char* str);
WXD_EXPORTED void wxd_ArrayString_Clear(wxd_ArrayString_t* self);

WXD_EXPORTED wxd_FileDialog_t* wxd_FileDialog_Create(wxd_Window_t* parent, const char* message, const char* defaultDir, const char* defaultFile, const char* wildcard, wxd_Style_t style, int x, int y, int width, int height);
WXD_EXPORTED int wxd_FileDialog_GetPath(wxd_FileDialog_t* self, char* buffer, int bufLen);
WXD_EXPORTED void wxd_FileDialog_GetPaths(wxd_FileDialog_t* self, wxd_ArrayString_t* paths);
WXD_EXPORTED int wxd_FileDialog_GetFilename(wxd_FileDialog_t* self, char* buffer, int bufLen);
WXD_EXPORTED void wxd_FileDialog_GetFilenames(wxd_FileDialog_t* self, wxd_ArrayString_t* filenames);
WXD_EXPORTED int wxd_FileDialog_GetDirectory(wxd_FileDialog_t* self, char* buffer, int bufLen);
WXD_EXPORTED int wxd_FileDialog_GetFilterIndex(wxd_FileDialog_t* self);
WXD_EXPORTED int wxd_FileDialog_GetMessage(wxd_FileDialog_t* self, char* buffer, int bufLen);
WXD_EXPORTED int wxd_FileDialog_GetWildcard(wxd_FileDialog_t* self, char* buffer, int bufLen);
WXD_EXPORTED int wxd_FileDialog_GetCurrentlySelectedFilterIndex(wxd_FileDialog_t* self);
WXD_EXPORTED void wxd_FileDialog_SetMessage(wxd_FileDialog_t* self, const char* message);
WXD_EXPORTED void wxd_FileDialog_SetPath(wxd_FileDialog_t* self, const char* path);
WXD_EXPORTED void wxd_FileDialog_SetDirectory(wxd_FileDialog_t* self, const char* directory);
WXD_EXPORTED void wxd_FileDialog_SetFilename(wxd_FileDialog_t* self, const char* name);
WXD_EXPORTED void wxd_FileDialog_SetWildcard(wxd_FileDialog_t* self, const char* wildCard);
WXD_EXPORTED void wxd_FileDialog_SetFilterIndex(wxd_FileDialog_t* self, int filterIndex);

// --- ColourDialog ---
WXD_EXPORTED wxd_ColourData_t* wxd_ColourData_Create(void);
WXD_EXPORTED void wxd_ColourData_SetColour(wxd_ColourData_t* self, wxd_Colour_t colour);
WXD_EXPORTED wxd_Colour_t wxd_ColourData_GetColour(wxd_ColourData_t* self);
WXD_EXPORTED void wxd_ColourData_Destroy(wxd_ColourData_t* self);
WXD_EXPORTED wxd_ColourDialog_t* wxd_ColourDialog_Create(wxd_Window_t* parent, const char* title, wxd_ColourData_t* data);
WXD_EXPORTED wxd_ColourData_t* wxd_ColourDialog_GetColourData(wxd_ColourDialog_t* self);

// --- FontDialog ---
WXD_EXPORTED wxd_FontData_t* wxd_FontData_Create(void);
WXD_EXPORTED void wxd_FontData_Destroy(wxd_FontData_t* self);
WXD_EXPORTED void wxd_FontData_EnableEffects(wxd_FontData_t* self, bool enable);
WXD_EXPORTED bool wxd_FontData_GetEnableEffects(wxd_FontData_t* self);
WXD_EXPORTED void wxd_FontData_SetInitialFont(wxd_FontData_t* self, const wxd_Font_t* font);

WXD_EXPORTED wxd_Font_t* wxd_Font_Create(void);
WXD_EXPORTED wxd_Font_t* wxd_Font_CreateEx(int point_size, int family, int style, int weight, bool underlined, const char* face_name);
WXD_EXPORTED bool wxd_Font_AddPrivateFont(const char* font_file_path);
WXD_EXPORTED void wxd_Font_Destroy(wxd_Font_t* font);
WXD_EXPORTED int wxd_Font_GetPointSize(wxd_Font_t* self);
WXD_EXPORTED void wxd_Font_SetPointSize(wxd_Font_t* self, int point_size);
WXD_EXPORTED int wxd_Font_GetFamily(wxd_Font_t* self);
WXD_EXPORTED void wxd_Font_SetFamily(wxd_Font_t* self, int family);
WXD_EXPORTED int wxd_Font_GetStyle(wxd_Font_t* self);
WXD_EXPORTED void wxd_Font_SetStyle(wxd_Font_t* self, int style);
WXD_EXPORTED int wxd_Font_GetWeight(wxd_Font_t* self);
WXD_EXPORTED void wxd_Font_SetWeight(wxd_Font_t* self, int weight);
WXD_EXPORTED bool wxd_Font_GetUnderlined(wxd_Font_t* self);
WXD_EXPORTED void wxd_Font_SetUnderlined(wxd_Font_t* self, bool underlined);
WXD_EXPORTED int wxd_Font_GetFaceName(wxd_Font_t* self, char* buffer, int buffer_len);
WXD_EXPORTED bool wxd_Font_IsOk(wxd_Font_t* self);

WXD_EXPORTED wxd_FontDialog_t* wxd_FontDialog_Create(wxd_Window_t* parent, const char* title, wxd_FontData_t* data);
WXD_EXPORTED wxd_FontData_t* wxd_FontDialog_GetFontData(wxd_FontDialog_t* self);
WXD_EXPORTED wxd_Font_t* wxd_FontDialog_GetFont(wxd_FontDialog_t* self);

// --- TextEntryDialog ---
WXD_EXPORTED wxd_TextEntryDialog_t* wxd_TextEntryDialog_Create(wxd_Window_t* parent, const char* message, const char* caption, const char* defaultValue, wxd_Style_t style, int x, int y, int width, int height);
WXD_EXPORTED int wxd_TextEntryDialog_GetValue(wxd_TextEntryDialog_t* self, char* buffer, int bufLen);

// --- ProgressDialog ---
WXD_EXPORTED wxd_ProgressDialog_t* wxd_ProgressDialog_Create(wxd_Window_t* parent, const char* title, const char* message, int maximum, wxd_Style_t style);
WXD_EXPORTED bool wxd_ProgressDialog_Update(wxd_ProgressDialog_t* self, int value, const char* newmsg, bool* skip);
WXD_EXPORTED bool wxd_ProgressDialog_Pulse(wxd_ProgressDialog_t* self, const char* newmsg, bool* skip);
WXD_EXPORTED void wxd_ProgressDialog_Resume(wxd_ProgressDialog_t* self);
WXD_EXPORTED int wxd_ProgressDialog_GetValue(wxd_ProgressDialog_t* self);
WXD_EXPORTED int wxd_ProgressDialog_GetRange(wxd_ProgressDialog_t* self);
WXD_EXPORTED bool wxd_ProgressDialog_WasCancelled(wxd_ProgressDialog_t* self);
WXD_EXPORTED bool wxd_ProgressDialog_WasSkipped(wxd_ProgressDialog_t* self);

// --- DateTime Helper Functions (moved here as they are used by DatePickerCtrl in this file) ---
WXD_EXPORTED wxd_DateTime_t wxd_DateTime_Default();
WXD_EXPORTED wxd_DateTime_t wxd_DateTime_Now();
WXD_EXPORTED wxd_DateTime_t wxd_DateTime_FromComponents(int year, unsigned short month, short day, short hour, short minute, short second);
WXD_EXPORTED bool wxd_DateTime_IsValid(const wxd_DateTime_t* dt);

// --- SingleChoiceDialog ---
WXD_EXPORTED wxd_SingleChoiceDialog_t* wxd_SingleChoiceDialog_Create(wxd_Window_t* parent, const char* message, 
                                                                     const char* caption, wxd_ArrayString_t* choices,
                                                                     wxd_Style_t style, int x, int y, int width, int height);
WXD_EXPORTED int wxd_SingleChoiceDialog_GetSelection(wxd_SingleChoiceDialog_t* self);
WXD_EXPORTED void wxd_SingleChoiceDialog_SetSelection(wxd_SingleChoiceDialog_t* self, int selection);
WXD_EXPORTED int wxd_SingleChoiceDialog_GetStringSelection(wxd_SingleChoiceDialog_t* self, char* buffer, int bufLen);

// --- MultiChoiceDialog ---
WXD_EXPORTED wxd_MultiChoiceDialog_t* wxd_MultiChoiceDialog_Create(wxd_Window_t* parent, const char* message,
                                                                   const char* caption, wxd_ArrayString_t* choices,
                                                                   wxd_Style_t style, int x, int y, int width, int height);
WXD_EXPORTED void wxd_MultiChoiceDialog_GetSelections(wxd_MultiChoiceDialog_t* self, int* selections, int* count);
WXD_EXPORTED void wxd_MultiChoiceDialog_SetSelections(wxd_MultiChoiceDialog_t* self, const int* selections, int count);
WXD_EXPORTED void wxd_MultiChoiceDialog_GetStringSelections(wxd_MultiChoiceDialog_t* self, wxd_ArrayString_t* selections);

// --- DirDialog ---
WXD_EXPORTED wxd_DirDialog_t* wxd_DirDialog_Create(wxd_Window_t* parent, const char* message, 
                                                 const char* defaultPath, wxd_Style_t style,
                                                 int x, int y, int width, int height);
WXD_EXPORTED int wxd_DirDialog_GetPath(wxd_DirDialog_t* self, char* buffer, int bufLen);
WXD_EXPORTED void wxd_DirDialog_SetPath(wxd_DirDialog_t* self, const char* path);
WXD_EXPORTED int wxd_DirDialog_GetMessage(wxd_DirDialog_t* self, char* buffer, int bufLen);
WXD_EXPORTED void wxd_DirDialog_SetMessage(wxd_DirDialog_t* self, const char* message);

#endif // WXD_DIALOGS_H 