#include <wx/wxprec.h>
#include <wx/wx.h>
#include "../include/wxdragon.h"
#include "wx/treebook.h"
#include "wxd_utils.h" // Added for copy_wxstring_to_buffer

extern "C" {

// Wrapper for wxTreebook::wxTreebook(wxWindow*, wxWindowID, wxPoint const&, wxSize const&, long)
WXD_EXPORTED wxd_Treebook_t*
wxd_Treebook_new(wxd_Window_t* parent, int id, int x, int y, int width, int height, int64_t style)
{
    wxTreebook* treebook =
        new wxTreebook((wxWindow*)parent, id, wxPoint(x, y), wxSize(width, height), style);
    return (wxd_Treebook_t*)treebook;
}

// wxTreebook doesn't have a Destroy method, it's destroyed by its parent.
// However, if we need to explicitly delete it (e.g. if not added to a parent or sizer),
// we might need a way. For now, assuming standard wxWidgets parent-child destruction.
// If explicit deletion is needed, this function would call `delete (wxTreebook*)self;`
// but that might lead to double frees if also managed by a parent.
// Let's keep it simple and assume parent management for now, or require explicit deletion via wxWindow_Destroy
// if it's not parented in a way that handles its deletion.
// For consistency with other wxd_XXX_Destroy functions, we can provide one that does nothing
// if wxTreebook itself doesn't need explicit destruction beyond what wxWindow provides.
// wxWindow_Destroy( (wxdWindow*)self ) would be the generic way if it needs explicit deletion.
// Let's assume for now it's handled like other wxWindow-derived classes.
WXD_EXPORTED void
wxd_Treebook_Destroy(wxd_Treebook_t* self)
{
    // wxTreebook is a wxWindow, its destruction is typically handled by its parent.
    // If it were a top-level window or explicitly managed, `delete (wxTreebook*)self;` might be used,
    // but that's risky if it's also in a sizer or parent.
    // The safest bet for explicit destruction is often to call the generic wxd_Window_Destroy.
    // For now, this function can be a no-op or call wxd_Window_Destroy.
    // Let's assume for now that if a user calls wxd_Treebook_Destroy, they mean to destroy it as a window.
    if (self) {
        wxd_Window_Destroy((wxd_Window_t*)self);
    }
}

// Wrapper for wxTreebook::AddPage(wxWindow*, wxString const&, bool, int)
WXD_EXPORTED int
wxd_Treebook_AddPage(wxd_Treebook_t* self, wxd_Window_t* page, const char* text, int bSelect,
                     int imageId)
{
    if (!self || !page)
        return 0; // Or some error indicator, AddPage returns bool
    // Use FromUTF8 for robustness, and handle null text
    return ((wxTreebook*)self)
        ->AddPage((wxWindow*)page, wxString::FromUTF8(text ? text : ""), (bool)bSelect, imageId);
}

// Wrapper for wxTreebook::AddSubPage(wxWindow*, wxString const&, bool, int)
WXD_EXPORTED int
wxd_Treebook_AddSubPage(wxd_Treebook_t* self, wxd_Window_t* page, const char* text, int bSelect,
                        int imageId)
{
    if (!self || !page)
        return 0; // Or some error indicator, AddSubPage returns bool
    // Use FromUTF8 for robustness, and handle null text
    return ((wxTreebook*)self)
        ->AddSubPage((wxWindow*)page, wxString::FromUTF8(text ? text : ""), (bool)bSelect, imageId);
}

// Wrapper for wxBookCtrlBase::GetPageCount()
WXD_EXPORTED int
wxd_Treebook_GetPageCount(wxd_Treebook_t* self)
{
    return ((wxTreebook*)self)->GetPageCount();
}

// Wrapper for wxBookCtrlBase::GetPage(size_t)
WXD_EXPORTED wxd_Window_t*
wxd_Treebook_GetPage(wxd_Treebook_t* self, size_t n)
{
    return (wxd_Window_t*)((wxTreebook*)self)->GetPage(n);
}

// Wrapper for wxBookCtrlBase::GetSelection()
WXD_EXPORTED int
wxd_Treebook_GetSelection(wxd_Treebook_t* self)
{
    return ((wxTreebook*)self)->GetSelection();
}

// Wrapper for wxBookCtrlBase::SetSelection(size_t)
WXD_EXPORTED int
wxd_Treebook_SetSelection(wxd_Treebook_t* self, size_t n)
{
    return ((wxTreebook*)self)->SetSelection(n);
}

// Wrapper for wxBookCtrlBase::SetPageText(size_t, wxString const&)
WXD_EXPORTED void
wxd_Treebook_SetPageText(wxd_Treebook_t* self, size_t n, const char* strText)
{
    ((wxTreebook*)self)->SetPageText(n, wxString(strText));
}

// Wrapper for wxBookCtrlBase::GetPageText(size_t)
// Changed signature to use caller-provided buffer
WXD_EXPORTED int
wxd_Treebook_GetPageText(wxd_Treebook_t* self, size_t n, char* buffer, int buffer_len)
{
    if (!self) {
        if (buffer && buffer_len > 0)
            buffer[0] = '\0';
        return 0; // Return 0 for error / no length needed
    }
    // Check index validity?
    if (n >= ((wxTreebook*)self)->GetPageCount()) {
        if (buffer && buffer_len > 0)
            buffer[0] = '\0';
        return 0;
    }

    wxString str = ((wxTreebook*)self)->GetPageText(n);
    size_t source_len_no_null =
        wxd_cpp_utils::copy_wxstring_to_buffer(str, buffer, static_cast<size_t>(buffer_len));

    // Return needed length (including null terminator)
    return static_cast<int>(source_len_no_null + 1);
}

} // extern "C"