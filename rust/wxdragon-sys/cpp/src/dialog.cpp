#include <wx/wxprec.h>
#include <wx/wx.h>
#include "wxdragon.h"
#include "wx/dialog.h"

extern "C" {

wxd_Dialog_t*
wxd_Dialog_Create(wxd_Window_t* parent, const char* title, wxd_Style_t style, int x, int y,
                  int width, int height)
{
    wxWindow* wx_parent = (wxWindow*)parent;
    wxString wx_title = wxString::FromUTF8(title ? title : "");

    // Use wxDefaultPosition and wxDefaultSize if coordinates are -1 (default)
    wxPoint pos = (x == -1 && y == -1) ? wxDefaultPosition : wxPoint(x, y);
    wxSize size = (width == -1 && height == -1) ? wxDefaultSize : wxSize(width, height);

    // Create the dialog with the provided parameters
    wxDialog* dialog = new wxDialog();
    if (!dialog->Create(wx_parent, wxID_ANY, wx_title, pos, size, style)) {
        delete dialog;
        return nullptr;
    }

    return (wxd_Dialog_t*)dialog;
}

void
wxd_Dialog_SetIcon(wxd_Dialog* dlg, const wxd_Bitmap_t* bitmap)
{
    if (!dlg || !bitmap)
        return;

    wxDialog* dialog = reinterpret_cast<wxDialog*>(dlg);
    const wxBitmap* bmp = reinterpret_cast<const wxBitmap*>(bitmap);

    if (!bmp->IsOk())
        return;

    wxIcon icon;
    icon.CopyFromBitmap(*bmp);
    if (icon.IsOk()) {
        dialog->SetIcon(icon);
    }
}

int
wxd_Dialog_ShowModal(wxd_Dialog* self)
{
    if (!self)
        return wxID_NONE; // Or some other error indicator, wxDialog::ShowModal returns int
    return ((wxDialog*)self)->ShowModal();
}

void
wxd_Dialog_EndModal(wxd_Dialog* self, int retCode)
{
    if (!self)
        return;
    ((wxDialog*)self)->EndModal(retCode);
}

void
wxd_Dialog_SetEscapeId(wxd_Dialog* self, int id)
{
    if (!self)
        return;
    ((wxDialog*)self)->SetEscapeId(id);
}

int
wxd_Dialog_GetEscapeId(wxd_Dialog* self)
{
    if (!self)
        return wxID_NONE;
    return ((wxDialog*)self)->GetEscapeId();
}

void
wxd_Dialog_SetAffirmativeId(wxd_Dialog* self, int id)
{
    if (!self)
        return;
    ((wxDialog*)self)->SetAffirmativeId(id);
}

int
wxd_Dialog_GetAffirmativeId(wxd_Dialog* self)
{
    if (!self)
        return wxID_OK; // Default is wxID_OK
    return ((wxDialog*)self)->GetAffirmativeId();
}

void
wxd_Dialog_SetReturnCode(wxd_Dialog* self, int retCode)
{
    if (!self)
        return;
    ((wxDialog*)self)->SetReturnCode(retCode);
}

int
wxd_Dialog_GetReturnCode(wxd_Dialog* self)
{
    if (!self)
        return 0;
    return ((wxDialog*)self)->GetReturnCode();
}

// Note: wxDialog itself is usually not created directly with a simple 'Create' function in this C API.
// Derived dialogs (like wxMessageDialog) will have their own creation functions that return a wxd_Dialog* or wxd_SpecificDialog* castable to wxd_Dialog*.
// Destruction is handled by wxd_Window_Destroy, as wxDialog inherits from wxWindow.

} // extern "C"