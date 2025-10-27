#include <wx/wxprec.h>
#include <wx/wx.h>
#include "../include/wxdragon.h"
#include <wx/slider.h>

// Helper to convert wxd_Point to wxPoint
inline wxPoint
wxdPointToWxPoint(wxd_Point p)
{
    return wxPoint(p.x, p.y);
}

// Helper to convert wxd_Size to wxSize
inline wxSize
wxdSizeToWxSize(wxd_Size s)
{
    return wxSize(s.width, s.height);
}

// Implementation for wxd_Slider_Create
WXD_EXPORTED wxd_Slider_t*
wxd_Slider_Create(wxd_Window_t* parent, wxd_Id id, int value, int minValue, int maxValue,
                  wxd_Point pos, wxd_Size size, wxd_Style_t style)
{
    wxWindow* parentWin = reinterpret_cast<wxWindow*>(parent);
    wxSlider* slider = new wxSlider(parentWin, id, value, minValue, maxValue,
                                    wxdPointToWxPoint(pos), wxdSizeToWxSize(size), style);
    return reinterpret_cast<wxd_Slider_t*>(slider);
}

// Implementation for wxd_Slider_GetValue
WXD_EXPORTED int
wxd_Slider_GetValue(wxd_Slider_t* self)
{
    wxSlider* slider = reinterpret_cast<wxSlider*>(self);
    if (!slider)
        return 0; // Or some error value?
    return slider->GetValue();
}

// Implementation for wxd_Slider_SetValue
WXD_EXPORTED void
wxd_Slider_SetValue(wxd_Slider_t* self, int value)
{
    wxSlider* slider = reinterpret_cast<wxSlider*>(self);
    if (slider) {
        slider->SetValue(value);
    }
}

// Implementation for wxd_Slider_SetRange
WXD_EXPORTED void
wxd_Slider_SetRange(wxd_Slider_t* self, int minValue, int maxValue)
{
    wxSlider* slider = reinterpret_cast<wxSlider*>(self);
    if (slider) {
        slider->SetRange(minValue, maxValue);
    }
}

// Implementation for wxd_Slider_GetMin
WXD_EXPORTED int
wxd_Slider_GetMin(wxd_Slider_t* self)
{
    wxSlider* slider = reinterpret_cast<wxSlider*>(self);
    if (!slider)
        return 0; // Or error?
    return slider->GetMin();
}

// Implementation for wxd_Slider_GetMax
WXD_EXPORTED int
wxd_Slider_GetMax(wxd_Slider_t* self)
{
    wxSlider* slider = reinterpret_cast<wxSlider*>(self);
    if (!slider)
        return 0; // Or error?
    return slider->GetMax();
}