#include <wx/wxprec.h>
#include <wx/wx.h>
#include "../include/wxdragon.h"
#include "wxd_utils.h" // For wxd_cpp_utils::to_wxPoint, to_wxSize if used, and string conversions.
#include <wx/colordlg.h> // wxColourDialog
#include <wx/colour.h>   // wxColour

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

// --- ColourData Implementation ---

wxd_ColourData_t*
wxd_ColourData_Create(void)
{
    wxColourData* data = new wxColourData();
    return reinterpret_cast<wxd_ColourData_t*>(data);
}

void
wxd_ColourData_SetColour(wxd_ColourData_t* self, wxd_Colour_t colour)
{
    if (!self)
        return;
    wxColourData* data = reinterpret_cast<wxColourData*>(self);
    data->SetColour(to_wx(colour));
}

wxd_Colour_t
wxd_ColourData_GetColour(wxd_ColourData_t* self)
{
    if (!self) {
        // Return a default/invalid colour if null pointer
        return { 0, 0, 0, 0 };
    }
    wxColourData* data = reinterpret_cast<wxColourData*>(self);
    return to_wxd(data->GetColour());
}

void
wxd_ColourData_Destroy(wxd_ColourData_t* self)
{
    if (!self)
        return;
    wxColourData* data = reinterpret_cast<wxColourData*>(self);
    delete data;
}

// --- ColourDialog Implementation ---

wxd_ColourDialog_t*
wxd_ColourDialog_Create(wxd_Window_t* parent, const char* title, wxd_ColourData_t* data)
{
    wxWindow* parentWin = reinterpret_cast<wxWindow*>(parent);
    wxColourData* colourData = nullptr;

    if (data) {
        colourData = reinterpret_cast<wxColourData*>(data);
    }
    else {
        // If no data provided, create a default one
        colourData = new wxColourData();
    }

    wxColourDialog* dialog = new wxColourDialog(parentWin, colourData);

    // Set title if provided
    if (title && *title) {
        dialog->SetTitle(WXD_STR_TO_WX_STRING_UTF8_NULL_OK(title));
    }

    return reinterpret_cast<wxd_ColourDialog_t*>(dialog);
}

wxd_ColourData_t*
wxd_ColourDialog_GetColourData(wxd_ColourDialog_t* self)
{
    if (!self)
        return nullptr;

    wxColourDialog* dialog = reinterpret_cast<wxColourDialog*>(self);
    // Note: This returns a reference to the internal wxColourData, not a new instance
    // The pointer will be valid as long as the dialog exists
    return reinterpret_cast<wxd_ColourData_t*>(&dialog->GetColourData());
}

} // extern "C"