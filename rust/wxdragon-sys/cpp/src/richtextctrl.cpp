#include "wx/wxprec.h"

#ifndef WX_PRECOMP
#include "wx/wx.h"
#endif

#include "wx/richtext/richtextctrl.h"
#include "wxdragon.h"
#include "wxd_utils.h"

extern "C" {

// Create a new wxRichTextCtrl
WXD_EXPORTED wxd_RichTextCtrl_t*
wxd_RichTextCtrl_Create(wxd_Window_t* parent, wxd_Id id, const char* value, wxd_Point pos,
                        wxd_Size size, wxd_Style_t style)
{
    wxWindow* parentWin = (wxWindow*)parent;
    wxRichTextCtrl* ctrl = new wxRichTextCtrl(parentWin, id, wxString::FromUTF8(value ? value : ""),
                                              wxd_cpp_utils::to_wx(pos), wxd_cpp_utils::to_wx(size),
                                              style);
    return (wxd_RichTextCtrl_t*)ctrl;
}

// Text content operations

// Set the value of the wxRichTextCtrl
WXD_EXPORTED void
wxd_RichTextCtrl_SetValue(wxd_RichTextCtrl_t* self, const char* value)
{
    wxRichTextCtrl* ctrl = (wxRichTextCtrl*)self;
    if (ctrl) {
        ctrl->SetValue(wxString::FromUTF8(value ? value : ""));
    }
}

// Get the value of the wxRichTextCtrl
WXD_EXPORTED int
wxd_RichTextCtrl_GetValue(wxd_RichTextCtrl_t* self, char* buffer, int buffer_len)
{
    if (!self || !buffer || buffer_len <= 0)
        return -1;
    wxRichTextCtrl* ctrl = (wxRichTextCtrl*)self;
    wxString value = ctrl->GetValue();
    return wxd_cpp_utils::copy_wxstring_to_buffer(value, buffer, (size_t)buffer_len);
}

// Write text to the wxRichTextCtrl
WXD_EXPORTED void
wxd_RichTextCtrl_WriteText(wxd_RichTextCtrl_t* self, const char* text)
{
    wxRichTextCtrl* ctrl = (wxRichTextCtrl*)self;
    if (ctrl && text) {
        ctrl->WriteText(wxString::FromUTF8(text));
    }
}

// Append text to the wxRichTextCtrl
WXD_EXPORTED void
wxd_RichTextCtrl_AppendText(wxd_RichTextCtrl_t* self, const char* text)
{
    wxRichTextCtrl* ctrl = (wxRichTextCtrl*)self;
    if (ctrl && text) {
        ctrl->AppendText(wxString::FromUTF8(text));
    }
}

// Clear the wxRichTextCtrl contents
WXD_EXPORTED void
wxd_RichTextCtrl_Clear(wxd_RichTextCtrl_t* self)
{
    wxRichTextCtrl* ctrl = (wxRichTextCtrl*)self;
    if (ctrl) {
        ctrl->Clear();
    }
}

// Get the length of text in the control
WXD_EXPORTED int
wxd_RichTextCtrl_GetLength(wxd_RichTextCtrl_t* self)
{
    wxRichTextCtrl* ctrl = (wxRichTextCtrl*)self;
    if (!ctrl)
        return 0;
    return ctrl->GetLastPosition();
}

// Text range operations

// Get text in a specific range
WXD_EXPORTED int
wxd_RichTextCtrl_GetRange(wxd_RichTextCtrl_t* self, wxd_Long_t from, wxd_Long_t to, char* buffer,
                          int buffer_len)
{
    if (!self || !buffer || buffer_len <= 0)
        return -1;
    wxRichTextCtrl* ctrl = (wxRichTextCtrl*)self;
    wxString range_text = ctrl->GetRange(from, to);
    return wxd_cpp_utils::copy_wxstring_to_buffer(range_text, buffer, (size_t)buffer_len);
}

// Set selection range
WXD_EXPORTED void
wxd_RichTextCtrl_SetSelection(wxd_RichTextCtrl_t* self, wxd_Long_t from, wxd_Long_t to)
{
    wxRichTextCtrl* ctrl = (wxRichTextCtrl*)self;
    if (ctrl) {
        ctrl->SetSelection(from, to);
    }
}

// Get selection range
WXD_EXPORTED void
wxd_RichTextCtrl_GetSelection(wxd_RichTextCtrl_t* self, wxd_Long_t* from, wxd_Long_t* to)
{
    wxRichTextCtrl* ctrl = (wxRichTextCtrl*)self;
    if (ctrl && from && to) {
        // wxWidgets uses long internally, but we need to bridge the types
        long wx_from, wx_to;
        ctrl->GetSelection(&wx_from, &wx_to);
        *from = static_cast<wxd_Long_t>(wx_from);
        *to = static_cast<wxd_Long_t>(wx_to);
    }
}

// Get selected text
WXD_EXPORTED int
wxd_RichTextCtrl_GetSelectedText(wxd_RichTextCtrl_t* self, char* buffer, int buffer_len)
{
    if (!self || !buffer || buffer_len <= 0)
        return -1;
    wxRichTextCtrl* ctrl = (wxRichTextCtrl*)self;
    wxString selected_text = ctrl->GetStringSelection();
    return wxd_cpp_utils::copy_wxstring_to_buffer(selected_text, buffer, (size_t)buffer_len);
}

// Editing operations

// Cut text to clipboard
WXD_EXPORTED void
wxd_RichTextCtrl_Cut(wxd_RichTextCtrl_t* self)
{
    wxRichTextCtrl* ctrl = (wxRichTextCtrl*)self;
    if (ctrl) {
        ctrl->Cut();
    }
}

// Copy text to clipboard
WXD_EXPORTED void
wxd_RichTextCtrl_Copy(wxd_RichTextCtrl_t* self)
{
    wxRichTextCtrl* ctrl = (wxRichTextCtrl*)self;
    if (ctrl) {
        ctrl->Copy();
    }
}

// Paste text from clipboard
WXD_EXPORTED void
wxd_RichTextCtrl_Paste(wxd_RichTextCtrl_t* self)
{
    wxRichTextCtrl* ctrl = (wxRichTextCtrl*)self;
    if (ctrl) {
        ctrl->Paste();
    }
}

// Undo last action
WXD_EXPORTED void
wxd_RichTextCtrl_Undo(wxd_RichTextCtrl_t* self)
{
    wxRichTextCtrl* ctrl = (wxRichTextCtrl*)self;
    if (ctrl) {
        ctrl->Undo();
    }
}

// Redo last undone action
WXD_EXPORTED void
wxd_RichTextCtrl_Redo(wxd_RichTextCtrl_t* self)
{
    wxRichTextCtrl* ctrl = (wxRichTextCtrl*)self;
    if (ctrl) {
        ctrl->Redo();
    }
}

// Check if undo is available
WXD_EXPORTED bool
wxd_RichTextCtrl_CanUndo(wxd_RichTextCtrl_t* self)
{
    wxRichTextCtrl* ctrl = (wxRichTextCtrl*)self;
    if (!ctrl)
        return false;
    return ctrl->CanUndo();
}

// Check if redo is available
WXD_EXPORTED bool
wxd_RichTextCtrl_CanRedo(wxd_RichTextCtrl_t* self)
{
    wxRichTextCtrl* ctrl = (wxRichTextCtrl*)self;
    if (!ctrl)
        return false;
    return ctrl->CanRedo();
}

// State operations

// Set editable state
WXD_EXPORTED void
wxd_RichTextCtrl_SetEditable(wxd_RichTextCtrl_t* self, bool editable)
{
    wxRichTextCtrl* ctrl = (wxRichTextCtrl*)self;
    if (ctrl) {
        ctrl->SetEditable(editable);
    }
}

// Check if control is editable
WXD_EXPORTED bool
wxd_RichTextCtrl_IsEditable(wxd_RichTextCtrl_t* self)
{
    wxRichTextCtrl* ctrl = (wxRichTextCtrl*)self;
    if (!ctrl)
        return false;
    return ctrl->IsEditable();
}

// Check if control has been modified
WXD_EXPORTED bool
wxd_RichTextCtrl_IsModified(wxd_RichTextCtrl_t* self)
{
    wxRichTextCtrl* ctrl = (wxRichTextCtrl*)self;
    if (!ctrl)
        return false;
    return ctrl->IsModified();
}

// Mark control as dirty (modified)
WXD_EXPORTED void
wxd_RichTextCtrl_MarkDirty(wxd_RichTextCtrl_t* self)
{
    wxRichTextCtrl* ctrl = (wxRichTextCtrl*)self;
    if (ctrl) {
        ctrl->MarkDirty();
    }
}

// Discard edits (mark as unmodified)
WXD_EXPORTED void
wxd_RichTextCtrl_DiscardEdits(wxd_RichTextCtrl_t* self)
{
    wxRichTextCtrl* ctrl = (wxRichTextCtrl*)self;
    if (ctrl) {
        ctrl->DiscardEdits();
    }
}

// Position operations

// Get insertion point
WXD_EXPORTED wxd_Long_t
wxd_RichTextCtrl_GetInsertionPoint(wxd_RichTextCtrl_t* self)
{
    wxRichTextCtrl* ctrl = (wxRichTextCtrl*)self;
    if (!ctrl)
        return 0;
    return static_cast<wxd_Long_t>(ctrl->GetInsertionPoint());
}

// Set insertion point
WXD_EXPORTED void
wxd_RichTextCtrl_SetInsertionPoint(wxd_RichTextCtrl_t* self, wxd_Long_t pos)
{
    wxRichTextCtrl* ctrl = (wxRichTextCtrl*)self;
    if (ctrl) {
        ctrl->SetInsertionPoint(static_cast<long>(pos));
    }
}

// Set insertion point to end
WXD_EXPORTED void
wxd_RichTextCtrl_SetInsertionPointEnd(wxd_RichTextCtrl_t* self)
{
    wxRichTextCtrl* ctrl = (wxRichTextCtrl*)self;
    if (ctrl) {
        ctrl->SetInsertionPointEnd();
    }
}

// Get last position
WXD_EXPORTED wxd_Long_t
wxd_RichTextCtrl_GetLastPosition(wxd_RichTextCtrl_t* self)
{
    wxRichTextCtrl* ctrl = (wxRichTextCtrl*)self;
    if (!ctrl)
        return 0;
    return static_cast<wxd_Long_t>(ctrl->GetLastPosition());
}

// File operations

// Load file
WXD_EXPORTED bool
wxd_RichTextCtrl_LoadFile(wxd_RichTextCtrl_t* self, const char* filename, int type)
{
    wxRichTextCtrl* ctrl = (wxRichTextCtrl*)self;
    if (!ctrl || !filename)
        return false;
    return ctrl->LoadFile(wxString::FromUTF8(filename), type);
}

// Save file
WXD_EXPORTED bool
wxd_RichTextCtrl_SaveFile(wxd_RichTextCtrl_t* self, const char* filename, int type)
{
    wxRichTextCtrl* ctrl = (wxRichTextCtrl*)self;
    if (!ctrl || !filename)
        return false;
    return ctrl->SaveFile(wxString::FromUTF8(filename), type);
}

// Style operations

// Set style for a range of text
WXD_EXPORTED bool
wxd_RichTextCtrl_SetStyleRange(wxd_RichTextCtrl_t* self, wxd_Long_t start, wxd_Long_t end,
                               bool bold, bool italic, bool underline)
{
    wxRichTextCtrl* ctrl = (wxRichTextCtrl*)self;
    if (!ctrl)
        return false;

    wxRichTextAttr attr;
    if (bold)
        attr.SetFontWeight(wxFONTWEIGHT_BOLD);
    if (italic)
        attr.SetFontStyle(wxFONTSTYLE_ITALIC);
    if (underline)
        attr.SetFontUnderlined(true);

    return ctrl->SetStyle(wxRichTextRange(static_cast<long>(start), static_cast<long>(end)), attr);
}

// Apply bold to selection
WXD_EXPORTED bool
wxd_RichTextCtrl_ApplyBoldToSelection(wxd_RichTextCtrl_t* self)
{
    wxRichTextCtrl* ctrl = (wxRichTextCtrl*)self;
    if (!ctrl)
        return false;
    return ctrl->ApplyBoldToSelection();
}

// Apply italic to selection
WXD_EXPORTED bool
wxd_RichTextCtrl_ApplyItalicToSelection(wxd_RichTextCtrl_t* self)
{
    wxRichTextCtrl* ctrl = (wxRichTextCtrl*)self;
    if (!ctrl)
        return false;
    return ctrl->ApplyItalicToSelection();
}

// Apply underline to selection
WXD_EXPORTED bool
wxd_RichTextCtrl_ApplyUnderlineToSelection(wxd_RichTextCtrl_t* self)
{
    wxRichTextCtrl* ctrl = (wxRichTextCtrl*)self;
    if (!ctrl)
        return false;
    return ctrl->ApplyUnderlineToSelection();
}

// Check if selection is bold
WXD_EXPORTED bool
wxd_RichTextCtrl_IsSelectionBold(wxd_RichTextCtrl_t* self)
{
    wxRichTextCtrl* ctrl = (wxRichTextCtrl*)self;
    if (!ctrl)
        return false;
    return ctrl->IsSelectionBold();
}

// Check if selection is italic
WXD_EXPORTED bool
wxd_RichTextCtrl_IsSelectionItalics(wxd_RichTextCtrl_t* self)
{
    wxRichTextCtrl* ctrl = (wxRichTextCtrl*)self;
    if (!ctrl)
        return false;
    return ctrl->IsSelectionItalics();
}

// Check if selection is underlined
WXD_EXPORTED bool
wxd_RichTextCtrl_IsSelectionUnderlined(wxd_RichTextCtrl_t* self)
{
    wxRichTextCtrl* ctrl = (wxRichTextCtrl*)self;
    if (!ctrl)
        return false;
    return ctrl->IsSelectionUnderlined();
}

// Font operations

// Set font size for range
WXD_EXPORTED bool
wxd_RichTextCtrl_SetFontSize(wxd_RichTextCtrl_t* self, wxd_Long_t start, wxd_Long_t end, int size)
{
    wxRichTextCtrl* ctrl = (wxRichTextCtrl*)self;
    if (!ctrl)
        return false;

    wxRichTextAttr attr;
    attr.SetFontSize(size);

    return ctrl->SetStyle(wxRichTextRange(static_cast<long>(start), static_cast<long>(end)), attr);
}

// Set font size for selection
WXD_EXPORTED bool
wxd_RichTextCtrl_SetFontSizeSelection(wxd_RichTextCtrl_t* self, int size)
{
    wxRichTextCtrl* ctrl = (wxRichTextCtrl*)self;
    if (!ctrl)
        return false;

    wxRichTextAttr attr;
    attr.SetFontSize(size);

    long from, to;
    ctrl->GetSelection(&from, &to);
    return ctrl->SetStyle(wxRichTextRange(from, to), attr);
}

// Color operations

// Set text color for range
WXD_EXPORTED bool
wxd_RichTextCtrl_SetTextColor(wxd_RichTextCtrl_t* self, wxd_Long_t start, wxd_Long_t end,
                              wxd_Colour_t color)
{
    wxRichTextCtrl* ctrl = (wxRichTextCtrl*)self;
    if (!ctrl)
        return false;

    wxRichTextAttr attr;
    wxColour wx_color(color.r, color.g, color.b, color.a);
    attr.SetTextColour(wx_color);

    return ctrl->SetStyle(wxRichTextRange(static_cast<long>(start), static_cast<long>(end)), attr);
}

// Set text color for selection
WXD_EXPORTED bool
wxd_RichTextCtrl_SetTextColorSelection(wxd_RichTextCtrl_t* self, wxd_Colour_t color)
{
    wxRichTextCtrl* ctrl = (wxRichTextCtrl*)self;
    if (!ctrl)
        return false;

    wxRichTextAttr attr;
    wxColour wx_color(color.r, color.g, color.b, color.a);
    attr.SetTextColour(wx_color);

    long from, to;
    ctrl->GetSelection(&from, &to);
    return ctrl->SetStyle(wxRichTextRange(from, to), attr);
}

// Set background color for range
WXD_EXPORTED bool
wxd_RichTextCtrl_SetBackgroundColor(wxd_RichTextCtrl_t* self, wxd_Long_t start, wxd_Long_t end,
                                    wxd_Colour_t color)
{
    wxRichTextCtrl* ctrl = (wxRichTextCtrl*)self;
    if (!ctrl)
        return false;

    wxRichTextAttr attr;
    wxColour wx_color(color.r, color.g, color.b, color.a);
    attr.SetBackgroundColour(wx_color);

    return ctrl->SetStyle(wxRichTextRange(static_cast<long>(start), static_cast<long>(end)), attr);
}

// Set background color for selection
WXD_EXPORTED bool
wxd_RichTextCtrl_SetBackgroundColorSelection(wxd_RichTextCtrl_t* self, wxd_Colour_t color)
{
    wxRichTextCtrl* ctrl = (wxRichTextCtrl*)self;
    if (!ctrl)
        return false;

    wxRichTextAttr attr;
    wxColour wx_color(color.r, color.g, color.b, color.a);
    attr.SetBackgroundColour(wx_color);

    long from, to;
    ctrl->GetSelection(&from, &to);
    return ctrl->SetStyle(wxRichTextRange(from, to), attr);
}

} // extern "C"
