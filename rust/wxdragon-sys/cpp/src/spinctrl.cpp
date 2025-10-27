#include <wx/wxprec.h>
#include <wx/wx.h>
#include "../include/wxdragon.h"
#include <wx/spinctrl.h>

// Helper to convert wxd_Point to wxPoint (assuming it exists in another .cpp or is standard)
// inline wxPoint wxdPointToWxPoint(wxd_Point p) { return wxPoint(p.x, p.y); }

// Helper to convert wxd_Size to wxSize (assuming it exists in another .cpp or is standard)
// inline wxSize wxdSizeToWxSize(wxd_Size s) { return wxSize(s.width, s.height); }

// Implementation for wxd_SpinCtrl_Create
WXD_EXPORTED wxd_SpinCtrl_t*
wxd_SpinCtrl_Create(wxd_Window_t* parent, wxd_Id id,
                    const char* value, // Initial value as string
                    wxd_Point pos, wxd_Size size, wxd_Style_t style,
                    int min_val, // Explicit min/max required for spinctrl
                    int max_val, int initial_val)
{
    wxWindow* parentWin = reinterpret_cast<wxWindow*>(parent);
    // wxSpinCtrl constructor requires min/max/initial values
    wxSpinCtrl* spinCtrl = new wxSpinCtrl(
        parentWin, id,
        wxString::FromUTF8(value ? value : ""), // Use initial string value if provided
        wxPoint(pos.x, pos.y),                  // Use helpers if available
        wxSize(size.width, size.height),        // Use helpers if available
        style, min_val, max_val,
        initial_val // Set initial numeric value
    );
    return reinterpret_cast<wxd_SpinCtrl_t*>(spinCtrl);
}

// Implementation for wxd_SpinCtrl_GetValue
WXD_EXPORTED int
wxd_SpinCtrl_GetValue(wxd_SpinCtrl_t* self)
{
    wxSpinCtrl* spinCtrl = reinterpret_cast<wxSpinCtrl*>(self);
    if (!spinCtrl)
        return 0; // Or some error indicator?
    return spinCtrl->GetValue();
}

// Implementation for wxd_SpinCtrl_SetValue
WXD_EXPORTED void
wxd_SpinCtrl_SetValue(wxd_SpinCtrl_t* self, int value)
{
    wxSpinCtrl* spinCtrl = reinterpret_cast<wxSpinCtrl*>(self);
    if (spinCtrl) {
        spinCtrl->SetValue(value);
    }
}

// Implementation for wxd_SpinCtrl_SetRange
WXD_EXPORTED void
wxd_SpinCtrl_SetRange(wxd_SpinCtrl_t* self, int minVal, int maxVal)
{
    wxSpinCtrl* spinCtrl = reinterpret_cast<wxSpinCtrl*>(self);
    if (spinCtrl) {
        spinCtrl->SetRange(minVal, maxVal);
    }
}

// Implementation for wxd_SpinCtrl_GetMin
WXD_EXPORTED int
wxd_SpinCtrl_GetMin(wxd_SpinCtrl_t* self)
{
    wxSpinCtrl* spinCtrl = reinterpret_cast<wxSpinCtrl*>(self);
    if (!spinCtrl)
        return 0; // Or error?
    return spinCtrl->GetMin();
}

// Implementation for wxd_SpinCtrl_GetMax
WXD_EXPORTED int
wxd_SpinCtrl_GetMax(wxd_SpinCtrl_t* self)
{
    wxSpinCtrl* spinCtrl = reinterpret_cast<wxSpinCtrl*>(self);
    if (!spinCtrl)
        return 0; // Or error?
    return spinCtrl->GetMax();
}