#include <wx/wxprec.h>
#include <wx/wx.h>
#include "wx/rearrangectrl.h"
#include "wx/window.h"
#include "wx/string.h"
#include "wx/arrstr.h"
#include "../include/wxdragon.h"
#include "../include/array_string.h"
#include "wxd_utils.h"

// Helper function to convert wxArrayInt to a C array
static void
wxArrayIntToCArray(const wxArrayInt& wxArray, int* cArray, int arraySize)
{
    int size = wxMin(wxArray.GetCount(), static_cast<size_t>(arraySize));
    for (int i = 0; i < size; ++i) {
        cArray[i] = wxArray[i];
    }
}

// Helper function to convert C array to wxArrayInt
static wxArrayInt
CArrayToWxArrayInt(const int* cArray, int arraySize)
{
    wxArrayInt wxArray;
    for (int i = 0; i < arraySize; ++i) {
        wxArray.Add(cArray[i]);
    }
    return wxArray;
}

extern "C" {

wxd_RearrangeList_t*
wxd_RearrangeList_Create(wxd_Window_t* parent, wxd_Id id, wxd_Point pos, wxd_Size size,
                         const int* order, int orderCount, const char** items, int itemsCount,
                         wxd_Style_t style)
{
    wxWindow* wxParent = (wxWindow*)parent;
    if (!wxParent)
        return nullptr;

    // Convert C arrays to wxWidgets arrays
    wxArrayInt wxOrder = CArrayToWxArrayInt(order, orderCount);

    // Create a wxArrayString from char** using wxd_ArrayString utilities
    wxArrayString wxItems;
    for (int i = 0; i < itemsCount; ++i) {
        if (items[i]) {
            wxItems.Add(wxString::FromUTF8(items[i]));
        }
    }

    // Create the RearrangeList
    wxRearrangeList* list = new wxRearrangeList(wxParent, id, wxPoint(pos.x, pos.y),
                                                wxSize(size.width, size.height), wxOrder, wxItems,
                                                style);

    return (wxd_RearrangeList_t*)list;
}

void
wxd_RearrangeList_GetCurrentOrder(wxd_RearrangeList_t* self, int* orderArray, int arraySize)
{
    wxRearrangeList* list = (wxRearrangeList*)self;
    if (!list || !orderArray)
        return;

    wxArrayIntToCArray(list->GetCurrentOrder(), orderArray, arraySize);
}

bool
wxd_RearrangeList_MoveCurrentUp(wxd_RearrangeList_t* self)
{
    wxRearrangeList* list = (wxRearrangeList*)self;
    if (!list)
        return false;

    return list->MoveCurrentUp();
}

bool
wxd_RearrangeList_MoveCurrentDown(wxd_RearrangeList_t* self)
{
    wxRearrangeList* list = (wxRearrangeList*)self;
    if (!list)
        return false;

    return list->MoveCurrentDown();
}

bool
wxd_RearrangeList_CanMoveCurrentUp(wxd_RearrangeList_t* self)
{
    wxRearrangeList* list = (wxRearrangeList*)self;
    if (!list)
        return false;

    return list->CanMoveCurrentUp();
}

bool
wxd_RearrangeList_CanMoveCurrentDown(wxd_RearrangeList_t* self)
{
    wxRearrangeList* list = (wxRearrangeList*)self;
    if (!list)
        return false;

    return list->CanMoveCurrentDown();
}

int
wxd_RearrangeList_GetSelection(wxd_RearrangeList_t* self)
{
    wxRearrangeList* list = (wxRearrangeList*)self;
    if (!list)
        return -1;

    return list->GetSelection();
}

void
wxd_RearrangeList_SetSelection(wxd_RearrangeList_t* self, int index, bool select)
{
    wxRearrangeList* list = (wxRearrangeList*)self;
    if (!list)
        return;

    list->SetSelection(index, select);
}

int
wxd_RearrangeList_GetString(wxd_RearrangeList_t* self, int index, char* buffer, size_t bufferSize)
{
    wxRearrangeList* list = (wxRearrangeList*)self;
    if (!list)
        return -1;

    if (index < 0 || index >= static_cast<int>(list->GetCount())) {
        return -1;
    }

    wxString item = list->GetString(index);
    return (int)wxd_cpp_utils::copy_wxstring_to_buffer(item, buffer, bufferSize);
}

unsigned int
wxd_RearrangeList_GetCount(wxd_RearrangeList_t* self)
{
    wxRearrangeList* list = (wxRearrangeList*)self;
    if (!list)
        return 0;

    return list->GetCount();
}

bool
wxd_RearrangeList_IsChecked(wxd_RearrangeList_t* self, int index)
{
    wxRearrangeList* list = (wxRearrangeList*)self;
    if (!list)
        return false;

    if (index < 0 || index >= static_cast<int>(list->GetCount())) {
        return false;
    }

    // Get the current order to check if the item is checked
    wxArrayInt order = list->GetCurrentOrder();

    // Find the position of the original item index in the current order
    for (size_t i = 0; i < order.GetCount(); ++i) {
        int value = order[i];

        // If value is positive, the item is checked
        // If value is negative (bitwise complement), the item is unchecked
        if (value == index) {
            return true; // Found checked item
        }
        else if (value == ~index) {
            return false; // Found unchecked item
        }
    }

    return false; // Item not found or error
}

void
wxd_RearrangeList_Check(wxd_RearrangeList_t* self, unsigned int index, bool check)
{
    wxRearrangeList* list = (wxRearrangeList*)self;
    if (!list) {
        return;
    }

    // wxRearrangeList::Check takes unsigned int and handles index bounds internally.
    // It also handles not doing anything if the state is already as requested.
    // It calls base wxCheckListBox::Check and then updates its own m_order.
    list->Check(index, check);
}

} // extern "C"