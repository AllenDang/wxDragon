// Include all standard C library headers BEFORE wxWidgets headers
#include <iostream>
#include <string>
#include <vector>
#include <cstring>  // For strlen, strdup, memcpy
#include <cstdlib>  // For standard library functions
#include <cwchar>   // For wchar.h functions
#include <string.h> // C-style string functions
#include <stdlib.h> // C-style standard library
#include <wchar.h>  // Wide character functions

#include <wx/wx.h>  // Include the wxApp header for initialization

// Include necessary wxWidgets headers for the constants
#include <wx/defs.h>
#include <wx/event.h>
#include <wx/frame.h>
#include <wx/dialog.h>
#include <wx/sizer.h>
#include <wx/wrapsizer.h>
#include <wx/window.h>
#include <wx/textctrl.h> // Needed for wxEVT_TEXT*
#include <wx/listbox.h> // ADDED: Needed for ListBox styles
#include <wx/choice.h>  // ADDED: Needed for Choice styles (uses wxCB_)
#include <wx/combobox.h> // ADDED: Needed for ComboBox
#include <wx/treectrl.h> // ADDED: Needed for TreeCtrl styles
#include <wx/gauge.h> // ADDED: Needed for Gauge styles
#include <wx/slider.h> // ADDED: Needed for Slider styles
#include <wx/spinctrl.h> // ADDED: Needed for SpinCtrl styles
#include <wx/spinbutt.h> // ADDED: Needed for SpinButton constants/events
#include <wx/notebook.h> // ADDED: Needed for Notebook styles and events
#include <wx/splitter.h> // ADDED: Needed for SplitterWindow styles and events
#include <wx/button.h>   // Ensure button styles are included
#include <wx/bmpbuttn.h> // ADDED: Needed for wxBitmapButton
#include <wx/listctrl.h> // ADDED: Needed for wxListCtrl styles and constants
#include <wx/datectrl.h> // ADDED: Needed for wxDatePickerCtrl styles
#include <wx/srchctrl.h> // ADDED for wxSearchCtrl constants
#include <wx/hyperlink.h> // ADDED for wxHyperlinkCtrl constants
#include <wx/filedlg.h> // ADDED for wxFileDialog constants
#include <wx/filepicker.h> // ADDED for wxFilePickerCtrl constants
#include <wx/fontpicker.h> // ADDED for wxFontPickerCtrl constants
#include <wx/collpane.h> // ADDED for wxCollapsiblePane constants
#include <wx/artprov.h> // ADDED for wxArtProvider constants
#include <wx/colordlg.h> // ADDED for wxColourDialog
#include <wx/fontdlg.h> // ADDED for wxFontDialog
#include <wx/progdlg.h> // ADDED for wxProgressDialog
#include <wx/msgdlg.h> // ADDED for wxMessageDialog
#include <wx/statline.h>    // For wxStaticLine constants
#include <wx/statbmp.h> // ADDED for wxStaticBitmap constants
#include <wx/filectrl.h> // ADDED for wxFileCtrl constants
#include <wx/calctrl.h> // ADDED for wxCalendarCtrl constants
#include <wx/aui/aui.h>
#include <wx/clrpicker.h>
#include <wx/editlbox.h>
#include <wx/mediactrl.h>
#include <wx/dataview.h> // ADDED: Needed for wxDataView styles and constants
#include <wx/taskbar.h> // ADDED: Needed for TaskBarIcon constants
#include <wx/sysopt.h> // For wxSystemOptions

// A simple application class that initializes wxWidgets
class ConstExtractorApp : public wxApp {
public:
    virtual bool OnInit() override {
        // Just initialize, no need to create a window
        return true;
    }
};

// Implement the wxWidgets application
wxIMPLEMENT_APP_NO_MAIN(ConstExtractorApp);

int main(int argc, char **argv) {
    wxSystemOptions::SetOption("msw.no-manifest-check", 1);
    // Initialize wxWidgets to ensure event types are properly initialized
    wxApp::SetInstance(new ConstExtractorApp());
    wxEntryStart(argc, argv);
    
    if (!wxTheApp->OnInit()) {
        return 1;
    }

    // Map of constant names to their values
    std::vector<std::pair<std::string, long long>> constants_to_extract;

    // Populate the vector using push_back
    // IDs
    constants_to_extract.push_back({"wxID_ANY", wxID_ANY});
    constants_to_extract.push_back({"wxID_HIGHEST", wxID_HIGHEST});
    constants_to_extract.push_back({"wxID_OK", wxID_OK});
    constants_to_extract.push_back({"wxID_CANCEL", wxID_CANCEL});
    constants_to_extract.push_back({"wxID_YES", wxID_YES});
    constants_to_extract.push_back({"wxID_NO", wxID_NO});
    constants_to_extract.push_back({"wxID_CLOSE", wxID_CLOSE});
    constants_to_extract.push_back({"wxID_EXIT", wxID_EXIT});
    constants_to_extract.push_back({"wxID_ABOUT", wxID_ABOUT});
    constants_to_extract.push_back({"wxID_APPLY", wxID_APPLY});
    constants_to_extract.push_back({"wxID_PREFERENCES", wxID_PREFERENCES});
    constants_to_extract.push_back({"wxID_EDIT", wxID_EDIT});
    constants_to_extract.push_back({"wxID_VIEW_DETAILS", wxID_VIEW_DETAILS});
    constants_to_extract.push_back({"wxID_VIEW_LIST", wxID_VIEW_LIST});
    constants_to_extract.push_back({"wxID_VIEW_SMALLICONS", wxID_VIEW_SMALLICONS});
    constants_to_extract.push_back({"wxID_VIEW_LARGEICONS", wxID_VIEW_LARGEICONS});
    constants_to_extract.push_back({"wxID_FORWARD", wxID_FORWARD});
    constants_to_extract.push_back({"wxID_BACKWARD", wxID_BACKWARD});
    constants_to_extract.push_back({"wxID_UP", wxID_UP});
    constants_to_extract.push_back({"wxID_DOWN", wxID_DOWN});
    constants_to_extract.push_back({"wxID_HOME", wxID_HOME});
    constants_to_extract.push_back({"wxID_REFRESH", wxID_REFRESH});
    constants_to_extract.push_back({"wxID_STOP", wxID_STOP});
    constants_to_extract.push_back({"wxID_HELP", wxID_HELP});
    constants_to_extract.push_back({"wxID_MORE", wxID_MORE});
    constants_to_extract.push_back({"wxID_SETUP", wxID_SETUP});
    constants_to_extract.push_back({"wxID_PRINT", wxID_PRINT});
    constants_to_extract.push_back({"wxID_PREVIEW", wxID_PREVIEW});
    constants_to_extract.push_back({"wxID_OPEN", wxID_OPEN});
    constants_to_extract.push_back({"wxID_SAVE", wxID_SAVE});
    constants_to_extract.push_back({"wxID_SAVEAS", wxID_SAVEAS});
    constants_to_extract.push_back({"wxID_REVERT", wxID_REVERT});
    constants_to_extract.push_back({"wxID_NEW", wxID_NEW});
    constants_to_extract.push_back({"wxID_UNDO", wxID_UNDO});
    constants_to_extract.push_back({"wxID_REDO", wxID_REDO});
    constants_to_extract.push_back({"wxID_CUT", wxID_CUT});
    constants_to_extract.push_back({"wxID_COPY", wxID_COPY});
    constants_to_extract.push_back({"wxID_PASTE", wxID_PASTE});
    constants_to_extract.push_back({"wxID_CLEAR", wxID_CLEAR});
    constants_to_extract.push_back({"wxID_FIND", wxID_FIND});
    constants_to_extract.push_back({"wxID_DUPLICATE", wxID_DUPLICATE});
    constants_to_extract.push_back({"wxID_SELECTALL", wxID_SELECTALL});
    constants_to_extract.push_back({"wxID_DELETE", wxID_DELETE});
    constants_to_extract.push_back({"wxID_PROPERTIES", wxID_PROPERTIES});
    constants_to_extract.push_back({"wxID_CONVERT", wxID_CONVERT});
    constants_to_extract.push_back({"wxID_INDEX", wxID_INDEX});
    constants_to_extract.push_back({"wxID_BOLD", wxID_BOLD});
    constants_to_extract.push_back({"wxID_ITALIC", wxID_ITALIC});
    constants_to_extract.push_back({"wxID_JUSTIFY_CENTER", wxID_JUSTIFY_CENTER});
    constants_to_extract.push_back({"wxID_JUSTIFY_FILL", wxID_JUSTIFY_FILL});
    constants_to_extract.push_back({"wxID_JUSTIFY_LEFT", wxID_JUSTIFY_LEFT});
    constants_to_extract.push_back({"wxID_JUSTIFY_RIGHT", wxID_JUSTIFY_RIGHT});
    constants_to_extract.push_back({"wxID_UNDERLINE", wxID_UNDERLINE});
    constants_to_extract.push_back({"wxID_INDENT", wxID_INDENT});
    constants_to_extract.push_back({"wxID_UNINDENT", wxID_UNINDENT});
    constants_to_extract.push_back({"wxID_ZOOM_100", wxID_ZOOM_100});
    constants_to_extract.push_back({"wxID_ZOOM_FIT", wxID_ZOOM_FIT});
    constants_to_extract.push_back({"wxID_ZOOM_IN", wxID_ZOOM_IN});
    constants_to_extract.push_back({"wxID_ZOOM_OUT", wxID_ZOOM_OUT});
    // Styles (Common)
    constants_to_extract.push_back({"wxDEFAULT_FRAME_STYLE", wxDEFAULT_FRAME_STYLE});
    constants_to_extract.push_back({"wxDEFAULT_DIALOG_STYLE", wxDEFAULT_DIALOG_STYLE});
    constants_to_extract.push_back({"wxFRAME_TOOL_WINDOW", wxFRAME_TOOL_WINDOW});
    constants_to_extract.push_back({"wxFRAME_NO_TASKBAR", wxFRAME_NO_TASKBAR});
    constants_to_extract.push_back({"wxFRAME_FLOAT_ON_PARENT", wxFRAME_FLOAT_ON_PARENT});
    constants_to_extract.push_back({"wxCLIP_CHILDREN", wxCLIP_CHILDREN});

    constants_to_extract.push_back({"wxSIZE_AUTO", wxSIZE_AUTO});

    constants_to_extract.push_back({"wxCAPTION", wxCAPTION});
    constants_to_extract.push_back({"wxRESIZE_BORDER", wxRESIZE_BORDER});
    constants_to_extract.push_back({"wxSYSTEM_MENU", wxSYSTEM_MENU});
    constants_to_extract.push_back({"wxCLOSE_BOX", wxCLOSE_BOX});
    constants_to_extract.push_back({"wxMAXIMIZE_BOX", wxMAXIMIZE_BOX});
    constants_to_extract.push_back({"wxMINIMIZE_BOX", wxMINIMIZE_BOX});
    constants_to_extract.push_back({"wxTAB_TRAVERSAL", wxTAB_TRAVERSAL});
    constants_to_extract.push_back({"wxALIGN_LEFT", wxALIGN_LEFT});
    constants_to_extract.push_back({"wxALIGN_RIGHT", wxALIGN_RIGHT});
    constants_to_extract.push_back({"wxALIGN_CENTER", wxALIGN_CENTER}); // Alias for wxALIGN_CENTRE_HORIZONTAL
    constants_to_extract.push_back({"wxALIGN_CENTRE_HORIZONTAL", wxALIGN_CENTRE_HORIZONTAL});
    constants_to_extract.push_back({"wxALIGN_TOP", wxALIGN_TOP});
    constants_to_extract.push_back({"wxALIGN_BOTTOM", wxALIGN_BOTTOM});
    constants_to_extract.push_back({"wxALIGN_CENTER_VERTICAL", wxALIGN_CENTER_VERTICAL}); // Alias for wxALIGN_CENTRE_VERTICAL
    constants_to_extract.push_back({"wxALIGN_CENTRE_VERTICAL", wxALIGN_CENTRE_VERTICAL});
    constants_to_extract.push_back({"wxALIGN_CENTRE", wxALIGN_CENTRE}); // Combination of horizontal/vertical
    // Sizer Flags
    constants_to_extract.push_back({"wxEXPAND", wxEXPAND});
    constants_to_extract.push_back({"wxSHAPED", wxSHAPED});
    constants_to_extract.push_back({"wxALL", wxALL});
    constants_to_extract.push_back({"wxLEFT", wxLEFT});
    constants_to_extract.push_back({"wxRIGHT", wxRIGHT});
    constants_to_extract.push_back({"wxTOP", wxTOP});
    constants_to_extract.push_back({"wxBOTTOM", wxBOTTOM});
    constants_to_extract.push_back({"wxFIXED_MINSIZE", wxFIXED_MINSIZE});
    constants_to_extract.push_back({"wxRESERVE_SPACE_EVEN_IF_HIDDEN", wxRESERVE_SPACE_EVEN_IF_HIDDEN});

    // WrapSizer Flags
    constants_to_extract.push_back({"wxEXTEND_LAST_ON_EACH_LINE", wxEXTEND_LAST_ON_EACH_LINE});
    constants_to_extract.push_back({"wxREMOVE_LEADING_SPACES", wxREMOVE_LEADING_SPACES});
    constants_to_extract.push_back({"wxWRAPSIZER_DEFAULT_FLAGS", wxWRAPSIZER_DEFAULT_FLAGS});

    // Border Styles
    constants_to_extract.push_back({"wxBORDER_DEFAULT", wxBORDER_DEFAULT});
    constants_to_extract.push_back({"wxBORDER_SIMPLE", wxBORDER_SIMPLE});
    constants_to_extract.push_back({"wxBORDER_SUNKEN", wxBORDER_SUNKEN});
    constants_to_extract.push_back({"wxBORDER_RAISED", wxBORDER_RAISED});
    constants_to_extract.push_back({"wxBORDER_STATIC", wxBORDER_STATIC});
    constants_to_extract.push_back({"wxBORDER_THEME", wxBORDER_THEME});
    constants_to_extract.push_back({"wxBORDER_NONE", wxBORDER_NONE});
    // Orientation/Direction
    constants_to_extract.push_back({"wxHORIZONTAL", wxHORIZONTAL});
    constants_to_extract.push_back({"wxVERTICAL", wxVERTICAL});
    constants_to_extract.push_back({"wxBOTH", wxBOTH});
    // TextCtrl Styles
    constants_to_extract.push_back({"wxTE_PROCESS_ENTER", wxTE_PROCESS_ENTER});
    constants_to_extract.push_back({"wxTE_MULTILINE", wxTE_MULTILINE});
    constants_to_extract.push_back({"wxTE_PASSWORD", wxTE_PASSWORD});
    constants_to_extract.push_back({"wxTE_READONLY", wxTE_READONLY});
    constants_to_extract.push_back({"wxTE_RICH", wxTE_RICH}); // Basic rich text
    constants_to_extract.push_back({"wxTE_RICH2", wxTE_RICH2}); // Advanced rich text
    constants_to_extract.push_back({"wxTE_AUTO_URL", wxTE_AUTO_URL});
    constants_to_extract.push_back({"wxTE_PROCESS_TAB", wxTE_PROCESS_TAB}); // Process TAB key in the control
    constants_to_extract.push_back({"wxTE_NOHIDESEL", wxTE_NOHIDESEL}); // Always show selection, even when not focused
    constants_to_extract.push_back({"wxTE_LEFT", wxTE_LEFT}); // Left-align text (default)
    constants_to_extract.push_back({"wxTE_CENTRE", wxTE_CENTRE}); // Center-align text
    constants_to_extract.push_back({"wxTE_RIGHT", wxTE_RIGHT}); // Right-align text
    constants_to_extract.push_back({"wxTE_DONTWRAP", wxTE_DONTWRAP}); // Don't wrap text, show horizontal scrollbar
    constants_to_extract.push_back({"wxTE_CHARWRAP", wxTE_CHARWRAP}); // Wrap at any character
    constants_to_extract.push_back({"wxTE_WORDWRAP", wxTE_WORDWRAP}); // Wrap at word boundaries
    constants_to_extract.push_back({"wxTE_BESTWRAP", wxTE_BESTWRAP}); // Wrap at word boundaries or anywhere if word is too long
    constants_to_extract.push_back({"wxTE_CAPITALIZE", wxTE_CAPITALIZE}); // Capitalize first letter (mobile platforms)
    constants_to_extract.push_back({"wxTE_NO_VSCROLL", wxTE_NO_VSCROLL}); // Don't show vertical scrollbar for multiline
    
    constants_to_extract.push_back({"wxHSCROLL", wxHSCROLL}); // Also common window style
    // CheckBox/CheckListBox Styles
    constants_to_extract.push_back({"wxCHK_2STATE", wxCHK_2STATE});
    constants_to_extract.push_back({"wxCHK_3STATE", wxCHK_3STATE});
    constants_to_extract.push_back({"wxCHK_ALLOW_3RD_STATE_FOR_USER", wxCHK_ALLOW_3RD_STATE_FOR_USER});

    // wxFileCtrl styles
    constants_to_extract.push_back({"wxFC_OPEN", wxFC_OPEN});
    constants_to_extract.push_back({"wxFC_SAVE", wxFC_SAVE});
    constants_to_extract.push_back({"wxFC_MULTIPLE", wxFC_MULTIPLE});
    constants_to_extract.push_back({"wxFC_NOSHOWHIDDEN", wxFC_NOSHOWHIDDEN});
    constants_to_extract.push_back({"wxFC_DEFAULT_STYLE", wxFC_DEFAULT_STYLE});

    // TreeCtrl Styles
    constants_to_extract.push_back({"wxTR_DEFAULT_STYLE", wxTR_DEFAULT_STYLE});
    constants_to_extract.push_back({"wxTR_EDIT_LABELS", wxTR_EDIT_LABELS});
    constants_to_extract.push_back({"wxTR_HAS_BUTTONS", wxTR_HAS_BUTTONS});
    constants_to_extract.push_back({"wxTR_LINES_AT_ROOT", wxTR_LINES_AT_ROOT});
    constants_to_extract.push_back({"wxTR_NO_LINES", wxTR_NO_LINES});
    constants_to_extract.push_back({"wxTR_SINGLE", wxTR_SINGLE});
    // Menu Styles
    constants_to_extract.push_back({"wxMB_DOCKABLE", wxMB_DOCKABLE});
    // MenuItem Kinds
    constants_to_extract.push_back({"wxITEM_NORMAL", wxITEM_NORMAL});
    constants_to_extract.push_back({"wxITEM_CHECK", wxITEM_CHECK});
    constants_to_extract.push_back({"wxITEM_RADIO", wxITEM_RADIO});
    constants_to_extract.push_back({"wxITEM_SEPARATOR", wxITEM_SEPARATOR});
    // Gauge Styles
    constants_to_extract.push_back({"wxGA_HORIZONTAL", wxGA_HORIZONTAL});
    constants_to_extract.push_back({"wxGA_VERTICAL", wxGA_VERTICAL});
    constants_to_extract.push_back({"wxGA_PROGRESS", wxGA_PROGRESS});
    constants_to_extract.push_back({"wxGA_SMOOTH", wxGA_SMOOTH});
    // Slider Styles
    constants_to_extract.push_back({"wxSL_HORIZONTAL", wxSL_HORIZONTAL});
    constants_to_extract.push_back({"wxSL_VERTICAL", wxSL_VERTICAL});
    constants_to_extract.push_back({"wxSL_AUTOTICKS", wxSL_AUTOTICKS});
    constants_to_extract.push_back({"wxSL_LABELS", wxSL_LABELS});
    constants_to_extract.push_back({"wxSL_MIN_MAX_LABELS", wxSL_MIN_MAX_LABELS});
    constants_to_extract.push_back({"wxSL_VALUE_LABEL", wxSL_VALUE_LABEL});
    constants_to_extract.push_back({"wxSL_BOTH", wxSL_BOTH});
    constants_to_extract.push_back({"wxSL_SELRANGE", wxSL_SELRANGE});
    constants_to_extract.push_back({"wxSL_INVERSE", wxSL_INVERSE});
    // SpinCtrl Styles
    constants_to_extract.push_back({"wxSP_ARROW_KEYS", wxSP_ARROW_KEYS});
    constants_to_extract.push_back({"wxSP_WRAP", wxSP_WRAP});
    // SpinButton Styles
    constants_to_extract.push_back({"wxSP_HORIZONTAL", wxSP_HORIZONTAL});
    constants_to_extract.push_back({"wxSP_VERTICAL", wxSP_VERTICAL});
    // Notebook Styles
    constants_to_extract.push_back({"wxNB_DEFAULT", wxNB_DEFAULT});
    constants_to_extract.push_back({"wxNB_TOP", wxNB_TOP});
    constants_to_extract.push_back({"wxNB_BOTTOM", wxNB_BOTTOM});
    constants_to_extract.push_back({"wxNB_LEFT", wxNB_LEFT});
    constants_to_extract.push_back({"wxNB_RIGHT", wxNB_RIGHT});
    constants_to_extract.push_back({"wxNB_FIXEDWIDTH", wxNB_FIXEDWIDTH});
    constants_to_extract.push_back({"wxNB_MULTILINE", wxNB_MULTILINE});
    constants_to_extract.push_back({"wxNB_NOPAGETHEME", wxNB_NOPAGETHEME});
    // SplitterWindow Styles
    constants_to_extract.push_back({"wxSP_NOBORDER", wxSP_NOBORDER});
    constants_to_extract.push_back({"wxSP_THIN_SASH", wxSP_THIN_SASH});
    constants_to_extract.push_back({"wxSP_LIVE_UPDATE", wxSP_LIVE_UPDATE});
    constants_to_extract.push_back({"wxSP_3D", wxSP_3D});
    constants_to_extract.push_back({"wxSP_BORDER", wxSP_BORDER});
    constants_to_extract.push_back({"wxSP_PERMIT_UNSPLIT", wxSP_PERMIT_UNSPLIT});
    // BitmapButton Styles
    constants_to_extract.push_back({"wxBU_LEFT", wxBU_LEFT});
    constants_to_extract.push_back({"wxBU_TOP", wxBU_TOP});
    constants_to_extract.push_back({"wxBU_RIGHT", wxBU_RIGHT});
    constants_to_extract.push_back({"wxBU_BOTTOM", wxBU_BOTTOM});
    constants_to_extract.push_back({"wxBU_NOTEXT", wxBU_NOTEXT});
    constants_to_extract.push_back({"wxBU_EXACTFIT", wxBU_EXACTFIT});
    // ScrolledWindow Styles
    constants_to_extract.push_back({"wxVSCROLL", wxVSCROLL});
    // StatusBar Styles
    constants_to_extract.push_back({"wxSTB_DEFAULT_STYLE", wxSTB_DEFAULT_STYLE});
    constants_to_extract.push_back({"wxSTB_SIZEGRIP", wxSTB_SIZEGRIP});
    constants_to_extract.push_back({"wxSTB_SHOW_TIPS", wxSTB_SHOW_TIPS});
    constants_to_extract.push_back({"wxSTB_ELLIPSIZE_START", wxSTB_ELLIPSIZE_START});
    constants_to_extract.push_back({"wxSTB_ELLIPSIZE_MIDDLE", wxSTB_ELLIPSIZE_MIDDLE});
    constants_to_extract.push_back({"wxSTB_ELLIPSIZE_END", wxSTB_ELLIPSIZE_END});
    // ToolBar Styles
    constants_to_extract.push_back({"wxTB_DEFAULT_STYLE", wxTB_DEFAULT_STYLE});
    constants_to_extract.push_back({"wxTB_FLAT", wxTB_FLAT});
    constants_to_extract.push_back({"wxTB_DOCKABLE", wxTB_DOCKABLE});
    constants_to_extract.push_back({"wxTB_HORIZONTAL", wxTB_HORIZONTAL});
    constants_to_extract.push_back({"wxTB_VERTICAL", wxTB_VERTICAL});
    constants_to_extract.push_back({"wxTB_TEXT", wxTB_TEXT});
    constants_to_extract.push_back({"wxTB_NOICONS", wxTB_NOICONS});
    constants_to_extract.push_back({"wxTB_NODIVIDER", wxTB_NODIVIDER});
    constants_to_extract.push_back({"wxTB_NOALIGN", wxTB_NOALIGN});
    constants_to_extract.push_back({"wxTB_HORZ_LAYOUT", wxTB_HORZ_LAYOUT});
    // ListCtrl Styles
    constants_to_extract.push_back({"wxLC_LIST", wxLC_LIST});
    constants_to_extract.push_back({"wxLC_REPORT", wxLC_REPORT});
    constants_to_extract.push_back({"wxLC_ICON", wxLC_ICON});
    constants_to_extract.push_back({"wxLC_SMALL_ICON", wxLC_SMALL_ICON});
    constants_to_extract.push_back({"wxLC_ALIGN_TOP", wxLC_ALIGN_TOP});
    constants_to_extract.push_back({"wxLC_ALIGN_LEFT", wxLC_ALIGN_LEFT});
    constants_to_extract.push_back({"wxLC_AUTOARRANGE", wxLC_AUTOARRANGE});
    constants_to_extract.push_back({"wxLC_EDIT_LABELS", wxLC_EDIT_LABELS});
    constants_to_extract.push_back({"wxLC_NO_HEADER", wxLC_NO_HEADER});
    constants_to_extract.push_back({"wxLC_SINGLE_SEL", wxLC_SINGLE_SEL});
    constants_to_extract.push_back({"wxLC_SORT_ASCENDING", wxLC_SORT_ASCENDING});
    constants_to_extract.push_back({"wxLC_SORT_DESCENDING", wxLC_SORT_DESCENDING});
    constants_to_extract.push_back({"wxLC_VIRTUAL", wxLC_VIRTUAL});
    constants_to_extract.push_back({"wxLC_HRULES", wxLC_HRULES});
    constants_to_extract.push_back({"wxLC_VRULES", wxLC_VRULES});
    constants_to_extract.push_back({"wxLC_NO_SORT_HEADER", wxLC_NO_SORT_HEADER});
    // ListCtrl Item States
    constants_to_extract.push_back({"wxLIST_STATE_SELECTED", wxLIST_STATE_SELECTED});
    constants_to_extract.push_back({"wxLIST_STATE_FOCUSED", wxLIST_STATE_FOCUSED});
    constants_to_extract.push_back({"wxLIST_STATE_DISABLED", wxLIST_STATE_DISABLED});
    constants_to_extract.push_back({"wxLIST_STATE_DROPHILITED", wxLIST_STATE_DROPHILITED});
    // ListCtrl Hit Test Flags
    constants_to_extract.push_back({"wxLIST_HITTEST_ABOVE", wxLIST_HITTEST_ABOVE});
    constants_to_extract.push_back({"wxLIST_HITTEST_BELOW", wxLIST_HITTEST_BELOW});
    constants_to_extract.push_back({"wxLIST_HITTEST_NOWHERE", wxLIST_HITTEST_NOWHERE});
    constants_to_extract.push_back({"wxLIST_HITTEST_ONITEMICON", wxLIST_HITTEST_ONITEMICON});
    constants_to_extract.push_back({"wxLIST_HITTEST_ONITEMLABEL", wxLIST_HITTEST_ONITEMLABEL});
    constants_to_extract.push_back({"wxLIST_HITTEST_ONITEMRIGHT", wxLIST_HITTEST_ONITEMRIGHT});
    constants_to_extract.push_back({"wxLIST_HITTEST_ONITEMSTATEICON", wxLIST_HITTEST_ONITEMSTATEICON});
    constants_to_extract.push_back({"wxLIST_HITTEST_TOLEFT", wxLIST_HITTEST_TOLEFT});
    constants_to_extract.push_back({"wxLIST_HITTEST_TORIGHT", wxLIST_HITTEST_TORIGHT});
    // ListCtrl Column Formats
    constants_to_extract.push_back({"wxLIST_FORMAT_LEFT", wxLIST_FORMAT_LEFT});
    constants_to_extract.push_back({"wxLIST_FORMAT_RIGHT", wxLIST_FORMAT_RIGHT});
    constants_to_extract.push_back({"wxLIST_FORMAT_CENTRE", wxLIST_FORMAT_CENTRE});
    // ListCtrl GetNextItem geometry flags
    constants_to_extract.push_back({"wxLIST_NEXT_ALL", wxLIST_NEXT_ALL});
    constants_to_extract.push_back({"wxLIST_NEXT_ABOVE", wxLIST_NEXT_ABOVE});
    constants_to_extract.push_back({"wxLIST_NEXT_BELOW", wxLIST_NEXT_BELOW});
    constants_to_extract.push_back({"wxLIST_NEXT_LEFT", wxLIST_NEXT_LEFT});
    constants_to_extract.push_back({"wxLIST_NEXT_RIGHT", wxLIST_NEXT_RIGHT});
    // RadioBox Styles
    constants_to_extract.push_back({"wxRA_SPECIFY_COLS", wxRA_SPECIFY_COLS});
    constants_to_extract.push_back({"wxRA_SPECIFY_ROWS", wxRA_SPECIFY_ROWS});
    constants_to_extract.push_back({"wxRB_GROUP", wxRB_GROUP});
    constants_to_extract.push_back({"wxRB_SINGLE", wxRB_SINGLE});
    // ScrollBar Styles
    constants_to_extract.push_back({"wxSB_HORIZONTAL", wxSB_HORIZONTAL});
    constants_to_extract.push_back({"wxSB_VERTICAL", wxSB_VERTICAL});
    // Dialog Styles (General - some overlap with Frame)
    constants_to_extract.push_back({"wxDIALOG_NO_PARENT", wxDIALOG_NO_PARENT});
    constants_to_extract.push_back({"wxDIALOG_EX_CONTEXTHELP", wxDIALOG_EX_CONTEXTHELP});
    constants_to_extract.push_back({"wxDIALOG_EX_METAL", wxDIALOG_EX_METAL}); // macOS specific
    // MessageDialog Styles
    constants_to_extract.push_back({"wxOK", wxOK}); // Often used with wxMessageDialog
    constants_to_extract.push_back({"wxCANCEL", wxCANCEL}); // Often used with wxMessageDialog
    constants_to_extract.push_back({"wxYES", wxYES}); // Often used with wxMessageDialog
    constants_to_extract.push_back({"wxNO", wxNO}); // Often used with wxMessageDialog
    constants_to_extract.push_back({"wxYES_NO", wxYES_NO});
    constants_to_extract.push_back({"wxYES_DEFAULT", wxYES_DEFAULT});
    constants_to_extract.push_back({"wxNO_DEFAULT", wxNO_DEFAULT});
    constants_to_extract.push_back({"wxCANCEL_DEFAULT", wxCANCEL_DEFAULT});
    constants_to_extract.push_back({"wxICON_NONE", wxICON_NONE});
    constants_to_extract.push_back({"wxICON_EXCLAMATION", wxICON_EXCLAMATION});
    constants_to_extract.push_back({"wxICON_WARNING", wxICON_WARNING}); // Same as wxICON_EXCLAMATION
    constants_to_extract.push_back({"wxICON_HAND", wxICON_HAND});
    constants_to_extract.push_back({"wxICON_ERROR", wxICON_ERROR}); // Same as wxICON_HAND
    constants_to_extract.push_back({"wxICON_QUESTION", wxICON_QUESTION});
    constants_to_extract.push_back({"wxICON_INFORMATION", wxICON_INFORMATION});
    constants_to_extract.push_back({"wxICON_AUTH_NEEDED", wxICON_AUTH_NEEDED});
    constants_to_extract.push_back({"wxSTAY_ON_TOP", wxSTAY_ON_TOP});
    // FileDialog Styles
    constants_to_extract.push_back({"wxFD_DEFAULT_STYLE", wxFD_DEFAULT_STYLE});
    constants_to_extract.push_back({"wxFD_OPEN", wxFD_OPEN});
    constants_to_extract.push_back({"wxFD_SAVE", wxFD_SAVE});
    constants_to_extract.push_back({"wxFD_OVERWRITE_PROMPT", wxFD_OVERWRITE_PROMPT});
    constants_to_extract.push_back({"wxFD_FILE_MUST_EXIST", wxFD_FILE_MUST_EXIST});
    constants_to_extract.push_back({"wxFD_MULTIPLE", wxFD_MULTIPLE});
    constants_to_extract.push_back({"wxFD_CHANGE_DIR", wxFD_CHANGE_DIR});
    constants_to_extract.push_back({"wxFD_PREVIEW", wxFD_PREVIEW});
    // DirDialog Styles (wxDD_ is prefix for wxDirDialog, but many are similar to FileDialog)
    constants_to_extract.push_back({"wxDD_DEFAULT_STYLE", wxDD_DEFAULT_STYLE});
    constants_to_extract.push_back({"wxDD_DIR_MUST_EXIST", wxDD_DIR_MUST_EXIST});
    constants_to_extract.push_back({"wxDD_CHANGE_DIR", wxDD_CHANGE_DIR});
    // constants_to_extract.push_back({"wxDD_NEW_DIR_BUTTON", wxDD_NEW_DIR_BUTTON}); // This is often a default
    // FilePickerCtrl Styles
    constants_to_extract.push_back({"wxFLP_DEFAULT_STYLE", wxFLP_DEFAULT_STYLE});
    constants_to_extract.push_back({"wxFLP_USE_TEXTCTRL", wxFLP_USE_TEXTCTRL});
    constants_to_extract.push_back({"wxFLP_OPEN", wxFLP_OPEN});
    constants_to_extract.push_back({"wxFLP_SAVE", wxFLP_SAVE});
    constants_to_extract.push_back({"wxFLP_OVERWRITE_PROMPT", wxFLP_OVERWRITE_PROMPT});
    constants_to_extract.push_back({"wxFLP_FILE_MUST_EXIST", wxFLP_FILE_MUST_EXIST});
    constants_to_extract.push_back({"wxFLP_CHANGE_DIR", wxFLP_CHANGE_DIR});
    constants_to_extract.push_back({"wxFLP_SMALL", wxFLP_SMALL}); // Use small version of the button
    // DirPickerCtrl Styles
    constants_to_extract.push_back({"wxDIRP_DEFAULT_STYLE", wxDIRP_DEFAULT_STYLE});
    constants_to_extract.push_back({"wxDIRP_USE_TEXTCTRL", wxDIRP_USE_TEXTCTRL});
    constants_to_extract.push_back({"wxDIRP_DIR_MUST_EXIST", wxDIRP_DIR_MUST_EXIST});
    constants_to_extract.push_back({"wxDIRP_CHANGE_DIR", wxDIRP_CHANGE_DIR});
    constants_to_extract.push_back({"wxDIRP_SMALL", wxDIRP_SMALL}); // Use small version of the button
    // FontPickerCtrl Styles
    constants_to_extract.push_back({"wxFNTP_DEFAULT_STYLE", wxFNTP_DEFAULT_STYLE});
    constants_to_extract.push_back({"wxFNTP_USE_TEXTCTRL", wxFNTP_USE_TEXTCTRL});
    constants_to_extract.push_back({"wxFNTP_FONTDESC_AS_LABEL", wxFNTP_FONTDESC_AS_LABEL});
    constants_to_extract.push_back({"wxFNTP_USEFONT_FOR_LABEL", wxFNTP_USEFONT_FOR_LABEL});
    // ColourPickerCtrl Styles
    constants_to_extract.push_back({"wxCLRP_DEFAULT_STYLE", wxCLRP_DEFAULT_STYLE}); 
    constants_to_extract.push_back({"wxCLRP_USE_TEXTCTRL", wxCLRP_USE_TEXTCTRL}); 
    constants_to_extract.push_back({"wxCLRP_SHOW_LABEL", wxCLRP_SHOW_LABEL}); 
    constants_to_extract.push_back({"wxCLRP_SHOW_ALPHA", wxCLRP_SHOW_ALPHA}); 
    // CollapsiblePane Styles
    constants_to_extract.push_back({"wxCP_DEFAULT_STYLE", wxCP_DEFAULT_STYLE});
    constants_to_extract.push_back({"wxCP_NO_TLW_RESIZE", wxCP_NO_TLW_RESIZE});
    // ArtProvider Client IDs - REMOVING ALL wxART_ constants as they are strings, not numerical
    // constants_to_extract.push_back({"wxART_TOOLBAR", wxART_TOOLBAR});
    // ... (all other wxART_ constants removed up to wxART_REMOVABLE)
    // ProgressDialog Styles
    constants_to_extract.push_back({"wxPD_AUTO_HIDE", wxPD_AUTO_HIDE});
    constants_to_extract.push_back({"wxPD_CAN_ABORT", wxPD_CAN_ABORT});
    constants_to_extract.push_back({"wxPD_CAN_SKIP", wxPD_CAN_SKIP});
    constants_to_extract.push_back({"wxPD_ELAPSED_TIME", wxPD_ELAPSED_TIME});
    constants_to_extract.push_back({"wxPD_ESTIMATED_TIME", wxPD_ESTIMATED_TIME});
    constants_to_extract.push_back({"wxPD_REMAINING_TIME", wxPD_REMAINING_TIME});
    constants_to_extract.push_back({"wxPD_SMOOTH", wxPD_SMOOTH});
    constants_to_extract.push_back({"wxPD_APP_MODAL", wxPD_APP_MODAL});
    // constants_to_extract.push_back({"wxPD_SHOW_PERCENT", wxPD_SHOW_PERCENT}); // Temporarily comment out
    // StaticLine styles
    constants_to_extract.push_back({"wxLI_HORIZONTAL", wxLI_HORIZONTAL});
    constants_to_extract.push_back({"wxLI_VERTICAL", wxLI_VERTICAL});
    // FlexGridSizer Grow Modes
    constants_to_extract.push_back({"wxFLEX_GROWMODE_NONE", wxFLEX_GROWMODE_NONE});
    constants_to_extract.push_back({"wxFLEX_GROWMODE_SPECIFIED", wxFLEX_GROWMODE_SPECIFIED});
    constants_to_extract.push_back({"wxFLEX_GROWMODE_ALL", wxFLEX_GROWMODE_ALL});
    // Choice Styles
    constants_to_extract.push_back({"wxCB_SORT", wxCB_SORT}); // For Choice
    // ComboBox Styles
    constants_to_extract.push_back({"wxCB_SIMPLE", wxCB_SIMPLE});
    constants_to_extract.push_back({"wxCB_READONLY", wxCB_READONLY});
    constants_to_extract.push_back({"wxCB_DROPDOWN", wxCB_DROPDOWN});
    // DatePickerCtrl Styles
    constants_to_extract.push_back({"wxDP_SPIN", wxDP_SPIN});
    constants_to_extract.push_back({"wxDP_DROPDOWN", wxDP_DROPDOWN});
    constants_to_extract.push_back({"wxDP_DEFAULT", wxDP_DEFAULT});
    constants_to_extract.push_back({"wxDP_ALLOWNONE", wxDP_ALLOWNONE});
    constants_to_extract.push_back({"wxDP_SHOWCENTURY", wxDP_SHOWCENTURY});
    // Calendar Control Styles
    constants_to_extract.push_back({"wxCAL_SUNDAY_FIRST", wxCAL_SUNDAY_FIRST});
    constants_to_extract.push_back({"wxCAL_MONDAY_FIRST", wxCAL_MONDAY_FIRST});
    constants_to_extract.push_back({"wxCAL_SHOW_HOLIDAYS", wxCAL_SHOW_HOLIDAYS});
    constants_to_extract.push_back({"wxCAL_NO_YEAR_CHANGE", wxCAL_NO_YEAR_CHANGE});
    constants_to_extract.push_back({"wxCAL_NO_MONTH_CHANGE", wxCAL_NO_MONTH_CHANGE});
    constants_to_extract.push_back({"wxCAL_SEQUENTIAL_MONTH_SELECTION", wxCAL_SEQUENTIAL_MONTH_SELECTION});
    constants_to_extract.push_back({"wxCAL_SHOW_SURROUNDING_WEEKS", wxCAL_SHOW_SURROUNDING_WEEKS});
    // ListBox Styles (Re-added)
    constants_to_extract.push_back({"wxLB_SINGLE", wxLB_SINGLE});
    constants_to_extract.push_back({"wxLB_MULTIPLE", wxLB_MULTIPLE});
    constants_to_extract.push_back({"wxLB_EXTENDED", wxLB_EXTENDED});
    constants_to_extract.push_back({"wxLB_SORT", wxLB_SORT});
    constants_to_extract.push_back({"wxLB_OWNERDRAW", wxLB_OWNERDRAW});
    constants_to_extract.push_back({"wxLB_HSCROLL", wxLB_HSCROLL}); // Note: This is wxListBox specific HSCROLL
    constants_to_extract.push_back({"wxLB_ALWAYS_SB", wxLB_ALWAYS_SB}); // Added: Verify this is a valid wx constant

    constants_to_extract.push_back({"wxSP_3DBORDER", wxSP_3DBORDER});
    // StaticBitmap Styles
    constants_to_extract.push_back({"wxBITMAP_TYPE_PNG", wxBITMAP_TYPE_PNG});
    constants_to_extract.push_back({"wxBITMAP_TYPE_JPEG", wxBITMAP_TYPE_JPEG});
    constants_to_extract.push_back({"wxBITMAP_TYPE_GIF", wxBITMAP_TYPE_GIF});
    constants_to_extract.push_back({"wxBITMAP_TYPE_BMP", wxBITMAP_TYPE_BMP});
    constants_to_extract.push_back({"wxBITMAP_TYPE_XPM", wxBITMAP_TYPE_XPM});
    // StaticBitmap Scale Modes
    constants_to_extract.push_back({"wxStaticBitmap::Scale_None", wxStaticBitmap::Scale_None});
    constants_to_extract.push_back({"wxStaticBitmap::Scale_Fill", wxStaticBitmap::Scale_Fill});
    constants_to_extract.push_back({"wxStaticBitmap::Scale_AspectFit", wxStaticBitmap::Scale_AspectFit});
    constants_to_extract.push_back({"wxStaticBitmap::Scale_AspectFill", wxStaticBitmap::Scale_AspectFill});
    // Treebook Styles
    // constants_to_extract.push_back({"wxTR_DEFAULT_STYLE", wxTR_DEFAULT_STYLE}); // DUPLICATE REMOVED
    constants_to_extract.push_back({"wxTR_HIDE_ROOT", wxTR_HIDE_ROOT});
    // constants_to_extract.push_back({"wxTR_HAS_BUTTONS", wxTR_HAS_BUTTONS}); // DUPLICATE REMOVED
    // constants_to_extract.push_back({"wxTR_EDIT_LABELS", wxTR_EDIT_LABELS}); // DUPLICATE REMOVED
    constants_to_extract.push_back({"wxTR_ROW_LINES", wxTR_ROW_LINES});
    // BookCtrlBase Styles (for Notebook, Listbook, Treebook, etc.)
    constants_to_extract.push_back({"wxBK_DEFAULT", wxBK_DEFAULT});
    constants_to_extract.push_back({"wxBK_TOP", wxBK_TOP});
    constants_to_extract.push_back({"wxBK_BOTTOM", wxBK_BOTTOM});
    constants_to_extract.push_back({"wxBK_LEFT", wxBK_LEFT});
    constants_to_extract.push_back({"wxBK_RIGHT", wxBK_RIGHT});

    // Dialogs
    constants_to_extract.push_back({"wxCENTRE", wxCENTRE});
    
    // wxAuiNotebook styles
    constants_to_extract.push_back({"wxAUI_NB_DEFAULT_STYLE", wxAUI_NB_DEFAULT_STYLE});
    constants_to_extract.push_back({"wxAUI_NB_TOP", wxAUI_NB_TOP});
    constants_to_extract.push_back({"wxAUI_NB_BOTTOM", wxAUI_NB_BOTTOM}); // Added for completeness, though not in DEFAULT_STYLE
    constants_to_extract.push_back({"wxAUI_NB_LEFT", wxAUI_NB_LEFT});   // Added for completeness
    constants_to_extract.push_back({"wxAUI_NB_RIGHT", wxAUI_NB_RIGHT});  // Added for completeness
    constants_to_extract.push_back({"wxAUI_NB_TAB_SPLIT", wxAUI_NB_TAB_SPLIT});
    constants_to_extract.push_back({"wxAUI_NB_TAB_MOVE", wxAUI_NB_TAB_MOVE});
    constants_to_extract.push_back({"wxAUI_NB_SCROLL_BUTTONS", wxAUI_NB_SCROLL_BUTTONS});
    constants_to_extract.push_back({"wxAUI_NB_WINDOWLIST_BUTTON", wxAUI_NB_WINDOWLIST_BUTTON});
    constants_to_extract.push_back({"wxAUI_NB_CLOSE_BUTTON", wxAUI_NB_CLOSE_BUTTON});
    constants_to_extract.push_back({"wxAUI_NB_CLOSE_ON_ACTIVE_TAB", wxAUI_NB_CLOSE_ON_ACTIVE_TAB});
    constants_to_extract.push_back({"wxAUI_NB_CLOSE_ON_ALL_TABS", wxAUI_NB_CLOSE_ON_ALL_TABS});
    constants_to_extract.push_back({"wxAUI_NB_MIDDLE_CLICK_CLOSE", wxAUI_NB_MIDDLE_CLICK_CLOSE});
    constants_to_extract.push_back({"wxAUI_NB_TAB_EXTERNAL_MOVE", wxAUI_NB_TAB_EXTERNAL_MOVE});
    constants_to_extract.push_back({"wxAUI_NB_TAB_FIXED_WIDTH", wxAUI_NB_TAB_FIXED_WIDTH});

    // wxAuiToolBar styles
    constants_to_extract.push_back({"wxAUI_TB_TEXT", wxAUI_TB_TEXT});
    constants_to_extract.push_back({"wxAUI_TB_NO_TOOLTIPS", wxAUI_TB_NO_TOOLTIPS});
    constants_to_extract.push_back({"wxAUI_TB_NO_AUTORESIZE", wxAUI_TB_NO_AUTORESIZE});
    constants_to_extract.push_back({"wxAUI_TB_GRIPPER", wxAUI_TB_GRIPPER});
    constants_to_extract.push_back({"wxAUI_TB_OVERFLOW", wxAUI_TB_OVERFLOW});
    constants_to_extract.push_back({"wxAUI_TB_VERTICAL", wxAUI_TB_VERTICAL});
    constants_to_extract.push_back({"wxAUI_TB_HORZ_LAYOUT", wxAUI_TB_HORZ_LAYOUT});
    constants_to_extract.push_back({"wxAUI_TB_HORIZONTAL", wxAUI_TB_HORIZONTAL});
    constants_to_extract.push_back({"wxAUI_TB_DEFAULT_STYLE", wxAUI_TB_DEFAULT_STYLE});

    // wxEditableListBox styles
    constants_to_extract.push_back({"wxEL_ALLOW_NEW", wxEL_ALLOW_NEW});
    constants_to_extract.push_back({"wxEL_ALLOW_EDIT", wxEL_ALLOW_EDIT});
    constants_to_extract.push_back({"wxEL_ALLOW_DELETE", wxEL_ALLOW_DELETE});
    constants_to_extract.push_back({"wxEL_NO_REORDER", wxEL_NO_REORDER});
    constants_to_extract.push_back({"wxEL_DEFAULT_STYLE", wxEL_DEFAULT_STYLE});

    // wxMediaCtrl styles
    constants_to_extract.push_back({"wxMC_NO_AUTORESIZE", wxMC_NO_AUTORESIZE});
    constants_to_extract.push_back({"wxMEDIASTATE_STOPPED", wxMEDIASTATE_STOPPED});
    constants_to_extract.push_back({"wxMEDIASTATE_PAUSED", wxMEDIASTATE_PAUSED});
    constants_to_extract.push_back({"wxMEDIASTATE_PLAYING", wxMEDIASTATE_PLAYING});

    // wxMediaCtrl Player Controls
    constants_to_extract.push_back({"wxMEDIACTRLPLAYERCONTROLS_NONE", wxMEDIACTRLPLAYERCONTROLS_NONE});
    constants_to_extract.push_back({"wxMEDIACTRLPLAYERCONTROLS_STEP", wxMEDIACTRLPLAYERCONTROLS_STEP});
    constants_to_extract.push_back({"wxMEDIACTRLPLAYERCONTROLS_VOLUME", wxMEDIACTRLPLAYERCONTROLS_VOLUME});
    constants_to_extract.push_back({"wxMEDIACTRLPLAYERCONTROLS_DEFAULT", wxMEDIACTRLPLAYERCONTROLS_DEFAULT});
    
    // DataViewCtrl style flags
    constants_to_extract.push_back({"wxDV_SINGLE", wxDV_SINGLE});
    constants_to_extract.push_back({"wxDV_MULTIPLE", wxDV_MULTIPLE});
    constants_to_extract.push_back({"wxDV_ROW_LINES", wxDV_ROW_LINES});
    constants_to_extract.push_back({"wxDV_HORIZ_RULES", wxDV_HORIZ_RULES});
    constants_to_extract.push_back({"wxDV_VERT_RULES", wxDV_VERT_RULES});
    constants_to_extract.push_back({"wxDV_VARIABLE_LINE_HEIGHT", wxDV_VARIABLE_LINE_HEIGHT});
    constants_to_extract.push_back({"wxDV_NO_HEADER", wxDV_NO_HEADER});
    
    // DataViewRenderer cell modes
    constants_to_extract.push_back({"wxDATAVIEW_CELL_INERT", wxDATAVIEW_CELL_INERT});
    constants_to_extract.push_back({"wxDATAVIEW_CELL_ACTIVATABLE", wxDATAVIEW_CELL_ACTIVATABLE});
    constants_to_extract.push_back({"wxDATAVIEW_CELL_EDITABLE", wxDATAVIEW_CELL_EDITABLE});

    // DataViewColumnFlags
    constants_to_extract.push_back({"wxDATAVIEW_COL_RESIZABLE", wxDATAVIEW_COL_RESIZABLE});
    constants_to_extract.push_back({"wxDATAVIEW_COL_SORTABLE", wxDATAVIEW_COL_SORTABLE});
    constants_to_extract.push_back({"wxDATAVIEW_COL_REORDERABLE", wxDATAVIEW_COL_REORDERABLE});
    constants_to_extract.push_back({"wxDATAVIEW_COL_HIDDEN", wxDATAVIEW_COL_HIDDEN});
    
    // DC Polygon fill styles
    constants_to_extract.push_back({"wxODDEVEN_RULE", wxODDEVEN_RULE});
    constants_to_extract.push_back({"wxWINDING_RULE", wxWINDING_RULE});
    
    // DC Flood fill styles
    constants_to_extract.push_back({"wxFLOOD_SURFACE", wxFLOOD_SURFACE});
    constants_to_extract.push_back({"wxFLOOD_BORDER", wxFLOOD_BORDER});
    
    // DC Logical functions
    constants_to_extract.push_back({"wxCLEAR", wxCLEAR});
    constants_to_extract.push_back({"wxXOR", wxXOR});
    constants_to_extract.push_back({"wxINVERT", wxINVERT});
    constants_to_extract.push_back({"wxOR_REVERSE", wxOR_REVERSE});
    constants_to_extract.push_back({"wxAND_REVERSE", wxAND_REVERSE});
    constants_to_extract.push_back({"wxCOPY", wxCOPY});
    constants_to_extract.push_back({"wxAND", wxAND});
    constants_to_extract.push_back({"wxAND_INVERT", wxAND_INVERT});
    constants_to_extract.push_back({"wxNO_OP", wxNO_OP});
    constants_to_extract.push_back({"wxNOR", wxNOR});
    constants_to_extract.push_back({"wxEQUIV", wxEQUIV});
    constants_to_extract.push_back({"wxSRC_INVERT", wxSRC_INVERT});
    constants_to_extract.push_back({"wxOR_INVERT", wxOR_INVERT});
    constants_to_extract.push_back({"wxNAND", wxNAND});
    constants_to_extract.push_back({"wxOR", wxOR});
    constants_to_extract.push_back({"wxSET", wxSET});
    
    // DC Map modes
    constants_to_extract.push_back({"wxMM_TEXT", wxMM_TEXT});
    constants_to_extract.push_back({"wxMM_LOMETRIC", wxMM_LOMETRIC});
    constants_to_extract.push_back({"wxMM_TWIPS", wxMM_TWIPS});
    constants_to_extract.push_back({"wxMM_METRIC", wxMM_METRIC});
    
    // Text alignment constants (only the unique ones not already defined)
    constants_to_extract.push_back({"wxALIGN_INVALID", wxALIGN_INVALID});
    // Note: Other wxALIGN_* constants already defined in the general alignment section above
    
    // Gradient direction constants
    constants_to_extract.push_back({"wxNORTH", wxNORTH});
    constants_to_extract.push_back({"wxSOUTH", wxSOUTH});
    constants_to_extract.push_back({"wxEAST", wxEAST});
    constants_to_extract.push_back({"wxWEST", wxWEST});
    
    // DC Brush and Pen styles (additional to basic ones)
    constants_to_extract.push_back({"wxTRANSPARENT", wxTRANSPARENT});
    constants_to_extract.push_back({"wxSOLID", wxSOLID});
    
    // Pen styles
    constants_to_extract.push_back({"wxPENSTYLE_SOLID", wxPENSTYLE_SOLID});
    constants_to_extract.push_back({"wxPENSTYLE_DOT", wxPENSTYLE_DOT});
    constants_to_extract.push_back({"wxPENSTYLE_LONG_DASH", wxPENSTYLE_LONG_DASH});
    constants_to_extract.push_back({"wxPENSTYLE_SHORT_DASH", wxPENSTYLE_SHORT_DASH});
    constants_to_extract.push_back({"wxPENSTYLE_DOT_DASH", wxPENSTYLE_DOT_DASH});
    constants_to_extract.push_back({"wxPENSTYLE_TRANSPARENT", wxPENSTYLE_TRANSPARENT});
    constants_to_extract.push_back({"wxPENSTYLE_STIPPLE", wxPENSTYLE_STIPPLE});
    constants_to_extract.push_back({"wxPENSTYLE_USER_DASH", wxPENSTYLE_USER_DASH});
    constants_to_extract.push_back({"wxPENSTYLE_BDIAGONAL_HATCH", wxPENSTYLE_BDIAGONAL_HATCH});
    constants_to_extract.push_back({"wxPENSTYLE_CROSSDIAG_HATCH", wxPENSTYLE_CROSSDIAG_HATCH});
    constants_to_extract.push_back({"wxPENSTYLE_FDIAGONAL_HATCH", wxPENSTYLE_FDIAGONAL_HATCH});
    constants_to_extract.push_back({"wxPENSTYLE_CROSS_HATCH", wxPENSTYLE_CROSS_HATCH});
    constants_to_extract.push_back({"wxPENSTYLE_HORIZONTAL_HATCH", wxPENSTYLE_HORIZONTAL_HATCH});
    constants_to_extract.push_back({"wxPENSTYLE_VERTICAL_HATCH", wxPENSTYLE_VERTICAL_HATCH});
    
    // Brush styles
    constants_to_extract.push_back({"wxBRUSHSTYLE_SOLID", wxBRUSHSTYLE_SOLID});
    constants_to_extract.push_back({"wxBRUSHSTYLE_TRANSPARENT", wxBRUSHSTYLE_TRANSPARENT});
    constants_to_extract.push_back({"wxBRUSHSTYLE_BDIAGONAL_HATCH", wxBRUSHSTYLE_BDIAGONAL_HATCH});
    constants_to_extract.push_back({"wxBRUSHSTYLE_CROSSDIAG_HATCH", wxBRUSHSTYLE_CROSSDIAG_HATCH});
    constants_to_extract.push_back({"wxBRUSHSTYLE_FDIAGONAL_HATCH", wxBRUSHSTYLE_FDIAGONAL_HATCH});
    constants_to_extract.push_back({"wxBRUSHSTYLE_CROSS_HATCH", wxBRUSHSTYLE_CROSS_HATCH});
    constants_to_extract.push_back({"wxBRUSHSTYLE_HORIZONTAL_HATCH", wxBRUSHSTYLE_HORIZONTAL_HATCH});
    constants_to_extract.push_back({"wxBRUSHSTYLE_VERTICAL_HATCH", wxBRUSHSTYLE_VERTICAL_HATCH});
    constants_to_extract.push_back({"wxBRUSHSTYLE_STIPPLE", wxBRUSHSTYLE_STIPPLE});
    constants_to_extract.push_back({"wxBRUSHSTYLE_STIPPLE_MASK_OPAQUE", wxBRUSHSTYLE_STIPPLE_MASK_OPAQUE});
    constants_to_extract.push_back({"wxBRUSHSTYLE_STIPPLE_MASK", wxBRUSHSTYLE_STIPPLE_MASK});
    
    // Default coordinate value
    constants_to_extract.push_back({"wxDefaultCoord", wxDefaultCoord});
    
    // Background styles
    constants_to_extract.push_back({"wxBG_STYLE_ERASE", wxBG_STYLE_ERASE});
    constants_to_extract.push_back({"wxBG_STYLE_SYSTEM", wxBG_STYLE_SYSTEM});
    constants_to_extract.push_back({"wxBG_STYLE_PAINT", wxBG_STYLE_PAINT});
    constants_to_extract.push_back({"wxBG_STYLE_COLOUR", wxBG_STYLE_COLOUR});

    // Stretch flags
    constants_to_extract.push_back({"wxSTRETCH_NOT", wxSTRETCH_NOT});
    constants_to_extract.push_back({"wxSHRINK", wxSHRINK});
    constants_to_extract.push_back({"wxGROW", wxGROW});
    constants_to_extract.push_back({"wxTILE", wxTILE});
    constants_to_extract.push_back({"wxSTRETCH_MASK", wxSTRETCH_MASK});
    
    // Window styles (standard wxWidgets window styles that apply to all windows)
    constants_to_extract.push_back({"wxWS_BORDER", wxBORDER_SIMPLE}); // Alias for wxBORDER_SIMPLE
    constants_to_extract.push_back({"wxWS_CAPTION", wxCAPTION}); // Already defined above but adding for completeness
    constants_to_extract.push_back({"wxWS_CHILD", 0x40000000L}); // Child window style
    constants_to_extract.push_back({"wxWS_CHILDWINDOW", 0x40000000L}); // Same as wxWS_CHILD
    constants_to_extract.push_back({"wxWS_CLIPCHILDREN", wxCLIP_CHILDREN}); // Already defined above
    constants_to_extract.push_back({"wxWS_CLIPSIBLINGS", 0x04000000L}); // Clips sibling windows
    constants_to_extract.push_back({"wxWS_DISABLED", 0x08000000L}); // Initially disabled
    constants_to_extract.push_back({"wxWS_DLGFRAME", 0x00400000L}); // Dialog frame style
    constants_to_extract.push_back({"wxWS_GROUP", 0x00020000L}); // Group control flag
    constants_to_extract.push_back({"wxWS_HSCROLL", wxHSCROLL}); // Already defined above
    constants_to_extract.push_back({"wxWS_ICONIC", 0x20000000L}); // Initially minimized
    constants_to_extract.push_back({"wxWS_MAXIMIZE", 0x01000000L}); // Initially maximized
    constants_to_extract.push_back({"wxWS_MAXIMIZEBOX", wxMAXIMIZE_BOX}); // Already defined above
    constants_to_extract.push_back({"wxWS_MINIMIZE", 0x20000000L}); // Same as wxWS_ICONIC
    constants_to_extract.push_back({"wxWS_MINIMIZEBOX", wxMINIMIZE_BOX}); // Already defined above
    constants_to_extract.push_back({"wxWS_OVERLAPPED", 0x00000000L}); // Overlapped window (default)
    constants_to_extract.push_back({"wxWS_POPUP", 0x80000000L}); // Popup window
    constants_to_extract.push_back({"wxWS_SIZEBOX", wxRESIZE_BORDER}); // Alias for wxRESIZE_BORDER
    constants_to_extract.push_back({"wxWS_SYSMENU", wxSYSTEM_MENU}); // Already defined above
    constants_to_extract.push_back({"wxWS_TABSTOP", 0x00010000L}); // Tab stop for controls

    // Navigation direction constants
    constants_to_extract.push_back({"wxNAVIGATION_NEXT", 0});
    constants_to_extract.push_back({"wxNAVIGATION_PREVIOUS", 1});

    constants_to_extract.push_back({"wxWS_THICKFRAME", wxRESIZE_BORDER}); // Same as wxWS_SIZEBOX
    constants_to_extract.push_back({"wxWS_TILED", 0x00000000L}); // Same as wxWS_OVERLAPPED
    constants_to_extract.push_back({"wxWS_VISIBLE", 0x10000000L}); // Initially visible
    constants_to_extract.push_back({"wxWS_VSCROLL", wxVSCROLL}); // Already defined above
    
    // Extra window styles (wxWS_EX_*)
    constants_to_extract.push_back({"wxWS_EX_VALIDATE_RECURSIVELY", wxWS_EX_VALIDATE_RECURSIVELY});
    constants_to_extract.push_back({"wxWS_EX_BLOCK_EVENTS", wxWS_EX_BLOCK_EVENTS});
    constants_to_extract.push_back({"wxWS_EX_TRANSIENT", wxWS_EX_TRANSIENT});
    constants_to_extract.push_back({"wxWS_EX_CONTEXTHELP", wxWS_EX_CONTEXTHELP});
    constants_to_extract.push_back({"wxWS_EX_PROCESS_IDLE", wxWS_EX_PROCESS_IDLE});
    constants_to_extract.push_back({"wxWS_EX_PROCESS_UI_UPDATES", wxWS_EX_PROCESS_UI_UPDATES});

    // wxDataViewCtrl styles

    // TaskBarIcon types
    constants_to_extract.push_back({"wxTBI_DEFAULT_TYPE", wxTBI_DEFAULT_TYPE});
    constants_to_extract.push_back({"wxTBI_DOCK", wxTBI_DOCK});
    constants_to_extract.push_back({"wxTBI_CUSTOM_STATUSITEM", wxTBI_CUSTOM_STATUSITEM});

    // Font family constants
    constants_to_extract.push_back({"wxFONTFAMILY_DEFAULT", wxFONTFAMILY_DEFAULT});
    constants_to_extract.push_back({"wxFONTFAMILY_DECORATIVE", wxFONTFAMILY_DECORATIVE});
    constants_to_extract.push_back({"wxFONTFAMILY_ROMAN", wxFONTFAMILY_ROMAN});
    constants_to_extract.push_back({"wxFONTFAMILY_SCRIPT", wxFONTFAMILY_SCRIPT});
    constants_to_extract.push_back({"wxFONTFAMILY_SWISS", wxFONTFAMILY_SWISS});
    constants_to_extract.push_back({"wxFONTFAMILY_MODERN", wxFONTFAMILY_MODERN});
    constants_to_extract.push_back({"wxFONTFAMILY_TELETYPE", wxFONTFAMILY_TELETYPE});

    // Font style constants  
    constants_to_extract.push_back({"wxFONTSTYLE_NORMAL", wxFONTSTYLE_NORMAL});
    constants_to_extract.push_back({"wxFONTSTYLE_ITALIC", wxFONTSTYLE_ITALIC});
    constants_to_extract.push_back({"wxFONTSTYLE_SLANT", wxFONTSTYLE_SLANT});

    // Font weight constants
    constants_to_extract.push_back({"wxFONTWEIGHT_THIN", wxFONTWEIGHT_THIN});
    constants_to_extract.push_back({"wxFONTWEIGHT_EXTRALIGHT", wxFONTWEIGHT_EXTRALIGHT});
    constants_to_extract.push_back({"wxFONTWEIGHT_LIGHT", wxFONTWEIGHT_LIGHT});
    constants_to_extract.push_back({"wxFONTWEIGHT_NORMAL", wxFONTWEIGHT_NORMAL});
    constants_to_extract.push_back({"wxFONTWEIGHT_MEDIUM", wxFONTWEIGHT_MEDIUM});
    constants_to_extract.push_back({"wxFONTWEIGHT_SEMIBOLD", wxFONTWEIGHT_SEMIBOLD});
    constants_to_extract.push_back({"wxFONTWEIGHT_BOLD", wxFONTWEIGHT_BOLD});
    constants_to_extract.push_back({"wxFONTWEIGHT_EXTRABOLD", wxFONTWEIGHT_EXTRABOLD});
    constants_to_extract.push_back({"wxFONTWEIGHT_HEAVY", wxFONTWEIGHT_HEAVY});

    // Output the constants in the Rust pub const format
    for (const auto& pair : constants_to_extract) {
        std::string original_name = pair.first;
        std::string processed_name = original_name;

        // If the original name starts with "wx" and the third char is uppercase or an underscore, strip "wx"
        if (processed_name.rfind("wx", 0) == 0 && processed_name.length() > 2) {
            if (isupper(processed_name[2]) || processed_name[2] == '_') {
                 processed_name = processed_name.substr(2);
            }
        }
        // For constants like "wxStaticBitmap::Scale_None", replace "::" with "_"
        // This should also handle cases where "wx" was stripped, e.g. "StaticBitmap::Scale_None"
        size_t pos = processed_name.find("::");
        while(pos != std::string::npos) {
            processed_name.replace(pos, 2, "_");
            pos = processed_name.find("::", pos + 1);
        }

        // Prefix with WXD_ to avoid clashes and indicate it's from our wrapper scheme
        std::cout << "pub const WXD_" << processed_name << ": i64 = " << pair.second << ";" << std::endl;
    }

    wxTheApp->OnExit();
    wxEntryCleanup();
    return 0;
} 