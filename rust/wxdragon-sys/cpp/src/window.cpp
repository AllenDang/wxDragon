#include <wx/wxprec.h>
#include <wx/wx.h>
#include "../include/wxdragon.h" // Needed for WXD_EXPORTED, wxd_* types, WXD_ID_ANY
#include <wx/window.h> // Needed for wxWindow
#include <wx/sizer.h>  // Needed for wxSizer
#include "wx/gdicmn.h" // For wxSize, wxPoint
#include <cstring> // For strdup
#include <cstdlib> // For free (though free typically comes from stdlib.h, cstdlib is C++ way)
#include <wx/font.h> // For wxFont in SetFont
#include <wx/settings.h> // For wxSystemSettings and wxSYS_DEFAULT_GUI_FONT
#include <wx/cursor.h> // For wxCursor
#include <wx/textctrl.h> // For wxTextCtrl scrolling

// Conditional includes for optional features
#if wxdUSE_RICHTEXT
#include <wx/richtext/richtextctrl.h> // For wxRichTextCtrl scrolling
#endif

#if wxdUSE_STC
#include <wx/stc/stc.h> // For wxStyledTextCtrl scrolling
#endif

extern "C" {

// --- General Window Functions ---

// Implementation for wxd_Window_SetSizer
WXD_EXPORTED void wxd_Window_SetSizer(wxd_Window_t* window, wxd_Sizer_t* sizer, bool deleteOldSizer) {
    wxWindow* wx_window = reinterpret_cast<wxWindow*>(window);
    wxSizer* wx_sizer = reinterpret_cast<wxSizer*>(sizer);
    if (wx_window && wx_sizer) {
        wx_window->SetSizer(wx_sizer, deleteOldSizer);
    }
}

// Implementation for wxd_Window_SetSizerAndFit
WXD_EXPORTED void wxd_Window_SetSizerAndFit(wxd_Window_t* window, wxd_Sizer_t* sizer, bool deleteOldSizer) {
    wxWindow* wx_window = reinterpret_cast<wxWindow*>(window);
    wxSizer* wx_sizer = reinterpret_cast<wxSizer*>(sizer);
    if (wx_window && wx_sizer) {
        wx_window->SetSizerAndFit(wx_sizer, deleteOldSizer);
    }
}

// Implementation for wxd_Window_GetSizer
WXD_EXPORTED wxd_Sizer_t* wxd_Window_GetSizer(wxd_Window_t* window) {
    wxWindow* wx_window = reinterpret_cast<wxWindow*>(window);
    if (wx_window) {
        wxSizer* wx_sizer = wx_window->GetSizer();
        return reinterpret_cast<wxd_Sizer_t*>(wx_sizer);
    }
    return nullptr;
}

// Implementation for wxd_Window_GetId
WXD_EXPORTED int wxd_Window_GetId(wxd_Window_t* window) {
    wxWindow* wx_window = reinterpret_cast<wxWindow*>(window);
    if (!wx_window) {
        return wxID_ANY; // Use wxID_ANY instead of WXD_ID_ANY
    }
    return wx_window->GetId();
}

// Placeholder for wxd_Window_Destroy if needed later
WXD_EXPORTED void wxd_Window_Destroy(wxd_Window_t* window) {
    wxWindow* wx_window = reinterpret_cast<wxWindow*>(window);
    if (wx_window) {
        // For non-top-level windows, Destroy() schedules deletion.
        // For top-level windows (wxFrame, wxDialog), it tries to close them first.
        // If the window is a child of another, its parent will typically manage its actual deletion
        // from memory after it's Destroy()'d (removed from window hierarchy and pending deletion).
        // If it's a dynamically allocated top-level window not managed by wxApp, direct deletion
        // might be considered but Destroy() is safer for proper cleanup.
        wx_window->Destroy();
    }
}

WXD_EXPORTED void wxd_Window_SetBackgroundColor(wxd_Window_t* window, wxd_Colour_t color) {
    wxWindow* wx_window = reinterpret_cast<wxWindow*>(window);
    if (wx_window) {
        wx_window->SetBackgroundColour(wxColour(color.r, color.g, color.b, color.a));
    }
}

WXD_EXPORTED void wxd_Window_SetMinSize(wxd_Window_t* window, wxd_Size size) {
    wxWindow* wx_window = reinterpret_cast<wxWindow*>(window);
    if (wx_window) {
        wx_window->SetMinSize(wxSize(size.width, size.height));
    }
}

WXD_EXPORTED void wxd_Window_Refresh(wxd_Window_t* window, int eraseBackground, const wxd_Rect* rect) {
    wxWindow* wx_window = reinterpret_cast<wxWindow*>(window);
    if (wx_window) {
        if (rect) {
            wx_window->RefreshRect(wxRect(rect->x, rect->y, rect->width, rect->height), eraseBackground);
        } else {
            wx_window->Refresh(eraseBackground);
        }
    }
}

WXD_EXPORTED void wxd_Window_Fit(wxd_Window_t* window) {
    wxWindow* win = (wxWindow*)window;
    if (win) {
        win->Fit();
    }
}

WXD_EXPORTED void wxd_Window_Layout(wxd_Window_t* window) {
    wxWindow* win = (wxWindow*)window;
    if (win) {
        win->Layout();
    }
}

WXD_EXPORTED wxd_Size wxd_Window_GetBestSize(wxd_Window_t* window) {
    wxWindow* win = (wxWindow*)window;
    wxd_Size result = { -1, -1 }; // Default invalid size
    if (win) {
        wxSize bestSize = win->GetBestSize();
        result.width = bestSize.GetWidth();
        result.height = bestSize.GetHeight();
    }
    return result;
}

// ADDED: Implementation for wxd_Window_SetToolTip
WXD_EXPORTED void wxd_Window_SetToolTip(wxd_Window_t* window, const char* tipString) {
    wxWindow* wx_window = reinterpret_cast<wxWindow*>(window);
    if (wx_window) {
        // wxString::FromUTF8 handles NULL tipString gracefully (creates empty string)
        wx_window->SetToolTip(wxString::FromUTF8(tipString)); 
    }
}

// Implement other wxd_Window_* functions from the header here (e.g., Show, Destroy) if needed.

// Note: SetSizer and SetSizerAndFit are often implemented in sizer.cpp or 
// directly where wxSizer is involved, but could be here too if preferred.
// Checking existing code... It seems SetSizer and SetSizerAndFit are in sizer.cpp.

// Attach/Detach/Notify cleanup functions are already implemented (likely in app.cpp or event.cpp).

// --- Window Manipulation --- 

WXD_EXPORTED void wxd_Window_Show(wxd_Window_t* self, bool show) {
    if (self) {
        reinterpret_cast<wxWindow*>(self)->Show(show);
    }
}

WXD_EXPORTED bool wxd_Window_Close(wxd_Window_t* self, bool force) {
    if (self) {
        return reinterpret_cast<wxWindow*>(self)->Close(force);
    }
    return false;
}

WXD_EXPORTED void wxd_Window_SetId(wxd_Window_t* self, int id) {
    if (self) {
        reinterpret_cast<wxWindow*>(self)->SetId(id);
    }
}

// Note: GetLabel for a generic wxWindow might not always be what's expected,
// as not all windows have a visible label in the same way as controls.
// However, wxWindow itself does have GetLabel/SetLabel.
WXD_EXPORTED void wxd_Window_SetLabel(wxd_Window_t* self, const char* label) {
    if (self) {
        reinterpret_cast<wxWindow*>(self)->SetLabel(wxString::FromUTF8(label ? label : ""));
    }
}

WXD_EXPORTED char* wxd_Window_GetLabel(wxd_Window_t* self) {
    if (self) {
        wxString label = reinterpret_cast<wxWindow*>(self)->GetLabel();
        const wxScopedCharBuffer utf8_buf = label.ToUTF8();
        if (utf8_buf.data()) { // Check if data is not null
            return strdup(utf8_buf.data()); // Allocate and copy string
        }
    }
    // Return a duplicated empty string if self is null or label is empty to avoid returning NULL
    // which Rust CString::from_raw would panic on.
    // Callers should check if the string is empty if that has meaning.
    return strdup(""); 
}

WXD_EXPORTED void wxd_Window_SetFont(wxd_Window_t* self, const wxd_Font_t* font) {
    if (!self) return;
    // If font is NULL, wxWidgets SetFont will likely use a default or do nothing.
    // If it requires a valid font, we might need to pass wxNullFont or wxSystemSettings::GetFont(wxSYS_DEFAULT_GUI_FONT).
    // For now, assume passing NULL (if wxd_Font_t* is NULL) is handled gracefully by wxFont constructor or SetFont.
    // wxFont takes a wxFont*, so if font is NULL, it will be wxFont(*NULL) which might be an issue.
    // Better to check for NULL font and pass wxNullFont explicitly.
    if (font) {
        ((wxWindow*)self)->SetFont(*((wxFont*)font));
    } else {
        // Attempt to set a default font or do nothing if wxNullFont isn't appropriate.
        // For now, let's try setting the system default GUI font if null is passed.
        // This behavior might need adjustment based on desired semantics for a null font.
        ((wxWindow*)self)->SetFont(wxSystemSettings::GetFont(wxSYS_DEFAULT_GUI_FONT));
    }
}

WXD_EXPORTED wxd_Font_t* wxd_Window_GetFont(wxd_Window_t* self) {
    if (!self) return NULL;
    
    wxWindow* window = reinterpret_cast<wxWindow*>(self);
    wxFont font = window->GetFont();
    
    // Only allocate and return a new wxFont if it's valid
    if (font.IsOk()) {
        wxFont* new_font = new wxFont(font);
        return reinterpret_cast<wxd_Font_t*>(new_font);
    }
    
    return NULL;
}

WXD_EXPORTED wxd_Point wxd_Window_GetPosition(wxd_Window_t* self) {
    wxWindow* wx_window = reinterpret_cast<wxWindow*>(self);
    if (!wx_window) {
        return { -1, -1 }; // Default invalid position
    }
    wxPoint wx_position = wx_window->GetPosition();
    return { wx_position.x, wx_position.y };
}

WXD_EXPORTED wxd_Window_t* wxd_Window_GetParent(wxd_Window_t* self) {
    wxWindow* wx_window = reinterpret_cast<wxWindow*>(self);
    if (!wx_window) return NULL;
    return reinterpret_cast<wxd_Window_t*>(wx_window->GetParent());
}

WXD_EXPORTED wxd_Window_t* wxd_Window_GetGrandParent(wxd_Window_t* self) {
    if (!self) return NULL;
    wxWindow* self_wnd = reinterpret_cast<wxWindow*>(self);
    return reinterpret_cast<wxd_Window_t*>(self_wnd->GetGrandParent());
}

WXD_EXPORTED bool wxd_Window_IsEnabled(wxd_Window_t *self) {
    if (!self) return false;
    return reinterpret_cast<wxWindow*>(self)->IsEnabled();
}

WXD_EXPORTED void wxd_Window_Enable(wxd_Window_t *self, bool enable) {
    if (!self) return;
    reinterpret_cast<wxWindow*>(self)->Enable(enable);
}

// New size and position functions

WXD_EXPORTED wxd_Size wxd_Window_GetSize(wxd_Window_t* window) {
    wxWindow* wx_window = reinterpret_cast<wxWindow*>(window);
    if (!wx_window) {
        return { -1, -1 }; // Default invalid size
    }
    wxSize wx_size = wx_window->GetSize();
    return { wx_size.GetWidth(), wx_size.GetHeight() };
}

WXD_EXPORTED void wxd_Window_SetSize(wxd_Window_t* window, wxd_Size size) {
    wxWindow* wx_window = reinterpret_cast<wxWindow*>(window);
    if (wx_window) {
        wx_window->SetSize(wxSize(size.width, size.height));
    }
}

WXD_EXPORTED void wxd_Window_SetSizeWithPos(wxd_Window_t* window, int x, int y, int width, int height, int sizeFlags) {
    wxWindow* wx_window = reinterpret_cast<wxWindow*>(window);
    if (wx_window) {
        wx_window->SetSize(x, y, width, height, sizeFlags);
    }
}

WXD_EXPORTED wxd_Size wxd_Window_GetClientSize(wxd_Window_t* window) {
    wxWindow* wx_window = reinterpret_cast<wxWindow*>(window);
    if (!wx_window) {
        return { -1, -1 }; // Default invalid size
    }
    wxSize wx_size = wx_window->GetClientSize();
    return { wx_size.GetWidth(), wx_size.GetHeight() };
}

WXD_EXPORTED void wxd_Window_SetClientSize(wxd_Window_t* window, wxd_Size size) {
    wxWindow* wx_window = reinterpret_cast<wxWindow*>(window);
    if (wx_window) {
        wx_window->SetClientSize(wxSize(size.width, size.height));
    }
}

WXD_EXPORTED wxd_Size wxd_Window_GetMinSize(wxd_Window_t* window) {
    wxWindow* wx_window = reinterpret_cast<wxWindow*>(window);
    if (!wx_window) {
        return { -1, -1 }; // Default invalid size
    }
    wxSize wx_size = wx_window->GetMinSize();
    return { wx_size.GetWidth(), wx_size.GetHeight() };
}

WXD_EXPORTED void wxd_Window_Move(wxd_Window_t* window, int x, int y) {
    wxWindow* wx_window = reinterpret_cast<wxWindow*>(window);
    if (wx_window) {
        wx_window->Move(x, y);
    }
}

WXD_EXPORTED void wxd_Window_Center(wxd_Window_t* window) {
    wxWindow* wx_window = reinterpret_cast<wxWindow*>(window);
    if (wx_window) {
        wx_window->Center();
    }
}

// Background style functions
WXD_EXPORTED void wxd_Window_SetBackgroundStyle(wxd_Window_t* window, int style) {
    wxWindow* wx_window = reinterpret_cast<wxWindow*>(window);
    if (wx_window) {
        wxBackgroundStyle wx_style = static_cast<wxBackgroundStyle>(style);
        wx_window->SetBackgroundStyle(wx_style);
    }
}

WXD_EXPORTED int wxd_Window_GetBackgroundStyle(wxd_Window_t* window) {
    wxWindow* wx_window = reinterpret_cast<wxWindow*>(window);
    if (wx_window) {
        return static_cast<int>(wx_window->GetBackgroundStyle());
    }
    return static_cast<int>(wxBG_STYLE_SYSTEM); // Default fallback
}

WXD_EXPORTED wxd_Point wxd_Window_ClientToScreen(wxd_Window_t* window, wxd_Point pt) {
    wxWindow* wx_window = reinterpret_cast<wxWindow*>(window);
    if (!wx_window) {
        return pt; // Return the input point if window is null
    }
    wxPoint wx_pt(pt.x, pt.y);
    wxPoint wx_result = wx_window->ClientToScreen(wx_pt);
    return { wx_result.x, wx_result.y };
}

WXD_EXPORTED wxd_Point wxd_Window_ScreenToClient(wxd_Window_t* window, wxd_Point pt) {
    wxWindow* wx_window = reinterpret_cast<wxWindow*>(window);
    if (!wx_window) {
        return pt; // Return the input point if window is null
    }
    wxPoint wx_pt(pt.x, pt.y);
    wxPoint wx_result = wx_window->ScreenToClient(wx_pt);
    return { wx_result.x, wx_result.y };
}

// Extra window style functions
WXD_EXPORTED void wxd_Window_SetExtraStyle(wxd_Window_t* window, int64_t exStyle) {
    wxWindow* wx_window = reinterpret_cast<wxWindow*>(window);
    if (wx_window) {
        wx_window->SetExtraStyle(static_cast<int64_t>(exStyle));
    }
}

WXD_EXPORTED int64_t wxd_Window_GetExtraStyle(wxd_Window_t* window) {
    wxWindow* wx_window = reinterpret_cast<wxWindow*>(window);
    if (wx_window) {
        return static_cast<int64_t>(wx_window->GetExtraStyle());
    }
    return 0; // Default fallback
}

// Color management functions
WXD_EXPORTED void wxd_Window_SetForegroundColor(wxd_Window_t* window, wxd_Colour_t color) {
    wxWindow* wx_window = reinterpret_cast<wxWindow*>(window);
    if (wx_window) {
        wx_window->SetForegroundColour(wxColour(color.r, color.g, color.b, color.a));
    }
}

WXD_EXPORTED wxd_Colour_t wxd_Window_GetForegroundColor(wxd_Window_t* window) {
    wxWindow* wx_window = reinterpret_cast<wxWindow*>(window);
    if (wx_window) {
        wxColour wx_color = wx_window->GetForegroundColour();
        return { wx_color.Red(), wx_color.Green(), wx_color.Blue(), wx_color.Alpha() };
    }
    return { 0, 0, 0, 255 }; // Default black color
}

WXD_EXPORTED wxd_Colour_t wxd_Window_GetBackgroundColor(wxd_Window_t* window) {
    wxWindow* wx_window = reinterpret_cast<wxWindow*>(window);
    if (wx_window) {
        wxColour wx_color = wx_window->GetBackgroundColour();
        return { wx_color.Red(), wx_color.Green(), wx_color.Blue(), wx_color.Alpha() };
    }
    return { 255, 255, 255, 255 }; // Default white color
}

// Focus management functions
WXD_EXPORTED void wxd_Window_SetFocus(wxd_Window_t* window) {
    wxWindow* wx_window = reinterpret_cast<wxWindow*>(window);
    if (wx_window) {
        wx_window->SetFocus();
    }
}

WXD_EXPORTED bool wxd_Window_HasFocus(wxd_Window_t* window) {
    wxWindow* wx_window = reinterpret_cast<wxWindow*>(window);
    if (wx_window) {
        return wx_window->HasFocus();
    }
    return false;
}

WXD_EXPORTED bool wxd_Window_CanAcceptFocus(wxd_Window_t* window) {
    wxWindow* wx_window = reinterpret_cast<wxWindow*>(window);
    if (wx_window) {
        return wx_window->CanAcceptFocus();
    }
    return false;
}

// Visibility functions
WXD_EXPORTED bool wxd_Window_IsShown(wxd_Window_t* window) {
    wxWindow* wx_window = reinterpret_cast<wxWindow*>(window);
    if (wx_window) {
        return wx_window->IsShown();
    }
    return false;
}

// Size constraint functions
WXD_EXPORTED void wxd_Window_SetMaxSize(wxd_Window_t* window, wxd_Size size) {
    wxWindow* wx_window = reinterpret_cast<wxWindow*>(window);
    if (wx_window) {
        wx_window->SetMaxSize(wxSize(size.width, size.height));
    }
}

WXD_EXPORTED wxd_Size wxd_Window_GetMaxSize(wxd_Window_t* window) {
    wxWindow* wx_window = reinterpret_cast<wxWindow*>(window);
    if (!wx_window) {
        return { -1, -1 }; // Default invalid size
    }
    wxSize wx_size = wx_window->GetMaxSize();
    return { wx_size.GetWidth(), wx_size.GetHeight() };
}

// Window properties functions
WXD_EXPORTED void wxd_Window_SetName(wxd_Window_t* window, const char* name) {
    wxWindow* wx_window = reinterpret_cast<wxWindow*>(window);
    if (wx_window) {
        wx_window->SetName(wxString::FromUTF8(name ? name : ""));
    }
}

WXD_EXPORTED char* wxd_Window_GetName(wxd_Window_t* window) {
    wxWindow* wx_window = reinterpret_cast<wxWindow*>(window);
    if (wx_window) {
        wxString name = wx_window->GetName();
        const wxScopedCharBuffer utf8_buf = name.ToUTF8();
        if (utf8_buf.data()) {
            return strdup(utf8_buf.data());
        }
    }
    return strdup(""); // Return empty string if window is null or name is empty
}

// Window finding functions
WXD_EXPORTED wxd_Window_t* wxd_Window_FindWindowByName(wxd_Window_t* window, const char* name) {
    wxWindow* wx_window = reinterpret_cast<wxWindow*>(window);
    if (!wx_window || !name) {
        return nullptr;
    }
    
    wxString windowName = wxString::FromUTF8(name);
    wxWindow* child = wx_window->FindWindow(windowName);
    return reinterpret_cast<wxd_Window_t*>(child);
}

WXD_EXPORTED wxd_Window_t* wxd_Window_FindWindowById(wxd_Window_t* window, int id) {
    wxWindow* wx_window = reinterpret_cast<wxWindow*>(window);
    if (!wx_window) {
        return nullptr;
    }
    
    wxWindow* child = wx_window->FindWindow(id);
    return reinterpret_cast<wxd_Window_t*>(child);
}

// --- Cursor Management Functions ---
WXD_EXPORTED void wxd_Window_SetCursor(wxd_Window_t* window, wxd_Cursor_t* cursor) {
    wxWindow* wx_window = reinterpret_cast<wxWindow*>(window);
    if (wx_window) {
        if (cursor) {
            wxCursor* wx_cursor = reinterpret_cast<wxCursor*>(cursor);
            wx_window->SetCursor(*wx_cursor);
        } else {
            // Set to default cursor if null is passed
            wx_window->SetCursor(wxNullCursor);
        }
    }
}

WXD_EXPORTED wxd_Cursor_t* wxd_Window_GetCursor(wxd_Window_t* window) {
    wxWindow* wx_window = reinterpret_cast<wxWindow*>(window);
    if (wx_window) {
        wxCursor cursor = wx_window->GetCursor();
        if (cursor.IsOk()) {
            // Create a new wxCursor and return it
            wxCursor* new_cursor = new wxCursor(cursor);
            return reinterpret_cast<wxd_Cursor_t*>(new_cursor);
        }
    }
    return nullptr;
}

// --- Z-Order Management Functions ---
WXD_EXPORTED void wxd_Window_Raise(wxd_Window_t* window) {
    wxWindow* wx_window = reinterpret_cast<wxWindow*>(window);
    if (wx_window) {
        wx_window->Raise();
    }
}

WXD_EXPORTED void wxd_Window_Lower(wxd_Window_t* window) {
    wxWindow* wx_window = reinterpret_cast<wxWindow*>(window);
    if (wx_window) {
        wx_window->Lower();
    }
}

// --- Mouse Capture Functions ---
WXD_EXPORTED void wxd_Window_CaptureMouse(wxd_Window_t* window) {
    wxWindow* wx_window = reinterpret_cast<wxWindow*>(window);
    if (wx_window) {
        wx_window->CaptureMouse();
    }
}

WXD_EXPORTED void wxd_Window_ReleaseMouse(wxd_Window_t* window) {
    wxWindow* wx_window = reinterpret_cast<wxWindow*>(window);
    if (wx_window) {
        wx_window->ReleaseMouse();
    }
}

WXD_EXPORTED bool wxd_Window_HasCapture(wxd_Window_t* window) {
    wxWindow* wx_window = reinterpret_cast<wxWindow*>(window);
    if (wx_window) {
        return wx_window->HasCapture();
    }
    return false;
}

WXD_EXPORTED wxd_Window_t* wxd_Window_GetCapture() {
    wxWindow* captured_window = wxWindow::GetCapture();
    return reinterpret_cast<wxd_Window_t*>(captured_window);
}

// --- Text Measurement Functions ---
WXD_EXPORTED wxd_Size wxd_Window_GetTextExtent(wxd_Window_t* window, const char* text) {
    wxWindow* wx_window = reinterpret_cast<wxWindow*>(window);
    if (wx_window && text) {
        wxString wx_text = wxString::FromUTF8(text);
        wxSize size = wx_window->GetTextExtent(wx_text);
        return { size.GetWidth(), size.GetHeight() };
    }
    return { 0, 0 }; // Default size if window is null or text is null
}

WXD_EXPORTED void wxd_Window_GetFullTextExtent(wxd_Window_t* window, const char* text, wxd_Size* size, int* descent, int* external_leading, wxd_Font_t* font) {
    wxWindow* wx_window = reinterpret_cast<wxWindow*>(window);
    if (!wx_window || !text || !size) {
        if (size) {
            size->width = 0;
            size->height = 0;
        }
        if (descent) *descent = 0;
        if (external_leading) *external_leading = 0;
        return;
    }

    wxString wx_text = wxString::FromUTF8(text);
    wxFont* wx_font = font ? reinterpret_cast<wxFont*>(font) : nullptr;
    
    int w, h, desc, ext_lead;
    wx_window->GetTextExtent(wx_text, &w, &h, &desc, &ext_lead, wx_font);
    
    size->width = w;
    size->height = h;
    if (descent) *descent = desc;
    if (external_leading) *external_leading = ext_lead;
}

WXD_EXPORTED int wxd_Window_GetCharHeight(wxd_Window_t* window) {
    wxWindow* wx_window = reinterpret_cast<wxWindow*>(window);
    if (wx_window) {
        return wx_window->GetCharHeight();
    }
    return 0;
}

WXD_EXPORTED int wxd_Window_GetCharWidth(wxd_Window_t* window) {
    wxWindow* wx_window = reinterpret_cast<wxWindow*>(window);
    if (!wx_window) {
        return 0;
    }
    return wx_window->GetCharWidth();
}

// Window style functions
WXD_EXPORTED void wxd_Window_SetWindowStyle(wxd_Window_t* window, int64_t style) {
    wxWindow* wx_window = reinterpret_cast<wxWindow*>(window);
    if (wx_window) {
        wx_window->SetWindowStyle(static_cast<long>(style));
    }
}

WXD_EXPORTED int64_t wxd_Window_GetWindowStyle(wxd_Window_t* window) {
    wxWindow* wx_window = reinterpret_cast<wxWindow*>(window);
    if (wx_window) {
        return static_cast<int64_t>(wx_window->GetWindowStyle());
    }
    return 0; // Default fallback
}

// --- Scrolling Functions ---
WXD_EXPORTED void wxd_Window_ShowPosition(wxd_Window_t* window, int64_t position) {
    wxWindow* wx_window = reinterpret_cast<wxWindow*>(window);
    if (!wx_window) {
        return;
    }

    // Try to cast to wxTextCtrl first (most common case for text position scrolling)
    if (wxTextCtrl* text_ctrl = wxDynamicCast(wx_window, wxTextCtrl)) {
        text_ctrl->ShowPosition(static_cast<long>(position));
        return;
    }

    // Try to cast to wxRichTextCtrl if richtext feature is enabled
#if wxdUSE_RICHTEXT
    if (wxRichTextCtrl* rich_text = wxDynamicCast(wx_window, wxRichTextCtrl)) {
        rich_text->ShowPosition(static_cast<long>(position));
        return;
    }
#endif

    // Try to cast to wxStyledTextCtrl if STC feature is enabled
#if wxdUSE_STC
    if (wxStyledTextCtrl* stc = wxDynamicCast(wx_window, wxStyledTextCtrl)) {
        stc->GotoPos(static_cast<int>(position));
        stc->EnsureCaretVisible();
        return;
    }
#endif

    // For other scrollable windows, try generic scrolling
    // Note: This may not work for all widget types, but provides a fallback
}

WXD_EXPORTED void wxd_Window_ScrollIntoView(wxd_Window_t* window, int64_t position, int keyCode) {
    wxWindow* wx_window = reinterpret_cast<wxWindow*>(window);
    if (!wx_window) {
        return;
    }

    // Try to cast to wxRichTextCtrl first (has the most sophisticated scrolling)
#if wxdUSE_RICHTEXT
    if (wxRichTextCtrl* rich_text = wxDynamicCast(wx_window, wxRichTextCtrl)) {
        rich_text->ScrollIntoView(static_cast<long>(position), keyCode);
        return;
    }
#endif

    // For TextCtrl, use ShowPosition as a fallback
    if (wxTextCtrl* text_ctrl = wxDynamicCast(wx_window, wxTextCtrl)) {
        text_ctrl->ShowPosition(static_cast<long>(position));
        return;
    }

    // For StyledTextCtrl, use position scrolling
#if wxdUSE_STC
    if (wxStyledTextCtrl* stc = wxDynamicCast(wx_window, wxStyledTextCtrl)) {
        stc->GotoPos(static_cast<int>(position));
        stc->EnsureCaretVisible();
        return;
    }
#endif

    // Generic fallback - just call ShowPosition
    wxd_Window_ShowPosition(window, position);
}

WXD_EXPORTED bool wxd_Window_IsPositionVisible(wxd_Window_t* window, int64_t position) {
    wxWindow* wx_window = reinterpret_cast<wxWindow*>(window);
    if (!wx_window) {
        return false;
    }

    // Try to cast to wxRichTextCtrl first (has native support for this)
#if wxdUSE_RICHTEXT
    if (wxRichTextCtrl* rich_text = wxDynamicCast(wx_window, wxRichTextCtrl)) {
        return rich_text->IsPositionVisible(static_cast<long>(position));
    }
#endif

    // For TextCtrl, we need to estimate visibility
    if (wxTextCtrl* text_ctrl = wxDynamicCast(wx_window, wxTextCtrl)) {
        // Get the first and last visible positions (approximate)
        long x, y;
        text_ctrl->PositionToXY(static_cast<long>(position), &x, &y);
        
        // Check if the position is within the visible area
        wxSize client_size = text_ctrl->GetClientSize();
        wxFont font = text_ctrl->GetFont();
        int char_height = text_ctrl->GetCharHeight();
        int lines_visible = client_size.y / char_height;
        
        // This is a rough approximation - for exact visibility checking,
        // more sophisticated logic would be needed
        long first_visible = text_ctrl->XYToPosition(0, 0);
        long last_visible = text_ctrl->XYToPosition(0, lines_visible);
        
        return position >= first_visible && position <= last_visible;
    }

    // For StyledTextCtrl, check if position is visible
#if wxdUSE_STC
    if (wxStyledTextCtrl* stc = wxDynamicCast(wx_window, wxStyledTextCtrl)) {
        int line = stc->LineFromPosition(static_cast<int>(position));
        int first_visible_line = stc->GetFirstVisibleLine();
        int lines_on_screen = stc->LinesOnScreen();
        return line >= first_visible_line && line < (first_visible_line + lines_on_screen);
    }
#endif

    // Default fallback - assume not visible
    return false;
}

WXD_EXPORTED int64_t wxd_Window_GetLastPosition(wxd_Window_t* window) {
    wxWindow* wx_window = reinterpret_cast<wxWindow*>(window);
    if (!wx_window) {
        return 0;
    }

    // Try to cast to wxTextCtrl first
    if (wxTextCtrl* text_ctrl = wxDynamicCast(wx_window, wxTextCtrl)) {
        return static_cast<int64_t>(text_ctrl->GetLastPosition());
    }

    // Try to cast to wxRichTextCtrl
#if wxdUSE_RICHTEXT
    if (wxRichTextCtrl* rich_text = wxDynamicCast(wx_window, wxRichTextCtrl)) {
        return static_cast<int64_t>(rich_text->GetLastPosition());
    }
#endif

    // Try to cast to wxStyledTextCtrl
#if wxdUSE_STC
    if (wxStyledTextCtrl* stc = wxDynamicCast(wx_window, wxStyledTextCtrl)) {
        return static_cast<int64_t>(stc->GetLength());
    }
#endif

    // Default fallback
    return 0;
}

// --- Platform-specific Functions ---

WXD_EXPORTED void* wxd_Window_GetHandle(wxd_Window_t* self) {
    if (!self) {
        return nullptr;
    }
    wxWindow* wx_window = reinterpret_cast<wxWindow*>(self);
    return reinterpret_cast<void*>(wx_window->GetHandle());
}


// Get wxWidgets class name using built-in RTTI
WXD_EXPORTED const char* wxd_Window_GetClassName(wxd_Window_t* window) {
    if (!window) return nullptr;
    wxWindow* wx_window = reinterpret_cast<wxWindow*>(window);
    
    // Get the class name and convert from wxChar* to const char*
    const wxChar* wx_class_name = wx_window->GetClassInfo()->GetClassName();
    
    // Convert wxString to std::string, then to const char*
    wxString wx_str(wx_class_name);
    std::string std_str = wx_str.ToStdString();
    
    // We need to return a persistent string, so we'll use a static approach
    // This is a simple implementation - in production you might want a more sophisticated approach
    static std::string persistent_name;
    persistent_name = std_str;
    return persistent_name.c_str();
}

} // extern "C"