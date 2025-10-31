#include <wx/wxprec.h>
#include <wx/wx.h>
#include "../include/wxdragon.h"
#include "wxd_utils.h"
#include <wx/bmpcbox.h> // Include header for wxBitmapComboBox
#include <wx/bitmap.h>  // For wxBitmap
#include <wx/arrstr.h>  // For GetStrings

extern "C" {

WXD_EXPORTED wxd_BitmapComboBox_t*
wxd_BitmapComboBox_Create(wxd_Window_t* parent, wxd_Id id, const char* value, wxd_Point pos,
                          wxd_Size size, wxd_Style_t style)
{
    wxWindow* parentWin = (wxWindow*)parent;
    if (!parentWin)
        return nullptr;

    wxString wxValue = wxString::FromUTF8(value ? value : "");
    // wxBitmapComboBox needs an empty wxArrayString initially.
    wxBitmapComboBox* combo = new wxBitmapComboBox(parentWin, id, wxValue,
                                                   wxd_cpp_utils::to_wx(pos),
                                                   wxd_cpp_utils::to_wx(size),
                                                   wxArrayString(), // Empty choices initially
                                                   style);
    return (wxd_BitmapComboBox_t*)combo;
}

WXD_EXPORTED void
wxd_BitmapComboBox_Append(wxd_BitmapComboBox_t* self, const char* item,
                          const wxd_Bitmap_t* bitmap // Can be NULL
)
{
    wxBitmapComboBox* cb = (wxBitmapComboBox*)self;
    if (!cb)
        return;
    wxString wxItem = wxString::FromUTF8(item ? item : "");
    const wxBitmap* wxBmp = reinterpret_cast<const wxBitmap*>(bitmap);

    if (wxBmp && wxBmp->IsOk()) {
        cb->Append(wxItem, *wxBmp);
    }
    else {
        // Append without bitmap if bitmap is null or invalid
        cb->Append(wxItem);
        // Alternatively, use wxNullBitmap explicitly:
        // cb->Append(wxItem, wxNullBitmap);
    }
}

WXD_EXPORTED void
wxd_BitmapComboBox_Clear(wxd_BitmapComboBox_t* self)
{
    wxBitmapComboBox* cb = (wxBitmapComboBox*)self;
    if (cb) {
        cb->Clear();
        // cb->SetValue(""); // Optionally clear text field?
    }
}

WXD_EXPORTED int
wxd_BitmapComboBox_GetSelection(wxd_BitmapComboBox_t* self)
{
    wxBitmapComboBox* cb = (wxBitmapComboBox*)self;
    if (!cb)
        return wxNOT_FOUND;
    return cb->GetSelection();
}

WXD_EXPORTED void
wxd_BitmapComboBox_SetSelection(wxd_BitmapComboBox_t* self, int index)
{
    wxBitmapComboBox* cb = (wxBitmapComboBox*)self;
    if (cb) {
        cb->SetSelection(index);
    }
}

WXD_EXPORTED int
wxd_BitmapComboBox_GetString(wxd_BitmapComboBox_t* self, int index, char* buffer, size_t buffer_len)
{
    if (!self)
        return -1;
    wxBitmapComboBox* cb = (wxBitmapComboBox*)self;
    if (index < 0 || (unsigned int)index >= cb->GetCount())
        return -1;

    wxString item = cb->GetString((unsigned int)index);
    return (int)wxd_cpp_utils::copy_wxstring_to_buffer(item, buffer, buffer_len);
}

WXD_EXPORTED unsigned int
wxd_BitmapComboBox_GetCount(wxd_BitmapComboBox_t* self)
{
    wxBitmapComboBox* cb = (wxBitmapComboBox*)self;
    if (!cb)
        return 0;
    return cb->GetCount();
}

WXD_EXPORTED void
wxd_BitmapComboBox_SetValue(wxd_BitmapComboBox_t* self, const char* value)
{
    wxBitmapComboBox* cb = (wxBitmapComboBox*)self;
    if (cb) {
        cb->SetValue(wxString::FromUTF8(value ? value : ""));
    }
}

WXD_EXPORTED int
wxd_BitmapComboBox_GetValue(wxd_BitmapComboBox_t* self, char* buffer, size_t buffer_len)
{
    if (!self)
        return -1;
    wxBitmapComboBox* cb = (wxBitmapComboBox*)self;
    wxString value = cb->GetValue();
    return (int)wxd_cpp_utils::copy_wxstring_to_buffer(value, buffer, buffer_len);
}

WXD_EXPORTED wxd_Bitmap_t*
wxd_BitmapComboBox_GetItemBitmap(wxd_BitmapComboBox_t* self, unsigned int n)
{
    wxBitmapComboBox* cb = (wxBitmapComboBox*)self;
    if (!cb || n >= cb->GetCount())
        return nullptr;
    // GetItemBitmap returns a copy of the bitmap
    wxBitmap bmp = cb->GetItemBitmap(n);
    if (!bmp.IsOk())
        return nullptr;
    // Create a new wxBitmap on the heap to return a stable pointer
    return (wxd_Bitmap_t*)new (std::nothrow) wxBitmap(bmp);
}

WXD_EXPORTED void
wxd_BitmapComboBox_SetItemBitmap(wxd_BitmapComboBox_t* self, unsigned int n,
                                 const wxd_Bitmap_t* bitmap)
{
    wxBitmapComboBox* cb = (wxBitmapComboBox*)self;
    if (!cb || n >= cb->GetCount())
        return;
    const wxBitmap* wxBmp = reinterpret_cast<const wxBitmap*>(bitmap);
    // Use wxNullBitmap if the provided bitmap pointer is null or invalid
    cb->SetItemBitmap(n, (wxBmp && wxBmp->IsOk()) ? *wxBmp : wxNullBitmap);
}

// Destroy handled by parent window

} // extern "C"