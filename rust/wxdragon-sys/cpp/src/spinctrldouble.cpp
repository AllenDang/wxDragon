#include <wx/wxprec.h>
#include <wx/wx.h>
#include "../include/wxdragon.h"
#include <wx/spinctrl.h> // For wxSpinCtrlDouble, wxSP_ARROW_KEYS, wxSP_WRAP
#include <wx/string.h>   // For wxString
#include <wx/gdicmn.h>   // For wxPoint, wxSize
#include <wx/window.h>   // For wxWindow parent type

WXD_EXPORTED wxd_SpinCtrlDouble_t*
wxd_SpinCtrlDouble_Create(wxd_Window_t* parent, int id, const char* value_str, int x, int y, int w,
                          int h, int64_t style, double min_val, double max_val, double initial_val,
                          double inc)
{
    wxWindow* p = (wxWindow*)parent;
    wxPoint pos = (x == -1 && y == -1) ? wxDefaultPosition : wxPoint(x, y);
    wxSize size = (w == -1 && h == -1) ? wxDefaultSize : wxSize(w, h);
    wxString wx_value_str = wxString::FromUTF8(value_str ? value_str : "");

    // wxSpinCtrlDouble constructor doesn't take an initial string value if initial_val (double) is provided.
    // It uses the double initial_val. The string value is for display and text entry.
    // We can set the string value after creation if needed, or rely on initial_val formatting.
    // Default style wxSP_ARROW_KEYS is usually implied by wxSpinCtrl/wxSpinCtrlDouble.
    // wxSP_WRAP is a common additional style.
    int64_t actual_style = style;
    if (style == 0) { // If no style provided, default to arrow keys.
        actual_style = wxSP_ARROW_KEYS;
    } // User can pass wxSP_ARROW_KEYS | wxSP_WRAP etc.

    wxSpinCtrlDouble* ctrl = new wxSpinCtrlDouble(p, id, wx_value_str, pos, size, actual_style,
                                                  min_val, max_val, initial_val, inc);
    // The constructor takes initial_val (double), and value (wxString).
    // If value string is empty, it uses initial_val. If value string is not empty, it parses it.
    // To be safe, if the passed value_str is meant to override initial_val, one might call SetValue explicitly after creation.
    // However, the constructor wxSpinCtrlDouble(parent, id, value_wxString, pos, size, style, min, max, initial_double, inc_double)
    // should handle this: if value_wxString is empty, initial_double is used. Otherwise, value_wxString is parsed.
    // We will rely on this wxWidgets behavior.

    return (wxd_SpinCtrlDouble_t*)ctrl;
}

WXD_EXPORTED double
wxd_SpinCtrlDouble_GetValue(wxd_SpinCtrlDouble_t* self)
{
    wxSpinCtrlDouble* ctrl = (wxSpinCtrlDouble*)self;
    if (!ctrl)
        return 0.0; // Or some other error indicator like NaN if appropriate
    return ctrl->GetValue();
}

WXD_EXPORTED void
wxd_SpinCtrlDouble_SetValue(wxd_SpinCtrlDouble_t* self, double value)
{
    wxSpinCtrlDouble* ctrl = (wxSpinCtrlDouble*)self;
    if (!ctrl)
        return;
    ctrl->SetValue(value);
}

WXD_EXPORTED void
wxd_SpinCtrlDouble_SetRange(wxd_SpinCtrlDouble_t* self, double min_val, double max_val)
{
    wxSpinCtrlDouble* ctrl = (wxSpinCtrlDouble*)self;
    if (!ctrl)
        return;
    ctrl->SetRange(min_val, max_val);
}

WXD_EXPORTED double
wxd_SpinCtrlDouble_GetMin(wxd_SpinCtrlDouble_t* self)
{
    wxSpinCtrlDouble* ctrl = (wxSpinCtrlDouble*)self;
    if (!ctrl)
        return 0.0;
    return ctrl->GetMin();
}

WXD_EXPORTED double
wxd_SpinCtrlDouble_GetMax(wxd_SpinCtrlDouble_t* self)
{
    wxSpinCtrlDouble* ctrl = (wxSpinCtrlDouble*)self;
    if (!ctrl)
        return 0.0;
    return ctrl->GetMax();
}

// Corresponds to wxSpinCtrlDouble::SetIncrement in wxWidgets 3.1+, but API used SetIncrements
// Let's assume wxSpinCtrlDouble::SetIncrement is the intended function for a single double value.
// If wxWidgets version implies SetIncrements (plural for multiple steps), API needs change.
// wxWidgets 3.2.x has SetIncrement(double inc).
WXD_EXPORTED void
wxd_SpinCtrlDouble_SetIncrements(wxd_SpinCtrlDouble_t* self, double inc)
{
    wxSpinCtrlDouble* ctrl = (wxSpinCtrlDouble*)self;
    if (!ctrl)
        return;
    ctrl->SetIncrement(
        inc); // Assuming SetIncrement is the correct wxWidgets method for a single double
}

WXD_EXPORTED double
wxd_SpinCtrlDouble_GetIncrement(wxd_SpinCtrlDouble_t* self)
{
    wxSpinCtrlDouble* ctrl = (wxSpinCtrlDouble*)self;
    if (!ctrl)
        return 0.0;
    return ctrl->GetIncrement();
}

WXD_EXPORTED void
wxd_SpinCtrlDouble_SetDigits(wxd_SpinCtrlDouble_t* self, unsigned int digits)
{
    wxSpinCtrlDouble* ctrl = (wxSpinCtrlDouble*)self;
    if (!ctrl)
        return;
    ctrl->SetDigits(digits);
}

WXD_EXPORTED unsigned int
wxd_SpinCtrlDouble_GetDigits(wxd_SpinCtrlDouble_t* self)
{
    wxSpinCtrlDouble* ctrl = (wxSpinCtrlDouble*)self;
    if (!ctrl)
        return 0; // Default to 0 digits if control is null
    return ctrl->GetDigits();
}