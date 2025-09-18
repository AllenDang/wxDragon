#include <wx/wxprec.h>
#include <wx/wx.h>
#include "wxdragon.h"
#include "wx/dialog.h"

extern "C" {

wxd_Dialog_t* wxd_Dialog_Create(wxd_Window_t* parent, const char* title, wxd_Style_t style, int x, int y, int width, int height) {
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

int wxd_Dialog_ShowModal(wxd_Dialog* self) {
    if (!self) return wxID_NONE; // Or some other error indicator, wxDialog::ShowModal returns int
    return ((wxDialog*)self)->ShowModal();
}

void wxd_Dialog_EndModal(wxd_Dialog* self, int retCode) {
    if (!self) return;
    ((wxDialog*)self)->EndModal(retCode);
}

// Note: wxDialog itself is usually not created directly with a simple 'Create' function in this C API.
// Derived dialogs (like wxMessageDialog) will have their own creation functions that return a wxd_Dialog* or wxd_SpecificDialog* castable to wxd_Dialog*.
// Destruction is handled by wxd_Window_Destroy, as wxDialog inherits from wxWindow.

} // extern "C" 