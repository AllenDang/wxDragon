#include <wx/wxprec.h>
#include <wx/wx.h>
#include <wx/dnd.h>
#include <wx/dataobj.h>
#include <wx/window.h>
#include "../include/wxdragon.h" // Include the FFI header

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
