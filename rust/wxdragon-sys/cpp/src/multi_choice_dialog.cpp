#include <wx/wxprec.h>
#include <wx/wx.h>
#include "../include/wxdragon.h"
#include "wxd_utils.h"

#include <wx/choicdlg.h>
#include <wx/listbox.h>
#include <wx/checklst.h> // For wxCheckListBox

WXD_EXPORTED wxd_MultiChoiceDialog_t*
wxd_MultiChoiceDialog_Create(const wxd_Window_t* parent, const char* message, const char* caption,
                             const wxd_ArrayString_t* choices, wxd_Style_t style, int x, int y,
                             int width, int height)
{
    wxWindow* parent_wx = (wxWindow*)parent;
    const wxArrayString* wxChoices = reinterpret_cast<const wxArrayString*>(choices);

    // Default position/size if not specified
    wxPoint pos = (x == -1 && y == -1) ? wxDefaultPosition : wxPoint(x, y);

    wxMultiChoiceDialog* dialog =
        new wxMultiChoiceDialog(parent_wx, WXD_STR_TO_WX_STRING_UTF8_NULL_OK(message),
                                WXD_STR_TO_WX_STRING_UTF8_NULL_OK(caption), *wxChoices, style);

    // Set position and size if provided
    if (x != -1 && y != -1) {
        dialog->SetPosition(pos);
    }
    if (width != -1 && height != -1) {
        dialog->SetSize(width, height);
    }

    return reinterpret_cast<wxd_MultiChoiceDialog_t*>(dialog);
}

WXD_EXPORTED void
wxd_MultiChoiceDialog_GetSelections(const wxd_MultiChoiceDialog_t* self, int* selections,
                                    int* count)
{
    if (!self || !selections || !count)
        return;

    const wxMultiChoiceDialog* dialog = reinterpret_cast<const wxMultiChoiceDialog*>(self);
    wxArrayInt selectedItems = dialog->GetSelections();

    // Handle the case where no selections exist
    if (selectedItems.IsEmpty()) {
        *count = 0;
        return;
    }

    // Store the selection count
    *count = selectedItems.GetCount();

    // Copy the selections to the provided array
    for (size_t i = 0; i < selectedItems.GetCount(); i++) {
        selections[i] = selectedItems[i];
    }
}

WXD_EXPORTED void
wxd_MultiChoiceDialog_SetSelections(wxd_MultiChoiceDialog_t* self, const int* selections, int count)
{
    if (!self || !selections || count <= 0)
        return;

    wxMultiChoiceDialog* dialog = (wxMultiChoiceDialog*)self;
    wxArrayInt selectionArray;

    // Copy the selections from the provided array
    for (int i = 0; i < count; i++) {
        selectionArray.Add(selections[i]);
    }

    dialog->SetSelections(selectionArray);
}

// We need to get the original choices array that was passed to the dialog constructor
// and use the selection indices to retrieve the chosen strings
WXD_EXPORTED void
wxd_MultiChoiceDialog_GetStringSelections(const wxd_MultiChoiceDialog_t* self,
                                          wxd_ArrayString_t* selections)
{
    if (!self || !selections)
        return;

    wxMultiChoiceDialog* dialog = (wxMultiChoiceDialog*)self;
    wxArrayInt selectedIndices = dialog->GetSelections();

    // Get the underlying wxArrayString from the wxd_ArrayString_t
    wxArrayString* wxSelections = reinterpret_cast<wxArrayString*>(selections);

    // Clear the target array
    wxSelections->Clear();

    // Try to find the wxCheckListBox control inside the dialog
    // First look for children of type wxCheckListBox
    wxWindow* contentWin = wxDynamicCast(dialog->GetContentWindow(), wxWindow);
    if (!contentWin) {
        // If we can't get content window, try dialog itself
        contentWin = dialog;
    }

    // Find the first wxCheckListBox in the hierarchy
    wxCheckListBox* checkListBox = NULL;
    wxWindowList& children = contentWin->GetChildren();
    for (wxWindowList::iterator it = children.begin(); it != children.end(); ++it) {
        wxWindow* child = *it;
        checkListBox = wxDynamicCast(child, wxCheckListBox);
        if (checkListBox) {
            break;
        }

        // Try one level deeper
        wxWindowList& grandchildren = child->GetChildren();
        for (wxWindowList::iterator git = grandchildren.begin(); git != grandchildren.end();
             ++git) {
            wxWindow* grandchild = *git;
            checkListBox = wxDynamicCast(grandchild, wxCheckListBox);
            if (checkListBox) {
                break;
            }
        }

        if (checkListBox) {
            break;
        }
    }

    if (checkListBox) {
        // Found a wxCheckListBox, get the strings from it
        for (size_t i = 0; i < selectedIndices.GetCount(); i++) {
            int index = selectedIndices[i];
            if (index >= 0 && index < checkListBox->GetCount()) {
                wxSelections->Add(checkListBox->GetString(index));
            }
        }
    }
    else {
        // Fallback: Get the choices from the original choice array from our choices parameter
        // Since we don't have access to it directly, we'll use a workaround
        // if the dialog has been shown, we'll go through the dialog children to find a listbox-like control

        // Get the strings directly from the dialog's content if possible
        // This is a workaround for getting the strings when the dialog is still open
        wxArrayString allChoices;
        bool foundChoices = false;

        // Look for any listbox or similar control
        wxListBox* listBox = NULL;
        for (wxWindowList::iterator it = children.begin(); it != children.end(); ++it) {
            wxWindow* child = *it;
            listBox = wxDynamicCast(child, wxListBox);
            if (listBox) {
                break;
            }

            // Try one level deeper
            wxWindowList& grandchildren = child->GetChildren();
            for (wxWindowList::iterator git = grandchildren.begin(); git != grandchildren.end();
                 ++git) {
                wxWindow* grandchild = *git;
                listBox = wxDynamicCast(grandchild, wxListBox);
                if (listBox) {
                    break;
                }
            }

            if (listBox) {
                break;
            }
        }

        if (listBox) {
            // Found a listbox, get all the strings
            for (unsigned int i = 0; i < listBox->GetCount(); i++) {
                allChoices.Add(listBox->GetString(i));
            }
            foundChoices = true;
        }

        if (foundChoices) {
            // Now we have all choices, get the selected ones
            for (size_t i = 0; i < selectedIndices.GetCount(); i++) {
                int index = selectedIndices[i];
                if (index >= 0 && index < (int)allChoices.GetCount()) {
                    wxSelections->Add(allChoices[index]);
                }
            }
        }
        else {
            // Last resort: If we can't find any control with the choices
            // At least add placeholders for debugging
            for (size_t i = 0; i < selectedIndices.GetCount(); i++) {
                wxSelections->Add(wxString::Format("Item #%d", selectedIndices[i]));
            }
        }
    }
}