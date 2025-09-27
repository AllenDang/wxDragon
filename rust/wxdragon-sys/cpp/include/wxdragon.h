#ifndef WXDRAGON_H
#define WXDRAGON_H

// Define WXDRAGON_API for export/import
        #define WXDRAGON_API

// Define export macro (used by all sub-headers indirectly via wxd_types.h or if they need it themselves)
#ifndef WXD_EXPORTED
    #define WXD_EXPORTED WXDRAGON_API
#endif

// Include all fundamental types first
#include "wxd_types.h" // Contains all basic C types and opaque struct typedefs
#include "array_string.h" // ArrayString helper functions
#include "core/wxd_item.h" // Added for wxd_DataViewItem_t and its functions
#include "wxd_sysopt.h"

#ifdef __cplusplus
// Include C++ specific utility functions and macros
// This should make WXD_STR_TO_WX_STRING_UTF8_NULL_OK and GET_WX_STRING_RESULT available
// as well as the declaration for wxd_cpp_utils::copy_wxstring_to_buffer
#include "../src/wxd_utils.h" 
#endif

// Main C API block
#ifdef __cplusplus
extern "C" {
#endif

// Include API categories
#include "core/wxd_app.h"
#include "core/wxd_window_base.h"
#include "core/wxd_cursor.h"
#if wxdUSE_XRC
#include "core/wxd_xrc.h"
#endif
#include "events/wxd_event_api.h"
#include "events/wxd_droptarget_api.h" // Extended drop target callbacks

// Button and control widgets
#include "widgets/wxd_button.h"
#include "widgets/wxd_statictext.h"
#include "widgets/wxd_textctrl.h"
#if wxdUSE_RICHTEXT
#include "widgets/wxd_richtextctrl.h"
#endif
#include "widgets/wxd_styledtextctrl.h"
#include "widgets/wxd_checkbox.h"
#include "widgets/wxd_radiobutton.h"
#include "widgets/wxd_togglebutton.h"
#include "widgets/wxd_gauge.h"
#include "widgets/wxd_slider.h"
#include "widgets/wxd_spinctrl.h"
#include "widgets/wxd_spinbutton.h"
#include "widgets/wxd_searchctrl.h"
#include "widgets/wxd_hyperlinkctrl.h"
#include "widgets/wxd_activityindicator.h"
#include "widgets/wxd_animationctrl.h"
#include "widgets/wxd_mediactrl.h"
#include "widgets/wxd_commandlinkbutton.h"
#include "widgets/wxd_staticbitmap.h"
#include "widgets/wxd_genericstaticbitmap.h"
#include "widgets/wxd_platform_aware_staticbitmap_handler.h"
#include "widgets/wxd_staticline.h"
#include "widgets/wxd_scrollbar.h"
#include "widgets/wxd_bitmapbutton.h"

// List and choice widgets
#include "widgets/wxd_listbox.h"
#include "widgets/wxd_choice.h"
#include "widgets/wxd_combobox.h"
#include "widgets/wxd_checklistbox.h"
#include "widgets/wxd_radiobox.h"
#include "widgets/wxd_bitmapcombobox.h"
#include "widgets/wxd_treectrl.h"
#include "widgets/wxd_listctrl.h"
#include "widgets/wxd_dataview.h"

// Include ImageList FFI
#include "widgets/wxd_imagelist.h"

// Container widgets
#include "widgets/wxd_panel.h"
#include "widgets/wxd_staticbox.h"
#include "widgets/wxd_collapsiblepane.h"
#include "widgets/wxd_splitterwindow.h"
#include "widgets/wxd_scrolledwindow.h"
#include "widgets/wxd_treebook.h"
#include "widgets/wxd_notebook.h"
#include "widgets/wxd_simplebook.h"

// Window and UI elements
#include "widgets/wxd_frame.h"
#include "widgets/wxd_statusbar.h"
#include "widgets/wxd_toolbar.h"
#include "widgets/wxd_menu.h"
#include "widgets/wxd_bitmap.h"
#include "widgets/wxd_artprovider.h"
#include "widgets/wxd_calendar_ctrl.h"
#include "widgets/wxd_taskbar.h"

// Graphics
#include "graphics/wxd_bitmapbundle.h"

// Other widgets
#include "widgets/wxd_pickers.h"
#include "widgets/wxd_file_ctrl.h"
#include "widgets/wxd_adv_ui.h" // For wxNotificationMessage etc.
#include "widgets/wxd_editablelistbox.h" // For wxEditableListBox
#include "widgets/wxd_aui.h"

// Timer
#include "core/wxd_timer.h"

// Application progress indicator
#include "core/wxd_appprogress.h"

// Other categories
#include "sizers/wxd_sizers.h"
#include "dialogs/wxd_dialogs.h"
#include "dnd/wxd_dnd.h" // Drag and drop functionality
#include "graphics/wxd_dc.h" // Device context functionality

// DataView related includes.
// wxd_dataview.h provides main FFI for DataViewCtrl, ListCtrl, TreeCtrl (creation),
// DataViewModel, DataViewColumn, DataViewRenderer.
#include "widgets/wxd_dataview.h"

// wxd_dataviewtreectrl.h provides specific FFI functions for DataViewTreeCtrl methods
// beyond basic creation (e.g., AppendItem, DeleteItem).
#include "widgets/wxd_dataviewtreectrl.h"

// The following individual dataview component headers are likely redundant if wxd_dataview.h is comprehensive
// and are removed to prevent redefinition or missing file errors if they were just parts of wxd_dataview.h.
// #include "widgets/wxd_dataviewctrl.h"
// #include "widgets/wxd_dataviewlistctrl.h"
// #include "widgets/wxd_dataviewcolumn.h"
// #include "widgets/wxd_dataviewrenderer.h"

#include "widgets/wxd_rearrangelist.h"
#include "core/wxd_clipboard.h"
#include "core/wxd_dataobject.h"

#ifdef __cplusplus
} // extern "C"

#endif // __cplusplus

#endif // WXDRAGON_H