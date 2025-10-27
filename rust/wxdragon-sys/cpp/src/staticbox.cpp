#include "wx/wxprec.h"

#ifdef __BORLANDC__
#pragma hdrstop
#endif

#ifndef WX_PRECOMP
#include "wx/wx.h"
#endif

#include "wx/statbox.h"
#include "../include/wxdragon.h"

extern "C" {

// --- wxStaticBox Functions ---

WXD_EXPORTED wxd_StaticBox_t*
wxd_StaticBox_Create(wxd_Window_t* parent, wxd_Id id, const char* label, wxd_Point pos,
                     wxd_Size size, wxd_Style_t style)
{
    wxWindow* parentWin = (wxWindow*)parent;
    wxStaticBox* box =
        new wxStaticBox(parentWin, id,
                        wxString::FromUTF8(label ? label : ""), // Ensure non-null label
                        wxPoint(pos.x, pos.y), wxSize(size.width, size.height), style);

    return (wxd_StaticBox_t*)box;
}

} // extern "C"