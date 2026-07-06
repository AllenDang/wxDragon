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
wxd_Window_SetAccessibilityHelp(wxd_Window_t* window, const char* help)
{
    if (!window || !help) return;
    wxWindow* wx_window = reinterpret_cast<wxWindow*>(window);
    NSView* view = wx_window->GetHandle();
    if (view) {
        [view setAccessibilityHelp:[NSString stringWithUTF8String:help]];
    }
}

void
wxd_Window_SetAccessibilityValue(wxd_Window_t* window, const char* value)
{
    if (!window || !value) return;
    wxWindow* wx_window = reinterpret_cast<wxWindow*>(window);
    NSView* view = wx_window->GetHandle();
    if (view) {
        [view setAccessibilityValue:[NSString stringWithUTF8String:value]];
    }
}

void
wxd_App_ActivateMac(void)
{
    [[NSRunningApplication currentApplication]
        activateWithOptions:NSApplicationActivateIgnoringOtherApps];
}
