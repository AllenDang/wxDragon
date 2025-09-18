// --- Core Types & Traits ---
pub use crate::app::{call_after, get_app, main, set_appearance, set_top_window};
pub use crate::appearance::{
    get_app as get_app_for_appearance, get_system_appearance, is_system_dark_mode, AppAppearance,
    Appearance, AppearanceResult, SystemAppearance,
};
pub use crate::clipboard::{Clipboard, ClipboardLocker};
pub use crate::color::{colours, Colour};
pub use crate::cursor::{
    begin_busy_cursor, end_busy_cursor, is_busy, set_cursor, BitmapType, BusyCursor, Cursor,
    StockCursor,
};
pub use crate::datetime::DateTime;
pub use crate::event::{Event, EventType, IdleEvent, IdleMode, WindowEventData, WxEvtHandler};
// ADDED: Event category traits
pub use crate::event::{ButtonEvents, ScrollEvents, TextEvents, TreeEvents, WindowEvents};
// ADDED: Event Data Structs
pub use crate::event::event_data::{CommandEventData, KeyEventData, MouseEventData};
pub use crate::event::IdleEventData;
pub use crate::geometry::{Point, Rect, Size, DEFAULT_POSITION, DEFAULT_SIZE};
pub use crate::id::{Id, ID_ANY, ID_CANCEL, ID_HIGHEST, ID_NO, ID_OK, ID_YES};
pub use crate::sizers::WxSizer;
pub use crate::types::Style;
pub use crate::utils::WxdArrayString;
pub use crate::window::{
    BackgroundStyle, ExtraWindowStyle, Window, WindowStyle, WxWidget, WxWidgetDowncast,
};

// --- Sizers ---
pub use crate::sizers::box_sizer::{BoxSizer, BoxSizerBuilder};
pub use crate::sizers::flex_grid_sizer::{FlexGridSizer, FlexGridSizerBuilder, FlexGrowMode};
pub use crate::sizers::grid_bag_sizer::{
    GBPosition, GBSpan, GridBagSizer, GridBagSizerBuilder, DEFAULT_GB_POSITION, DEFAULT_GB_SPAN,
};
pub use crate::sizers::grid_sizer::{GridSizer, GridSizerBuilder};
pub use crate::sizers::staticbox_sizer::{StaticBoxSizer, StaticBoxSizerBuilder};
pub use crate::sizers::wrap_sizer::{WrapSizer, WrapSizerBuilder, WrapSizerFlag};
// Sizer Flags/Constants
pub use crate::sizers::base::{Orientation, SizerFlag};

// --- Widgets & Builders ---
pub use crate::widgets::activity_indicator::{
    ActivityIndicator, ActivityIndicatorBuilder, ActivityIndicatorStyle,
}; // Added Style
pub use crate::widgets::animation_ctrl::{AnimationCtrl, AnimationCtrlBuilder, AnimationCtrlStyle}; // Added Style
                                                                                                   // ADDED: AUI
#[cfg(feature = "aui")]
pub use crate::widgets::aui_manager::{AuiManager, AuiPaneInfo, DockDirection};
#[cfg(feature = "aui")]
pub use crate::widgets::aui_mdi_child_frame::{AuiMdiChildFrame, AuiMdiChildFrameBuilder};
#[cfg(feature = "aui")]
pub use crate::widgets::aui_mdi_parent_frame::{AuiMdiParentFrame, AuiMdiParentFrameBuilder};
#[cfg(feature = "aui")]
pub use crate::widgets::aui_notebook::{AuiNotebook, AuiNotebookBuilder, AuiNotebookStyle}; // Added Style
#[cfg(feature = "aui")]
pub use crate::widgets::aui_toolbar::{AuiToolBar, AuiToolBarBuilder, AuiToolBarStyle}; // Added Style
pub use crate::widgets::bitmap_button::{BitmapButton, BitmapButtonBuilder, BitmapButtonStyle}; // Added Style
pub use crate::widgets::bitmap_combobox::{BitmapComboBox, BitmapComboBoxBuilder}; // Style is ComboBoxStyle
pub use crate::widgets::button::{Button, ButtonBuilder, ButtonStyle};
pub use crate::widgets::calendar_ctrl::{CalendarCtrl, CalendarCtrlBuilder, CalendarCtrlStyle};
pub use crate::widgets::checkbox::{CheckBox, CheckBoxBuilder, CheckBoxStyle};
pub use crate::widgets::checklistbox::{CheckListBox, CheckListBoxBuilder, CheckListBoxStyle}; // Added Style
pub use crate::widgets::choice::{Choice, ChoiceBuilder, ChoiceStyle};
pub use crate::widgets::collapsible_pane::{
    CollapsiblePane, CollapsiblePaneBuilder, CollapsiblePaneStyle,
};
pub use crate::widgets::colour_picker_ctrl::{
    ColourPickerCtrl, ColourPickerCtrlBuilder, ColourPickerCtrlStyle,
};
pub use crate::widgets::combobox::{ComboBox, ComboBoxBuilder, ComboBoxStyle};
pub use crate::widgets::command_link_button::{
    CommandLinkButton, CommandLinkButtonBuilder, CommandLinkButtonStyle,
}; // Added Style
   // ADDED: DataView
pub use crate::widgets::dataview::{
    CustomDataViewVirtualListModel, // Added CustomDataViewVirtualListModel
    DataViewAlign,
    DataViewCellMode,
    DataViewColumn,
    DataViewCtrl,
    DataViewCtrlBuilder,
    DataViewCustomRenderer, // Added DataViewCustomRenderer
    DataViewEventHandler,
    DataViewIconTextRenderer, // Added DataViewIconTextRenderer
    DataViewItem,
    DataViewItemAttr, // Added DataViewItemAttr
    // Events for DataView are now in dataview/event.rs, re-exported from dataview/mod.rs
    DataViewListCtrl,
    DataViewListCtrlBuilder,
    DataViewListModel,
    DataViewModel,
    DataViewStyle,
    DataViewTextRenderer, // Added DataViewTextRenderer
    DataViewTreeCtrl,
    DataViewTreeCtrlBuilder,
    Variant,
    VariantType, // Added VariantType
};
// Added DataView enums
pub use crate::widgets::dataview::enums::DataViewColumnFlags;
pub use crate::widgets::date_picker_ctrl::{
    DatePickerCtrl, DatePickerCtrlBuilder, DatePickerCtrlStyle,
};
pub use crate::widgets::dir_picker_ctrl::{
    DirPickerCtrl, DirPickerCtrlBuilder, DirPickerCtrlStyle,
};
pub use crate::widgets::editable_listbox::{
    EditableListBox, EditableListBoxBuilder, EditableListBoxStyle,
};
pub use crate::widgets::file_ctrl::{FileCtrl, FileCtrlBuilder, FileCtrlStyle};
pub use crate::widgets::file_picker_ctrl::{
    FilePickerCtrl, FilePickerCtrlBuilder, FilePickerCtrlStyle,
};
pub use crate::widgets::font_picker_ctrl::{
    FontPickerCtrl, FontPickerCtrlBuilder, FontPickerCtrlStyle,
};
pub use crate::widgets::frame::{Frame, FrameBuilder, FrameStyle};
pub use crate::widgets::gauge::{Gauge, GaugeBuilder, GaugeStyle};
pub use crate::widgets::hyperlink_ctrl::{HyperlinkCtrl, HyperlinkCtrlBuilder, HyperlinkCtrlStyle};
// ADDED: ImageList
pub use crate::widgets::imagelist::ImageList;
// ADDED: ItemData trait
pub use crate::widgets::item_data::{HasItemData, ItemData};
pub use crate::widgets::list_ctrl::{
    ListColumnFormat,
    ListCtrl,
    ListCtrlBuilder,
    ListCtrlStyle,
    ListItemState,
    ListNextItemFlag,
    // Events for ListCtrl are now in list_ctrl/event.rs, re-exported from list_ctrl/mod.rs
}; // Added Events
   // Added image_list_type for ListCtrl
pub use crate::widgets::list_ctrl::image_list_type;
pub use crate::widgets::listbox::{ListBox, ListBoxBuilder, ListBoxStyle};
#[cfg(feature = "media-ctrl")]
pub use crate::widgets::media_ctrl::{
    MediaCtrl, MediaCtrlBuilder, MediaCtrlPlayerControls, MediaState,
};
pub use crate::widgets::notebook::{Notebook, NotebookBuilder, NotebookStyle};
pub use crate::widgets::notification_message::{
    NotificationMessage,
    NotificationMessageBuilder,
    NotificationStyle,
    // Events for NotificationMessage are now in notification_message/event.rs, re-exported from notification_message/mod.rs
    TIMEOUT_AUTO,
    TIMEOUT_NEVER,
}; // Added Events
pub use crate::widgets::panel::{Panel, PanelBuilder, PanelStyle};
pub use crate::widgets::radio_button::{RadioButton, RadioButtonBuilder, RadioButtonStyle};
pub use crate::widgets::radiobox::{RadioBox, RadioBoxBuilder, RadioBoxStyle};
// Added RearrangeList
pub use crate::widgets::rearrangelist::{RearrangeList, RearrangeListBuilder, RearrangeListStyle};
#[cfg(feature = "richtext")]
pub use crate::widgets::richtextctrl::{
    RichTextCtrl, RichTextCtrlBuilder, RichTextCtrlEvent, RichTextCtrlEventData, RichTextCtrlStyle,
    RichTextFileType,
};
pub use crate::widgets::scrollbar::{ScrollBar, ScrollBarBuilder, ScrollBarStyle};
pub use crate::widgets::scrolled_window::{
    ScrolledWindow, ScrolledWindowBuilder, ScrolledWindowStyle,
}; // Added Style
pub use crate::widgets::search_ctrl::{SearchCtrl, SearchCtrlBuilder, SearchCtrlStyle};
pub use crate::widgets::slider::{Slider, SliderBuilder, SliderStyle};
pub use crate::widgets::spinbutton::{SpinButton, SpinButtonBuilder, SpinButtonStyle};
pub use crate::widgets::spinctrl::{SpinCtrl, SpinCtrlBuilder, SpinCtrlStyle};
pub use crate::widgets::spinctrl_double::{
    SpinCtrlDouble, SpinCtrlDoubleBuilder, SpinCtrlDoubleStyle,
};
pub use crate::widgets::splitter_window::{
    SplitterWindow,
    SplitterWindowBuilder,
    SplitterWindowStyle,
    // Events for SplitterWindow are now in splitterwindow/event.rs, re-exported from splitterwindow/mod.rs
}; // Added Style & Events
pub use crate::widgets::static_bitmap::{
    ScaleMode, StaticBitmap, StaticBitmapBuilder, StaticBitmapStyle,
}; // Added Style & ScaleMode
pub use crate::widgets::static_line::{StaticLine, StaticLineBuilder, StaticLineStyle};
pub use crate::widgets::static_text::{StaticText, StaticTextBuilder, StaticTextStyle};
pub use crate::widgets::staticbox::{StaticBox, StaticBoxBuilder, StaticBoxStyle}; // Added Style
pub use crate::widgets::statusbar::{StatusBar, StatusBarBuilder};
#[cfg(feature = "stc")]
pub use crate::widgets::styledtextctrl::{
    EolMode, FindFlags, Lexer, MarginType, MarkerSymbol, SelectionMode, StyledTextCtrl,
    StyledTextCtrlBuilder, StyledTextCtrlEvent, StyledTextCtrlEventData, StyledTextCtrlStyle,
    WhiteSpaceView, WrapMode,
};
pub use crate::widgets::taskbar_icon::{
    TaskBarIcon, TaskBarIconBuilder, TaskBarIconStyle, TaskBarIconType,
};
pub use crate::widgets::textctrl::{TextCtrl, TextCtrlBuilder, TextCtrlStyle};
pub use crate::widgets::time_picker_ctrl::{
    TimePickerCtrl, TimePickerCtrlBuilder, TimePickerCtrlStyle,
};
pub use crate::widgets::togglebutton::{ToggleButton, ToggleButtonBuilder, ToggleButtonStyle};
pub use crate::widgets::toolbar::{ToolBar, ToolBarStyle}; // Added Style
pub use crate::widgets::treebook::{Treebook, TreebookBuilder, TreebookStyle}; // Added Style
pub use crate::widgets::treectrl::{
    TreeCtrl, TreeCtrlBuilder, TreeCtrlStyle, TreeItemIcon, TreeItemId,
};

// --- Menus ---
pub use crate::menus::menuitem::{ID_ABOUT, ID_EXIT, ID_SEPARATOR};
pub use crate::menus::{ItemKind, Menu, MenuBar, MenuItem};

// --- Widgets ItemKind (for toolbar) ---
#[cfg(feature = "aui")]
pub use crate::widgets::ItemKind as WidgetItemKind;

// --- Bitmaps & Art ---
pub use crate::art_provider::{ArtClient, ArtId, ArtProvider};
pub use crate::bitmap::Bitmap;
pub use crate::bitmap_bundle::BitmapBundle; // Added BitmapBundle

// --- Dialogs ---
pub use crate::dialogs::colour_dialog::{ColourDialog, ColourDialogBuilder}; // Added Builder
pub use crate::dialogs::dir_dialog::{DirDialog, DirDialogBuilder, DirDialogStyle}; // Added DirDialog
pub use crate::dialogs::file_dialog::{FileDialog, FileDialogBuilder, FileDialogStyle}; // Added Builder
pub use crate::dialogs::font_dialog::{FontDialog, FontDialogBuilder}; // Added Builder
pub use crate::dialogs::message_dialog::{MessageDialog, MessageDialogBuilder, MessageDialogStyle};
pub use crate::dialogs::multi_choice_dialog::{MultiChoiceDialog, MultiChoiceDialogBuilder}; // Added MultiChoiceDialog
pub use crate::dialogs::progress_dialog::{
    ProgressDialog, ProgressDialogBuilder, ProgressDialogStyle,
}; // Added Builder
pub use crate::dialogs::single_choice_dialog::{SingleChoiceDialog, SingleChoiceDialogBuilder}; // Added SingleChoiceDialog
pub use crate::dialogs::text_entry_dialog::{
    TextEntryDialog, TextEntryDialogBuilder, TextEntryDialogStyle,
};
pub use crate::dialogs::{Dialog, DialogBuilder, DialogStyle}; // Base Dialog struct and builder

// --- Fonts ---
pub use crate::font::{Font, FontBuilder, FontFamily, FontStyle, FontWeight}; // Added FontBuilder
pub use crate::font_data::FontData;

// --- Drag and Drop ---
pub use crate::data_object::{BitmapDataObject, DataFormat};
pub use crate::dnd::{
    DataObject, DragResult, DropSource, FileDataObject, FileDropTarget, TextDataObject,
    TextDropTarget,
};

// --- Painting & DeviceContexts ---
pub use crate::dc::{
    AutoBufferedPaintDC, BackgroundMode, BrushStyle, ClientDC, DeviceContext, MemoryDC, PaintDC,
    PenStyle, ScreenDC, WindowDC,
};

// --- Application & Misc ---
// pub use crate::app::App; // Commented out as per previous error, App is in main or app module
pub use crate::appprogress::AppProgressIndicator;
pub use crate::timer::Timer; // Added Timer

// --- Constants for specific widgets that might be commonly used ---
// Example: ListBox specific constants
pub use crate::widgets::listbox::NOT_FOUND as LISTBOX_NOT_FOUND;
// Example: ComboBox specific constants
pub use crate::widgets::combobox::NOT_FOUND as COMBOBOX_NOT_FOUND;
// Example: NotificationMessage timeouts were already there

// --- XRC Support ---
#[cfg(feature = "xrc")]
pub use crate::xrc::{FromXrcPtr, WindowXrcMethods, XmlResource}; // Added XRC functionality

// --- Macros for custom widget development ---
pub use crate::custom_widget;

// --- Scrolling ---
pub use crate::scrollable::WxScrollable;
