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

WXD_EXPORTED void
wxd_Slider_UseNaturalKeys(wxd_Slider_t* self)
{
    wxSlider* slider = reinterpret_cast<wxSlider*>(self);
    if (!slider)
        return;
    slider->Bind(wxEVT_KEY_DOWN, [slider](wxKeyEvent& event) {
        // Shortcuts such as Alt+Down belong to whoever bound them, not to the slider.
        if (event.HasAnyModifiers()) {
            event.Skip();
            return;
        }
        const int min = slider->GetMin();
        const int max = slider->GetMax();
        const int value = slider->GetValue();
        // The native page is a fifth of the range on Windows; a tenth is the same everywhere and finer.
        const int page = wxMax((max - min) / 10, 1);
        int target;
        switch (event.GetKeyCode()) {
        case WXK_UP:
        case WXK_RIGHT:
        case WXK_NUMPAD_UP:
        case WXK_NUMPAD_RIGHT:
            target = value + 1;
            break;
        case WXK_DOWN:
        case WXK_LEFT:
        case WXK_NUMPAD_DOWN:
        case WXK_NUMPAD_LEFT:
            target = value - 1;
            break;
        case WXK_PAGEUP:
        case WXK_NUMPAD_PAGEUP:
            target = value + page;
            break;
        case WXK_PAGEDOWN:
        case WXK_NUMPAD_PAGEDOWN:
            target = value - page;
            break;
        case WXK_HOME:
        case WXK_NUMPAD_HOME:
            target = min;
            break;
        case WXK_END:
        case WXK_NUMPAD_END:
            target = max;
            break;
        default:
            event.Skip();
            return;
        }
        target = wxMin(wxMax(target, min), max);
        if (target == value)
            return;
        slider->SetValue(target);
        // SetValue sends nothing, so send what dragging the thumb would, for on_slider and scroll handlers.
        wxScrollEvent scroll(wxEVT_SCROLL_CHANGED, slider->GetId(), target,
                             slider->HasFlag(wxSL_VERTICAL) ? wxVERTICAL : wxHORIZONTAL);
        scroll.SetEventObject(slider);
        slider->HandleWindowEvent(scroll);
        wxCommandEvent changed(wxEVT_SLIDER, slider->GetId());
        changed.SetInt(target);
        changed.SetEventObject(slider);
        slider->HandleWindowEvent(changed);
    });
}
