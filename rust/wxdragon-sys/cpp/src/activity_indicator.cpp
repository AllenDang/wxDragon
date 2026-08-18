#include "../include/wxdragon.h" // Use correct path for wxdragon.h
#include <wx/activityindicator.h>
#include <wx/gdicmn.h> // For wxPoint, wxSize, wxDefaultPosition, wxDefaultSize
#include <wx/window.h> // For wxWindow parent type

WXD_EXPORTED wxd_ActivityIndicator_t*
wxd_ActivityIndicator_Create(wxd_Window_t* parent, int id, int x, int y, int w, int h,
                             int64_t style)
{
    wxWindow* p = (wxWindow*)parent;
    wxPoint pos = wxd_cpp_utils::to_wx(wxd_Point{x, y});
    wxSize size = wxd_cpp_utils::to_wx(wxd_Size{w, h});
    // wxActivityIndicator does not take a style parameter in its constructor.
    // The 'style' parameter from C API is currently ignored for this widget.
    // It could be used if there were relevant window styles to apply.
    wxActivityIndicator* indicator = new wxActivityIndicator(p, id, pos, size);
    return (wxd_ActivityIndicator_t*)indicator;
}

WXD_EXPORTED void
wxd_ActivityIndicator_Start(wxd_ActivityIndicator_t* self)
{
    wxActivityIndicator* indicator = (wxActivityIndicator*)self;
    if (!indicator)
        return;
    indicator->Start();
}

WXD_EXPORTED void
wxd_ActivityIndicator_Stop(wxd_ActivityIndicator_t* self)
{
    wxActivityIndicator* indicator = (wxActivityIndicator*)self;
    if (!indicator)
        return;
    indicator->Stop();
}

WXD_EXPORTED bool
wxd_ActivityIndicator_IsRunning(wxd_ActivityIndicator_t* self)
{
    wxActivityIndicator* indicator = (wxActivityIndicator*)self;
    if (!indicator)
        return false;
    return indicator->IsRunning();
}