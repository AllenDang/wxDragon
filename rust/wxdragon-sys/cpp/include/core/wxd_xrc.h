#ifndef WXD_XRC_H
#define WXD_XRC_H

#include "../wxd_types.h"

#ifdef __cplusplus
extern "C" {
#endif

// Get the global wxXmlResource instance
WXD_EXPORTED wxd_XmlResource_t*
wxd_XmlResource_Get(void);

// Initialize all standard handlers
WXD_EXPORTED void
wxd_XmlResource_InitAllHandlers(wxd_XmlResource_t* self);

// Load XRC from file
WXD_EXPORTED bool
wxd_XmlResource_LoadFromFile(wxd_XmlResource_t* self, const char* filemask);

// Load XRC from string data
WXD_EXPORTED bool
wxd_XmlResource_LoadFromString(wxd_XmlResource_t* self, const char* xrc_data);

// Load a dialog from XRC
WXD_EXPORTED wxd_Dialog_t*
wxd_XmlResource_LoadDialog(wxd_XmlResource_t* self, wxd_Window_t* parent, const char* name);

// Load a frame from XRC
WXD_EXPORTED wxd_Frame_t*
wxd_XmlResource_LoadFrame(wxd_XmlResource_t* self, wxd_Window_t* parent, const char* name);

// Load a panel from XRC
WXD_EXPORTED wxd_Panel_t*
wxd_XmlResource_LoadPanel(wxd_XmlResource_t* self, wxd_Window_t* parent, const char* name);

// Load a menubar from XRC
WXD_EXPORTED wxd_MenuBar_t*
wxd_XmlResource_LoadMenuBar(wxd_XmlResource_t* self, wxd_Window_t* parent, const char* name);

// Load a generic object from XRC
WXD_EXPORTED wxd_Window_t*
wxd_XmlResource_LoadObject(wxd_XmlResource_t* self, wxd_Window_t* parent, const char* name,
                           const char* classname);

// Get XRC ID for a named control
WXD_EXPORTED int
wxd_XmlResource_GetXRCID(const char* name);

// Find a child window by XRC name
WXD_EXPORTED wxd_Window_t*
wxd_Window_FindWindowByXRCName(wxd_Window_t* parent, const char* xrc_name);

// Initialize platform-aware StaticBitmap handler for XRC files
WXD_EXPORTED void
wxd_XmlResource_InitPlatformAwareStaticBitmapHandler(wxd_XmlResource_t* resource);

#ifdef __cplusplus
}
#endif

#endif // WXD_XRC_H