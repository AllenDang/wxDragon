#include <wx/wxprec.h>
#include <wx/wx.h>
#include "../include/wxdragon.h"
#include "wxd_utils.h" // For wxd_cpp_utils::to_wxPoint, to_wxSize if used, and string conversions.
#include <wx/clrpicker.h> // wxColourPickerCtrl
#include <wx/colour.h>    // wxColour

// Helper to convert wxd_Colour_t to wxColour
static inline wxColour
to_wx(wxd_Colour_t c_col)
{
    return wxColour(c_col.r, c_col.g, c_col.b, c_col.a);
}

// Helper to convert wxColour to wxd_Colour_t
static inline wxd_Colour_t
to_wxd(wxColour wx_col)
{
    wxd_Colour_t c_col;
    c_col.r = wx_col.Red();
    c_col.g = wx_col.Green();
    c_col.b = wx_col.Blue();
    c_col.a = wx_col.Alpha();
    return c_col;
}

extern "C" {

WXD_EXPORTED wxd_ColourPickerCtrl_t*
wxd_ColourPickerCtrl_Create(wxd_Window_t* parent, wxd_Id id, wxd_Colour_t initial_colour,
                            wxd_Point pos, wxd_Size size, wxd_Style_t style)
{
    wxWindow* parentWin = (wxWindow*)parent;
    // It's possible for parentWin to be null if it's a top-level window,
    // but wxColourPickerCtrl usually requires a parent.
    // For safety, one might add a null check, but wxWidgets might handle it.
    // Let's assume parent is valid as per typical wx usage.

    wxColourPickerCtrl* ctrl = new wxColourPickerCtrl(parentWin, id, to_wx(initial_colour),
                                                      wxd_cpp_utils::to_wx(pos),
                                                      wxd_cpp_utils::to_wx(size), style);
    return (wxd_ColourPickerCtrl_t*)ctrl;
}

WXD_EXPORTED wxd_Colour_t
wxd_ColourPickerCtrl_GetColour(wxd_ColourPickerCtrl_t* self)
{
    wxColourPickerCtrl* ctrl = (wxColourPickerCtrl*)self;
    if (!ctrl) {
        // Return a default/invalid colour, e.g., black transparent or fully transparent white
        return { 0, 0, 0, 0 };
    }
    return to_wxd(ctrl->GetColour());
}

WXD_EXPORTED void
wxd_ColourPickerCtrl_SetColour(wxd_ColourPickerCtrl_t* self, wxd_Colour_t colour)
{
    wxColourPickerCtrl* ctrl = (wxColourPickerCtrl*)self;
    if (ctrl) {
        ctrl->SetColour(to_wx(colour));
    }
}

// Event Data Accessor
WXD_EXPORTED wxd_Colour_t
wxd_ColourPickerEvent_GetColour(wxd_Event_t* event_ptr)
{
    if (!event_ptr) {
        return { 0, 0, 0, 0 }; // Default/invalid
    }
    // Cast the opaque wxd_Event_t* (which is like void* here) to wxEvent* using reinterpret_cast,
    // then static_cast to wxColourPickerEvent*.
    wxEvent* base_event = reinterpret_cast<wxEvent*>(event_ptr);
    wxColourPickerEvent* colour_event = static_cast<wxColourPickerEvent*>(base_event);

    if (!colour_event) {
        // This could happen if base_event was not actually a wxColourPickerEvent.
        // Proper error handling or logging would be good here.
        return { 0, 0, 0, 0 };
    }
    return to_wxd(colour_event->GetColour());
}

// Destroy is handled by parent window destroying its children.
// No explicit wxd_ColourPickerCtrl_Destroy is typically needed.

} // extern "C"