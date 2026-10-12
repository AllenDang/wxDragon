// Native DataView regression for #160. Cocoa must run on the OS main thread.
#include <wx/wx.h>
#include <wx/dataview.h>
#include <iostream>
#include <memory>
#include <stdexcept>
#include "wxdragon.h"

#ifdef __WXOSX__
#import <AppKit/AppKit.h>
#endif

class DataViewTestApp : public wxApp {
public:
    bool
    OnInit() override
    { return true; }
};
wxIMPLEMENT_APP_NO_MAIN(DataViewTestApp);

static void
Check(bool condition, const char* message)
{
    if (!condition)
        throw std::runtime_error(message);
}

struct Node {
    wxString label;
    Node* parent;
    bool container;
    bool checked = true;
};

struct Tree {
    Node groupA{ "Group A", nullptr, true };
    Node groupB{ "Group B", nullptr, true };
    Node leafA{ "Leaf A", &groupA, false };
    Node leafB{ "Leaf B", &groupB, false };
    bool emptyToggle = false;
    bool deletedLeaf = false;
    bool callbacksFreed = false;
};

// The fixture uses C++ allocation instead of Rust Boxes. Provide the matching
// destructor entry point, without changing the production callback ownership ABI.
extern "C" void
wxd_Drop_Rust_DataViewTreeModelCallbacks(wxd_DataViewTreeModel_Callbacks* cb)
{
    static_cast<Tree*>(cb->userdata)->callbacksFreed = true;
    delete cb;
}

// No Rust logger is linked into this native-only executable.
extern "C" void
wxd_rust_log(int level, const char* message)
{
    if (level <= 2)
        std::cerr << message << '\n';
}

static void*
GetParent(void*, void* item)
{ return item ? static_cast<Node*>(item)->parent : nullptr; }

static bool
IsContainer(void*, void* item)
{ return !item || static_cast<Node*>(item)->container; }

static void
GetChildren(void* userdata, void* item, void*** outItems, int* outCount)
{
    auto& tree = *static_cast<Tree*>(userdata);
    *outItems = nullptr;
    *outCount = 0;
    if (!item) {
        *outItems = new void*[2]{ &tree.groupA, &tree.groupB };
        *outCount = 2;
    }
    else if (item == &tree.groupA || (item == &tree.groupB && !tree.deletedLeaf)) {
        *outItems = new void*[1]{ item == &tree.groupA ? &tree.leafA : &tree.leafB };
        *outCount = 1;
    }
}

static void
FreeChildren(void** items, int)
{ delete[] items; }

static wxd_Variant_t*
GetValue(void* userdata, void* item, unsigned col)
{
    auto& tree = *static_cast<Tree*>(userdata);
    auto& node = *static_cast<Node*>(item);
    auto* value = new wxVariant;
    if (col == 0) {
        if (!tree.emptyToggle || !node.container)
            *value = node.checked;
    }
    else {
        *value = node.label;
    }
    return reinterpret_cast<wxd_Variant_t*>(value);
}

static wxDataViewCtrl*
CreateView(wxWindow* parent, wxDataViewModel* model)
{
    auto* view = new wxDataViewCtrl(parent, wxID_ANY, wxDefaultPosition, wxSize(340, 240));
    view->AppendColumn(new wxDataViewColumn(
        "Check", new wxDataViewToggleRenderer("bool", wxDATAVIEW_CELL_ACTIVATABLE), 0, 70));
    auto* label = new wxDataViewColumn("Label", new wxDataViewTextRenderer(), 1, 240, wxALIGN_LEFT,
                                       wxDATAVIEW_COL_SORTABLE);
    view->AppendColumn(label);
    view->SetExpanderColumn(label);
    Check(view->AssociateModel(model), "could not associate model");
    return view;
}

#ifdef __WXOSX__
static NSOutlineView*
Outline(wxDataViewCtrl* view)
{
    NSView* native = view->GetHandle();
    if ([native isKindOfClass:[NSScrollView class]])
        native = [(NSScrollView*)native documentView];
    Check([native isKindOfClass:[NSOutlineView class]], "expected native outline view");
    return (NSOutlineView*)native;
}

// NSOutlineRow still uses the legacy attribute API, not accessibilityValue/etc.
static id
FindAttribute(id element, NSString* attribute, id expected, unsigned depth = 0)
{
    if (!element || depth > 8)
        return nil;
    if ([[element accessibilityAttributeValue:attribute] isEqual:expected])
        return element;
    for (id child in [element accessibilityAttributeValue:NSAccessibilityChildrenAttribute]) {
        if (id found = FindAttribute(child, attribute, expected, depth + 1))
            return found;
    }
    return nil;
}

static void
CheckRow(wxDataViewCtrl* view, unsigned index, const wxString& label, bool container,
         bool checked = true)
{
    NSArray* rows = [Outline(view) accessibilityAttributeValue:NSAccessibilityRowsAttribute];
    Check(index < [rows count], "missing accessible row");
    id row = [rows objectAtIndex:index];
    Check([[row accessibilityAttributeValue:NSAccessibilityRoleAttribute]
              isEqual:NSAccessibilityRowRole],
          "row role changed");
    NSString* text = [NSString stringWithUTF8String:label.utf8_str()];
    id textElement = FindAttribute(row, NSAccessibilityValueAttribute, text);
    Check(textElement != nil, "row text missing from native accessibility tree");
    Check([[textElement accessibilityAttributeValue:NSAccessibilityRoleAttribute]
              isEqual:NSAccessibilityStaticTextRole],
          "text role changed");
    id checkbox = FindAttribute(row, NSAccessibilityRoleAttribute, NSAccessibilityCheckBoxRole);
    Check(checkbox != nil, "checkbox role lost");
    Check([[checkbox accessibilityAttributeValue:NSAccessibilityValueAttribute] isEqual:@(checked)],
          "checkbox state lost");
    if (container)
        Check(FindAttribute(row, NSAccessibilityRoleAttribute,
                            NSAccessibilityDisclosureTriangleRole) != nil,
              "disclosure triangle lost");
}
#endif

static void
TestTree(wxFrame* frame, Tree& tree)
{
    auto* cb = new wxd_DataViewTreeModel_Callbacks{};
    cb->userdata = &tree;
    cb->get_parent = GetParent;
    cb->is_container = IsContainer;
    cb->get_children = GetChildren;
    cb->free_children = FreeChildren;
    cb->get_value = GetValue;
    auto* handle = wxd_DataViewTreeModel_CreateWithCallbacks(cb);
    auto* model = reinterpret_cast<wxDataViewModel*>(handle);
    // Release the caller's reference on every exit, including assertion failure.
    auto release = [](wxDataViewModel* m) { m->DecRef(); };
    std::unique_ptr<wxDataViewModel, decltype(release)> owned(model, release);
    const wxDataViewItem groupA(&tree.groupA), groupB(&tree.groupB), leafA(&tree.leafA);
    Check(!model->HasContainerColumns(groupA), "default container policy changed");
    Check(model->HasValue(groupA, 0), "first container column hidden");
    Check(!model->HasValue(groupA, 1), "default must hide non-first container columns");
    Check(model->HasValue(leafA, 1), "leaf label hidden by default");
    wxd_DataViewTreeModel_SetContainerColumns(handle, true);
    Check(model->HasValue(groupA, 1), "setting before association did not take effect");
    wxd_DataViewTreeModel_SetContainerColumns(handle, false);
    Check(!model->HasValue(groupA, 1), "default was not restored before association");

    auto* first = CreateView(frame, model);
    auto* second = CreateView(frame, model);
    auto* sizer = new wxBoxSizer(wxHORIZONTAL);
    sizer->Add(first, 1, wxEXPAND);
    sizer->Add(second, 1, wxEXPAND);
    frame->SetSizer(sizer);
    frame->Show();
    wxYield();
#ifdef __WXOSX__
    Check(!FindAttribute(Outline(first), NSAccessibilityValueAttribute, @"Group B"),
          "baseline unexpectedly exposes suppressed label");
#endif
    first->Expand(groupA);
    wxYield();
#ifdef __WXOSX__
    CheckRow(first, 1, tree.leafA.label, false);
#endif
    first->Collapse(groupA);
    wxYield();

    wxd_DataViewTreeModel_SetContainerColumns(handle, true);
    Check(model->HasContainerColumns(groupB) && model->HasValue(groupB, 1),
          "opt-in did not enable container columns");
    Check(model->HasValue(leafA, 1), "opt-in changed leaf policy");
    wxYield();
#ifdef __WXOSX__
    CheckRow(first, 1, tree.groupB.label, true);
    CheckRow(second, 1, tree.groupB.label, true);
#endif

    first->Expand(groupA);
    first->Expand(groupB);
    wxYield();
#ifdef __WXOSX__
    CheckRow(first, 0, tree.groupA.label, true);
    CheckRow(first, 1, tree.leafA.label, false);
    CheckRow(first, 2, tree.groupB.label, true);
    CheckRow(first, 3, tree.leafB.label, false);
#endif
    first->Select(groupB);
    // Idempotent calls must not clear selection or expansion.
    wxd_DataViewTreeModel_SetContainerColumns(handle, true);
    Check(first->GetSelection() == groupB && first->IsExpanded(groupA),
          "repeated setting rebuilt the view");

    tree.groupB.label =
        wxString::FromUTF8("Group B \xe2\x80\x94 \xe6\x94\xaf\xe6\x89\x95\xe3\x81\x84");
    tree.groupB.checked = false;
    model->ItemChanged(groupB);
    wxYield();
#ifdef __WXOSX__
    CheckRow(first, 2, tree.groupB.label, true, false);
#endif
    first->Collapse(groupA);
    first->Expand(groupA);
    first->GetColumn(1)->SetSortOrder(false);
    model->Resort();
    wxYield();
#ifdef __WXOSX__
    CheckRow(first, 0, tree.groupB.label, true, false);
    CheckRow(first, 1, tree.leafB.label, false);
#endif
    tree.deletedLeaf = true;
    model->ItemDeleted(groupB, wxDataViewItem(&tree.leafB));
    wxYield();
#ifdef __WXOSX__
    Check(!FindAttribute(Outline(first), NSAccessibilityValueAttribute, @"Leaf B"),
          "deleted leaf remains accessible");
    CheckRow(first, 0, tree.groupB.label, true, false);
#endif

    // Containers with empty toggle values still expose their text.
    tree.emptyToggle = true;
    model->ItemChanged(groupB);
    wxYield();
#ifdef __WXOSX__
    Check(FindAttribute(Outline(first), NSAccessibilityValueAttribute,
                        [NSString stringWithUTF8String:tree.groupB.label.utf8_str()]) != nil,
          "empty container toggle hid text");
#endif
    wxd_DataViewTreeModel_SetContainerColumns(handle, false);
    Check(!model->HasValue(groupB, 1) && model->HasValue(leafA, 1),
          "disabling did not restore default policy");
    wxYield();
#ifdef __WXOSX__
    Check(!FindAttribute(Outline(second), NSAccessibilityValueAttribute,
                         [NSString stringWithUTF8String:tree.groupB.label.utf8_str()]),
          "disabled policy left stale accessible text");
#endif
}

int
main(int argc, char** argv)
{
    wxd_DataViewTreeModel_SetContainerColumns(nullptr, true);
    wxd_DataViewTreeModel_SetContainerColumns(nullptr, false);
    if (!wxEntryStart(argc, argv))
        return 1;
    if (!wxTheApp->CallOnInit()) {
        wxEntryCleanup();
        return 1;
    }
    Tree tree;
    auto* frame =
        new wxFrame(nullptr, wxID_ANY, "DataView regressions", wxDefaultPosition, wxSize(700, 300));
    int result = 0;
    try {
        TestTree(frame, tree);
        delete frame;
        frame = nullptr;
        Check(tree.callbacksFreed, "callback ownership leaked");
        std::cout << "DataView container column regressions passed\n";
    }
    catch (const std::exception& error) {
        std::cerr << error.what() << '\n';
        result = 1;
    }
    delete frame;
    wxTheApp->OnExit();
    wxEntryCleanup();
    return result;
}
