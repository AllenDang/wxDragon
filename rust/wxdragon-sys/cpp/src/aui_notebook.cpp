#include <wx/wxprec.h>
#include <wx/wx.h>
#include "../include/wxdragon.h"
#include <wx/aui/auibook.h> // For wxAuiNotebook
#include "wxd_utils.h"

// Ensure this is part of wx/aui/auibook.h or wx/aui/aui.h
// If wxAuiNotebook is in wx/aui/aui.h, this might need adjustment,
// but typically dedicated controls are in their own headers like auibook.h

extern "C" {

WXD_EXPORTED wxd_AuiNotebook_t*
wxd_AuiNotebook_Create(wxd_Window_t* parent, int id, wxd_Point pos, wxd_Size size, int64_t style)
{
    wxWindow* parentPtr = (wxWindow*)parent;
    // Note: wxAuiNotebook parent can be null according to some docs, but safer to require for now.
    // if (!parentPtr) return nullptr;

    wxPoint wxPos = wxPoint(pos.x, pos.y);
    wxSize wxSizeInstance = wxSize(size.width, size.height);

    // Default style for wxAuiNotebook is wxAUI_NB_DEFAULT_STYLE
    // If style is 0 or some sentinel indicating default, wxAUI_NB_DEFAULT_STYLE should be used.
    // For now, assume `style` passed is intentional.
    wxAuiNotebook* notebook = new wxAuiNotebook(parentPtr, id, wxPos, wxSizeInstance, style);
    return (wxd_AuiNotebook_t*)notebook;
}

WXD_EXPORTED bool
wxd_AuiNotebook_AddPage(wxd_AuiNotebook_t* self, wxd_Window_t* page, const char* caption,
                        bool select, int bitmap_id)
{
    wxAuiNotebook* notebook = (wxAuiNotebook*)self;
    wxWindow* pagePtr = (wxWindow*)page;

    if (!notebook || !pagePtr) {
        return false;
    }

    (void)bitmap_id; // Mark as unused for now

    wxString wxCaption = WXD_STR_TO_WX_STRING_UTF8_NULL_OK(caption);
    // wxBitmapBundle can be added later if needed via another FFI function or by extending this one.
    // The wxWidgets AddPage has overload: (wxWindow* page, const wxString& text, bool sel = false, int imageId = -1)
    // And also: (wxWindow* page, const wxString& text, bool sel = false, const wxBitmapBundle& bitmap = wxBitmapBundle())
    // Our C API uses int bitmap_id, let's assume it maps to imageId if an ImageList is used, or it's a placeholder.
    // For now, using the version that takes imageId if wxWidgets 3.2+ supports it for AuiNotebook,
    // otherwise will use the bitmapbundle version with wxNullBitmap or default.
    // wxAuiNotebook::AddPage in 3.2.x has (..., const wxBitmapBundle& bitmap = wxBitmapBundle()) and (..., int imageId = wxID_NONE)
    // Let's use the imageId version.
    return notebook->AddPage(pagePtr, wxCaption, select, bitmap_id);
}

WXD_EXPORTED size_t
wxd_AuiNotebook_GetPageCount(wxd_AuiNotebook_t* self)
{
    wxAuiNotebook* notebook = (wxAuiNotebook*)self;
    if (!notebook) {
        return 0; // Or some error indication if size_t allows
    }
    return notebook->GetPageCount();
}

WXD_EXPORTED size_t
wxd_AuiNotebook_SetSelection(wxd_AuiNotebook_t* self, size_t new_page)
{
    wxAuiNotebook* notebook = (wxAuiNotebook*)self;
    if (!notebook) {
        // Return current selection or an error indicator. wxwidgets returns old selection.
        // For simplicity, if notebook is null, perhaps 0 or new_page itself if no better error val.
        return new_page;
    }
    return notebook->SetSelection(new_page);
}

WXD_EXPORTED int
wxd_AuiNotebook_GetSelection(wxd_AuiNotebook_t* self)
{
    wxAuiNotebook* notebook = (wxAuiNotebook*)self;
    if (!notebook) return -1;
    return notebook->GetSelection();
}

WXD_EXPORTED bool
wxd_AuiNotebook_InsertPage(wxd_AuiNotebook_t* self, size_t page_idx, wxd_Window_t* page,
                           const char* caption, bool select, int bitmap_id)
{
    wxAuiNotebook* notebook = (wxAuiNotebook*)self;
    wxWindow* pagePtr = (wxWindow*)page;
    if (!notebook || !pagePtr) return false;

    wxString wxCaption = WXD_STR_TO_WX_STRING_UTF8_NULL_OK(caption);
    return notebook->InsertPage(page_idx, pagePtr, wxCaption, select, bitmap_id);
}

WXD_EXPORTED bool
wxd_AuiNotebook_DeletePage(wxd_AuiNotebook_t* self, size_t page)
{
    wxAuiNotebook* notebook = (wxAuiNotebook*)self;
    if (!notebook) return false;
    return notebook->DeletePage(page);
}

WXD_EXPORTED bool
wxd_AuiNotebook_RemovePage(wxd_AuiNotebook_t* self, size_t page)
{
    wxAuiNotebook* notebook = (wxAuiNotebook*)self;
    if (!notebook) return false;
    return notebook->RemovePage(page);
}

WXD_EXPORTED bool
wxd_AuiNotebook_DeleteAllPages(wxd_AuiNotebook_t* self)
{
    wxAuiNotebook* notebook = (wxAuiNotebook*)self;
    if (!notebook) return false;
    return notebook->DeleteAllPages();
}

WXD_EXPORTED wxd_Window_t*
wxd_AuiNotebook_GetPage(wxd_AuiNotebook_t* self, size_t page_idx)
{
    wxAuiNotebook* notebook = (wxAuiNotebook*)self;
    if (!notebook) return nullptr;
    return (wxd_Window_t*)notebook->GetPage(page_idx);
}

WXD_EXPORTED int
wxd_AuiNotebook_GetPageIndex(wxd_AuiNotebook_t* self, wxd_Window_t* page_wnd)
{
    wxAuiNotebook* notebook = (wxAuiNotebook*)self;
    wxWindow* page = (wxWindow*)page_wnd;
    if (!notebook || !page) return -1;
    return notebook->GetPageIndex(page);
}

WXD_EXPORTED int
wxd_AuiNotebook_GetPageText(wxd_AuiNotebook_t* self, size_t page_idx, char* buffer, size_t buffer_len)
{
    wxAuiNotebook* notebook = (wxAuiNotebook*)self;
    if (!notebook) return -1;
    
    // Safety check for page index? wxWidgets might assert.
    if (page_idx >= notebook->GetPageCount()) return -1;
    
    wxString text = notebook->GetPageText(page_idx);
    return (int)wxd_cpp_utils::copy_wxstring_to_buffer(text, buffer, buffer_len);
}

WXD_EXPORTED bool
wxd_AuiNotebook_SetPageText(wxd_AuiNotebook_t* self, size_t page_idx, const char* text)
{
    wxAuiNotebook* notebook = (wxAuiNotebook*)self;
    if (!notebook) return false;
    
    if (page_idx >= notebook->GetPageCount()) return false;

    wxString wxText = WXD_STR_TO_WX_STRING_UTF8_NULL_OK(text);
    return notebook->SetPageText(page_idx, wxText);
}

} // extern "C"