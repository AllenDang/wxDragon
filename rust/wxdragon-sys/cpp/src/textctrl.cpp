#include "wx/wxprec.h"

#ifndef WX_PRECOMP
#include "wx/wx.h"
#endif

#include "wx/textctrl.h"
#include "wxdragon.h"
#include "wxd_utils.h"

extern "C" {

// Create a new wxTextCtrl
WXD_EXPORTED wxd_TextCtrl_t*
wxd_TextCtrl_Create(wxd_Window_t* parent, wxd_Id id, const char* value, wxd_Point pos,
                    wxd_Size size, wxd_Style_t style)
{
    wxWindow* parentWin = (wxWindow*)parent;
    wxTextCtrl* ctrl = new wxTextCtrl(parentWin, id, wxString::FromUTF8(value ? value : ""),
                                      wxd_cpp_utils::to_wx(pos), wxd_cpp_utils::to_wx(size), style);
    return (wxd_TextCtrl_t*)ctrl;
}

// Set the value of the wxTextCtrl
WXD_EXPORTED void
wxd_TextCtrl_SetValue(wxd_TextCtrl_t* textCtrl, const char* value)
{
    wxTextCtrl* ctrl = (wxTextCtrl*)textCtrl;
    if (ctrl) {
        ctrl->SetValue(wxString::FromUTF8(value ? value : ""));
    }
}

// Get the value of the wxTextCtrl
WXD_EXPORTED int
wxd_TextCtrl_GetValue(wxd_TextCtrl_t* textCtrl, char* buffer, int buffer_len)
{
    if (!textCtrl || !buffer || buffer_len <= 0)
        return -1;
    wxTextCtrl* ctrl = (wxTextCtrl*)textCtrl;
    wxString value = ctrl->GetValue();
    return wxd_cpp_utils::copy_wxstring_to_buffer(value, buffer, (size_t)buffer_len);
}

// Append text to the wxTextCtrl
WXD_EXPORTED void
wxd_TextCtrl_AppendText(wxd_TextCtrl_t* textCtrl, const char* text)
{
    wxTextCtrl* ctrl = (wxTextCtrl*)textCtrl;
    if (ctrl && text) {
        ctrl->AppendText(wxString::FromUTF8(text));
    }
}

// Clear the wxTextCtrl contents
WXD_EXPORTED void
wxd_TextCtrl_Clear(wxd_TextCtrl_t* textCtrl)
{
    wxTextCtrl* ctrl = (wxTextCtrl*)textCtrl;
    if (ctrl) {
        ctrl->Clear();
    }
}

WXD_EXPORTED void
wxd_TextCtrl_WriteText(wxd_TextCtrl_t* textCtrl, const char* text)
{
    wxTextCtrl* ctrl = (wxTextCtrl*)textCtrl;
    if (ctrl && text) {
        ctrl->WriteText(wxString::FromUTF8(text));
    }
}

WXD_EXPORTED void
wxd_TextCtrl_ChangeValue(wxd_TextCtrl_t* textCtrl, const char* value)
{
    wxTextCtrl* ctrl = (wxTextCtrl*)textCtrl;
    if (ctrl) {
        ctrl->ChangeValue(wxString::FromUTF8(value ? value : ""));
    }
}

WXD_EXPORTED void
wxd_TextCtrl_Remove(wxd_TextCtrl_t* textCtrl, wxd_Long_t from, wxd_Long_t to)
{
    wxTextCtrl* ctrl = (wxTextCtrl*)textCtrl;
    if (ctrl) {
        ctrl->Remove(from, to);
    }
}

WXD_EXPORTED void
wxd_TextCtrl_Replace(wxd_TextCtrl_t* textCtrl, wxd_Long_t from, wxd_Long_t to, const char* value)
{
    wxTextCtrl* ctrl = (wxTextCtrl*)textCtrl;
    if (ctrl && value) {
        ctrl->Replace(from, to, wxString::FromUTF8(value));
    }
}

WXD_EXPORTED int
wxd_TextCtrl_GetNumberOfLines(wxd_TextCtrl_t* textCtrl)
{
    wxTextCtrl* ctrl = (wxTextCtrl*)textCtrl;
    if (!ctrl) return 0;
    return ctrl->GetNumberOfLines();
}

WXD_EXPORTED int
wxd_TextCtrl_GetLineText(wxd_TextCtrl_t* textCtrl, wxd_Long_t lineNo, char* buffer, int buffer_len)
{
    if (!textCtrl || !buffer || buffer_len <= 0)
        return -1;
    wxTextCtrl* ctrl = (wxTextCtrl*)textCtrl;
    wxString text = ctrl->GetLineText(lineNo);
    return wxd_cpp_utils::copy_wxstring_to_buffer(text, buffer, (size_t)buffer_len);
}

// Check if the wxTextCtrl has been modified
WXD_EXPORTED bool
wxd_TextCtrl_IsModified(wxd_TextCtrl_t* textCtrl)
{
    wxTextCtrl* ctrl = (wxTextCtrl*)textCtrl;
    if (!ctrl)
        return false;
    return ctrl->IsModified();
}

// Set the modified state of the wxTextCtrl
WXD_EXPORTED void
wxd_TextCtrl_SetModified(wxd_TextCtrl_t* textCtrl, bool modified)
{
    wxTextCtrl* ctrl = (wxTextCtrl*)textCtrl;
    if (ctrl) {
        ctrl->SetModified(modified);
    }
}

// Make the wxTextCtrl editable or read-only
WXD_EXPORTED void
wxd_TextCtrl_SetEditable(wxd_TextCtrl_t* textCtrl, bool editable)
{
    wxTextCtrl* ctrl = (wxTextCtrl*)textCtrl;
    if (ctrl) {
        ctrl->SetEditable(editable);
    }
}

// Check if the wxTextCtrl is editable
WXD_EXPORTED bool
wxd_TextCtrl_IsEditable(wxd_TextCtrl_t* textCtrl)
{
    wxTextCtrl* ctrl = (wxTextCtrl*)textCtrl;
    if (!ctrl)
        return false;
    return ctrl->IsEditable();
}

// Get the insertion point position
WXD_EXPORTED wxd_Long_t
wxd_TextCtrl_GetInsertionPoint(wxd_TextCtrl_t* textCtrl)
{
    wxTextCtrl* ctrl = (wxTextCtrl*)textCtrl;
    if (!ctrl)
        return 0;
    return ctrl->GetInsertionPoint();
}

// Set the insertion point position
WXD_EXPORTED void
wxd_TextCtrl_SetInsertionPoint(wxd_TextCtrl_t* textCtrl, wxd_Long_t pos)
{
    wxTextCtrl* ctrl = (wxTextCtrl*)textCtrl;
    if (ctrl) {
        ctrl->SetInsertionPoint(pos);
    }
}

// Set the maximum length of text that can be entered
WXD_EXPORTED void
wxd_TextCtrl_SetMaxLength(wxd_TextCtrl_t* textCtrl, wxd_Long_t len)
{
    wxTextCtrl* ctrl = (wxTextCtrl*)textCtrl;
    if (ctrl) {
        ctrl->SetMaxLength(len);
    }
}

// Get the last position in the control (text length)
WXD_EXPORTED wxd_Long_t
wxd_TextCtrl_GetLastPosition(wxd_TextCtrl_t* textCtrl)
{
    wxTextCtrl* ctrl = (wxTextCtrl*)textCtrl;
    if (!ctrl)
        return 0;
    return ctrl->GetLastPosition();
}

// Check if the control is a multiline text control
WXD_EXPORTED bool
wxd_TextCtrl_IsMultiLine(wxd_TextCtrl_t* textCtrl)
{
    wxTextCtrl* ctrl = (wxTextCtrl*)textCtrl;
    if (!ctrl)
        return false;
    return ctrl->IsMultiLine();
}

// Check if the control is a single-line text control
WXD_EXPORTED bool
wxd_TextCtrl_IsSingleLine(wxd_TextCtrl_t* textCtrl)
{
    wxTextCtrl* ctrl = (wxTextCtrl*)textCtrl;
    if (!ctrl)
        return false;
    return ctrl->IsSingleLine();
}

// Selection operations
WXD_EXPORTED void
wxd_TextCtrl_SetSelection(wxd_TextCtrl_t* textCtrl, wxd_Long_t from, wxd_Long_t to)
{
    wxTextCtrl* ctrl = (wxTextCtrl*)textCtrl;
    if (ctrl) {
        ctrl->SetSelection(from, to);
    }
}

WXD_EXPORTED void
wxd_TextCtrl_GetSelection(wxd_TextCtrl_t* textCtrl, wxd_Long_t* from, wxd_Long_t* to)
{
    wxTextCtrl* ctrl = (wxTextCtrl*)textCtrl;
    if (ctrl && from && to) {
        long from_long, to_long;
        ctrl->GetSelection(&from_long, &to_long);
        *from = static_cast<wxd_Long_t>(from_long);
        *to = static_cast<wxd_Long_t>(to_long);
    }
}

WXD_EXPORTED void
wxd_TextCtrl_SelectAll(wxd_TextCtrl_t* textCtrl)
{
    wxTextCtrl* ctrl = (wxTextCtrl*)textCtrl;
    if (ctrl) {
        ctrl->SelectAll();
    }
}

WXD_EXPORTED int
wxd_TextCtrl_GetStringSelection(wxd_TextCtrl_t* textCtrl, char* buffer, int buffer_len)
{
    if (!textCtrl || !buffer || buffer_len <= 0)
        return -1;
    wxTextCtrl* ctrl = (wxTextCtrl*)textCtrl;
    wxString selection = ctrl->GetStringSelection();
    return wxd_cpp_utils::copy_wxstring_to_buffer(selection, buffer, (size_t)buffer_len);
}

WXD_EXPORTED void
wxd_TextCtrl_SetInsertionPointEnd(wxd_TextCtrl_t* textCtrl)
{
    wxTextCtrl* ctrl = reinterpret_cast<wxTextCtrl*>(textCtrl);
    if (ctrl) {
        ctrl->SetInsertionPointEnd();
    }
}

// Helper to convert wxd_Colour_t to wxColour
static inline wxColour
to_wx(wxd_Colour_t c_col)
{
    return wxColour(c_col.r, c_col.g, c_col.b, c_col.a);
}

WXD_EXPORTED wxd_TextAttr_t*
wxd_TextAttr_Create()
{
    return reinterpret_cast<wxd_TextAttr_t*>(new wxTextAttr());
}

WXD_EXPORTED void
wxd_TextAttr_Delete(wxd_TextAttr_t* attr)
{
    delete reinterpret_cast<wxTextAttr*>(attr);
}

WXD_EXPORTED void
wxd_TextAttr_SetTextColour(wxd_TextAttr_t* attr, wxd_Colour_t colText)
{
    if (attr) {
        reinterpret_cast<wxTextAttr*>(attr)->SetTextColour(to_wx(colText));
    }
}

WXD_EXPORTED void
wxd_TextAttr_SetBackgroundColour(wxd_TextAttr_t* attr, wxd_Colour_t colBack)
{
    if (attr) {
        reinterpret_cast<wxTextAttr*>(attr)->SetBackgroundColour(to_wx(colBack));
    }
}

WXD_EXPORTED void
wxd_TextAttr_SetFont(wxd_TextAttr_t* attr, const wxd_Font_t* font)
{
    if (attr && font) {
        reinterpret_cast<wxTextAttr*>(attr)->SetFont(*reinterpret_cast<const wxFont*>(font));
    }
}

WXD_EXPORTED void
wxd_TextAttr_SetAlignment(wxd_TextAttr_t* attr, int alignment)
{
    if (attr) {
        reinterpret_cast<wxTextAttr*>(attr)->SetAlignment(static_cast<wxTextAttrAlignment>(alignment));
    }
}

WXD_EXPORTED void
wxd_TextAttr_SetLeftIndent(wxd_TextAttr_t* attr, int indent, int subIndent)
{
    if (attr) {
        reinterpret_cast<wxTextAttr*>(attr)->SetLeftIndent(indent, subIndent);
    }
}

WXD_EXPORTED void
wxd_TextAttr_SetRightIndent(wxd_TextAttr_t* attr, int indent)
{
    if (attr) {
        reinterpret_cast<wxTextAttr*>(attr)->SetRightIndent(indent);
    }
}

WXD_EXPORTED void
wxd_TextAttr_SetLineSpacing(wxd_TextAttr_t* attr, int spacing)
{
    if (attr) {
        reinterpret_cast<wxTextAttr*>(attr)->SetLineSpacing(spacing);
    }
}

WXD_EXPORTED void
wxd_TextAttr_SetParagraphSpacingAfter(wxd_TextAttr_t* attr, int spacing)
{
    if (attr) {
        reinterpret_cast<wxTextAttr*>(attr)->SetParagraphSpacingAfter(spacing);
    }
}

WXD_EXPORTED void
wxd_TextAttr_SetParagraphSpacingBefore(wxd_TextAttr_t* attr, int spacing)
{
    if (attr) {
        reinterpret_cast<wxTextAttr*>(attr)->SetParagraphSpacingBefore(spacing);
    }
}

WXD_EXPORTED void
wxd_TextAttr_SetBulletStyle(wxd_TextAttr_t* attr, int style)
{
    if (attr) {
        reinterpret_cast<wxTextAttr*>(attr)->SetBulletStyle(style);
    }
}

WXD_EXPORTED void wxd_TextAttr_SetFlags(wxd_TextAttr_t* attr, wxd_Long_t flags) { reinterpret_cast<wxTextAttr*>(attr)->SetFlags(flags); }
WXD_EXPORTED wxd_Long_t wxd_TextAttr_GetFlags(wxd_TextAttr_t* attr) { return reinterpret_cast<wxTextAttr*>(attr)->GetFlags(); }
WXD_EXPORTED bool wxd_TextAttr_HasFlag(wxd_TextAttr_t* attr, wxd_Long_t flag) { return reinterpret_cast<wxTextAttr*>(attr)->HasFlag(flag); }

WXD_EXPORTED void wxd_TextAttr_SetFontSize(wxd_TextAttr_t* attr, int pointSize) { reinterpret_cast<wxTextAttr*>(attr)->SetFontSize(pointSize); }
WXD_EXPORTED int wxd_TextAttr_GetFontSize(wxd_TextAttr_t* attr) { return reinterpret_cast<wxTextAttr*>(attr)->GetFontSize(); }

WXD_EXPORTED void wxd_TextAttr_SetFontStyle(wxd_TextAttr_t* attr, int fontStyle) { reinterpret_cast<wxTextAttr*>(attr)->SetFontStyle(static_cast<wxFontStyle>(fontStyle)); }
WXD_EXPORTED int wxd_TextAttr_GetFontStyle(wxd_TextAttr_t* attr) { return static_cast<int>(reinterpret_cast<wxTextAttr*>(attr)->GetFontStyle()); }

WXD_EXPORTED void wxd_TextAttr_SetFontWeight(wxd_TextAttr_t* attr, int fontWeight) { reinterpret_cast<wxTextAttr*>(attr)->SetFontWeight(static_cast<wxFontWeight>(fontWeight)); }
WXD_EXPORTED int wxd_TextAttr_GetFontWeight(wxd_TextAttr_t* attr) { return static_cast<int>(reinterpret_cast<wxTextAttr*>(attr)->GetFontWeight()); }

WXD_EXPORTED void wxd_TextAttr_SetFontFaceName(wxd_TextAttr_t* attr, const char* faceName) { reinterpret_cast<wxTextAttr*>(attr)->SetFontFaceName(wxString::FromUTF8(faceName ? faceName : "")); }
WXD_EXPORTED void wxd_TextAttr_SetFontUnderlined(wxd_TextAttr_t* attr, bool underlined) { reinterpret_cast<wxTextAttr*>(attr)->SetFontUnderlined(underlined); }
WXD_EXPORTED void wxd_TextAttr_SetFontStrikethrough(wxd_TextAttr_t* attr, bool strikethrough) { reinterpret_cast<wxTextAttr*>(attr)->SetFontStrikethrough(strikethrough); }
WXD_EXPORTED void wxd_TextAttr_SetFontEncoding(wxd_TextAttr_t* attr, int encoding) { reinterpret_cast<wxTextAttr*>(attr)->SetFontEncoding(static_cast<wxFontEncoding>(encoding)); }
WXD_EXPORTED void wxd_TextAttr_SetFontFamily(wxd_TextAttr_t* attr, int family) { reinterpret_cast<wxTextAttr*>(attr)->SetFontFamily(static_cast<wxFontFamily>(family)); }

WXD_EXPORTED void wxd_TextAttr_SetCharacterStyleName(wxd_TextAttr_t* attr, const char* name) { reinterpret_cast<wxTextAttr*>(attr)->SetCharacterStyleName(wxString::FromUTF8(name ? name : "")); }
WXD_EXPORTED void wxd_TextAttr_SetParagraphStyleName(wxd_TextAttr_t* attr, const char* name) { reinterpret_cast<wxTextAttr*>(attr)->SetParagraphStyleName(wxString::FromUTF8(name ? name : "")); }
WXD_EXPORTED void wxd_TextAttr_SetListStyleName(wxd_TextAttr_t* attr, const char* name) { reinterpret_cast<wxTextAttr*>(attr)->SetListStyleName(wxString::FromUTF8(name ? name : "")); }

WXD_EXPORTED void wxd_TextAttr_SetBulletNumber(wxd_TextAttr_t* attr, int n) { reinterpret_cast<wxTextAttr*>(attr)->SetBulletNumber(n); }
WXD_EXPORTED void wxd_TextAttr_SetBulletText(wxd_TextAttr_t* attr, const char* text) { reinterpret_cast<wxTextAttr*>(attr)->SetBulletText(wxString::FromUTF8(text ? text : "")); }
WXD_EXPORTED void wxd_TextAttr_SetBulletFont(wxd_TextAttr_t* attr, const char* bulletFont) { reinterpret_cast<wxTextAttr*>(attr)->SetBulletFont(wxString::FromUTF8(bulletFont ? bulletFont : "")); }
WXD_EXPORTED void wxd_TextAttr_SetBulletName(wxd_TextAttr_t* attr, const char* name) { reinterpret_cast<wxTextAttr*>(attr)->SetBulletName(wxString::FromUTF8(name ? name : "")); }
WXD_EXPORTED void wxd_TextAttr_SetURL(wxd_TextAttr_t* attr, const char* url) { reinterpret_cast<wxTextAttr*>(attr)->SetURL(wxString::FromUTF8(url ? url : "")); }
WXD_EXPORTED void wxd_TextAttr_SetPageBreak(wxd_TextAttr_t* attr, bool pageBreak) { reinterpret_cast<wxTextAttr*>(attr)->SetPageBreak(pageBreak); }
WXD_EXPORTED void wxd_TextAttr_SetTextEffects(wxd_TextAttr_t* attr, int effects) { reinterpret_cast<wxTextAttr*>(attr)->SetTextEffects(effects); }
WXD_EXPORTED void wxd_TextAttr_SetTextEffectFlags(wxd_TextAttr_t* attr, int effects) { reinterpret_cast<wxTextAttr*>(attr)->SetTextEffectFlags(effects); }
WXD_EXPORTED void wxd_TextAttr_SetOutlineLevel(wxd_TextAttr_t* attr, int level) { reinterpret_cast<wxTextAttr*>(attr)->SetOutlineLevel(level); }

static inline wxd_Colour_t from_wx(const wxColour& col) {
    wxd_Colour_t c = { col.Red(), col.Green(), col.Blue(), col.Alpha() };
    return c;
}

WXD_EXPORTED wxd_Colour_t wxd_TextAttr_GetTextColour(wxd_TextAttr_t* attr) { return from_wx(reinterpret_cast<wxTextAttr*>(attr)->GetTextColour()); }
WXD_EXPORTED wxd_Colour_t wxd_TextAttr_GetBackgroundColour(wxd_TextAttr_t* attr) { return from_wx(reinterpret_cast<wxTextAttr*>(attr)->GetBackgroundColour()); }
WXD_EXPORTED int wxd_TextAttr_GetAlignment(wxd_TextAttr_t* attr) { return static_cast<int>(reinterpret_cast<wxTextAttr*>(attr)->GetAlignment()); }
WXD_EXPORTED int wxd_TextAttr_GetLeftIndent(wxd_TextAttr_t* attr) { return static_cast<int>(reinterpret_cast<wxTextAttr*>(attr)->GetLeftIndent()); }
WXD_EXPORTED int wxd_TextAttr_GetLeftSubIndent(wxd_TextAttr_t* attr) { return static_cast<int>(reinterpret_cast<wxTextAttr*>(attr)->GetLeftSubIndent()); }
WXD_EXPORTED int wxd_TextAttr_GetRightIndent(wxd_TextAttr_t* attr) { return static_cast<int>(reinterpret_cast<wxTextAttr*>(attr)->GetRightIndent()); }
WXD_EXPORTED int wxd_TextAttr_GetLineSpacing(wxd_TextAttr_t* attr) { return reinterpret_cast<wxTextAttr*>(attr)->GetLineSpacing(); }
WXD_EXPORTED int wxd_TextAttr_GetParagraphSpacingAfter(wxd_TextAttr_t* attr) { return reinterpret_cast<wxTextAttr*>(attr)->GetParagraphSpacingAfter(); }
WXD_EXPORTED int wxd_TextAttr_GetParagraphSpacingBefore(wxd_TextAttr_t* attr) { return reinterpret_cast<wxTextAttr*>(attr)->GetParagraphSpacingBefore(); }
WXD_EXPORTED int wxd_TextAttr_GetBulletStyle(wxd_TextAttr_t* attr) { return reinterpret_cast<wxTextAttr*>(attr)->GetBulletStyle(); }

WXD_EXPORTED void
wxd_TextCtrl_SetStyle(wxd_TextCtrl_t* textCtrl, wxd_Long_t start, wxd_Long_t end, const wxd_TextAttr_t* style)
{
    wxTextCtrl* ctrl = reinterpret_cast<wxTextCtrl*>(textCtrl);
    if (ctrl && style) {
        ctrl->SetStyle(start, end, *reinterpret_cast<const wxTextAttr*>(style));
    }
}

WXD_EXPORTED wxd_TextAttr_t*
wxd_TextCtrl_GetDefaultStyle(wxd_TextCtrl_t* textCtrl)
{
    wxTextCtrl* ctrl = reinterpret_cast<wxTextCtrl*>(textCtrl);
    if (!ctrl) return nullptr;
    return reinterpret_cast<wxd_TextAttr_t*>(new wxTextAttr(ctrl->GetDefaultStyle()));
}

WXD_EXPORTED void
wxd_TextCtrl_SetDefaultStyle(wxd_TextCtrl_t* textCtrl, const wxd_TextAttr_t* style)
{
    wxTextCtrl* ctrl = reinterpret_cast<wxTextCtrl*>(textCtrl);
    if (ctrl && style) {
        ctrl->SetDefaultStyle(*reinterpret_cast<const wxTextAttr*>(style));
    }
}

WXD_EXPORTED bool
wxd_TextCtrl_PositionToXY(wxd_TextCtrl_t* textCtrl, wxd_Long_t pos, wxd_Long_t* x, wxd_Long_t* y)
{
    wxTextCtrl* ctrl = reinterpret_cast<wxTextCtrl*>(textCtrl);
    if (!ctrl || !x || !y) return false;
    long lx, ly;
    bool res = ctrl->PositionToXY(static_cast<long>(pos), &lx, &ly);
    *x = static_cast<wxd_Long_t>(lx);
    *y = static_cast<wxd_Long_t>(ly);
    return res;
}

WXD_EXPORTED wxd_Long_t
wxd_TextCtrl_XYToPosition(wxd_TextCtrl_t* textCtrl, wxd_Long_t x, wxd_Long_t y)
{
    wxTextCtrl* ctrl = reinterpret_cast<wxTextCtrl*>(textCtrl);
    if (!ctrl) return 0;
    return static_cast<wxd_Long_t>(ctrl->XYToPosition(static_cast<long>(x), static_cast<long>(y)));
}

WXD_EXPORTED int
wxd_TextCtrl_GetLineLength(wxd_TextCtrl_t* textCtrl, wxd_Long_t lineNo)
{
    wxTextCtrl* ctrl = reinterpret_cast<wxTextCtrl*>(textCtrl);
    if (!ctrl) return 0;
    return ctrl->GetLineLength(static_cast<long>(lineNo));
}

} // extern "C"