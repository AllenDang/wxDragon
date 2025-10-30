#include <wx/wxprec.h>
#include <wx/wx.h>
#include "../include/wxdragon.h"
#include <wx/window.h>
#include <wx/editlbox.h>

// Cast helpers for EditableListBox
#define TO_WX_EDITABLELB(x) ((wxEditableListBox*)(x))

// Create a new wxEditableListBox
wxd_Window_t*
wxd_EditableListBox_New(wxd_Window_t* parent, int id, const char* label, int x, int y, int width,
                        int height, int64_t style)
{
    wxString wx_label = WXD_STR_TO_WX_STRING_UTF8_NULL_OK(label);

    wxEditableListBox* editablelistbox = new wxEditableListBox((wxWindow*)parent, id, wx_label,
                                                               wxPoint(x, y), wxSize(width, height),
                                                               style);

    return (wxd_Window_t*)editablelistbox;
}

// Set the string items in the wxEditableListBox
void
wxd_EditableListBox_SetStrings(wxd_Window_t* self, const char** strings, int count)
{
    if (!self || !strings || count < 0) {
        return;
    }

    wxArrayString wx_strings;
    for (int i = 0; i < count; i++) {
        if (strings[i]) {
            wx_strings.Add(wxString::FromUTF8(strings[i]));
        }
        else {
            wx_strings.Add(wxString());
        }
    }

    TO_WX_EDITABLELB(self)->SetStrings(wx_strings);
}

// Add a string item to the wxEditableListBox
void
wxd_EditableListBox_AddString(wxd_Window_t* self, const char* string)
{
    if (!self) {
        return;
    }

    wxString wx_string = WXD_STR_TO_WX_STRING_UTF8_NULL_OK(string);

    // Get current strings
    wxArrayString strings;
    TO_WX_EDITABLELB(self)->GetStrings(strings);

    // Add the new string
    strings.Add(wx_string);

    // Set the updated strings
    TO_WX_EDITABLELB(self)->SetStrings(strings);
}

// Get the underlying wxListBox from the wxEditableListBox
wxd_Window_t*
wxd_EditableListBox_GetListCtrl(wxd_Window_t* self)
{
    if (!self) {
        return NULL;
    }

    wxListCtrl* list_ctrl = TO_WX_EDITABLELB(self)->GetListCtrl();
    return (wxd_Window_t*)list_ctrl;
}

wxd_ArrayString_t*
wxd_EditableListBox_CopyStringsToArrayString(wxd_Window_t* self_ptr)
{
    if (!self_ptr)
        return nullptr;
    wxEditableListBox* el = (wxEditableListBox*)self_ptr;
    if (!el)
        return nullptr; // Extra check

    wxArrayString* result_array = new wxArrayString(); // wxd_ArrayString_t is wxArrayString*
    el->GetStrings(*result_array);                     // Fill the wxArrayString

    return (wxd_ArrayString_t*)result_array;
}