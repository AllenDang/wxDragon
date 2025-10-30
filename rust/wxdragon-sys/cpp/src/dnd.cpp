#include <wx/wxprec.h>
#include <wx/wx.h>
#include <wx/dnd.h>
#include <wx/dataobj.h>
#include <wx/window.h>
#include "../include/wxdragon.h" // Include the FFI header

// --- wxDataObject implementations ---

// TextDataObject functions are now implemented in dataobject.cpp to avoid duplicate symbols
// The following functions are commented out:
// - wxd_TextDataObject_Create
// - wxd_TextDataObject_Destroy
// - wxd_TextDataObject_SetText

// FileDataObject
// These functions are now defined in dataobject.cpp
/*
extern "C" WXDRAGON_API wxd_FileDataObject_t* wxd_FileDataObject_Create() {
    return reinterpret_cast<wxd_FileDataObject_t*>(new wxFileDataObject());
}

extern "C" WXDRAGON_API void wxd_FileDataObject_Destroy(wxd_FileDataObject_t* obj) {
    if (obj) {
        delete reinterpret_cast<wxFileDataObject*>(obj);
    }
}

extern "C" WXDRAGON_API int wxd_FileDataObject_GetFilenames(wxd_FileDataObject_t* obj, wxd_ArrayString_t* filenames) {
    if (!obj || !filenames) return 0;
    
    wxFileDataObject* file_obj = reinterpret_cast<wxFileDataObject*>(obj);
    wxArrayString wx_filenames = file_obj->GetFilenames();
    
    if (!filenames->internal_data) {
        filenames->internal_data = new wxArrayString(wx_filenames);
    } else {
        *reinterpret_cast<wxArrayString*>(filenames->internal_data) = wx_filenames;
    }
    
    return wx_filenames.GetCount();
}

extern "C" WXDRAGON_API void wxd_FileDataObject_AddFile(wxd_FileDataObject_t* obj, const char* filename) {
    if (!obj || !filename) return;
    
    wxFileDataObject* file_obj = reinterpret_cast<wxFileDataObject*>(obj);
    wxString wx_filename = wxString::FromUTF8(filename);
    file_obj->AddFile(wx_filename);
}
*/

// --- DropSource implementations ---

extern "C" WXDRAGON_API wxd_DropSource_t*
wxd_DropSource_Create(wxd_Window_t* window)
{
    wxWindow* wx_window = reinterpret_cast<wxWindow*>(window);
    if (!wx_window)
        return nullptr;

    return reinterpret_cast<wxd_DropSource_t*>(new wxDropSource(wx_window));
}

extern "C" WXDRAGON_API void
wxd_DropSource_Destroy(wxd_DropSource_t* source)
{
    if (source) {
        delete reinterpret_cast<wxDropSource*>(source);
    }
}

extern "C" WXDRAGON_API void
wxd_DropSource_SetData(wxd_DropSource_t* source, wxd_DataObject_t* data)
{
    if (!source || !data)
        return;

    wxDropSource* drop_source = reinterpret_cast<wxDropSource*>(source);
    wxDataObject* data_obj = reinterpret_cast<wxDataObject*>(data);

    drop_source->SetData(*data_obj);
}

extern "C" WXDRAGON_API WXDDragResultCEnum
wxd_DropSource_DoDragDrop(wxd_DropSource_t* source, bool allow_move)
{
    if (!source)
        return WXD_DRAG_ERROR;

    wxDropSource* drop_source = reinterpret_cast<wxDropSource*>(source);
    wxDragResult result = drop_source->DoDragDrop(allow_move ? wxDrag_AllowMove : wxDrag_CopyOnly);

    switch (result) {
    case wxDragNone:
        return WXD_DRAG_NONE;
    case wxDragCopy:
        return WXD_DRAG_COPY;
    case wxDragMove:
        return WXD_DRAG_MOVE;
    case wxDragLink:
        return WXD_DRAG_LINK;
    case wxDragCancel:
        return WXD_DRAG_CANCEL;
    case wxDragError:
        return WXD_DRAG_ERROR;
    default:
        return WXD_DRAG_ERROR;
    }
}

// --- Custom DropTarget implementations ---

// Custom implementation for TextDropTarget to allow Rust callback
class WxdTextDropTarget : public wxTextDropTarget {
private:
    typedef bool (*OnDropTextFn)(const char* text, int x, int y, void* closure_ptr);
    OnDropTextFn m_on_drop_text_fn;
    void* m_closure_ptr;

public:
    WxdTextDropTarget(OnDropTextFn on_drop_text_fn, void* closure_ptr)
        : m_on_drop_text_fn(on_drop_text_fn), m_closure_ptr(closure_ptr)
    {
    }

    virtual ~WxdTextDropTarget()
    {
        // Free the closure data when the drop target is destroyed
        if (m_closure_ptr) {
            drop_rust_event_closure_box(m_closure_ptr);
            m_closure_ptr = nullptr;
        }
    }

    virtual bool
    OnDropText(wxCoord x, wxCoord y, const wxString& text) override
    {
        if (m_on_drop_text_fn && m_closure_ptr) {
            wxScopedCharBuffer utf8 = text.utf8_str();
            return m_on_drop_text_fn(utf8.data(), x, y, m_closure_ptr);
        }
        return false;
    }
};

extern "C" WXDRAGON_API wxd_TextDropTarget_t*
wxd_TextDropTarget_Create(wxd_Window_t* window, void* rust_on_drop_text_fn, void* rust_closure_ptr)
{
    wxWindow* wx_window = reinterpret_cast<wxWindow*>(window);
    if (!wx_window || !rust_on_drop_text_fn || !rust_closure_ptr)
        return nullptr;

    WxdTextDropTarget* target = new WxdTextDropTarget(
        reinterpret_cast<bool (*)(const char*, int, int, void*)>(rust_on_drop_text_fn),
        rust_closure_ptr);

    wx_window->SetDropTarget(target);

    return reinterpret_cast<wxd_TextDropTarget_t*>(target);
}

extern "C" WXDRAGON_API void
wxd_TextDropTarget_Destroy(wxd_TextDropTarget_t* target)
{
    // Not needed - wxWidgets will automatically delete the drop target when the window is destroyed
    // or when a new drop target is set
}

// Custom implementation for FileDropTarget to allow Rust callback
class WxdFileDropTarget : public wxFileDropTarget {
private:
    typedef bool (*OnDropFilesFn)(const wxArrayString* filenames, int x, int y, void* closure_ptr);
    OnDropFilesFn m_on_drop_files_fn;
    void* m_closure_ptr;

public:
    WxdFileDropTarget(OnDropFilesFn on_drop_files_fn, void* closure_ptr)
        : m_on_drop_files_fn(on_drop_files_fn), m_closure_ptr(closure_ptr)
    {
    }

    virtual ~WxdFileDropTarget()
    {
        // Free the closure data when the drop target is destroyed
        if (m_closure_ptr) {
            drop_rust_event_closure_box(m_closure_ptr);
            m_closure_ptr = nullptr;
        }
    }

    virtual bool
    OnDropFiles(wxCoord x, wxCoord y, const wxArrayString& filenames) override
    {
        if (m_on_drop_files_fn && m_closure_ptr) {
            return m_on_drop_files_fn(&filenames, x, y, m_closure_ptr);
        }
        return false;
    }
};

extern "C" WXDRAGON_API wxd_FileDropTarget_t*
wxd_FileDropTarget_Create(wxd_Window_t* window, void* rust_on_drop_files_fn, void* rust_closure_ptr)
{
    wxWindow* wx_window = reinterpret_cast<wxWindow*>(window);
    if (!wx_window || !rust_on_drop_files_fn || !rust_closure_ptr)
        return nullptr;

    WxdFileDropTarget* target = new WxdFileDropTarget(
        reinterpret_cast<bool (*)(const wxArrayString*, int, int, void*)>(rust_on_drop_files_fn),
        rust_closure_ptr);

    wx_window->SetDropTarget(target);

    return reinterpret_cast<wxd_FileDropTarget_t*>(target);
}

extern "C" WXDRAGON_API void
wxd_FileDropTarget_Destroy(wxd_FileDropTarget_t* target)
{
    // Not needed - wxWidgets will automatically delete the drop target when the window is destroyed
    // or when a new drop target is set
}

// Window drop target setter
extern "C" WXDRAGON_API void
wxd_Window_SetDropTarget(wxd_Window_t* window, wxd_DropTarget_t* target)
{
    wxWindow* wx_window = reinterpret_cast<wxWindow*>(window);
    wxDropTarget* wx_target = reinterpret_cast<wxDropTarget*>(target);

    if (wx_window) {
        wx_window->SetDropTarget(wx_target);
    }
}