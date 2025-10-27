#include <wx/wxprec.h>
#include <wx/wx.h>
#include "wxdragon.h"
#include <wx/xrc/xmlres.h>
#include <wx/xml/xml.h>
#include <wx/stream.h>
#include <wx/mstream.h>
#include <wx/dialog.h>
#include <wx/frame.h>
#include <wx/panel.h>
#include <wx/filename.h>
#include <wx/file.h>

// Get the global wxXmlResource instance
extern "C" WXD_EXPORTED wxd_XmlResource_t*
wxd_XmlResource_Get(void)
{
    return reinterpret_cast<wxd_XmlResource_t*>(wxXmlResource::Get());
}

// Initialize all standard handlers
extern "C" WXD_EXPORTED void
wxd_XmlResource_InitAllHandlers(wxd_XmlResource_t* self)
{
    wxXmlResource* resource = reinterpret_cast<wxXmlResource*>(self);
    if (resource) {
        resource->InitAllHandlers();
    }
}

// Load XRC from file
extern "C" WXD_EXPORTED bool
wxd_XmlResource_LoadFromFile(wxd_XmlResource_t* self, const char* filemask)
{
    if (!self || !filemask)
        return false;

    wxXmlResource* resource = reinterpret_cast<wxXmlResource*>(self);
    wxString filename = wxString::FromUTF8(filemask);
    return resource->Load(filename);
}

// Load XRC from string data
extern "C" WXD_EXPORTED bool
wxd_XmlResource_LoadFromString(wxd_XmlResource_t* self, const char* xrc_data)
{
    if (!self || !xrc_data)
        return false;

    wxXmlResource* resource = reinterpret_cast<wxXmlResource*>(self);

    // Create a memory input stream from the string data
    wxString xmlString = wxString::FromUTF8(xrc_data);
    wxCharBuffer utf8Buffer = xmlString.mb_str(wxConvUTF8);
    wxMemoryInputStream stream(utf8Buffer.data(), utf8Buffer.length());

    // Create an XML document and load from the stream
    wxXmlDocument* doc = new wxXmlDocument();
    if (!doc->Load(stream)) {
        delete doc;
        return false;
    }

    // Use LoadDocument to load the XML document into the resource
    // This takes ownership of the document and handles proper lifecycle
    return resource->LoadDocument(doc, wxT("embedded_xrc"));
}

// Load a dialog from XRC
extern "C" WXD_EXPORTED wxd_Dialog_t*
wxd_XmlResource_LoadDialog(wxd_XmlResource_t* self, wxd_Window_t* parent, const char* name)
{
    if (!self || !name)
        return nullptr;

    wxXmlResource* resource = reinterpret_cast<wxXmlResource*>(self);
    wxWindow* parentWindow = reinterpret_cast<wxWindow*>(parent);
    wxString dialogName = wxString::FromUTF8(name);

    wxDialog* dialog = resource->LoadDialog(parentWindow, dialogName);
    return reinterpret_cast<wxd_Dialog_t*>(dialog);
}

// Load a frame from XRC
extern "C" WXD_EXPORTED wxd_Frame_t*
wxd_XmlResource_LoadFrame(wxd_XmlResource_t* self, wxd_Window_t* parent, const char* name)
{
    if (!self || !name)
        return nullptr;

    wxXmlResource* resource = reinterpret_cast<wxXmlResource*>(self);
    wxWindow* parentWindow = reinterpret_cast<wxWindow*>(parent);
    wxString frameName = wxString::FromUTF8(name);

    wxFrame* frame = resource->LoadFrame(parentWindow, frameName);
    return reinterpret_cast<wxd_Frame_t*>(frame);
}

// Load a panel from XRC
extern "C" WXD_EXPORTED wxd_Panel_t*
wxd_XmlResource_LoadPanel(wxd_XmlResource_t* self, wxd_Window_t* parent, const char* name)
{
    if (!self || !name)
        return nullptr;

    wxXmlResource* resource = reinterpret_cast<wxXmlResource*>(self);
    wxWindow* parentWindow = reinterpret_cast<wxWindow*>(parent);
    wxString panelName = wxString::FromUTF8(name);

    wxPanel* panel = resource->LoadPanel(parentWindow, panelName);
    return reinterpret_cast<wxd_Panel_t*>(panel);
}

// Load a menubar from XRC
extern "C" WXD_EXPORTED wxd_MenuBar_t*
wxd_XmlResource_LoadMenuBar(wxd_XmlResource_t* self, wxd_Window_t* parent, const char* name)
{
    if (!self || !name)
        return nullptr;

    wxXmlResource* resource = reinterpret_cast<wxXmlResource*>(self);
    wxWindow* parentWindow = reinterpret_cast<wxWindow*>(parent); // parent can be null for menubars
    wxString menubarName = wxString::FromUTF8(name);

    wxMenuBar* menubar = resource->LoadMenuBar(parentWindow, menubarName);
    return reinterpret_cast<wxd_MenuBar_t*>(menubar);
}

// Load a generic object from XRC
extern "C" WXD_EXPORTED wxd_Window_t*
wxd_XmlResource_LoadObject(wxd_XmlResource_t* self, wxd_Window_t* parent, const char* name,
                           const char* classname)
{
    if (!self || !name || !classname)
        return nullptr;

    wxXmlResource* resource = reinterpret_cast<wxXmlResource*>(self);
    wxWindow* parentWindow = reinterpret_cast<wxWindow*>(parent); // parent can be null
    wxString wxName = wxString::FromUTF8(name);
    wxString wxClassname = wxString::FromUTF8(classname);

    wxObject* object = resource->LoadObject(parentWindow, wxName, wxClassname);
    if (object && object->IsKindOf(wxCLASSINFO(wxWindow))) {
        return reinterpret_cast<wxd_Window_t*>(dynamic_cast<wxWindow*>(object));
    }
    return nullptr;
}

// Get XRC ID for a control name
extern "C" WXD_EXPORTED int
wxd_XmlResource_GetXRCID(const char* str_id)
{
    if (!str_id)
        return wxID_NONE;

    wxString name = wxString::FromUTF8(str_id);
    return wxXmlResource::GetXRCID(name);
}

// Find a window by XRC name
extern "C" WXD_EXPORTED wxd_Window_t*
wxd_Window_FindWindowByXRCName(wxd_Window_t* self, const char* name)
{
    if (!self || !name)
        return nullptr;

    wxWindow* window = reinterpret_cast<wxWindow*>(self);
    wxString windowName = wxString::FromUTF8(name);

    // Use FindWindow to find child by name
    wxWindow* child = window->FindWindow(windowName);
    return reinterpret_cast<wxd_Window_t*>(child);
}