#include <wx/wxprec.h>
#include <wx/wx.h>
#include "../include/wxdragon.h"
#include <wx/dnd.h>
#include <wx/tokenzr.h>

// Full-featured text drop target implementation
class WxdTextDropTargetFull : public wxTextDropTarget {
public:
    WxdTextDropTargetFull(wxd_OnEnter_Callback onEnter, wxd_OnDragOver_Callback onDragOver,
                          wxd_OnLeave_Callback onLeave, wxd_OnDrop_Callback onDrop,
                          wxd_OnData_Callback onData, wxd_OnDropText_Callback onDropText,
                          void* userData, wxd_FreeUserData_Callback freeUserData)
        : m_onEnter(onEnter), m_onDragOver(onDragOver), m_onLeave(onLeave), m_onDrop(onDrop),
          m_onData(onData), m_onDropText(onDropText), m_userData(userData),
          m_freeUserData(freeUserData)
    {
        WXD_LOG_TRACE("WxdTextDropTargetFull created");
    }

    virtual ~WxdTextDropTargetFull()
    {
        WXD_LOG_TRACE("WxdTextDropTargetFull destroyed");
        if (m_userData && m_freeUserData) {
            m_freeUserData(m_userData);
            m_userData = nullptr;
        }
    }

    virtual wxDragResult
    OnEnter(wxCoord x, wxCoord y, wxDragResult defResult) override
    {
        if (m_onEnter) {
            wxd_DragResult result =
                m_onEnter(x, y, static_cast<wxd_DragResult>(defResult), m_userData);
            return static_cast<wxDragResult>(result);
        }
        return wxTextDropTarget::OnEnter(x, y, defResult);
    }

    virtual wxDragResult
    OnDragOver(wxCoord x, wxCoord y, wxDragResult defResult) override
    {
        if (m_onDragOver) {
            wxd_DragResult result =
                m_onDragOver(x, y, static_cast<wxd_DragResult>(defResult), m_userData);
            return static_cast<wxDragResult>(result);
        }
        return wxTextDropTarget::OnDragOver(x, y, defResult);
    }

    virtual void
    OnLeave() override
    {
        if (m_onLeave) {
            m_onLeave(m_userData);
        }
        else {
            wxTextDropTarget::OnLeave();
        }
    }

    virtual bool
    OnDrop(wxCoord x, wxCoord y) override
    {
        if (m_onDrop) {
            return m_onDrop(x, y, m_userData);
        }
        return wxTextDropTarget::OnDrop(x, y);
    }

    virtual wxDragResult
    OnData(wxCoord x, wxCoord y, wxDragResult defResult) override
    {
        // Call the base implementation to get the data
        wxDragResult baseResult = wxTextDropTarget::OnData(x, y, defResult);

        // If the base implementation succeeded and we have custom handler
        if (baseResult != wxDragNone && m_onData) {
            wxd_DragResult result =
                m_onData(x, y, static_cast<wxd_DragResult>(defResult), m_userData);
            return static_cast<wxDragResult>(result);
        }

        return baseResult;
    }

    virtual bool
    OnDropText(wxCoord x, wxCoord y, const wxString& text) override
    {
        if (m_onDropText) {
            wxScopedCharBuffer utf8 = text.utf8_str();
            return m_onDropText(utf8.data(), x, y, m_userData);
        }
        return false;
    }

private:
    wxd_OnEnter_Callback m_onEnter;
    wxd_OnDragOver_Callback m_onDragOver;
    wxd_OnLeave_Callback m_onLeave;
    wxd_OnDrop_Callback m_onDrop;
    wxd_OnData_Callback m_onData;
    wxd_OnDropText_Callback m_onDropText;
    void* m_userData;
    wxd_FreeUserData_Callback m_freeUserData;
};

// Full-featured file drop target implementation
class WxdFileDropTargetFull : public wxFileDropTarget {
public:
    WxdFileDropTargetFull(wxd_OnEnter_Callback onEnter, wxd_OnDragOver_Callback onDragOver,
                          wxd_OnLeave_Callback onLeave, wxd_OnDrop_Callback onDrop,
                          wxd_OnData_Callback onData, wxd_OnDropFiles_Callback onDropFiles,
                          void* userData, wxd_FreeUserData_Callback freeUserData)
        : m_onEnter(onEnter), m_onDragOver(onDragOver), m_onLeave(onLeave), m_onDrop(onDrop),
          m_onData(onData), m_onDropFiles(onDropFiles), m_userData(userData),
          m_freeUserData(freeUserData)
    {
        WXD_LOG_TRACE("WxdFileDropTargetFull created");
    }

    virtual ~WxdFileDropTargetFull()
    {
        WXD_LOG_TRACE("WxdFileDropTargetFull destroyed");
        if (m_userData && m_freeUserData) {
            m_freeUserData(m_userData);
            m_userData = nullptr;
        }
    }

    virtual wxDragResult
    OnEnter(wxCoord x, wxCoord y, wxDragResult defResult) override
    {
        if (m_onEnter) {
            wxd_DragResult result =
                m_onEnter(x, y, static_cast<wxd_DragResult>(defResult), m_userData);
            return static_cast<wxDragResult>(result);
        }
        return wxFileDropTarget::OnEnter(x, y, defResult);
    }

    virtual wxDragResult
    OnDragOver(wxCoord x, wxCoord y, wxDragResult defResult) override
    {
        if (m_onDragOver) {
            wxd_DragResult result =
                m_onDragOver(x, y, static_cast<wxd_DragResult>(defResult), m_userData);
            return static_cast<wxDragResult>(result);
        }
        return wxFileDropTarget::OnDragOver(x, y, defResult);
    }

    virtual void
    OnLeave() override
    {
        if (m_onLeave) {
            m_onLeave(m_userData);
        }
        else {
            wxFileDropTarget::OnLeave();
        }
    }

    virtual bool
    OnDrop(wxCoord x, wxCoord y) override
    {
        if (m_onDrop) {
            return m_onDrop(x, y, m_userData);
        }
        return wxFileDropTarget::OnDrop(x, y);
    }

    virtual wxDragResult
    OnData(wxCoord x, wxCoord y, wxDragResult defResult) override
    {
        // Call the base implementation to get the data
        wxDragResult baseResult = wxFileDropTarget::OnData(x, y, defResult);

        // If the base implementation succeeded and we have custom handler
        if (baseResult != wxDragNone && m_onData) {
            wxd_DragResult result =
                m_onData(x, y, static_cast<wxd_DragResult>(defResult), m_userData);
            return static_cast<wxDragResult>(result);
        }

        return baseResult;
    }

    virtual bool
    OnDropFiles(wxCoord x, wxCoord y, const wxArrayString& filenames) override
    {
        if (m_onDropFiles) {
            return m_onDropFiles(reinterpret_cast<const wxd_ArrayString_t*>(&filenames), x, y,
                                 m_userData);
        }
        return false;
    }

private:
    wxd_OnEnter_Callback m_onEnter;
    wxd_OnDragOver_Callback m_onDragOver;
    wxd_OnLeave_Callback m_onLeave;
    wxd_OnDrop_Callback m_onDrop;
    wxd_OnData_Callback m_onData;
    wxd_OnDropFiles_Callback m_onDropFiles;
    void* m_userData;
    wxd_FreeUserData_Callback m_freeUserData;
};

extern "C" {

// Create text drop target with full callback set
WXD_EXPORTED wxd_TextDropTarget_t*
wxd_TextDropTarget_CreateFull(wxd_Window_t* window, wxd_OnEnter_Callback onEnter,
                              wxd_OnDragOver_Callback onDragOver, wxd_OnLeave_Callback onLeave,
                              wxd_OnDrop_Callback onDrop, wxd_OnData_Callback onData,
                              wxd_OnDropText_Callback onDropText, void* userData,
                              wxd_FreeUserData_Callback freeUserData)
{
    if (!window)
        return nullptr;

    wxWindow* wx_window = reinterpret_cast<wxWindow*>(window);

    WxdTextDropTargetFull* drop_target = new WxdTextDropTargetFull(onEnter, onDragOver, onLeave,
                                                                   onDrop, onData, onDropText,
                                                                   userData, freeUserData);

    wx_window->SetDropTarget(drop_target);

    return reinterpret_cast<wxd_TextDropTarget_t*>(drop_target);
}

// Create file drop target with full callback set
WXD_EXPORTED wxd_FileDropTarget_t*
wxd_FileDropTarget_CreateFull(wxd_Window_t* window, wxd_OnEnter_Callback onEnter,
                              wxd_OnDragOver_Callback onDragOver, wxd_OnLeave_Callback onLeave,
                              wxd_OnDrop_Callback onDrop, wxd_OnData_Callback onData,
                              wxd_OnDropFiles_Callback onDropFiles, void* userData,
                              wxd_FreeUserData_Callback freeUserData)
{
    if (!window)
        return nullptr;

    wxWindow* wx_window = reinterpret_cast<wxWindow*>(window);

    WxdFileDropTargetFull* drop_target = new WxdFileDropTargetFull(onEnter, onDragOver, onLeave,
                                                                   onDrop, onData, onDropFiles,
                                                                   userData, freeUserData);

    wx_window->SetDropTarget(drop_target);

    return reinterpret_cast<wxd_FileDropTarget_t*>(drop_target);
}

} // extern "C"