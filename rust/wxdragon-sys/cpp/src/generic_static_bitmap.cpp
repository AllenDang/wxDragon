#include <wx/wxprec.h>
#include <wx/wx.h>
#include "../include/wxdragon.h" // Main header
#include <wx/generic/statbmpg.h> // For wxGenericStaticBitmap
#include <wx/bitmap.h>           // For wxBitmap

// Note: No top-level extern "C" here; wxdragon.h handles it.

// Helper to convert wxd_Point to wxPoint
static inline wxPoint
wxd_to_wx_point_gsb(const wxd_Point& p)
{
    if (p.x == -1 && p.y == -1)
        return wxDefaultPosition;
    return wxPoint(p.x, p.y);
}

// Helper to convert wxd_Size to wxSize
static inline wxSize
wxd_to_wx_size_gsb(const wxd_Size& s)
{
    if (s.width == -1 && s.height == -1)
        return wxDefaultSize;
    return wxSize(s.width, s.height);
}

/**
 * @brief Creates a generic static bitmap control displaying a wxBitmap.
 *
 * If the provided bitmap is invalid or null, the control will be created with wxNullBitmap.
 * The wxGenericStaticBitmap makes its own copy of the bitmap, so the caller retains ownership
 * of the passed wxd_Bitmap_t, unless it's intended to be consumed.
 */
WXD_EXPORTED wxd_GenericStaticBitmap_t*
wxd_GenericStaticBitmap_CreateWithBitmap(wxd_Window_t* parent, wxd_Id id,
                                         const wxd_Bitmap_t* bitmap, wxd_Point pos, wxd_Size size,
                                         wxd_Style_t style, const char* name)
{
    wxWindow* parentWin = reinterpret_cast<wxWindow*>(parent);
    const wxBitmap* bmp = reinterpret_cast<const wxBitmap*>(bitmap);

    if (!parentWin) {
        WXD_LOG_ERROR("wxd_GenericStaticBitmap_CreateWithBitmap: Parent window is null.");
        return nullptr;
    }

    // wxGenericStaticBitmap constructor requires a const wxBitmap&.
    // If bmp is null or not OK, we use wxNullBitmap.
    // wxGenericStaticBitmap makes a copy of the bitmap data.
    const wxBitmap& bitmap_ref = (bmp && bmp->IsOk()) ? *bmp : wxNullBitmap;
    if (!(bmp && bmp->IsOk())) {
        WXD_LOG_WARN(
            "wxd_GenericStaticBitmap_CreateWithBitmap: Bitmap is null or not OK. Creating GenericStaticBitmap with wxNullBitmap.");
    }

    wxGenericStaticBitmap* statBmp =
        new wxGenericStaticBitmap(parentWin, id, bitmap_ref, wxd_to_wx_point_gsb(pos),
                                  wxd_to_wx_size_gsb(size), style,
                                  WXD_STR_TO_WX_STRING_UTF8_NULL_OK(name));

    return reinterpret_cast<wxd_GenericStaticBitmap_t*>(statBmp);
}

/**
 * @brief Sets the bitmap for the generic static bitmap control.
 *
 * The wxGenericStaticBitmap makes its own copy of the bitmap.
 */
WXD_EXPORTED void
wxd_GenericStaticBitmap_SetBitmap(wxd_GenericStaticBitmap_t* self, const wxd_Bitmap_t* bitmap)
{
    wxGenericStaticBitmap* statBmp = reinterpret_cast<wxGenericStaticBitmap*>(self);
    const wxBitmap* bmp = reinterpret_cast<const wxBitmap*>(bitmap);

    if (!statBmp)
        return;

    if (!bmp || !bmp->IsOk()) {
        statBmp->SetBitmap(wxNullBitmap);
    }
    else {
        statBmp->SetBitmap(*bmp);
    }
}

/**
 * @brief Gets the current bitmap from the generic static bitmap control.
 *
 * The function creates a new wxBitmap that the caller takes ownership of.
 * The caller is responsible for destroying the bitmap when done with it.
 */
WXD_EXPORTED wxd_Bitmap_t*
wxd_GenericStaticBitmap_GetBitmap(wxd_GenericStaticBitmap_t* self)
{
    wxGenericStaticBitmap* statBmp = reinterpret_cast<wxGenericStaticBitmap*>(self);
    if (!statBmp)
        return nullptr;

    const wxBitmap& currentBmp = statBmp->GetBitmap();
    if (!currentBmp.IsOk())
        return nullptr;

    // Return a copy, as the internal one might be changed or deleted
    wxBitmap* newBmp = new (std::nothrow) wxBitmap(currentBmp);
    return reinterpret_cast<wxd_Bitmap_t*>(newBmp);
}

/**
 * @brief Creates a generic static bitmap control displaying a wxBitmapBundle.
 */
WXD_EXPORTED wxd_GenericStaticBitmap_t*
wxd_GenericStaticBitmap_CreateWithBitmapBundle(wxd_Window_t* parent, wxd_Id id,
                                               wxd_BitmapBundle_t* bundle)
{
    wxWindow* parentWin = reinterpret_cast<wxWindow*>(parent);
    wxBitmapBundle* bundlePtr = reinterpret_cast<wxBitmapBundle*>(bundle);

    if (!parentWin) {
        WXD_LOG_ERROR("wxd_GenericStaticBitmap_CreateWithBitmapBundle: Parent window is null.");
        return nullptr;
    }

    wxGenericStaticBitmap* statBmp =
        new wxGenericStaticBitmap(parentWin, id, bundle ? *bundlePtr : wxBitmapBundle());

    return reinterpret_cast<wxd_GenericStaticBitmap_t*>(statBmp);
}

/**
 * @brief Sets the bitmap bundle for the generic static bitmap control.
 */
WXD_EXPORTED void
wxd_GenericStaticBitmap_SetBitmapBundle(wxd_GenericStaticBitmap_t* self, wxd_BitmapBundle_t* bundle)
{
    wxGenericStaticBitmap* statBmp = reinterpret_cast<wxGenericStaticBitmap*>(self);
    wxBitmapBundle* bundlePtr = reinterpret_cast<wxBitmapBundle*>(bundle);

    if (!statBmp)
        return;

    statBmp->SetBitmap(bundle ? *bundlePtr : wxBitmapBundle());
}

/**
 * @brief Sets the scale mode for the generic static bitmap control.
 *
 * The scale mode determines how the bitmap is scaled within the control.
 * Available modes are defined in the WXD_StaticBitmap_Scale_* constants.
 */
WXD_EXPORTED void
wxd_GenericStaticBitmap_SetScaleMode(wxd_GenericStaticBitmap_t* self, int scaleMode)
{
    wxGenericStaticBitmap* statBmp = reinterpret_cast<wxGenericStaticBitmap*>(self);
    if (!statBmp)
        return;

    wxStaticBitmap::ScaleMode mode = static_cast<wxStaticBitmap::ScaleMode>(scaleMode);
    statBmp->SetScaleMode(mode);
}

/**
 * @brief Gets the current scale mode of the generic static bitmap control.
 *
 * Returns the current scale mode as an integer value corresponding to
 * the WXD_StaticBitmap_Scale_* constants.
 */
WXD_EXPORTED int
wxd_GenericStaticBitmap_GetScaleMode(wxd_GenericStaticBitmap_t* self)
{
    wxGenericStaticBitmap* statBmp = reinterpret_cast<wxGenericStaticBitmap*>(self);
    if (!statBmp)
        return 0; // Default to Scale_None

    return static_cast<int>(statBmp->GetScaleMode());
}