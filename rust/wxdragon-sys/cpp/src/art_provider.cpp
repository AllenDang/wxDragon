#include <wx/wxprec.h>
#include <wx/wx.h>
#include "../include/wxdragon.h"
#include <wx/artprov.h>
#include <wx/bitmap.h>
#include <wx/bmpbndl.h>

// We add extern "C" for all the function definitions to make sure they use C linkage
extern "C" {

WXD_EXPORTED wxd_Bitmap_t*
wxd_ArtProvider_GetBitmap(const char* id, const char* client, wxd_Size size_req)
{
    wxString wxId = wxString::FromUTF8(id);
    wxString wxClient = wxString::FromUTF8(client);
    if (wxClient.IsEmpty()) {
        // wxWidgets uses wxART_OTHER if client is empty string, but being explicit is fine.
        // However, wxArtProvider::GetBitmap takes wxArtClient which is also a wxString.
        // An empty wxString is the correct way to represent wxART_OTHER if no specific client is given.
    }

    wxSize wxSizeReq(size_req.width, size_req.height);

    // wxArtProvider::GetBitmap directly returns wxBitmap
    // For more modern approach, wxArtProvider::GetBitmapBundle then bundle.GetBitmap(size) is used.
    // Let's use the direct GetBitmap for simplicity matching the older API style if GetBitmapBundle isn't straightforward here.
    // wxArtProvider::GetBitmap is static.
    wxBitmap bitmap = wxArtProvider::GetBitmap(wxId, wxClient, wxSizeReq);

    if (!bitmap.IsOk()) {
        return nullptr;
    }

    // The returned bitmap from wxArtProvider is a new copy or a ref-counted shared instance.
    // We need to return a heap-allocated wxBitmap for consistency with other Create functions.
    wxBitmap* new_bitmap = new wxBitmap(bitmap);
    return reinterpret_cast<wxd_Bitmap_t*>(new_bitmap);
}

// Get a BitmapBundle with all available sizes for an art ID
WXD_EXPORTED wxd_BitmapBundle_t*
wxd_ArtProvider_GetBitmapBundle(const char* id, const char* client, wxd_Size size_req)
{
    wxString wxId = wxString::FromUTF8(id);
    wxString wxClient = wxString::FromUTF8(client);
    wxSize wxSizeReq(size_req.width, size_req.height);

    // wxArtProvider::GetBitmapBundle returns a wxBitmapBundle
    wxBitmapBundle bundle = wxArtProvider::GetBitmapBundle(wxId, wxClient, wxSizeReq);

    if (!bundle.IsOk()) {
        return nullptr;
    }

    // Create a heap-allocated bundle to return
    wxBitmapBundle* new_bundle = new wxBitmapBundle(bundle);
    return reinterpret_cast<wxd_BitmapBundle_t*>(new_bundle);
}

// Get the suitable size hint for a client in device-independent pixels
WXD_EXPORTED wxd_Size
wxd_ArtProvider_GetDIPSizeHint(const char* client)
{
    wxString wxClient = wxString::FromUTF8(client);
    wxSize wx_size = wxArtProvider::GetDIPSizeHint(wxClient);

    wxd_Size size;
    size.width = wx_size.GetWidth();
    size.height = wx_size.GetHeight();

    return size;
}

// Get the suitable size hint for a client (scaled for a window)
WXD_EXPORTED wxd_Size
wxd_ArtProvider_GetSizeHint(const char* client, wxd_Window_t* window)
{
    wxString wxClient = wxString::FromUTF8(client);
    wxWindow* wx_window = window ? reinterpret_cast<wxWindow*>(window) : nullptr;

    wxSize wx_size = wxArtProvider::GetSizeHint(wxClient, wx_window);

    wxd_Size size;
    size.width = wx_size.GetWidth();
    size.height = wx_size.GetHeight();

    return size;
}

// Get the native size hint for a client in device-independent pixels
WXD_EXPORTED wxd_Size
wxd_ArtProvider_GetNativeDIPSizeHint(const char* client)
{
    wxString wxClient = wxString::FromUTF8(client);
    wxSize wx_size = wxArtProvider::GetNativeDIPSizeHint(wxClient);

    wxd_Size size;
    size.width = wx_size.GetWidth();
    size.height = wx_size.GetHeight();

    return size;
}

// Get the native size hint for a client (scaled for a window)
WXD_EXPORTED wxd_Size
wxd_ArtProvider_GetNativeSizeHint(const char* client, wxd_Window_t* window)
{
    wxString wxClient = wxString::FromUTF8(client);
    wxWindow* wx_window = window ? reinterpret_cast<wxWindow*>(window) : nullptr;

    wxSize wx_size = wxArtProvider::GetNativeSizeHint(wxClient, wx_window);

    wxd_Size size;
    size.width = wx_size.GetWidth();
    size.height = wx_size.GetHeight();

    return size;
}

// Check if the platform has a native provider
WXD_EXPORTED bool
wxd_ArtProvider_HasNativeProvider(void)
{
    return wxArtProvider::HasNativeProvider();
}

} // extern "C"