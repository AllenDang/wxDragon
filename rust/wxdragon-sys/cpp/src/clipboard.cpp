#include <wx/wxprec.h>
#include <wx/wx.h>
#include "../include/wxdragon.h"
#include <wx/clipbrd.h>
#include <wx/dataobj.h>

extern "C" {

wxd_Clipboard_t*
wxd_Clipboard_Get()
{
    return reinterpret_cast<wxd_Clipboard_t*>(wxClipboard::Get());
}

bool
wxd_Clipboard_Open(wxd_Clipboard_t* clipboard)
{
    if (!clipboard)
        return false;
    wxClipboard* wx_clipboard = reinterpret_cast<wxClipboard*>(clipboard);
    return wx_clipboard->Open();
}

void
wxd_Clipboard_Close(wxd_Clipboard_t* clipboard)
{
    if (!clipboard)
        return;
    wxClipboard* wx_clipboard = reinterpret_cast<wxClipboard*>(clipboard);
    wx_clipboard->Close();
}

bool
wxd_Clipboard_IsOpened(wxd_Clipboard_t* clipboard)
{
    if (!clipboard)
        return false;
    wxClipboard* wx_clipboard = reinterpret_cast<wxClipboard*>(clipboard);
    return wx_clipboard->IsOpened();
}

bool
wxd_Clipboard_AddData(wxd_Clipboard_t* clipboard, wxd_DataObject_t* data)
{
    if (!clipboard || !data)
        return false;
    wxClipboard* wx_clipboard = reinterpret_cast<wxClipboard*>(clipboard);
    wxDataObject* wx_data = reinterpret_cast<wxDataObject*>(data);
    return wx_clipboard->AddData(wx_data);
}

bool
wxd_Clipboard_SetData(wxd_Clipboard_t* clipboard, wxd_DataObject_t* data)
{
    if (!clipboard || !data)
        return false;
    wxClipboard* wx_clipboard = reinterpret_cast<wxClipboard*>(clipboard);
    wxDataObject* wx_data = reinterpret_cast<wxDataObject*>(data);
    return wx_clipboard->SetData(wx_data);
}

bool
wxd_Clipboard_IsSupported(wxd_Clipboard_t* clipboard, int format)
{
    if (!clipboard)
        return false;
    wxClipboard* wx_clipboard = reinterpret_cast<wxClipboard*>(clipboard);

    wxDataFormat dataFormat;

    // Special handling for different formats
    switch (format) {
    case 1: // wxDF_TEXT
        dataFormat = wxDF_TEXT;
        break;
    case 2: // wxDF_BITMAP
        dataFormat = wxDF_BITMAP;
        break;
    case 4: // wxDF_FILENAME
        dataFormat = wxDF_FILENAME;
        break;
    default:
        dataFormat = (wxDataFormatId)format;
        break;
    }

    return wx_clipboard->IsSupported(dataFormat);
}

bool
wxd_Clipboard_GetData(wxd_Clipboard_t* clipboard, wxd_DataObject_t* data)
{
    if (!clipboard || !data)
        return false;
    wxClipboard* wx_clipboard = reinterpret_cast<wxClipboard*>(clipboard);
    wxDataObject* wx_data = reinterpret_cast<wxDataObject*>(data);
    return wx_clipboard->GetData(*wx_data);
}

void
wxd_Clipboard_Clear(wxd_Clipboard_t* clipboard)
{
    if (!clipboard)
        return;
    wxClipboard* wx_clipboard = reinterpret_cast<wxClipboard*>(clipboard);
    wx_clipboard->Clear();
}

bool
wxd_Clipboard_Flush(wxd_Clipboard_t* clipboard)
{
    if (!clipboard)
        return false;
    wxClipboard* wx_clipboard = reinterpret_cast<wxClipboard*>(clipboard);
    return wx_clipboard->Flush();
}

void
wxd_Clipboard_UsePrimarySelection(wxd_Clipboard_t* clipboard, bool use_primary)
{
    if (!clipboard)
        return;
    wxClipboard* wx_clipboard = reinterpret_cast<wxClipboard*>(clipboard);
    wx_clipboard->UsePrimarySelection(use_primary);
}

// Convenience functions
bool
wxd_Clipboard_SetText(wxd_Clipboard_t* clipboard, const char* text)
{
    if (!clipboard || !text)
        return false;
    wxClipboard* wx_clipboard = reinterpret_cast<wxClipboard*>(clipboard);

    if (!wx_clipboard->Open()) {
        return false;
    }

    wxTextDataObject* data = new wxTextDataObject(wxString::FromUTF8(text));
    bool success = wx_clipboard->SetData(data);
    wx_clipboard->Close();

    return success;
}

WXD_EXPORTED int
wxd_Clipboard_GetText(const wxd_Clipboard_t* clipboard, char* buffer, size_t buffer_len)
{
    if (!clipboard)
        return -1;
    wxClipboard* cb = const_cast<wxClipboard*>(reinterpret_cast<const wxClipboard*>(clipboard));

    if (!cb->Open()) {
        return -1;
    }

    int len = -1;
    if (cb->IsSupported(wxDF_TEXT)) {
        wxTextDataObject data;
        if (cb->GetData(data)) {
            wxString text = data.GetText();
            len = (int)wxd_cpp_utils::copy_wxstring_to_buffer(text, buffer, buffer_len);
        }
    }

    cb->Close();
    return len;
}

} // extern "C"