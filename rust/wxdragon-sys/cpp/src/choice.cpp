#include "wx/choice.h"
#include "wx/window.h"
#include "wx/string.h"
#include "wx/arrstr.h"
#include "wx/defs.h" // For wxNOT_FOUND
#include "../include/wxdragon.h"
#include "wxd_utils.h"

extern "C" {

WXD_EXPORTED wxd_Choice_t* wxd_Choice_Create(
    wxd_Window_t* parent,
    wxd_Id id,
    wxd_Point pos,
    wxd_Size size,
    wxd_Style_t style
) {
    wxWindow* parentWin = (wxWindow*)parent;
    if (!parentWin) return nullptr;

    wxChoice* choice = new wxChoice(parentWin, id, 
                                  wxd_cpp_utils::to_wx(pos),
                                  wxd_cpp_utils::to_wx(size),
                                  0, nullptr, style);
    return (wxd_Choice_t*)choice;
}

WXD_EXPORTED void wxd_Choice_Append(wxd_Choice_t* choice, const char* item) {
    wxChoice* ch = (wxChoice*)choice;
    if (ch && item) {
        ch->Append(wxString::FromUTF8(item));
    }
}

WXD_EXPORTED void wxd_Choice_Clear(wxd_Choice_t* choice) {
    wxChoice* ch = (wxChoice*)choice;
    if (ch) {
        ch->Clear();
    }
}

WXD_EXPORTED int wxd_Choice_GetSelection(wxd_Choice_t* choice) {
    wxChoice* ch = (wxChoice*)choice;
    if (!ch) return wxNOT_FOUND;
    return ch->GetSelection();
}

WXD_EXPORTED int wxd_Choice_GetStringSelection(wxd_Choice_t* choice, char* buffer, int buffer_len) {
    if (!choice || !buffer || buffer_len <= 0) return -1;
    wxChoice* ch = (wxChoice*)choice;
    wxString selection = ch->GetStringSelection();
    return wxd_cpp_utils::copy_wxstring_to_buffer(selection, buffer, (size_t)buffer_len);
}

WXD_EXPORTED void wxd_Choice_SetSelection(wxd_Choice_t* choice, int index) {
    wxChoice* ch = (wxChoice*)choice;
    if (ch) {
        ch->SetSelection(index);
    }
}

WXD_EXPORTED int wxd_Choice_GetString(wxd_Choice_t* choice, int index, char* buffer, int buffer_len) {
    if (!choice || !buffer || buffer_len <= 0) return -1;
    wxChoice* ch = (wxChoice*)choice;
    if (index < 0 || (unsigned int)index >= ch->GetCount()) return -1;

    wxString item = ch->GetString((unsigned int)index);
    return wxd_cpp_utils::copy_wxstring_to_buffer(item, buffer, (size_t)buffer_len);
}

WXD_EXPORTED unsigned int wxd_Choice_GetCount(wxd_Choice_t* choice) {
    wxChoice* ch = (wxChoice*)choice;
    if (!ch) return 0;
    return ch->GetCount();
}

// Note: No explicit Destroy function needed. Choice is a child control,
// destroyed when its parent window is destroyed.

} // extern "C" 