#ifndef WXD_RICHTEXTCTRL_H
#define WXD_RICHTEXTCTRL_H

#include "../wxd_types.h"

// --- RichTextCtrl Functions ---

// Creation and basic operations
WXD_EXPORTED wxd_RichTextCtrl_t*
wxd_RichTextCtrl_Create(wxd_Window_t* parent, wxd_Id id, const char* value, wxd_Point pos,
                        wxd_Size size, wxd_Style_t style);

// Text content operations
WXD_EXPORTED void
wxd_RichTextCtrl_SetValue(wxd_RichTextCtrl_t* self, const char* value);
WXD_EXPORTED int
wxd_RichTextCtrl_GetValue(wxd_RichTextCtrl_t* self, char* buffer, int buffer_len);
WXD_EXPORTED void
wxd_RichTextCtrl_WriteText(wxd_RichTextCtrl_t* self, const char* text);
WXD_EXPORTED void
wxd_RichTextCtrl_AppendText(wxd_RichTextCtrl_t* self, const char* text);
WXD_EXPORTED void
wxd_RichTextCtrl_Clear(wxd_RichTextCtrl_t* self);
WXD_EXPORTED int
wxd_RichTextCtrl_GetLength(wxd_RichTextCtrl_t* self);

// Text range operations
WXD_EXPORTED int
wxd_RichTextCtrl_GetRange(wxd_RichTextCtrl_t* self, wxd_Long_t from, wxd_Long_t to, char* buffer,
                          int buffer_len);
WXD_EXPORTED void
wxd_RichTextCtrl_SetSelection(wxd_RichTextCtrl_t* self, wxd_Long_t from, wxd_Long_t to);
WXD_EXPORTED void
wxd_RichTextCtrl_GetSelection(wxd_RichTextCtrl_t* self, wxd_Long_t* from, wxd_Long_t* to);
WXD_EXPORTED int
wxd_RichTextCtrl_GetSelectedText(wxd_RichTextCtrl_t* self, char* buffer, int buffer_len);

// Editing operations
WXD_EXPORTED void
wxd_RichTextCtrl_Cut(wxd_RichTextCtrl_t* self);
WXD_EXPORTED void
wxd_RichTextCtrl_Copy(wxd_RichTextCtrl_t* self);
WXD_EXPORTED void
wxd_RichTextCtrl_Paste(wxd_RichTextCtrl_t* self);
WXD_EXPORTED void
wxd_RichTextCtrl_Undo(wxd_RichTextCtrl_t* self);
WXD_EXPORTED void
wxd_RichTextCtrl_Redo(wxd_RichTextCtrl_t* self);
WXD_EXPORTED bool
wxd_RichTextCtrl_CanUndo(wxd_RichTextCtrl_t* self);
WXD_EXPORTED bool
wxd_RichTextCtrl_CanRedo(wxd_RichTextCtrl_t* self);

// State operations
WXD_EXPORTED void
wxd_RichTextCtrl_SetEditable(wxd_RichTextCtrl_t* self, bool editable);
WXD_EXPORTED bool
wxd_RichTextCtrl_IsEditable(wxd_RichTextCtrl_t* self);
WXD_EXPORTED bool
wxd_RichTextCtrl_IsModified(wxd_RichTextCtrl_t* self);
WXD_EXPORTED void
wxd_RichTextCtrl_MarkDirty(wxd_RichTextCtrl_t* self);
WXD_EXPORTED void
wxd_RichTextCtrl_DiscardEdits(wxd_RichTextCtrl_t* self);

// Position operations
WXD_EXPORTED wxd_Long_t
wxd_RichTextCtrl_GetInsertionPoint(wxd_RichTextCtrl_t* self);
WXD_EXPORTED void
wxd_RichTextCtrl_SetInsertionPoint(wxd_RichTextCtrl_t* self, wxd_Long_t pos);
WXD_EXPORTED void
wxd_RichTextCtrl_SetInsertionPointEnd(wxd_RichTextCtrl_t* self);
WXD_EXPORTED wxd_Long_t
wxd_RichTextCtrl_GetLastPosition(wxd_RichTextCtrl_t* self);

// File operations
WXD_EXPORTED bool
wxd_RichTextCtrl_LoadFile(wxd_RichTextCtrl_t* self, const char* filename, int type);
WXD_EXPORTED bool
wxd_RichTextCtrl_SaveFile(wxd_RichTextCtrl_t* self, const char* filename, int type);

// Style operations
WXD_EXPORTED bool
wxd_RichTextCtrl_SetStyleRange(wxd_RichTextCtrl_t* self, wxd_Long_t start, wxd_Long_t end,
                               bool bold, bool italic, bool underline);
WXD_EXPORTED bool
wxd_RichTextCtrl_ApplyBoldToSelection(wxd_RichTextCtrl_t* self);
WXD_EXPORTED bool
wxd_RichTextCtrl_ApplyItalicToSelection(wxd_RichTextCtrl_t* self);
WXD_EXPORTED bool
wxd_RichTextCtrl_ApplyUnderlineToSelection(wxd_RichTextCtrl_t* self);
WXD_EXPORTED bool
wxd_RichTextCtrl_IsSelectionBold(wxd_RichTextCtrl_t* self);
WXD_EXPORTED bool
wxd_RichTextCtrl_IsSelectionItalics(wxd_RichTextCtrl_t* self);
WXD_EXPORTED bool
wxd_RichTextCtrl_IsSelectionUnderlined(wxd_RichTextCtrl_t* self);

// Font operations
WXD_EXPORTED bool
wxd_RichTextCtrl_SetFontSize(wxd_RichTextCtrl_t* self, wxd_Long_t start, wxd_Long_t end, int size);
WXD_EXPORTED bool
wxd_RichTextCtrl_SetFontSizeSelection(wxd_RichTextCtrl_t* self, int size);

// Color operations
WXD_EXPORTED bool
wxd_RichTextCtrl_SetTextColor(wxd_RichTextCtrl_t* self, wxd_Long_t start, wxd_Long_t end,
                              wxd_Colour_t color);
WXD_EXPORTED bool
wxd_RichTextCtrl_SetTextColorSelection(wxd_RichTextCtrl_t* self, wxd_Colour_t color);
WXD_EXPORTED bool
wxd_RichTextCtrl_SetBackgroundColor(wxd_RichTextCtrl_t* self, wxd_Long_t start, wxd_Long_t end,
                                    wxd_Colour_t color);
WXD_EXPORTED bool
wxd_RichTextCtrl_SetBackgroundColorSelection(wxd_RichTextCtrl_t* self, wxd_Colour_t color);

// Scroll operations
WXD_EXPORTED void
wxd_RichTextCtrl_ShowPosition(wxd_RichTextCtrl_t* self, wxd_Long_t pos);
WXD_EXPORTED bool
wxd_RichTextCtrl_ScrollIntoView(wxd_RichTextCtrl_t* self, wxd_Long_t position, int keyCode);
WXD_EXPORTED bool
wxd_RichTextCtrl_IsPositionVisible(wxd_RichTextCtrl_t* self, wxd_Long_t pos);

#endif // WXD_RICHTEXTCTRL_H