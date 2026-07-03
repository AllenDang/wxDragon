use std::os::raw::{c_char, c_int, c_long, c_void};
use wxdragon_sys as ffi;

/// Status returned from every [`AccessibleImpl`] method, mirroring `wxAccStatus`.
///
/// The default trait methods return [`AccStatus::NotImplemented`], letting wxWidgets
/// fall back to its built-in accessible behaviour.
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AccStatus {
    Fail = ffi::wxd_AccStatus_WXD_ACC_FAIL as i32,
    False = ffi::wxd_AccStatus_WXD_ACC_FALSE as i32,
    Ok = ffi::wxd_AccStatus_WXD_ACC_OK as i32,
    NotImplemented = ffi::wxd_AccStatus_WXD_ACC_NOT_IMPLEMENTED as i32,
    NotSupported = ffi::wxd_AccStatus_WXD_ACC_NOT_SUPPORTED as i32,
    InvalidArg = ffi::wxd_AccStatus_WXD_ACC_INVALID_ARG as i32,
}

impl AccStatus {
    /// The raw `wxd_AccStatus` FFI value for this status.
    pub(crate) fn to_ffi(self) -> ffi::wxd_AccStatus {
        self as ffi::wxd_AccStatus
    }
}

/// Direction passed to [`AccessibleImpl::navigate`], mirroring `wxNavDir`.
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NavDir {
    Down = ffi::wxd_NavDir_WXD_NAVDIR_DOWN as i32,
    FirstChild = ffi::wxd_NavDir_WXD_NAVDIR_FIRSTCHILD as i32,
    LastChild = ffi::wxd_NavDir_WXD_NAVDIR_LASTCHILD as i32,
    Left = ffi::wxd_NavDir_WXD_NAVDIR_LEFT as i32,
    Next = ffi::wxd_NavDir_WXD_NAVDIR_NEXT as i32,
    Previous = ffi::wxd_NavDir_WXD_NAVDIR_PREVIOUS as i32,
    Right = ffi::wxd_NavDir_WXD_NAVDIR_RIGHT as i32,
    Up = ffi::wxd_NavDir_WXD_NAVDIR_UP as i32,
}

impl NavDir {
    /// Converts a raw `wxd_NavDir` from wxWidgets into a [`NavDir`], or `None` if the
    /// value is not a recognised direction.
    pub(crate) fn from_ffi(v: ffi::wxd_NavDir) -> Option<NavDir> {
        match v {
            ffi::wxd_NavDir_WXD_NAVDIR_DOWN => Some(NavDir::Down),
            ffi::wxd_NavDir_WXD_NAVDIR_FIRSTCHILD => Some(NavDir::FirstChild),
            ffi::wxd_NavDir_WXD_NAVDIR_LASTCHILD => Some(NavDir::LastChild),
            ffi::wxd_NavDir_WXD_NAVDIR_LEFT => Some(NavDir::Left),
            ffi::wxd_NavDir_WXD_NAVDIR_NEXT => Some(NavDir::Next),
            ffi::wxd_NavDir_WXD_NAVDIR_PREVIOUS => Some(NavDir::Previous),
            ffi::wxd_NavDir_WXD_NAVDIR_RIGHT => Some(NavDir::Right),
            ffi::wxd_NavDir_WXD_NAVDIR_UP => Some(NavDir::Up),
            _ => None,
        }
    }
}

/// Object type identifying the target of [`Accessible::notify_event`], mirroring the
/// MSAA `OBJID_*` values (`wxd_AccObjectType`).
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AccObjectType {
    Window = ffi::wxd_AccObjectType_WXD_ACC_OBJ_WINDOW as i32,
    SysMenu = ffi::wxd_AccObjectType_WXD_ACC_OBJ_SYSMENU as i32,
    TitleBar = ffi::wxd_AccObjectType_WXD_ACC_OBJ_TITLEBAR as i32,
    Menu = ffi::wxd_AccObjectType_WXD_ACC_OBJ_MENU as i32,
    Client = ffi::wxd_AccObjectType_WXD_ACC_OBJ_CLIENT as i32,
    VScroll = ffi::wxd_AccObjectType_WXD_ACC_OBJ_VSCROLL as i32,
    HScroll = ffi::wxd_AccObjectType_WXD_ACC_OBJ_HSCROLL as i32,
    SizeGrip = ffi::wxd_AccObjectType_WXD_ACC_OBJ_SIZEGRIP as i32,
    Caret = ffi::wxd_AccObjectType_WXD_ACC_OBJ_CARET as i32,
    Cursor = ffi::wxd_AccObjectType_WXD_ACC_OBJ_CURSOR as i32,
    Alert = ffi::wxd_AccObjectType_WXD_ACC_OBJ_ALERT as i32,
    Sound = ffi::wxd_AccObjectType_WXD_ACC_OBJ_SOUND as i32,
}

impl AccObjectType {
    /// The raw object-type value (an MSAA `OBJID_*`) for `wxd_Accessible_NotifyEvent`.
    pub(crate) fn to_ffi(self) -> i32 {
        self as i32
    }
}

/// Accessibility role of an object (the MSAA `ROLE_SYSTEM_*` set).
///
/// Used by [`crate::window::WxWidget::set_accessibility_role`] and returned from
/// [`AccessibleImpl::get_role`]. Variants drop the redundant `SYSTEM_` prefix
/// (`ROLE_SYSTEM_TEXT` becomes [`AccRole::Text`]); discriminants map directly onto
/// `wxAccRole`.
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AccRole {
    None = ffi::wxd_AccRole_WXD_ROLE_NONE as i32,
    Alert = ffi::wxd_AccRole_WXD_ROLE_SYSTEM_ALERT as i32,
    Animation = ffi::wxd_AccRole_WXD_ROLE_SYSTEM_ANIMATION as i32,
    Application = ffi::wxd_AccRole_WXD_ROLE_SYSTEM_APPLICATION as i32,
    Border = ffi::wxd_AccRole_WXD_ROLE_SYSTEM_BORDER as i32,
    ButtonDropDown = ffi::wxd_AccRole_WXD_ROLE_SYSTEM_BUTTONDROPDOWN as i32,
    ButtonDropDownGrid = ffi::wxd_AccRole_WXD_ROLE_SYSTEM_BUTTONDROPDOWNGRID as i32,
    ButtonMenu = ffi::wxd_AccRole_WXD_ROLE_SYSTEM_BUTTONMENU as i32,
    Caret = ffi::wxd_AccRole_WXD_ROLE_SYSTEM_CARET as i32,
    Cell = ffi::wxd_AccRole_WXD_ROLE_SYSTEM_CELL as i32,
    Character = ffi::wxd_AccRole_WXD_ROLE_SYSTEM_CHARACTER as i32,
    Chart = ffi::wxd_AccRole_WXD_ROLE_SYSTEM_CHART as i32,
    CheckButton = ffi::wxd_AccRole_WXD_ROLE_SYSTEM_CHECKBUTTON as i32,
    Client = ffi::wxd_AccRole_WXD_ROLE_SYSTEM_CLIENT as i32,
    Clock = ffi::wxd_AccRole_WXD_ROLE_SYSTEM_CLOCK as i32,
    Column = ffi::wxd_AccRole_WXD_ROLE_SYSTEM_COLUMN as i32,
    ColumnHeader = ffi::wxd_AccRole_WXD_ROLE_SYSTEM_COLUMNHEADER as i32,
    ComboBox = ffi::wxd_AccRole_WXD_ROLE_SYSTEM_COMBOBOX as i32,
    Cursor = ffi::wxd_AccRole_WXD_ROLE_SYSTEM_CURSOR as i32,
    Diagram = ffi::wxd_AccRole_WXD_ROLE_SYSTEM_DIAGRAM as i32,
    Dial = ffi::wxd_AccRole_WXD_ROLE_SYSTEM_DIAL as i32,
    Dialog = ffi::wxd_AccRole_WXD_ROLE_SYSTEM_DIALOG as i32,
    Document = ffi::wxd_AccRole_WXD_ROLE_SYSTEM_DOCUMENT as i32,
    DropList = ffi::wxd_AccRole_WXD_ROLE_SYSTEM_DROPLIST as i32,
    Equation = ffi::wxd_AccRole_WXD_ROLE_SYSTEM_EQUATION as i32,
    Graphic = ffi::wxd_AccRole_WXD_ROLE_SYSTEM_GRAPHIC as i32,
    Grip = ffi::wxd_AccRole_WXD_ROLE_SYSTEM_GRIP as i32,
    Grouping = ffi::wxd_AccRole_WXD_ROLE_SYSTEM_GROUPING as i32,
    HelpBalloon = ffi::wxd_AccRole_WXD_ROLE_SYSTEM_HELPBALLOON as i32,
    HotkeyField = ffi::wxd_AccRole_WXD_ROLE_SYSTEM_HOTKEYFIELD as i32,
    Indicator = ffi::wxd_AccRole_WXD_ROLE_SYSTEM_INDICATOR as i32,
    Link = ffi::wxd_AccRole_WXD_ROLE_SYSTEM_LINK as i32,
    List = ffi::wxd_AccRole_WXD_ROLE_SYSTEM_LIST as i32,
    ListItem = ffi::wxd_AccRole_WXD_ROLE_SYSTEM_LISTITEM as i32,
    MenuBar = ffi::wxd_AccRole_WXD_ROLE_SYSTEM_MENUBAR as i32,
    MenuItem = ffi::wxd_AccRole_WXD_ROLE_SYSTEM_MENUITEM as i32,
    MenuPopup = ffi::wxd_AccRole_WXD_ROLE_SYSTEM_MENUPOPUP as i32,
    Outline = ffi::wxd_AccRole_WXD_ROLE_SYSTEM_OUTLINE as i32,
    OutlineItem = ffi::wxd_AccRole_WXD_ROLE_SYSTEM_OUTLINEITEM as i32,
    PageTab = ffi::wxd_AccRole_WXD_ROLE_SYSTEM_PAGETAB as i32,
    PageTabList = ffi::wxd_AccRole_WXD_ROLE_SYSTEM_PAGETABLIST as i32,
    Pane = ffi::wxd_AccRole_WXD_ROLE_SYSTEM_PANE as i32,
    ProgressBar = ffi::wxd_AccRole_WXD_ROLE_SYSTEM_PROGRESSBAR as i32,
    PropertyPage = ffi::wxd_AccRole_WXD_ROLE_SYSTEM_PROPERTYPAGE as i32,
    PushButton = ffi::wxd_AccRole_WXD_ROLE_SYSTEM_PUSHBUTTON as i32,
    RadioButton = ffi::wxd_AccRole_WXD_ROLE_SYSTEM_RADIOBUTTON as i32,
    Row = ffi::wxd_AccRole_WXD_ROLE_SYSTEM_ROW as i32,
    RowHeader = ffi::wxd_AccRole_WXD_ROLE_SYSTEM_ROWHEADER as i32,
    ScrollBar = ffi::wxd_AccRole_WXD_ROLE_SYSTEM_SCROLLBAR as i32,
    Separator = ffi::wxd_AccRole_WXD_ROLE_SYSTEM_SEPARATOR as i32,
    Slider = ffi::wxd_AccRole_WXD_ROLE_SYSTEM_SLIDER as i32,
    Sound = ffi::wxd_AccRole_WXD_ROLE_SYSTEM_SOUND as i32,
    SpinButton = ffi::wxd_AccRole_WXD_ROLE_SYSTEM_SPINBUTTON as i32,
    StaticText = ffi::wxd_AccRole_WXD_ROLE_SYSTEM_STATICTEXT as i32,
    StatusBar = ffi::wxd_AccRole_WXD_ROLE_SYSTEM_STATUSBAR as i32,
    Table = ffi::wxd_AccRole_WXD_ROLE_SYSTEM_TABLE as i32,
    Text = ffi::wxd_AccRole_WXD_ROLE_SYSTEM_TEXT as i32,
    TitleBar = ffi::wxd_AccRole_WXD_ROLE_SYSTEM_TITLEBAR as i32,
    ToolBar = ffi::wxd_AccRole_WXD_ROLE_SYSTEM_TOOLBAR as i32,
    ToolTip = ffi::wxd_AccRole_WXD_ROLE_SYSTEM_TOOLTIP as i32,
    Whitespace = ffi::wxd_AccRole_WXD_ROLE_SYSTEM_WHITESPACE as i32,
    Window = ffi::wxd_AccRole_WXD_ROLE_SYSTEM_WINDOW as i32,
}

impl AccRole {
    /// The raw `wxd_AccRole` FFI value for this role.
    pub(crate) fn to_ffi(self) -> ffi::wxd_AccRole {
        self as ffi::wxd_AccRole
    }
}

bitflags::bitflags! {
    /// Accessibility state flags (the MSAA `STATE_SYSTEM_*` set).
    ///
    /// A state is a bitmask, so values are combined with `|`, e.g.
    /// `AccState::FOCUSED | AccState::SELECTED`. Used by
    /// [`crate::window::WxWidget::set_accessibility_state`] and returned from
    /// [`AccessibleImpl::get_state`].
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct AccState: i64 {
        const UNAVAILABLE = ffi::WXD_ACC_STATE_SYSTEM_UNAVAILABLE as i64;
        const SELECTED = ffi::WXD_ACC_STATE_SYSTEM_SELECTED as i64;
        const FOCUSED = ffi::WXD_ACC_STATE_SYSTEM_FOCUSED as i64;
        const PRESSED = ffi::WXD_ACC_STATE_SYSTEM_PRESSED as i64;
        const CHECKED = ffi::WXD_ACC_STATE_SYSTEM_CHECKED as i64;
        const MIXED = ffi::WXD_ACC_STATE_SYSTEM_MIXED as i64;
        const READONLY = ffi::WXD_ACC_STATE_SYSTEM_READONLY as i64;
        const HOTTRACKED = ffi::WXD_ACC_STATE_SYSTEM_HOTTRACKED as i64;
        const DEFAULT = ffi::WXD_ACC_STATE_SYSTEM_DEFAULT as i64;
        const EXPANDED = ffi::WXD_ACC_STATE_SYSTEM_EXPANDED as i64;
        const COLLAPSED = ffi::WXD_ACC_STATE_SYSTEM_COLLAPSED as i64;
        const BUSY = ffi::WXD_ACC_STATE_SYSTEM_BUSY as i64;
        const FLOATING = ffi::WXD_ACC_STATE_SYSTEM_FLOATING as i64;
        const MARQUEED = ffi::WXD_ACC_STATE_SYSTEM_MARQUEED as i64;
        const ANIMATED = ffi::WXD_ACC_STATE_SYSTEM_ANIMATED as i64;
        const INVISIBLE = ffi::WXD_ACC_STATE_SYSTEM_INVISIBLE as i64;
        const OFFSCREEN = ffi::WXD_ACC_STATE_SYSTEM_OFFSCREEN as i64;
        const SIZEABLE = ffi::WXD_ACC_STATE_SYSTEM_SIZEABLE as i64;
        const MOVEABLE = ffi::WXD_ACC_STATE_SYSTEM_MOVEABLE as i64;
        const SELFVOICING = ffi::WXD_ACC_STATE_SYSTEM_SELFVOICING as i64;
        const FOCUSABLE = ffi::WXD_ACC_STATE_SYSTEM_FOCUSABLE as i64;
        const SELECTABLE = ffi::WXD_ACC_STATE_SYSTEM_SELECTABLE as i64;
        const LINKED = ffi::WXD_ACC_STATE_SYSTEM_LINKED as i64;
        const TRAVERSED = ffi::WXD_ACC_STATE_SYSTEM_TRAVERSED as i64;
        const MULTISELECTABLE = ffi::WXD_ACC_STATE_SYSTEM_MULTISELECTABLE as i64;
        const EXTSELECTABLE = ffi::WXD_ACC_STATE_SYSTEM_EXTSELECTABLE as i64;
        const HASPOPUP = ffi::WXD_ACC_STATE_SYSTEM_HASPOPUP as i64;
    }
}

bitflags::bitflags! {
    /// Selection flags passed to [`AccessibleImpl::select`] (the MSAA `SELFLAG_*` set).
    ///
    /// A selection request is a bitmask, so values are combined with `|`, e.g.
    /// `AccSelectFlags::TAKEFOCUS | AccSelectFlags::TAKESELECTION`. An empty set
    /// ([`AccSelectFlags::empty`]) corresponds to `SELFLAG_NONE`.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct AccSelectFlags: i32 {
        const TAKEFOCUS = ffi::WXD_ACC_SEL_TAKEFOCUS as i32;
        const TAKESELECTION = ffi::WXD_ACC_SEL_TAKESELECTION as i32;
        const EXTENDSELECTION = ffi::WXD_ACC_SEL_EXTENDSELECTION as i32;
        const ADDSELECTION = ffi::WXD_ACC_SEL_ADDSELECTION as i32;
        const REMOVESELECTION = ffi::WXD_ACC_SEL_REMOVESELECTION as i32;
    }
}

/// A trait that can be implemented to provide custom accessibility information.
pub trait AccessibleImpl {
    fn get_child_count(&self) -> (AccStatus, i32) {
        (AccStatus::Ok, 0)
    }
    fn get_child(&self, _child_id: i32) -> (AccStatus, Option<Accessible>) {
        (AccStatus::NotImplemented, None)
    }
    fn get_parent(&self) -> (AccStatus, Option<Accessible>) {
        (AccStatus::NotImplemented, None)
    }
    fn get_role(&self, _child_id: i32) -> (AccStatus, AccRole) {
        (AccStatus::NotImplemented, AccRole::None)
    }
    fn get_state(&self, _child_id: i32) -> (AccStatus, AccState) {
        (AccStatus::NotImplemented, AccState::empty())
    }
    fn get_name(&self, _child_id: i32) -> (AccStatus, Option<String>) {
        (AccStatus::NotImplemented, None)
    }
    fn get_description(&self, _child_id: i32) -> (AccStatus, Option<String>) {
        (AccStatus::NotImplemented, None)
    }
    fn get_help_text(&self, _child_id: i32) -> (AccStatus, Option<String>) {
        (AccStatus::NotImplemented, None)
    }
    fn get_keyboard_shortcut(&self, _child_id: i32) -> (AccStatus, Option<String>) {
        (AccStatus::NotImplemented, None)
    }
    fn get_default_action(&self, _child_id: i32) -> (AccStatus, Option<String>) {
        (AccStatus::NotImplemented, None)
    }
    fn get_value(&self, _child_id: i32) -> (AccStatus, Option<String>) {
        (AccStatus::NotImplemented, None)
    }
    fn select(&self, _child_id: i32, _select_flags: AccSelectFlags) -> AccStatus {
        AccStatus::NotImplemented
    }
    fn get_selections(&self) -> (AccStatus, crate::widgets::dataview::Variant) {
        (AccStatus::NotImplemented, crate::widgets::dataview::Variant::new())
    }
    fn get_focus(&self) -> (AccStatus, i32, Option<Accessible>) {
        (AccStatus::NotImplemented, 0, None)
    }
    fn do_default_action(&self, _child_id: i32) -> AccStatus {
        AccStatus::NotImplemented
    }
    fn get_location(&self, _child_id: i32) -> (AccStatus, crate::geometry::Rect) {
        (AccStatus::NotImplemented, crate::geometry::Rect::default())
    }
    fn hit_test(&self, _pt: crate::geometry::Point) -> (AccStatus, i32, Option<Accessible>) {
        (AccStatus::NotImplemented, 0, None)
    }
    fn navigate(&self, _nav_dir: NavDir, _from_id: i32) -> (AccStatus, i32, Option<Accessible>) {
        (AccStatus::NotImplemented, 0, None)
    }
}

/// A wrapper around wxAccessible for providing accessibility information.
pub struct Accessible {
    pub(crate) ptr: *mut ffi::wxd_Accessible_t,
    pub(crate) owned: bool,
}

impl Accessible {
    /// Creates a new custom Accessible object.
    pub fn new<T: AccessibleImpl + 'static>(window: &dyn crate::window::WxWidget, implementation: T) -> Self {
        let user_data = Box::into_raw(Box::new(implementation));

        let callbacks = ffi::wxd_AccessibleCallbacks {
            GetChildCount: Some(accessible_get_child_count::<T>),
            GetChild: Some(accessible_get_child::<T>),
            GetParent: Some(accessible_get_parent::<T>),
            GetRole: Some(accessible_get_role::<T>),
            GetState: Some(accessible_get_state::<T>),
            GetName: Some(accessible_get_name::<T>),
            GetDescription: Some(accessible_get_description::<T>),
            GetHelpText: Some(accessible_get_help_text::<T>),
            GetKeyboardShortcut: Some(accessible_get_keyboard_shortcut::<T>),
            GetDefaultAction: Some(accessible_get_default_action::<T>),
            GetValue: Some(accessible_get_value::<T>),
            Select: Some(accessible_select::<T>),
            GetSelections: Some(accessible_get_selections::<T>),
            GetFocus: Some(accessible_get_focus::<T>),
            DoDefaultAction: Some(accessible_do_default_action::<T>),
            GetLocation: Some(accessible_get_location::<T>),
            HitTest: Some(accessible_hit_test::<T>),
            Navigate: Some(accessible_navigate::<T>),
        };

        let ptr = unsafe { ffi::wxd_Accessible_Create(window.handle_ptr(), callbacks, user_data as *mut c_void) };
        Self { ptr, owned: true }
    }

    /// Creates an `Accessible` from a raw pointer.
    ///
    /// # Safety
    /// The caller must ensure the pointer is valid.
    pub unsafe fn from_ptr(ptr: *mut ffi::wxd_Accessible_t, owned: bool) -> Self {
        Self { ptr, owned }
    }

    /// Returns the underlying raw pointer.
    pub fn as_ptr(&self) -> *mut ffi::wxd_Accessible_t {
        self.ptr
    }

    /// Notifies the accessibility system of an event.
    pub fn notify_event(event_type: u32, window: &dyn crate::window::WxWidget, object_type: AccObjectType, object_id: i32) {
        unsafe {
            ffi::wxd_Accessible_NotifyEvent(event_type, window.handle_ptr(), object_type.to_ffi(), object_id);
        }
    }
}

impl Drop for Accessible {
    fn drop(&mut self) {
        if self.owned && !self.ptr.is_null() {
            unsafe {
                ffi::wxd_Accessible_Destroy(self.ptr);
            }
        }
    }
}

// --- Internal Callback Forwarders ---

unsafe extern "C" fn accessible_get_child_count<T: AccessibleImpl>(
    user_data: *mut c_void,
    count: *mut c_int,
) -> ffi::wxd_AccStatus {
    let impl_ptr = user_data as *const T;
    let (status, c) = unsafe { (*impl_ptr).get_child_count() };
    unsafe { *count = c };
    status.to_ffi()
}

unsafe extern "C" fn accessible_get_child<T: AccessibleImpl>(
    user_data: *mut c_void,
    child_id: c_int,
    child: *mut *mut ffi::wxd_Accessible_t,
) -> ffi::wxd_AccStatus {
    let impl_ptr = user_data as *const T;
    let (status, acc) = unsafe { (*impl_ptr).get_child(child_id) };
    if let Some(a) = acc {
        unsafe { *child = a.as_ptr() };
        std::mem::forget(a); // C++ will manage the pointer
    } else {
        unsafe { *child = std::ptr::null_mut() };
    }
    status.to_ffi()
}

unsafe extern "C" fn accessible_get_parent<T: AccessibleImpl>(
    user_data: *mut c_void,
    parent: *mut *mut ffi::wxd_Accessible_t,
) -> ffi::wxd_AccStatus {
    let impl_ptr = user_data as *const T;
    let (status, acc) = unsafe { (*impl_ptr).get_parent() };
    if let Some(a) = acc {
        unsafe { *parent = a.as_ptr() };
        std::mem::forget(a);
    } else {
        unsafe { *parent = std::ptr::null_mut() };
    }
    status.to_ffi()
}

unsafe extern "C" fn accessible_get_role<T: AccessibleImpl>(
    user_data: *mut c_void,
    child_id: c_int,
    role: *mut ffi::wxd_AccRole,
) -> ffi::wxd_AccStatus {
    let impl_ptr = user_data as *const T;
    let (status, r) = unsafe { (*impl_ptr).get_role(child_id) };
    unsafe { *role = r.to_ffi() };
    status.to_ffi()
}

unsafe extern "C" fn accessible_get_state<T: AccessibleImpl>(
    user_data: *mut c_void,
    child_id: c_int,
    state: *mut c_long,
) -> ffi::wxd_AccStatus {
    let impl_ptr = user_data as *const T;
    let (status, s) = unsafe { (*impl_ptr).get_state(child_id) };
    unsafe { *state = s.bits() as c_long };
    status.to_ffi()
}

unsafe extern "C" fn accessible_get_name<T: AccessibleImpl>(
    user_data: *mut c_void,
    child_id: c_int,
    out_name: *mut c_char,
    max_len: usize,
) -> ffi::wxd_AccStatus {
    let impl_ptr = user_data as *const T;
    let (status, name) = unsafe { (*impl_ptr).get_name(child_id) };
    if let Some(n) = name {
        copy_string_to_c(n, out_name, max_len);
    }
    status.to_ffi()
}

unsafe extern "C" fn accessible_get_description<T: AccessibleImpl>(
    user_data: *mut c_void,
    child_id: c_int,
    out_description: *mut c_char,
    max_len: usize,
) -> ffi::wxd_AccStatus {
    let impl_ptr = user_data as *const T;
    let (status, desc) = unsafe { (*impl_ptr).get_description(child_id) };
    if let Some(d) = desc {
        copy_string_to_c(d, out_description, max_len);
    }
    status.to_ffi()
}

unsafe extern "C" fn accessible_get_help_text<T: AccessibleImpl>(
    user_data: *mut c_void,
    child_id: c_int,
    out_help_text: *mut c_char,
    max_len: usize,
) -> ffi::wxd_AccStatus {
    let impl_ptr = user_data as *const T;
    let (status, text) = unsafe { (*impl_ptr).get_help_text(child_id) };
    if let Some(t) = text {
        copy_string_to_c(t, out_help_text, max_len);
    }
    status.to_ffi()
}

unsafe extern "C" fn accessible_get_keyboard_shortcut<T: AccessibleImpl>(
    user_data: *mut c_void,
    child_id: c_int,
    out_shortcut: *mut c_char,
    max_len: usize,
) -> ffi::wxd_AccStatus {
    let impl_ptr = user_data as *const T;
    let (status, shortcut) = unsafe { (*impl_ptr).get_keyboard_shortcut(child_id) };
    if let Some(s) = shortcut {
        copy_string_to_c(s, out_shortcut, max_len);
    }
    status.to_ffi()
}

unsafe extern "C" fn accessible_get_default_action<T: AccessibleImpl>(
    user_data: *mut c_void,
    child_id: c_int,
    out_action: *mut c_char,
    max_len: usize,
) -> ffi::wxd_AccStatus {
    let impl_ptr = user_data as *const T;
    let (status, action) = unsafe { (*impl_ptr).get_default_action(child_id) };
    if let Some(a) = action {
        copy_string_to_c(a, out_action, max_len);
    }
    status.to_ffi()
}

unsafe extern "C" fn accessible_get_value<T: AccessibleImpl>(
    user_data: *mut c_void,
    child_id: c_int,
    out_value: *mut c_char,
    max_len: usize,
) -> ffi::wxd_AccStatus {
    let impl_ptr = user_data as *const T;
    let (status, value) = unsafe { (*impl_ptr).get_value(child_id) };
    if let Some(v) = value {
        copy_string_to_c(v, out_value, max_len);
    }
    status.to_ffi()
}

unsafe extern "C" fn accessible_select<T: AccessibleImpl>(
    user_data: *mut c_void,
    child_id: c_int,
    select_flags: c_int,
) -> ffi::wxd_AccStatus {
    let impl_ptr = user_data as *const T;
    let flags = AccSelectFlags::from_bits_retain(select_flags);
    unsafe { (*impl_ptr).select(child_id, flags) }.to_ffi()
}

unsafe extern "C" fn accessible_get_selections<T: AccessibleImpl>(
    user_data: *mut c_void,
    selections: *mut ffi::wxd_Variant_t,
) -> ffi::wxd_AccStatus {
    let impl_ptr = user_data as *const T;
    let (status, sel) = unsafe { (*impl_ptr).get_selections() };
    if status == AccStatus::Ok && !selections.is_null() {
        unsafe { ffi::wxd_Variant_Assign(selections, sel.as_const_ptr()) };
    }
    status.to_ffi()
}

unsafe extern "C" fn accessible_get_focus<T: AccessibleImpl>(
    user_data: *mut c_void,
    child_id: *mut c_int,
    child: *mut *mut ffi::wxd_Accessible_t,
) -> ffi::wxd_AccStatus {
    let impl_ptr = user_data as *const T;
    let (status, id, acc) = unsafe { (*impl_ptr).get_focus() };
    unsafe { *child_id = id };
    if let Some(a) = acc {
        unsafe { *child = a.as_ptr() };
        std::mem::forget(a);
    } else {
        unsafe { *child = std::ptr::null_mut() };
    }
    status.to_ffi()
}

unsafe extern "C" fn accessible_do_default_action<T: AccessibleImpl>(
    user_data: *mut c_void,
    child_id: c_int,
) -> ffi::wxd_AccStatus {
    let impl_ptr = user_data as *const T;
    unsafe { (*impl_ptr).do_default_action(child_id) }.to_ffi()
}

unsafe extern "C" fn accessible_get_location<T: AccessibleImpl>(
    user_data: *mut c_void,
    child_id: c_int,
    rect: *mut ffi::wxd_Rect,
) -> ffi::wxd_AccStatus {
    let impl_ptr = user_data as *const T;
    let (status, r) = unsafe { (*impl_ptr).get_location(child_id) };
    unsafe { *rect = r.into() };
    status.to_ffi()
}

unsafe extern "C" fn accessible_hit_test<T: AccessibleImpl>(
    user_data: *mut c_void,
    pt: ffi::wxd_Point,
    child_id: *mut c_int,
    child_object: *mut *mut ffi::wxd_Accessible_t,
) -> ffi::wxd_AccStatus {
    let impl_ptr = user_data as *const T;
    let (status, id, acc) = unsafe { (*impl_ptr).hit_test(crate::geometry::Point { x: pt.x, y: pt.y }) };
    unsafe { *child_id = id };
    if let Some(a) = acc {
        unsafe { *child_object = a.as_ptr() };
        std::mem::forget(a);
    } else {
        unsafe { *child_object = std::ptr::null_mut() };
    }
    status.to_ffi()
}

unsafe extern "C" fn accessible_navigate<T: AccessibleImpl>(
    user_data: *mut c_void,
    nav_dir: ffi::wxd_NavDir,
    from_id: c_int,
    to_id: *mut c_int,
    to_object: *mut *mut ffi::wxd_Accessible_t,
) -> ffi::wxd_AccStatus {
    let impl_ptr = user_data as *const T;
    let Some(dir) = NavDir::from_ffi(nav_dir) else {
        unsafe {
            *to_id = 0;
            *to_object = std::ptr::null_mut();
        }
        return AccStatus::InvalidArg.to_ffi();
    };
    let (status, id, acc) = unsafe { (*impl_ptr).navigate(dir, from_id) };
    unsafe { *to_id = id };
    if let Some(a) = acc {
        unsafe { *to_object = a.as_ptr() };
        std::mem::forget(a);
    } else {
        unsafe { *to_object = std::ptr::null_mut() };
    }
    status.to_ffi()
}

fn copy_string_to_c(s: String, out_buf: *mut c_char, max_len: usize) {
    let c_str = std::ffi::CString::new(s).unwrap_or_default();
    let bytes = c_str.as_bytes_with_nul();
    let len = std::cmp::min(bytes.len(), max_len);
    unsafe {
        std::ptr::copy_nonoverlapping(bytes.as_ptr() as *const c_char, out_buf, len);
        if len < max_len {
            *out_buf.add(len) = 0;
        } else {
            *out_buf.add(max_len - 1) = 0;
        }
    }
}
