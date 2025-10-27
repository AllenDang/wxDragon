#include <wx/wxprec.h>
#include <wx/wx.h>
#include "wxdragon.h"
#include <wx/simplebook.h>

// Implementation for wxd_SimpleBook_Create
WXD_EXPORTED wxd_SimpleBook_t*
wxd_SimpleBook_Create(wxd_Window_t* parent, wxd_Id id, wxd_Point pos, wxd_Size size,
                      wxd_Style_t style)
{
    wxWindow* parentWin = reinterpret_cast<wxWindow*>(parent);
    wxSimplebook* simplebook = new wxSimplebook(parentWin, id, wxPoint(pos.x, pos.y),
                                                wxSize(size.width, size.height), style);

    return reinterpret_cast<wxd_SimpleBook_t*>(simplebook);
}

// Implementation for wxd_SimpleBook_AddPage
WXD_EXPORTED bool
wxd_SimpleBook_AddPage(wxd_SimpleBook_t* self, wxd_Window_t* page, const char* text, bool select)
{
    wxSimplebook* simplebook = reinterpret_cast<wxSimplebook*>(self);
    wxWindow* pageWin = reinterpret_cast<wxWindow*>(page);
    if (!simplebook || !pageWin)
        return false;

    return simplebook->AddPage(pageWin, wxString::FromUTF8(text ? text : ""), select);
}

// Implementation for wxd_SimpleBook_GetSelection
WXD_EXPORTED int
wxd_SimpleBook_GetSelection(wxd_SimpleBook_t* self)
{
    wxSimplebook* simplebook = reinterpret_cast<wxSimplebook*>(self);
    if (!simplebook)
        return wxNOT_FOUND; // wxNOT_FOUND is typically -1
    return simplebook->GetSelection();
}

// Implementation for wxd_SimpleBook_SetSelection
WXD_EXPORTED int
wxd_SimpleBook_SetSelection(wxd_SimpleBook_t* self, int page)
{
    wxSimplebook* simplebook = reinterpret_cast<wxSimplebook*>(self);
    if (!simplebook)
        return wxNOT_FOUND;
    return simplebook->SetSelection(page);
}

// Implementation for wxd_SimpleBook_GetPageCount
WXD_EXPORTED size_t
wxd_SimpleBook_GetPageCount(wxd_SimpleBook_t* self)
{
    wxSimplebook* simplebook = reinterpret_cast<wxSimplebook*>(self);
    if (!simplebook)
        return 0;
    return simplebook->GetPageCount();
}

// Implementation for wxd_SimpleBook_GetPage
WXD_EXPORTED wxd_Window_t*
wxd_SimpleBook_GetPage(wxd_SimpleBook_t* self, size_t n)
{
    wxSimplebook* simplebook = reinterpret_cast<wxSimplebook*>(self);
    if (!simplebook)
        return nullptr;
    wxWindow* page = simplebook->GetPage(n);
    return reinterpret_cast<wxd_Window_t*>(page);
}

// Implementation for wxd_SimpleBook_RemovePage
WXD_EXPORTED bool
wxd_SimpleBook_RemovePage(wxd_SimpleBook_t* self, size_t n)
{
    wxSimplebook* simplebook = reinterpret_cast<wxSimplebook*>(self);
    if (!simplebook)
        return false;
    return simplebook->RemovePage(n);
}

// Implementation for wxd_SimpleBook_ChangeSelection
WXD_EXPORTED int
wxd_SimpleBook_ChangeSelection(wxd_SimpleBook_t* self, size_t page)
{
    wxSimplebook* simplebook = reinterpret_cast<wxSimplebook*>(self);
    if (!simplebook)
        return wxNOT_FOUND;
    return simplebook->ChangeSelection(page);
}

// Implementation for wxd_SimpleBook_InsertPage
WXD_EXPORTED bool
wxd_SimpleBook_InsertPage(wxd_SimpleBook_t* self, size_t index, wxd_Window_t* page,
                          const char* text, bool select)
{
    wxSimplebook* simplebook = reinterpret_cast<wxSimplebook*>(self);
    wxWindow* pageWin = reinterpret_cast<wxWindow*>(page);
    if (!simplebook || !pageWin)
        return false;

    return simplebook->InsertPage(index, pageWin, wxString::FromUTF8(text ? text : ""), select);
}

// Implementation for wxd_SimpleBook_AddPageWithImageId
WXD_EXPORTED bool
wxd_SimpleBook_AddPageWithImageId(wxd_SimpleBook_t* self, wxd_Window_t* page, const char* text,
                                  bool select, int imageId)
{
    wxSimplebook* simplebook = reinterpret_cast<wxSimplebook*>(self);
    wxWindow* pageWin = reinterpret_cast<wxWindow*>(page);
    if (!simplebook || !pageWin)
        return false;

    // wxSimplebook doesn't use images visually, but we support the API for consistency
    // The imageId parameter is ignored as SimpleBook has no visual tabs
    return simplebook->AddPage(pageWin, wxString::FromUTF8(text ? text : ""), select);
}

// Implementation for wxd_SimpleBook_InsertPageWithImageId
WXD_EXPORTED bool
wxd_SimpleBook_InsertPageWithImageId(wxd_SimpleBook_t* self, size_t index, wxd_Window_t* page,
                                     const char* text, bool select, int imageId)
{
    wxSimplebook* simplebook = reinterpret_cast<wxSimplebook*>(self);
    wxWindow* pageWin = reinterpret_cast<wxWindow*>(page);
    if (!simplebook || !pageWin)
        return false;

    // wxSimplebook doesn't use images visually, but we support the API for consistency
    // The imageId parameter is ignored as SimpleBook has no visual tabs
    return simplebook->InsertPage(index, pageWin, wxString::FromUTF8(text ? text : ""), select);
}