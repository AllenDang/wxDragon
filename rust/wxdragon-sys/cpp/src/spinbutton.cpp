#include <wx/wxprec.h>
#include <wx/wx.h>
#include "../include/wxdragon.h"
#include <wx/spinbutt.h>

// Assume helpers wxdPointToWxPoint and wxdSizeToWxSize are available

// Implementation for wxd_SpinButton_Create
WXD_EXPORTED wxd_SpinButton_t*
wxd_SpinButton_Create(wxd_Window_t* parent, wxd_Id id, wxd_Point pos, wxd_Size size,
                      wxd_Style_t style)
{
    wxWindow* parentWin = reinterpret_cast<wxWindow*>(parent);
    wxSpinButton* spinButton = new wxSpinButton(parentWin, id, wxPoint(pos.x, pos.y),
                                                wxSize(size.width, size.height), style);
    // Default range is 0-100
    return reinterpret_cast<wxd_SpinButton_t*>(spinButton);
}

// Implementation for wxd_SpinButton_GetValue
WXD_EXPORTED int
wxd_SpinButton_GetValue(wxd_SpinButton_t* self)
{
    wxSpinButton* spinButton = reinterpret_cast<wxSpinButton*>(self);
    if (!spinButton)
        return 0;
    return spinButton->GetValue();
}

// Implementation for wxd_SpinButton_SetValue
WXD_EXPORTED void
wxd_SpinButton_SetValue(wxd_SpinButton_t* self, int value)
{
    wxSpinButton* spinButton = reinterpret_cast<wxSpinButton*>(self);
    if (spinButton) {
        spinButton->SetValue(value);
    }
}

// Implementation for wxd_SpinButton_SetRange
WXD_EXPORTED void
wxd_SpinButton_SetRange(wxd_SpinButton_t* self, int minVal, int maxVal)
{
    wxSpinButton* spinButton = reinterpret_cast<wxSpinButton*>(self);
    if (spinButton) {
        spinButton->SetRange(minVal, maxVal);
    }
}

// Implementation for wxd_SpinButton_GetMin
WXD_EXPORTED int
wxd_SpinButton_GetMin(wxd_SpinButton_t* self)
{
    wxSpinButton* spinButton = reinterpret_cast<wxSpinButton*>(self);
    if (!spinButton)
        return 0;
    return spinButton->GetMin();
}

// Implementation for wxd_SpinButton_GetMax
WXD_EXPORTED int
wxd_SpinButton_GetMax(wxd_SpinButton_t* self)
{
    wxSpinButton* spinButton = reinterpret_cast<wxSpinButton*>(self);
    if (!spinButton)
        return 0;
    return spinButton->GetMax();
}