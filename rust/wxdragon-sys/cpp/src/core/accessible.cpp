#include <wx/wxprec.h>
#include <wx/wx.h>
#include "../../include/wxdragon.h"

#if wxUSE_ACCESSIBILITY
#include <wx/access.h>

class WxdCustomAccessible : public wxAccessible {
public:
    WxdCustomAccessible(wxWindow* win, wxd_AccessibleCallbacks callbacks, void* userData)
        : wxAccessible(win), m_callbacks(callbacks), m_userData(userData)
    {}

    wxAccStatus GetChildCount(int* childCount) override {
        if (m_callbacks.GetChildCount) {
            return (wxAccStatus)m_callbacks.GetChildCount(m_userData, childCount);
        }
        return wxACC_NOT_IMPLEMENTED;
    }

    wxAccStatus GetChild(int childId, wxAccessible** child) override {
        if (m_callbacks.GetChild) {
            return (wxAccStatus)m_callbacks.GetChild(m_userData, childId, reinterpret_cast<wxd_Accessible_t**>(child));
        }
        return wxACC_NOT_IMPLEMENTED;
    }

    wxAccStatus GetParent(wxAccessible** parent) override {
        if (m_callbacks.GetParent) {
            return (wxAccStatus)m_callbacks.GetParent(m_userData, reinterpret_cast<wxd_Accessible_t**>(parent));
        }
        return wxACC_NOT_IMPLEMENTED;
    }

    wxAccStatus GetRole(int childId, wxAccRole* role) override {
        if (m_callbacks.GetRole) {
            return (wxAccStatus)m_callbacks.GetRole(m_userData, childId, reinterpret_cast<wxd_AccRole*>(role));
        }
        return wxACC_NOT_IMPLEMENTED;
    }

    wxAccStatus GetState(int childId, long* state) override {
        if (m_callbacks.GetState) {
            return (wxAccStatus)m_callbacks.GetState(m_userData, childId, state);
        }
        return wxACC_NOT_IMPLEMENTED;
    }

    wxAccStatus GetName(int childId, wxString* name) override {
        if (m_callbacks.GetName) {
            char buf[1024];
            wxd_AccStatus status = m_callbacks.GetName(m_userData, childId, buf, sizeof(buf));
            if (status == WXD_ACC_OK) {
                *name = wxString::FromUTF8(buf);
            }
            return (wxAccStatus)status;
        }
        return wxACC_NOT_IMPLEMENTED;
    }

    wxAccStatus GetDescription(int childId, wxString* description) override {
        if (m_callbacks.GetDescription) {
            char buf[1024];
            wxd_AccStatus status = m_callbacks.GetDescription(m_userData, childId, buf, sizeof(buf));
            if (status == WXD_ACC_OK) {
                *description = wxString::FromUTF8(buf);
            }
            return (wxAccStatus)status;
        }
        return wxACC_NOT_IMPLEMENTED;
    }

    wxAccStatus GetHelpText(int childId, wxString* helpText) override {
        if (m_callbacks.GetHelpText) {
            char buf[1024];
            wxd_AccStatus status = m_callbacks.GetHelpText(m_userData, childId, buf, sizeof(buf));
            if (status == WXD_ACC_OK) {
                *helpText = wxString::FromUTF8(buf);
            }
            return (wxAccStatus)status;
        }
        return wxACC_NOT_IMPLEMENTED;
    }

    wxAccStatus GetKeyboardShortcut(int childId, wxString* shortcut) override {
        if (m_callbacks.GetKeyboardShortcut) {
            char buf[1024];
            wxd_AccStatus status = m_callbacks.GetKeyboardShortcut(m_userData, childId, buf, sizeof(buf));
            if (status == WXD_ACC_OK) {
                *shortcut = wxString::FromUTF8(buf);
            }
            return (wxAccStatus)status;
        }
        return wxACC_NOT_IMPLEMENTED;
    }

    wxAccStatus GetDefaultAction(int childId, wxString* actionName) override {
        if (m_callbacks.GetDefaultAction) {
            char buf[1024];
            wxd_AccStatus status = m_callbacks.GetDefaultAction(m_userData, childId, buf, sizeof(buf));
            if (status == WXD_ACC_OK) {
                *actionName = wxString::FromUTF8(buf);
            }
            return (wxAccStatus)status;
        }
        return wxACC_NOT_IMPLEMENTED;
    }

    wxAccStatus GetValue(int childId, wxString* value) override {
        if (m_callbacks.GetValue) {
            char buf[1024];
            wxd_AccStatus status = m_callbacks.GetValue(m_userData, childId, buf, sizeof(buf));
            if (status == WXD_ACC_OK) {
                *value = wxString::FromUTF8(buf);
            }
            return (wxAccStatus)status;
        }
        return wxACC_NOT_IMPLEMENTED;
    }

    wxAccStatus Select(int childId, wxAccSelectionFlags selectFlags) override {
        if (m_callbacks.Select) {
            return (wxAccStatus)m_callbacks.Select(m_userData, childId, (int)selectFlags);
        }
        return wxACC_NOT_IMPLEMENTED;
    }

    wxAccStatus GetSelections(wxVariant* selections) override {
        if (m_callbacks.GetSelections) {
            return (wxAccStatus)m_callbacks.GetSelections(m_userData, reinterpret_cast<wxd_Variant_t*>(selections));
        }
        return wxACC_NOT_IMPLEMENTED;
    }

    wxAccStatus GetFocus(int* childId, wxAccessible** child) override {
        if (m_callbacks.GetFocus) {
            return (wxAccStatus)m_callbacks.GetFocus(m_userData, childId, reinterpret_cast<wxd_Accessible_t**>(child));
        }
        return wxACC_NOT_IMPLEMENTED;
    }

    wxAccStatus DoDefaultAction(int childId) override {
        if (m_callbacks.DoDefaultAction) {
            return (wxAccStatus)m_callbacks.DoDefaultAction(m_userData, childId);
        }
        return wxACC_NOT_IMPLEMENTED;
    }

    wxAccStatus GetLocation(wxRect& rect, int childId) override {
        if (m_callbacks.GetLocation) {
            wxd_Rect r;
            wxd_AccStatus status = m_callbacks.GetLocation(m_userData, childId, &r);
            if (status == WXD_ACC_OK) {
                rect = wxRect(r.x, r.y, r.width, r.height);
            }
            return (wxAccStatus)status;
        }
        return wxACC_NOT_IMPLEMENTED;
    }

    wxAccStatus HitTest(const wxPoint& pt, int* childId, wxAccessible** childObject) override {
        if (m_callbacks.HitTest) {
            wxd_Point p = { pt.x, pt.y };
            return (wxAccStatus)m_callbacks.HitTest(m_userData, p, childId, reinterpret_cast<wxd_Accessible_t**>(childObject));
        }
        return wxACC_NOT_IMPLEMENTED;
    }

    wxAccStatus Navigate(wxNavDir navDir, int fromId, int* toId, wxAccessible** toObject) override {
        if (m_callbacks.Navigate) {
            return (wxAccStatus)m_callbacks.Navigate(m_userData, (wxd_NavDir)navDir, fromId, toId, reinterpret_cast<wxd_Accessible_t**>(toObject));
        }
        return wxACC_NOT_IMPLEMENTED;
    }

private:
    wxd_AccessibleCallbacks m_callbacks;
    void* m_userData;
};
#endif

extern "C" {

wxd_Accessible_t*
wxd_Accessible_Create(wxd_Window_t* window, wxd_AccessibleCallbacks callbacks, void* userData)
{
#if wxUSE_ACCESSIBILITY
    return reinterpret_cast<wxd_Accessible_t*>(new WxdCustomAccessible(reinterpret_cast<wxWindow*>(window), callbacks, userData));
#else
    return nullptr;
#endif
}

void
wxd_Accessible_Destroy(wxd_Accessible_t* self)
{
#if wxUSE_ACCESSIBILITY
    if (self) {
        delete reinterpret_cast<wxAccessible*>(self);
    }
#endif
}

void
wxd_Accessible_NotifyEvent(uint32_t eventType, wxd_Window_t* window, int objectType, int objectId)
{
#if wxUSE_ACCESSIBILITY
    wxAccessible::NotifyEvent(eventType, reinterpret_cast<wxWindow*>(window),
                               static_cast<wxAccObject>(objectType), objectId);
#endif
}

void
wxd_Window_SetAccessible(wxd_Window_t* self, wxd_Accessible_t* accessible)
{
#if wxUSE_ACCESSIBILITY
    wxWindow* window = reinterpret_cast<wxWindow*>(self);
    wxAccessible* acc = reinterpret_cast<wxAccessible*>(accessible);
    if (window) {
        window->SetAccessible(acc);
    }
#endif
}

wxd_Accessible_t*
wxd_Window_GetAccessible(wxd_Window_t* self)
{
#if wxUSE_ACCESSIBILITY
    wxWindow* window = reinterpret_cast<wxWindow*>(self);
    if (window) {
        return reinterpret_cast<wxd_Accessible_t*>(window->GetAccessible());
    }
#endif
    return nullptr;
}

} // extern "C"
