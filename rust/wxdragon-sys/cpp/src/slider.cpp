#include <wx/wxprec.h>
#include <wx/wx.h>
#include "../include/wxdragon.h"
#include <wx/slider.h>

// The other wxSlider functions are generated from cpp/spec/slider.wxd.

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
