#include <wx/wxprec.h>
#include <wx/wx.h>
#include "../include/wxdragon.h"

// Only compile when XRC is enabled
#if wxdUSE_XRC

#include <wx/xrc/xmlres.h>
#include <wx/xrc/xh_stbmp.h> // Include for XRC_MAKE_INSTANCE macro
#include <wx/xml/xml.h>      // Include for wxXmlNode
#include <wx/statbmp.h>
// Include generic StaticBitmap for Windows (both native and cross-compiled)
#if defined(__WXMSW__) || defined(WXD_TARGET_WINDOWS)
#include <wx/generic/statbmpg.h>
#pragma message( \
    "wxDragon: Compiling with Windows StaticBitmap support (will use wxGenericStaticBitmap)")
#ifdef __WXMSW__
#pragma message("wxDragon: __WXMSW__ is defined (native Windows build)")
#endif
#ifdef WXD_TARGET_WINDOWS
#pragma message("wxDragon: WXD_TARGET_WINDOWS is defined (cross-compilation target)")
#endif
#else
#pragma message("wxDragon: Compiling for non-Windows platform (will use native wxStaticBitmap)")
#endif

/**
 * Custom XRC handler that creates platform-appropriate StaticBitmap widgets:
 * - On Windows: creates wxGenericStaticBitmap for proper scaling support
 * - On other platforms: creates native wxStaticBitmap
 */
class WxdPlatformAwareStaticBitmapHandler : public wxXmlResourceHandler {
public:
    WxdPlatformAwareStaticBitmapHandler()
    {
        // Register all standard window styles (including border styles)
        // This is required for XRC to recognize style flags like wxBORDER_THEME
        AddWindowStyles();
    }

    virtual wxObject*
    DoCreateResource()
    {
        // Get the bitmap - can be from 'bitmap' attribute or child <bitmap> node
        wxBitmap bitmap = GetBitmap(wxT("bitmap"), wxART_OTHER);

#if defined(__WXMSW__) || defined(WXD_TARGET_WINDOWS)
        // On Windows, use wxGenericStaticBitmap for proper scaling
        wxGenericStaticBitmap* control = new wxGenericStaticBitmap(m_parentAsWindow, GetID(),
                                                                   bitmap, GetPosition(), GetSize(),
                                                                   GetStyle(), GetName());

        // Handle scale mode if specified
        wxString scaleMode = GetParamValue(wxT("scalemode"));
        if (!scaleMode.IsEmpty()) {
            if (scaleMode == wxT("None") || scaleMode == wxT("0")) {
                control->SetScaleMode(wxStaticBitmap::Scale_None);
            }
            else if (scaleMode == wxT("Fill") || scaleMode == wxT("1")) {
                control->SetScaleMode(wxStaticBitmap::Scale_Fill);
            }
            else if (scaleMode == wxT("AspectFit") || scaleMode == wxT("2")) {
                control->SetScaleMode(wxStaticBitmap::Scale_AspectFit);
            }
            else if (scaleMode == wxT("AspectFill") || scaleMode == wxT("3")) {
                control->SetScaleMode(wxStaticBitmap::Scale_AspectFill);
            }
        }

        SetupWindow(control);
        return control;
#else
        // On other platforms, use native wxStaticBitmap
        wxStaticBitmap* control = new wxStaticBitmap(m_parentAsWindow, GetID(), bitmap,
                                                     GetPosition(), GetSize(), GetStyle(),
                                                     GetName());

        // Handle scale mode if specified
        wxString scaleMode = GetParamValue(wxT("scalemode"));
        if (!scaleMode.IsEmpty()) {
            if (scaleMode == wxT("None") || scaleMode == wxT("0")) {
                control->SetScaleMode(wxStaticBitmap::Scale_None);
            }
            else if (scaleMode == wxT("Fill") || scaleMode == wxT("1")) {
                control->SetScaleMode(wxStaticBitmap::Scale_Fill);
            }
            else if (scaleMode == wxT("AspectFit") || scaleMode == wxT("2")) {
                control->SetScaleMode(wxStaticBitmap::Scale_AspectFit);
            }
            else if (scaleMode == wxT("AspectFill") || scaleMode == wxT("3")) {
                control->SetScaleMode(wxStaticBitmap::Scale_AspectFill);
            }
        }

        SetupWindow(control);
        return control;
#endif
    }

    virtual bool
    CanHandle(wxXmlNode* node)
    {
        return IsOfClass(node, wxT("wxStaticBitmap"));
    }
};

/**
 * @brief Registers the platform-aware StaticBitmap XRC handler
 * 
 * This function should be called after creating the XmlResource but before loading XRC files.
 * It replaces the default wxStaticBitmap handler with our platform-aware version.
 */
WXD_EXPORTED void
wxd_XmlResource_InitPlatformAwareStaticBitmapHandler(wxd_XmlResource_t* resource)
{
    wxXmlResource* res = reinterpret_cast<wxXmlResource*>(resource);
    if (!res)
        return;

    // Add our custom handler - it should take precedence if registered after standard ones
    res->AddHandler(new WxdPlatformAwareStaticBitmapHandler());
}

#else // wxdUSE_XRC

// Stub implementation when XRC is not enabled
WXD_EXPORTED void
wxd_XmlResource_InitPlatformAwareStaticBitmapHandler(wxd_XmlResource_t* resource)
{
    // Do nothing when XRC is not enabled
    (void)resource; // Suppress unused parameter warning
}

#endif // wxdUSE_XRC