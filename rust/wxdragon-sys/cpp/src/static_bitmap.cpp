#include <wx/wxprec.h>
#include <wx/wx.h>
#include "../include/wxdragon.h" // Main header
#include <wx/statbmp.h>          // For wxStaticBitmap
#include <wx/bitmap.h>           // For wxBitmap

// Note: No top-level extern "C" here; wxdragon.h handles it.

/**
 * @brief Creates a static bitmap control displaying a wxBitmap.
 *
 * If the provided bitmap is invalid or null, the control will be created with wxNullBitmap.
 * The wxStaticBitmap makes its own copy of the bitmap, so the caller retains ownership
 * of the passed wxd_Bitmap_t, unless it's intended to be consumed.
 */
WXD_EXPORTED wxd_StaticBitmap_t*
wxd_StaticBitmap_CreateWithBitmap(wxd_Window_t* parent, wxd_Id id, const wxd_Bitmap_t* bitmap,
                                  wxd_Point pos, wxd_Size size, wxd_Style_t style, const char* name)
{
    wxWindow* parentWin = reinterpret_cast<wxWindow*>(parent);
    const wxBitmap* bmp = reinterpret_cast<const wxBitmap*>(bitmap);

    if (!parentWin) {
        WXD_LOG_ERROR("wxd_StaticBitmap_CreateWithBitmap: Parent window is null.");
        return nullptr;
    }

    // wxStaticBitmap constructor requires a const wxBitmap&.
    // If bmp is null or not OK, we use wxNullBitmap.
    // wxStaticBitmap makes a copy of the bitmap data.
    const wxBitmap& bitmap_ref = (bmp && bmp->IsOk()) ? *bmp : wxNullBitmap;
    if (!(bmp && bmp->IsOk())) {
        WXD_LOG_WARN(
            "wxd_StaticBitmap_CreateWithBitmap: Bitmap is null or not OK. Creating StaticBitmap with wxNullBitmap.");
    }

    wxStaticBitmap* statBmp = new wxStaticBitmap(parentWin, id, bitmap_ref, wxd_cpp_utils::to_wx(pos),
                                                 wxd_cpp_utils::to_wx(size), style,
                                                 WXD_STR_TO_WX_STRING_UTF8_NULL_OK(name));

    return reinterpret_cast<wxd_StaticBitmap_t*>(statBmp);
}

/**
 * @brief Sets the bitmap for the static bitmap control.
 *
 * The wxStaticBitmap makes its own copy of the bitmap.
 */
WXD_EXPORTED void
wxd_StaticBitmap_SetBitmap(wxd_StaticBitmap_t* self, const wxd_Bitmap_t* bitmap)
{
    // Cast to the common base, not the concrete wxStaticBitmap type: on Windows
    // the object may actually be a wxGenericStaticBitmap (see
    // platform_aware_staticbitmap_handler.cpp), a sibling class that does NOT
    // derive from wxStaticBitmap. Both derive from wxStaticBitmapBase, which
    // declares this method virtual, so dispatch through it instead.
    wxStaticBitmapBase* statBmp = reinterpret_cast<wxStaticBitmapBase*>(self);
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
 * @brief Gets the current bitmap from the static bitmap control.
 *
 * The function creates a new wxBitmap that the caller takes ownership of.
 * The caller is responsible for destroying the bitmap when done with it.
 */
WXD_EXPORTED wxd_Bitmap_t*
wxd_StaticBitmap_GetBitmap(wxd_StaticBitmap_t* self)
{
    // See comment in wxd_StaticBitmap_SetBitmap: must cast to the shared base,
    // not the concrete wxStaticBitmap type.
    wxStaticBitmapBase* statBmp = reinterpret_cast<wxStaticBitmapBase*>(self);
    if (!statBmp)
        return nullptr;

    const wxBitmap& currentBmp = statBmp->GetBitmap();
    if (!currentBmp.IsOk())
        return nullptr;

    // Return a copy, as the internal one might be changed or deleted
    wxBitmap* newBmp = new wxBitmap(currentBmp);
    return reinterpret_cast<wxd_Bitmap_t*>(newBmp);
}

/**
 * @brief Creates a static bitmap control displaying a wxBitmapBundle.
 */
WXD_EXPORTED wxd_StaticBitmap_t*
wxd_StaticBitmap_CreateWithBitmapBundle(wxd_Window_t* parent, wxd_Id id, wxd_BitmapBundle_t* bundle)
{
    wxWindow* parentWin = reinterpret_cast<wxWindow*>(parent);
    wxBitmapBundle* bundlePtr = reinterpret_cast<wxBitmapBundle*>(bundle);

    if (!parentWin) {
        WXD_LOG_ERROR("wxd_StaticBitmap_CreateWithBitmapBundle: Parent window is null.");
        return nullptr;
    }

    wxStaticBitmap* statBmp =
        new wxStaticBitmap(parentWin, id, bundle ? *bundlePtr : wxBitmapBundle());

    return reinterpret_cast<wxd_StaticBitmap_t*>(statBmp);
}

/**
 * @brief Sets the bitmap bundle for the static bitmap control.
 */
WXD_EXPORTED void
wxd_StaticBitmap_SetBitmapBundle(wxd_StaticBitmap_t* self, wxd_BitmapBundle_t* bundle)
{
    // See comment in wxd_StaticBitmap_SetBitmap: must cast to the shared base,
    // not the concrete wxStaticBitmap type.
    wxStaticBitmapBase* statBmp = reinterpret_cast<wxStaticBitmapBase*>(self);
    wxBitmapBundle* bundlePtr = reinterpret_cast<wxBitmapBundle*>(bundle);

    if (!statBmp)
        return;

    statBmp->SetBitmap(bundle ? *bundlePtr : wxBitmapBundle());
}

/**
 * @brief Sets the scale mode for the static bitmap control.
 *
 * The scale mode determines how the bitmap is scaled within the control.
 * Available modes are defined in the WXD_StaticBitmap_Scale_* constants.
 */
WXD_EXPORTED void
wxd_StaticBitmap_SetScaleMode(wxd_StaticBitmap_t* self, int scaleMode)
{
    // See comment in wxd_StaticBitmap_SetBitmap: must cast to the shared base,
    // not the concrete wxStaticBitmap type. ScaleMode itself is declared on
    // wxStaticBitmapBase, so both wxStaticBitmap and wxGenericStaticBitmap
    // share the same enum.
    wxStaticBitmapBase* statBmp = reinterpret_cast<wxStaticBitmapBase*>(self);
    if (!statBmp)
        return;

    wxStaticBitmapBase::ScaleMode mode = static_cast<wxStaticBitmapBase::ScaleMode>(scaleMode);
    statBmp->SetScaleMode(mode);
}

/**
 * @brief Gets the current scale mode of the static bitmap control.
 *
 * Returns the current scale mode as an integer value corresponding to
 * the WXD_StaticBitmap_Scale_* constants.
 */
WXD_EXPORTED int
wxd_StaticBitmap_GetScaleMode(wxd_StaticBitmap_t* self)
{
    // See comment in wxd_StaticBitmap_SetBitmap: must cast to the shared base,
    // not the concrete wxStaticBitmap type.
    wxStaticBitmapBase* statBmp = reinterpret_cast<wxStaticBitmapBase*>(self);
    if (!statBmp)
        return 0; // Default to Scale_None

    return static_cast<int>(statBmp->GetScaleMode());
}