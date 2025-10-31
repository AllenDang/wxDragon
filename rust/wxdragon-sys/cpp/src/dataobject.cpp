#include <wx/wxprec.h>
#include <wx/wx.h>
#include "../include/wxdragon.h"
#include <wx/clipbrd.h>
#include <wx/dataobj.h>
#include <wx/bitmap.h>

extern "C" {

void
wxd_DataObject_Destroy(wxd_DataObject_t* data_object)
{
    if (!data_object)
        return;
    wxDataObject* wx_data_object = reinterpret_cast<wxDataObject*>(data_object);
    delete wx_data_object;
}

// --- TextDataObject Functions ---

wxd_TextDataObject_t*
wxd_TextDataObject_Create(const char* text)
{
    wxTextDataObject* data_object = new wxTextDataObject(wxString::FromUTF8(text ? text : ""));
    return reinterpret_cast<wxd_TextDataObject_t*>(data_object);
}

void
wxd_TextDataObject_Destroy(wxd_TextDataObject_t* obj)
{
    if (obj) {
        delete reinterpret_cast<wxTextDataObject*>(obj);
    }
}

WXD_EXPORTED int
wxd_TextDataObject_GetText(const wxd_TextDataObject_t* data_object, char* buffer, size_t buffer_len)
{
    if (!data_object)
        return 0;
    const wxTextDataObject* wx_data_object = reinterpret_cast<const wxTextDataObject*>(data_object);

    wxString text = wx_data_object->GetText();
    return (int)wxd_cpp_utils::copy_wxstring_to_buffer(text, buffer, buffer_len);
}

void
wxd_TextDataObject_SetText(wxd_TextDataObject_t* data_object, const char* text)
{
    if (!data_object)
        return;
    wxTextDataObject* wx_data_object = reinterpret_cast<wxTextDataObject*>(data_object);
    wx_data_object->SetText(wxString::FromUTF8(text ? text : ""));
}

// --- FileDataObject Functions ---

wxd_FileDataObject_t*
wxd_FileDataObject_Create()
{
    wxFileDataObject* data_object = new wxFileDataObject();
    return reinterpret_cast<wxd_FileDataObject_t*>(data_object);
}

void
wxd_FileDataObject_Destroy(wxd_FileDataObject_t* obj)
{
    if (obj) {
        delete reinterpret_cast<wxFileDataObject*>(obj);
    }
}

void
wxd_FileDataObject_AddFile(wxd_FileDataObject_t* data_object, const char* file)
{
    if (!data_object || !file)
        return;
    wxFileDataObject* wx_data_object = reinterpret_cast<wxFileDataObject*>(data_object);
    wx_data_object->AddFile(wxString::FromUTF8(file));
}

int
wxd_FileDataObject_GetFileCount(wxd_FileDataObject_t* data_object)
{
    if (!data_object)
        return 0;
    wxFileDataObject* wx_data_object = reinterpret_cast<wxFileDataObject*>(data_object);
    return wx_data_object->GetFilenames().GetCount();
}

int
wxd_FileDataObject_GetFile(wxd_FileDataObject_t* data_object, int index, char* buffer,
                           size_t buffer_len)
{
    if (!data_object)
        return -1;
    wxFileDataObject* wx_data_object = reinterpret_cast<wxFileDataObject*>(data_object);

    const wxArrayString& filenames = wx_data_object->GetFilenames();
    if (index < 0 || index >= static_cast<int>(filenames.GetCount())) {
        return -1;
    }

    wxString file = filenames[index];
    return (int)wxd_cpp_utils::copy_wxstring_to_buffer(file, buffer, buffer_len);
}

int
wxd_FileDataObject_GetFilenames(wxd_FileDataObject_t* obj, wxd_ArrayString_t* filenames)
{
    if (!obj || !filenames)
        return 0;

    wxFileDataObject* data_object = reinterpret_cast<wxFileDataObject*>(obj);
    wxArrayString* array_string = reinterpret_cast<wxArrayString*>(filenames);

    // Clear the existing array and copy new values
    array_string->Clear();
    const wxArrayString& wx_filenames = data_object->GetFilenames();
    for (size_t i = 0; i < wx_filenames.GetCount(); i++) {
        array_string->Add(wx_filenames[i]);
    }

    return wx_filenames.GetCount();
}

// --- BitmapDataObject Functions ---

wxd_BitmapDataObject_t*
wxd_BitmapDataObject_Create(const wxd_Bitmap_t* bitmap)
{
    if (!bitmap)
        return nullptr;
    const wxBitmap* wx_bitmap = reinterpret_cast<const wxBitmap*>(bitmap);
    wxBitmapDataObject* data_object = new wxBitmapDataObject(*wx_bitmap);
    return reinterpret_cast<wxd_BitmapDataObject_t*>(data_object);
}

wxd_Bitmap_t*
wxd_BitmapDataObject_GetBitmap(wxd_BitmapDataObject_t* data_object)
{
    if (!data_object)
        return nullptr;
    wxBitmapDataObject* wx_data_object = reinterpret_cast<wxBitmapDataObject*>(data_object);

    // Get the bitmap from the data object
    // Note: We need to create a new wxBitmap since we need to return one that the caller owns
    wxBitmap original = wx_data_object->GetBitmap();
    if (!original.IsOk()) {
        return nullptr;
    }

    // Create a new bitmap that's a copy of the original
    wxBitmap* new_bitmap = new wxBitmap(original);
    return reinterpret_cast<wxd_Bitmap_t*>(new_bitmap);
}

} // extern "C"