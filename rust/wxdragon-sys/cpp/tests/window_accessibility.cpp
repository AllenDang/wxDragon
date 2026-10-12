// Run on the OS main thread: Cocoa cannot be initialized by Cargo's test workers.
#include <wx/wx.h>
#include <wx/artprov.h>
#include <wx/bmpbuttn.h>
#include <stdexcept>
#include <iostream>
#include <memory>
#include "wxdragon.h"

#ifdef __WXOSX__
#import <AppKit/AppKit.h>
#endif

#if wxUSE_ACCESSIBILITY
#include <wx/access.h>
#include <oleacc.h>
#endif

class AccessibilityTestApp : public wxApp {
public:
    bool
    OnInit() override
    { return true; }
};
wxIMPLEMENT_APP_NO_MAIN(AccessibilityTestApp);

static void
Check(bool condition, const char* message)
{
    if (!condition)
        throw std::runtime_error(message);
}

static wxd_Window_t*
WxHandle(wxWindow* window)
{ return reinterpret_cast<wxd_Window_t*>(window); }

#if wxUSE_ACCESSIBILITY
static wxString
AccessibleName(wxWindow* window)
{
    Check(window->GetAccessible() != nullptr, "missing accessible object");
    // Query the exposed COM object, not GetIAccessibleStd(), which is the
    // native fallback and intentionally bypasses wxWidgets' overrides.
    IAccessible* object = nullptr;
    HRESULT result = AccessibleObjectFromWindow(reinterpret_cast<HWND>(window->GetHandle()),
                                                static_cast<DWORD>(OBJID_CLIENT), IID_IAccessible,
                                                reinterpret_cast<void**>(&object));
    Check(SUCCEEDED(result) && object, "AccessibleObjectFromWindow failed");
    VARIANT child;
    VariantInit(&child);
    child.vt = VT_I4;
    child.lVal = CHILDID_SELF;
    BSTR name = nullptr;
    result = object->get_accName(child, &name);
    object->Release();
    wxString value = name ? wxString(name) : wxString();
    SysFreeString(name);
    Check(SUCCEEDED(result), "get_accName failed");
    return value;
}

class CustomAccessible : public wxAccessible {
public:
    CustomAccessible(wxWindow* window, std::shared_ptr<bool> destroyed)
        : wxAccessible(window), m_destroyed(destroyed)
    {
    }
    ~CustomAccessible() override
    { *m_destroyed = true; }
    wxAccStatus
    GetName(int childId, wxString* name) override
    {
        if (childId != 0)
            return wxACC_NOT_IMPLEMENTED;
        *name = "Custom name";
        return wxACC_OK;
    }

private:
    std::shared_ptr<bool> m_destroyed;
};
#endif

static void
TestNames(wxWindow* parent)
{
    auto* button = new wxButton(parent, wxID_ANY, "Continue");
    auto* handle = WxHandle(button);
    const char* unicodeName =
        "Continue to payment \xe2\x80\x94 \xe6\x94\xaf\xe6\x89\x95\xe3\x81\x84";
    wxd_Window_SetAccessibleName(handle, unicodeName);
#if wxUSE_ACCESSIBILITY
    Check(AccessibleName(button) == wxString::FromUTF8(unicodeName), "Unicode name lost");

    // Installing the remaining overrides must not erase the upstream name.
    wxd_Window_SetAccessibleDescription(handle, "Description");
    wxd_Window_SetAccessibleValue(handle, "Value");
    wxd_Window_SetAccessibleRole(handle, WXD_ROLE_SYSTEM_PUSHBUTTON);
    wxd_Window_SetAccessibleState(handle, 0);
    Check(AccessibleName(button) == wxString::FromUTF8(unicodeName), "property setter erased name");
    wxAccessible* accessible = button->GetAccessible();
    wxString property;
    Check(accessible->GetDescription(0, &property) == wxACC_OK && property == "Description",
          "description lost");
    Check(accessible->GetValue(0, &property) == wxACC_OK && property == "Value", "value lost");
    wxAccRole role;
    Check(accessible->GetRole(0, &role) == wxACC_OK && role == wxROLE_SYSTEM_PUSHBUTTON,
          "role lost");
    long state = -1;
    Check(accessible->GetState(0, &state) == wxACC_OK && state == 0, "state lost");

    // Also check the opposite ordering and default-name restoration.
    wxd_Window_SetAccessibleName(handle, "Replacement");
    Check(button->GetAccessible() == accessible, "name setter replaced property object");
    Check(AccessibleName(button) == "Replacement", "new name not applied");
    wxd_Window_SetAccessibleName(handle, "");
    Check(AccessibleName(button) == "Continue", "empty string did not restore native name");
    wxd_Window_SetAccessibleName(handle, "Temporary");
    wxd_Window_SetAccessibleName(handle, nullptr);
    Check(AccessibleName(button) == "Continue", "null did not restore native name");

    auto destroyed = std::make_shared<bool>(false);
    auto* custom = new CustomAccessible(button, destroyed);
    button->SetAccessible(custom);
    wxd_Window_SetAccessibleName(handle, unicodeName);
    Check(button->GetAccessible() == custom && !*destroyed,
          "name setter replaced custom accessible");
    Check(AccessibleName(button) == wxString::FromUTF8(unicodeName),
          "custom name was not overridden");
    wxd_Window_SetAccessibleName(handle, "");
    Check(AccessibleName(button) == "Custom name", "custom default name not restored");
    button->SetAccessible(nullptr);
    Check(*destroyed, "custom accessible was not destroyed by its owner");
#elif defined(__WXOSX__)
    NSView* view = button->GetHandle();
    NSString* expected = [NSString stringWithUTF8String:unicodeName];
    Check([[view accessibilityTitle] isEqualToString:expected], "button title not overridden");
    wxd_Window_SetAccessibleDescription(handle, "Description");
    Check([[view accessibilityTitle] isEqualToString:expected], "description erased name");
    wxd_Window_SetAccessibleName(handle, "");
    Check([[view accessibilityTitle] isEqualToString:@"Continue"],
          "default button title not restored");
    wxd_Window_SetAccessibilityLabel(handle, unicodeName);
    Check([[view accessibilityTitle] isEqualToString:expected], "legacy alias diverged");
    wxd_Window_SetAccessibilityLabel(handle, nullptr);
    Check([[view accessibilityTitle] isEqualToString:@"Continue"],
          "legacy alias did not clear name");
#endif
    delete button;

    auto* bitmap =
        new wxBitmapButton(parent, wxID_ANY, wxArtProvider::GetBitmap(wxART_INFORMATION));
    wxd_Window_SetAccessibleName(WxHandle(bitmap), "Information");
#if wxUSE_ACCESSIBILITY
    Check(AccessibleName(bitmap) == "Information", "bitmap button name missing");
#elif defined(__WXOSX__)
    Check([[bitmap->GetHandle() accessibilityTitle] isEqualToString:@"Information"],
          "bitmap button title missing");
#endif
    delete bitmap;
}

#ifdef __WXOSX__
template <typename Control>
static void
TestAutomaticLabel(wxWindow* parent)
{
    auto* label = new wxStaticText(parent, wxID_ANY, "Associated label");
    auto* control = new Control(parent, wxID_ANY);
    NSView* view = control->GetHandle();
    if ([view isKindOfClass:[NSScrollView class]]) {
        view = [(NSScrollView*)view documentView];
    }
    id element = NSAccessibilityUnignoredDescendant(view);
    id labelElement = NSAccessibilityUnignoredDescendant(label->GetHandle());
    Check([[element accessibilityTitleUIElement] isEqual:labelElement],
          "preceding static label was not associated with native control");
    delete control;
    delete label;
}
#endif

static void
TestKeyboardFocus(wxWindow* parent)
{
    auto* text = new wxTextCtrl(parent, wxID_ANY, "ABC-123", wxDefaultPosition, wxDefaultSize,
                                wxTE_READONLY);
    auto* handle = WxHandle(text);
#ifdef __WXMSW__
    Check(!wxd_Window_AcceptsFocusFromKeyboard(handle),
          "read-only field should not accept Tab by default on MSW");
#endif
    wxd_Window_EnableFocusFromKeyboard(handle);
    Check(wxd_Window_AcceptsFocusFromKeyboard(handle), "read-only field not keyboard focusable");
    wxd_Window_DisableFocusFromKeyboard(handle);
    Check(!wxd_Window_AcceptsFocusFromKeyboard(handle), "disable did not skip read-only field");
    wxd_Window_EnableFocusFromKeyboard(handle);
    Check(wxd_Window_AcceptsFocusFromKeyboard(handle), "re-enable did not undo disable");
    text->Disable();
    wxd_Window_EnableFocusFromKeyboard(handle);
    Check(!text->CanAcceptFocusFromKeyboard(), "focus override enabled a disabled window");
    delete text;

    auto* label = new wxStaticText(parent, wxID_ANY, "Not focusable");
    wxd_Window_EnableFocusFromKeyboard(WxHandle(label));
    Check(!label->AcceptsFocusFromKeyboard(), "focus override made a static label focusable");
    delete label;
}

int
main(int argc, char** argv)
{
    wxd_Window_SetAccessibleName(nullptr, "Name");
    wxd_Window_SetAccessibleName(nullptr, nullptr);
    wxd_Window_EnableFocusFromKeyboard(nullptr);
    wxd_Window_DisableFocusFromKeyboard(nullptr);
#ifdef __WXOSX__
    wxd_Window_SetAccessibilityLabel(nullptr, "Name");
#endif
    if (!wxEntryStart(argc, argv))
        return 1;
    if (!wxTheApp->CallOnInit()) {
        wxEntryCleanup();
        return 1;
    }
    auto* frame = new wxFrame(nullptr, wxID_ANY, "Accessibility regressions");
    int result = 0;
    try {
        TestNames(frame);
        TestKeyboardFocus(frame);
#ifdef __WXOSX__
        TestAutomaticLabel<wxChoice>(frame);
        TestAutomaticLabel<wxComboBox>(frame);
        TestAutomaticLabel<wxListBox>(frame);
#endif
        std::cout << "Accessibility regressions passed\n";
    }
    catch (const std::exception& error) {
        std::cerr << error.what() << '\n';
        result = 1;
    }
    delete frame;
    wxTheApp->OnExit();
    wxEntryCleanup();
    return result;
}
