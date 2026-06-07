#import <AppKit/AppKit.h>
#include "../include/wxdragon.h"

void
wxd_Window_SetAccessibilityLabel(wxd_Window_t* window, const char* label)
{
    if (!window || !label) return;
    wxWindow* wx_window = reinterpret_cast<wxWindow*>(window);
    NSView* view = wx_window->GetHandle();
    if (view) {
        [view setAccessibilityLabel:[NSString stringWithUTF8String:label]];
    }
}

void
wxd_Window_HideFromAccessibility(wxd_Window_t* window)
{
    if (!window) return;
    wxWindow* wx_window = reinterpret_cast<wxWindow*>(window);
    NSView* view = wx_window->GetHandle();
    if (view) {
        [view setAccessibilityElement:NO];
    }
}
