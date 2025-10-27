#include <wx/wxprec.h>
#include <wx/wx.h>
#include "wxdragon.h"
#include <wx/notebook.h>

// Implementation for wxd_Notebook_Create
WXD_EXPORTED wxd_Notebook_t*
wxd_Notebook_Create(wxd_Window_t* parent, wxd_Id id, wxd_Point pos, wxd_Size size,
                    wxd_Style_t style)
{
    wxWindow* parentWin = reinterpret_cast<wxWindow*>(parent);
    wxNotebook* notebook = new wxNotebook(parentWin, id, wxPoint(pos.x, pos.y),
                                          wxSize(size.width, size.height), style);

    return reinterpret_cast<wxd_Notebook_t*>(notebook);
}

// Implementation for wxd_Notebook_AddPage
WXD_EXPORTED bool
wxd_Notebook_AddPage(wxd_Notebook_t* self, wxd_Window_t* page, const char* text, bool select)
{
    wxNotebook* notebook = reinterpret_cast<wxNotebook*>(self);
    wxWindow* pageWin = reinterpret_cast<wxWindow*>(page);
    if (!notebook || !pageWin)
        return false;

    return notebook->AddPage(pageWin, wxString::FromUTF8(text ? text : ""), select);
}

// Implementation for wxd_Notebook_GetSelection
WXD_EXPORTED int
wxd_Notebook_GetSelection(wxd_Notebook_t* self)
{
    wxNotebook* notebook = reinterpret_cast<wxNotebook*>(self);
    if (!notebook)
        return wxNOT_FOUND; // wxNOT_FOUND is typically -1
    return notebook->GetSelection();
}

// Implementation for wxd_Notebook_SetSelection
WXD_EXPORTED int
wxd_Notebook_SetSelection(wxd_Notebook_t* self, int page)
{
    wxNotebook* notebook = reinterpret_cast<wxNotebook*>(self);
    if (!notebook)
        return wxNOT_FOUND;
    return notebook->SetSelection(static_cast<size_t>(page));
}

// Implementation for wxd_NotebookEvent_GetSelection
WXD_EXPORTED int
wxd_NotebookEvent_GetSelection(wxd_Event_t* event)
{
    if (!event)
        return wxNOT_FOUND;
    wxEvent* baseEvent = reinterpret_cast<wxEvent*>(event);
    wxBookCtrlEvent* notebookEvent = dynamic_cast<wxBookCtrlEvent*>(baseEvent);
    if (!notebookEvent)
        return wxNOT_FOUND;
    return notebookEvent->GetSelection();
}

// Implementation for wxd_NotebookEvent_GetOldSelection
WXD_EXPORTED int
wxd_NotebookEvent_GetOldSelection(wxd_Event_t* event)
{
    if (!event)
        return wxNOT_FOUND;
    wxEvent* baseEvent = reinterpret_cast<wxEvent*>(event);
    wxBookCtrlEvent* notebookEvent = dynamic_cast<wxBookCtrlEvent*>(baseEvent);
    if (!notebookEvent)
        return wxNOT_FOUND;
    return notebookEvent->GetOldSelection();
}

// --- Restored/Added implementations ---

WXD_EXPORTED void
wxd_Notebook_SetPadding(wxd_Notebook_t* self, wxd_Size padding)
{
    wxNotebook* notebook = reinterpret_cast<wxNotebook*>(self);
    if (notebook) {
        notebook->SetPadding(wxSize(padding.width, padding.height));
    }
}

WXD_EXPORTED int
wxd_Notebook_ChangeSelection(wxd_Notebook_t* self, size_t page)
{
    wxNotebook* notebook = reinterpret_cast<wxNotebook*>(self);
    if (!notebook)
        return wxNOT_FOUND;
    return notebook->ChangeSelection(page);
}

// Implementation for wxd_Notebook_AdvanceSelection (already correct, single arg)
WXD_EXPORTED void
wxd_Notebook_AdvanceSelection(wxd_Notebook_t* self, bool forward)
{
    wxNotebook* notebook = reinterpret_cast<wxNotebook*>(self);
    if (notebook) {
        notebook->AdvanceSelection(forward);
    }
}

// ImageList support
WXD_EXPORTED void
wxd_Notebook_SetImageList(wxd_Notebook_t* self, wxd_ImageList_t* imageList)
{
    wxNotebook* notebook = reinterpret_cast<wxNotebook*>(self);
    wxImageList* wx_imageList = reinterpret_cast<wxImageList*>(imageList);
    if (notebook) {
        notebook->SetImageList(wx_imageList); // wxNotebook takes ownership
    }
}

WXD_EXPORTED wxd_ImageList_t*
wxd_Notebook_GetImageList(wxd_Notebook_t* self)
{
    wxNotebook* notebook = reinterpret_cast<wxNotebook*>(self);
    if (!notebook)
        return nullptr;
    return reinterpret_cast<wxd_ImageList_t*>(notebook->GetImageList());
}

// Modified page manipulation to include imageId
WXD_EXPORTED bool
wxd_Notebook_AddPageWithImageId(wxd_Notebook_t* self, wxd_Window_t* page, const char* text,
                                bool select, int imageId)
{
    wxNotebook* notebook = reinterpret_cast<wxNotebook*>(self);
    wxWindow* pageWin = reinterpret_cast<wxWindow*>(page);
    if (!notebook || !pageWin)
        return false;
    return notebook->AddPage(pageWin, wxString::FromUTF8(text ? text : ""), select, imageId);
}

WXD_EXPORTED bool
wxd_Notebook_InsertPage(wxd_Notebook_t* self, size_t index, wxd_Window_t* page, const char* text,
                        bool select)
{
    wxNotebook* notebook = reinterpret_cast<wxNotebook*>(self);
    wxWindow* pageWin = reinterpret_cast<wxWindow*>(page);
    if (!notebook || !pageWin)
        return false;
    return notebook->InsertPage(index, pageWin, wxString::FromUTF8(text ? text : ""), select);
}

WXD_EXPORTED bool
wxd_Notebook_InsertPageWithImageId(wxd_Notebook_t* self, size_t index, wxd_Window_t* page,
                                   const char* text, bool select, int imageId)
{
    wxNotebook* notebook = reinterpret_cast<wxNotebook*>(self);
    wxWindow* pageWin = reinterpret_cast<wxWindow*>(page);
    if (!notebook || !pageWin)
        return false;
    return notebook->InsertPage(index, pageWin, wxString::FromUTF8(text ? text : ""), select,
                                imageId);
}

// Destroy function is implicitly handled by wxWidgets hierarchy when parent is destroyed,
// or via manual Destroy() call if needed (e.g., wxd_Window_Destroy).
// Explicit wxd_Notebook_Destroy is likely not needed if it's always owned by a parent.

WXD_EXPORTED size_t
wxd_Notebook_GetPageCount(wxd_Notebook_t* self)
{
    wxNotebook* notebook = reinterpret_cast<wxNotebook*>(self);
    if (!notebook)
        return 0;
    return notebook->GetPageCount();
}

WXD_EXPORTED wxd_Window_t*
wxd_Notebook_GetPage(wxd_Notebook_t* self, size_t n)
{
    wxNotebook* notebook = reinterpret_cast<wxNotebook*>(self);
    if (!notebook)
        return nullptr;
    wxWindow* page = notebook->GetPage(n);
    return reinterpret_cast<wxd_Window_t*>(page);
}

WXD_EXPORTED bool
wxd_Notebook_RemovePage(wxd_Notebook_t* self, size_t n)
{
    wxNotebook* notebook = reinterpret_cast<wxNotebook*>(self);
    if (!notebook)
        return false;
    return notebook->RemovePage(n);
}
