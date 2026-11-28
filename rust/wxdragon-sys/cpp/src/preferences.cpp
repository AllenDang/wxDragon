#include <wx/wxprec.h>
#include <wx/wx.h>
#include <wx/preferences.h>
#include "../include/wxdragon.h"

namespace {

// Holds the Rust closure that builds the page, and frees it with the page.
class PageCallbacks {
public:
    PageCallbacks(wxd_PreferencesPage_CreateWindowCallback create,
                  wxd_PreferencesPage_DropCallback drop, void* userData)
        : m_create(create), m_drop(drop), m_userData(userData)
    {
    }

    ~PageCallbacks()
    {
        if (m_drop && m_userData)
            m_drop(m_userData);
    }

    wxWindow*
    Create(wxWindow* parent)
    {
        wxWindow* window = nullptr;
        if (m_create)
            window = reinterpret_cast<wxWindow*>(
                m_create(m_userData, reinterpret_cast<wxd_Window_t*>(parent)));
        // wxWidgets uses the page window without checking it, so a failed callback gets an empty one.
        return window ? window : new wxPanel(parent);
    }

    wxDECLARE_NO_COPY_CLASS(PageCallbacks);

private:
    wxd_PreferencesPage_CreateWindowCallback m_create;
    wxd_PreferencesPage_DropCallback m_drop;
    void* m_userData;
};

class WxdPreferencesPage : public wxPreferencesPage {
public:
    WxdPreferencesPage(const wxString& name, const wxBitmapBundle& icon,
                       wxd_PreferencesPage_CreateWindowCallback create,
                       wxd_PreferencesPage_DropCallback drop, void* userData)
        : m_name(name), m_icon(icon), m_callbacks(create, drop, userData)
    {
    }

    wxString
    GetName() const override
    { return m_name; }

    wxBitmapBundle
    GetIcon() const override
    { return m_icon; }

    wxWindow*
    CreateWindow(wxWindow* parent) override
    { return m_callbacks.Create(parent); }

private:
    wxString m_name;
    wxBitmapBundle m_icon;
    PageCallbacks m_callbacks;
};

class WxdStockPreferencesPage : public wxStockPreferencesPage {
public:
    WxdStockPreferencesPage(Kind kind, wxd_PreferencesPage_CreateWindowCallback create,
                            wxd_PreferencesPage_DropCallback drop, void* userData)
        : wxStockPreferencesPage(kind), m_callbacks(create, drop, userData)
    {
    }

    wxWindow*
    CreateWindow(wxWindow* parent) override
    { return m_callbacks.Create(parent); }

private:
    PageCallbacks m_callbacks;
};

} // namespace

extern "C" {

wxd_PreferencesEditor_t*
wxd_PreferencesEditor_Create(const char* title)
{
    return reinterpret_cast<wxd_PreferencesEditor_t*>(
        new wxPreferencesEditor(wxString::FromUTF8(title ? title : "")));
}

void
wxd_PreferencesEditor_Destroy(wxd_PreferencesEditor_t* editor)
{ delete reinterpret_cast<wxPreferencesEditor*>(editor); }

void
wxd_PreferencesEditor_AddPage(wxd_PreferencesEditor_t* editor, const char* name,
                              const wxd_BitmapBundle_t* icon,
                              wxd_PreferencesPage_CreateWindowCallback create,
                              wxd_PreferencesPage_DropCallback drop, void* userData)
{
    if (!editor) {
        if (drop && userData)
            drop(userData);
        return;
    }
    wxBitmapBundle bundle = icon ? *reinterpret_cast<const wxBitmapBundle*>(icon) :
                                   wxBitmapBundle();
    reinterpret_cast<wxPreferencesEditor*>(editor)->AddPage(new WxdPreferencesPage(
        wxString::FromUTF8(name ? name : ""), bundle, create, drop, userData));
}

void
wxd_PreferencesEditor_AddStockPage(wxd_PreferencesEditor_t* editor, int kind,
                                   wxd_PreferencesPage_CreateWindowCallback create,
                                   wxd_PreferencesPage_DropCallback drop, void* userData)
{
    if (!editor ||
        (kind != WXD_PREFERENCES_PAGE_GENERAL && kind != WXD_PREFERENCES_PAGE_ADVANCED)) {
        if (drop && userData)
            drop(userData);
        return;
    }
    wxStockPreferencesPage::Kind stockKind = kind == WXD_PREFERENCES_PAGE_GENERAL ?
                                                 wxStockPreferencesPage::Kind_General :
                                                 wxStockPreferencesPage::Kind_Advanced;
    reinterpret_cast<wxPreferencesEditor*>(editor)->AddPage(
        new WxdStockPreferencesPage(stockKind, create, drop, userData));
}

void
wxd_PreferencesEditor_Show(wxd_PreferencesEditor_t* editor, wxd_Window_t* parent)
{
    if (!editor)
        return;
    reinterpret_cast<wxPreferencesEditor*>(editor)->Show(reinterpret_cast<wxWindow*>(parent));
}

void
wxd_PreferencesEditor_Dismiss(wxd_PreferencesEditor_t* editor)
{
    if (!editor)
        return;
    reinterpret_cast<wxPreferencesEditor*>(editor)->Dismiss();
}

bool
wxd_PreferencesEditor_ShouldApplyChangesImmediately(void)
{ return wxPreferencesEditor::ShouldApplyChangesImmediately(); }

bool
wxd_PreferencesEditor_ShownModally(void)
{ return wxPreferencesEditor::ShownModally(); }

} // extern "C"
