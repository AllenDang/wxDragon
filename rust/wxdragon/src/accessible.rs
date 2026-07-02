use std::os::raw::{c_char, c_int, c_long, c_void};
use wxdragon_sys as ffi;

pub type AccStatus = ffi::wxd_AccStatus;
pub type NavDir = ffi::wxd_NavDir;
pub type AccRole = ffi::wxd_AccRole;

/// State flags for accessibility objects.
pub mod acc_state {
    use wxdragon_sys as ffi;
    pub const UNAVAILABLE: i64 = ffi::WXD_ACC_STATE_SYSTEM_UNAVAILABLE as i64;
    pub const SELECTED: i64 = ffi::WXD_ACC_STATE_SYSTEM_SELECTED as i64;
    pub const FOCUSED: i64 = ffi::WXD_ACC_STATE_SYSTEM_FOCUSED as i64;
    pub const PRESSED: i64 = ffi::WXD_ACC_STATE_SYSTEM_PRESSED as i64;
    pub const CHECKED: i64 = ffi::WXD_ACC_STATE_SYSTEM_CHECKED as i64;
    pub const MIXED: i64 = ffi::WXD_ACC_STATE_SYSTEM_MIXED as i64;
    pub const READONLY: i64 = ffi::WXD_ACC_STATE_SYSTEM_READONLY as i64;
    pub const HOTTRACKED: i64 = ffi::WXD_ACC_STATE_SYSTEM_HOTTRACKED as i64;
    pub const DEFAULT: i64 = ffi::WXD_ACC_STATE_SYSTEM_DEFAULT as i64;
    pub const EXPANDED: i64 = ffi::WXD_ACC_STATE_SYSTEM_EXPANDED as i64;
    pub const COLLAPSED: i64 = ffi::WXD_ACC_STATE_SYSTEM_COLLAPSED as i64;
    pub const BUSY: i64 = ffi::WXD_ACC_STATE_SYSTEM_BUSY as i64;
    pub const FLOATING: i64 = ffi::WXD_ACC_STATE_SYSTEM_FLOATING as i64;
    pub const MARQUEED: i64 = ffi::WXD_ACC_STATE_SYSTEM_MARQUEED as i64;
    pub const ANIMATED: i64 = ffi::WXD_ACC_STATE_SYSTEM_ANIMATED as i64;
    pub const INVISIBLE: i64 = ffi::WXD_ACC_STATE_SYSTEM_INVISIBLE as i64;
    pub const OFFSCREEN: i64 = ffi::WXD_ACC_STATE_SYSTEM_OFFSCREEN as i64;
    pub const SIZEABLE: i64 = ffi::WXD_ACC_STATE_SYSTEM_SIZEABLE as i64;
    pub const MOVEABLE: i64 = ffi::WXD_ACC_STATE_SYSTEM_MOVEABLE as i64;
    pub const SELFVOICING: i64 = ffi::WXD_ACC_STATE_SYSTEM_SELFVOICING as i64;
    pub const FOCUSABLE: i64 = ffi::WXD_ACC_STATE_SYSTEM_FOCUSABLE as i64;
    pub const SELECTABLE: i64 = ffi::WXD_ACC_STATE_SYSTEM_SELECTABLE as i64;
    pub const LINKED: i64 = ffi::WXD_ACC_STATE_SYSTEM_LINKED as i64;
    pub const TRAVERSED: i64 = ffi::WXD_ACC_STATE_SYSTEM_TRAVERSED as i64;
    pub const MULTISELECTABLE: i64 = ffi::WXD_ACC_STATE_SYSTEM_MULTISELECTABLE as i64;
    pub const EXTSELECTABLE: i64 = ffi::WXD_ACC_STATE_SYSTEM_EXTSELECTABLE as i64;
    pub const HASPOPUP: i64 = ffi::WXD_ACC_STATE_SYSTEM_HASPOPUP as i64;
}

/// Named [`AccRole`] values (the MSAA role set).
///
/// Use these with [`crate::window::WxWidget::set_accessibility_role`] instead of the raw
/// FFI constants, e.g. `widget.set_accessibility_role(acc_role::SYSTEM_TEXT)`.
pub mod acc_role {
    use super::AccRole;
    use wxdragon_sys as ffi;

    pub const NONE: AccRole = ffi::wxd_AccRole_WXD_ROLE_NONE;
    pub const SYSTEM_ALERT: AccRole = ffi::wxd_AccRole_WXD_ROLE_SYSTEM_ALERT;
    pub const SYSTEM_ANIMATION: AccRole = ffi::wxd_AccRole_WXD_ROLE_SYSTEM_ANIMATION;
    pub const SYSTEM_APPLICATION: AccRole = ffi::wxd_AccRole_WXD_ROLE_SYSTEM_APPLICATION;
    pub const SYSTEM_BORDER: AccRole = ffi::wxd_AccRole_WXD_ROLE_SYSTEM_BORDER;
    pub const SYSTEM_BUTTONDROPDOWN: AccRole = ffi::wxd_AccRole_WXD_ROLE_SYSTEM_BUTTONDROPDOWN;
    pub const SYSTEM_BUTTONDROPDOWNGRID: AccRole = ffi::wxd_AccRole_WXD_ROLE_SYSTEM_BUTTONDROPDOWNGRID;
    pub const SYSTEM_BUTTONMENU: AccRole = ffi::wxd_AccRole_WXD_ROLE_SYSTEM_BUTTONMENU;
    pub const SYSTEM_CARET: AccRole = ffi::wxd_AccRole_WXD_ROLE_SYSTEM_CARET;
    pub const SYSTEM_CELL: AccRole = ffi::wxd_AccRole_WXD_ROLE_SYSTEM_CELL;
    pub const SYSTEM_CHARACTER: AccRole = ffi::wxd_AccRole_WXD_ROLE_SYSTEM_CHARACTER;
    pub const SYSTEM_CHART: AccRole = ffi::wxd_AccRole_WXD_ROLE_SYSTEM_CHART;
    pub const SYSTEM_CHECKBUTTON: AccRole = ffi::wxd_AccRole_WXD_ROLE_SYSTEM_CHECKBUTTON;
    pub const SYSTEM_CLIENT: AccRole = ffi::wxd_AccRole_WXD_ROLE_SYSTEM_CLIENT;
    pub const SYSTEM_CLOCK: AccRole = ffi::wxd_AccRole_WXD_ROLE_SYSTEM_CLOCK;
    pub const SYSTEM_COLUMN: AccRole = ffi::wxd_AccRole_WXD_ROLE_SYSTEM_COLUMN;
    pub const SYSTEM_COLUMNHEADER: AccRole = ffi::wxd_AccRole_WXD_ROLE_SYSTEM_COLUMNHEADER;
    pub const SYSTEM_COMBOBOX: AccRole = ffi::wxd_AccRole_WXD_ROLE_SYSTEM_COMBOBOX;
    pub const SYSTEM_CURSOR: AccRole = ffi::wxd_AccRole_WXD_ROLE_SYSTEM_CURSOR;
    pub const SYSTEM_DIAGRAM: AccRole = ffi::wxd_AccRole_WXD_ROLE_SYSTEM_DIAGRAM;
    pub const SYSTEM_DIAL: AccRole = ffi::wxd_AccRole_WXD_ROLE_SYSTEM_DIAL;
    pub const SYSTEM_DIALOG: AccRole = ffi::wxd_AccRole_WXD_ROLE_SYSTEM_DIALOG;
    pub const SYSTEM_DOCUMENT: AccRole = ffi::wxd_AccRole_WXD_ROLE_SYSTEM_DOCUMENT;
    pub const SYSTEM_DROPLIST: AccRole = ffi::wxd_AccRole_WXD_ROLE_SYSTEM_DROPLIST;
    pub const SYSTEM_EQUATION: AccRole = ffi::wxd_AccRole_WXD_ROLE_SYSTEM_EQUATION;
    pub const SYSTEM_GRAPHIC: AccRole = ffi::wxd_AccRole_WXD_ROLE_SYSTEM_GRAPHIC;
    pub const SYSTEM_GRIP: AccRole = ffi::wxd_AccRole_WXD_ROLE_SYSTEM_GRIP;
    pub const SYSTEM_GROUPING: AccRole = ffi::wxd_AccRole_WXD_ROLE_SYSTEM_GROUPING;
    pub const SYSTEM_HELPBALLOON: AccRole = ffi::wxd_AccRole_WXD_ROLE_SYSTEM_HELPBALLOON;
    pub const SYSTEM_HOTKEYFIELD: AccRole = ffi::wxd_AccRole_WXD_ROLE_SYSTEM_HOTKEYFIELD;
    pub const SYSTEM_INDICATOR: AccRole = ffi::wxd_AccRole_WXD_ROLE_SYSTEM_INDICATOR;
    pub const SYSTEM_LINK: AccRole = ffi::wxd_AccRole_WXD_ROLE_SYSTEM_LINK;
    pub const SYSTEM_LIST: AccRole = ffi::wxd_AccRole_WXD_ROLE_SYSTEM_LIST;
    pub const SYSTEM_LISTITEM: AccRole = ffi::wxd_AccRole_WXD_ROLE_SYSTEM_LISTITEM;
    pub const SYSTEM_MENUBAR: AccRole = ffi::wxd_AccRole_WXD_ROLE_SYSTEM_MENUBAR;
    pub const SYSTEM_MENUITEM: AccRole = ffi::wxd_AccRole_WXD_ROLE_SYSTEM_MENUITEM;
    pub const SYSTEM_MENUPOPUP: AccRole = ffi::wxd_AccRole_WXD_ROLE_SYSTEM_MENUPOPUP;
    pub const SYSTEM_OUTLINE: AccRole = ffi::wxd_AccRole_WXD_ROLE_SYSTEM_OUTLINE;
    pub const SYSTEM_OUTLINEITEM: AccRole = ffi::wxd_AccRole_WXD_ROLE_SYSTEM_OUTLINEITEM;
    pub const SYSTEM_PAGETAB: AccRole = ffi::wxd_AccRole_WXD_ROLE_SYSTEM_PAGETAB;
    pub const SYSTEM_PAGETABLIST: AccRole = ffi::wxd_AccRole_WXD_ROLE_SYSTEM_PAGETABLIST;
    pub const SYSTEM_PANE: AccRole = ffi::wxd_AccRole_WXD_ROLE_SYSTEM_PANE;
    pub const SYSTEM_PROGRESSBAR: AccRole = ffi::wxd_AccRole_WXD_ROLE_SYSTEM_PROGRESSBAR;
    pub const SYSTEM_PROPERTYPAGE: AccRole = ffi::wxd_AccRole_WXD_ROLE_SYSTEM_PROPERTYPAGE;
    pub const SYSTEM_PUSHBUTTON: AccRole = ffi::wxd_AccRole_WXD_ROLE_SYSTEM_PUSHBUTTON;
    pub const SYSTEM_RADIOBUTTON: AccRole = ffi::wxd_AccRole_WXD_ROLE_SYSTEM_RADIOBUTTON;
    pub const SYSTEM_ROW: AccRole = ffi::wxd_AccRole_WXD_ROLE_SYSTEM_ROW;
    pub const SYSTEM_ROWHEADER: AccRole = ffi::wxd_AccRole_WXD_ROLE_SYSTEM_ROWHEADER;
    pub const SYSTEM_SCROLLBAR: AccRole = ffi::wxd_AccRole_WXD_ROLE_SYSTEM_SCROLLBAR;
    pub const SYSTEM_SEPARATOR: AccRole = ffi::wxd_AccRole_WXD_ROLE_SYSTEM_SEPARATOR;
    pub const SYSTEM_SLIDER: AccRole = ffi::wxd_AccRole_WXD_ROLE_SYSTEM_SLIDER;
    pub const SYSTEM_SOUND: AccRole = ffi::wxd_AccRole_WXD_ROLE_SYSTEM_SOUND;
    pub const SYSTEM_SPINBUTTON: AccRole = ffi::wxd_AccRole_WXD_ROLE_SYSTEM_SPINBUTTON;
    pub const SYSTEM_STATICTEXT: AccRole = ffi::wxd_AccRole_WXD_ROLE_SYSTEM_STATICTEXT;
    pub const SYSTEM_STATUSBAR: AccRole = ffi::wxd_AccRole_WXD_ROLE_SYSTEM_STATUSBAR;
    pub const SYSTEM_TABLE: AccRole = ffi::wxd_AccRole_WXD_ROLE_SYSTEM_TABLE;
    pub const SYSTEM_TEXT: AccRole = ffi::wxd_AccRole_WXD_ROLE_SYSTEM_TEXT;
    pub const SYSTEM_TITLEBAR: AccRole = ffi::wxd_AccRole_WXD_ROLE_SYSTEM_TITLEBAR;
    pub const SYSTEM_TOOLBAR: AccRole = ffi::wxd_AccRole_WXD_ROLE_SYSTEM_TOOLBAR;
    pub const SYSTEM_TOOLTIP: AccRole = ffi::wxd_AccRole_WXD_ROLE_SYSTEM_TOOLTIP;
    pub const SYSTEM_WHITESPACE: AccRole = ffi::wxd_AccRole_WXD_ROLE_SYSTEM_WHITESPACE;
    pub const SYSTEM_WINDOW: AccRole = ffi::wxd_AccRole_WXD_ROLE_SYSTEM_WINDOW;
}

/// A trait that can be implemented to provide custom accessibility information.
pub trait AccessibleImpl {
    fn get_child_count(&self) -> (AccStatus, i32) {
        (ffi::wxd_AccStatus_WXD_ACC_OK, 0)
    }
    fn get_child(&self, _child_id: i32) -> (AccStatus, Option<Accessible>) {
        (ffi::wxd_AccStatus_WXD_ACC_NOT_IMPLEMENTED, None)
    }
    fn get_parent(&self) -> (AccStatus, Option<Accessible>) {
        (ffi::wxd_AccStatus_WXD_ACC_NOT_IMPLEMENTED, None)
    }
    fn get_role(&self, _child_id: i32) -> (AccStatus, AccRole) {
        (ffi::wxd_AccStatus_WXD_ACC_NOT_IMPLEMENTED, ffi::wxd_AccRole_WXD_ROLE_NONE)
    }
    fn get_state(&self, _child_id: i32) -> (AccStatus, i64) {
        (ffi::wxd_AccStatus_WXD_ACC_NOT_IMPLEMENTED, 0)
    }
    fn get_name(&self, _child_id: i32) -> (AccStatus, Option<String>) {
        (ffi::wxd_AccStatus_WXD_ACC_NOT_IMPLEMENTED, None)
    }
    fn get_description(&self, _child_id: i32) -> (AccStatus, Option<String>) {
        (ffi::wxd_AccStatus_WXD_ACC_NOT_IMPLEMENTED, None)
    }
    fn get_help_text(&self, _child_id: i32) -> (AccStatus, Option<String>) {
        (ffi::wxd_AccStatus_WXD_ACC_NOT_IMPLEMENTED, None)
    }
    fn get_keyboard_shortcut(&self, _child_id: i32) -> (AccStatus, Option<String>) {
        (ffi::wxd_AccStatus_WXD_ACC_NOT_IMPLEMENTED, None)
    }
    fn get_default_action(&self, _child_id: i32) -> (AccStatus, Option<String>) {
        (ffi::wxd_AccStatus_WXD_ACC_NOT_IMPLEMENTED, None)
    }
    fn get_value(&self, _child_id: i32) -> (AccStatus, Option<String>) {
        (ffi::wxd_AccStatus_WXD_ACC_NOT_IMPLEMENTED, None)
    }
    fn select(&self, _child_id: i32, _select_flags: i32) -> AccStatus {
        ffi::wxd_AccStatus_WXD_ACC_NOT_IMPLEMENTED
    }
    fn get_selections(&self) -> (AccStatus, crate::widgets::dataview::Variant) {
        (
            ffi::wxd_AccStatus_WXD_ACC_NOT_IMPLEMENTED,
            crate::widgets::dataview::Variant::new(),
        )
    }
    fn get_focus(&self) -> (AccStatus, i32, Option<Accessible>) {
        (ffi::wxd_AccStatus_WXD_ACC_NOT_IMPLEMENTED, 0, None)
    }
    fn do_default_action(&self, _child_id: i32) -> AccStatus {
        ffi::wxd_AccStatus_WXD_ACC_NOT_IMPLEMENTED
    }
    fn get_location(&self, _child_id: i32) -> (AccStatus, crate::geometry::Rect) {
        (ffi::wxd_AccStatus_WXD_ACC_NOT_IMPLEMENTED, crate::geometry::Rect::default())
    }
    fn hit_test(&self, _pt: crate::geometry::Point) -> (AccStatus, i32, Option<Accessible>) {
        (ffi::wxd_AccStatus_WXD_ACC_NOT_IMPLEMENTED, 0, None)
    }
    fn navigate(&self, _nav_dir: NavDir, _from_id: i32) -> (AccStatus, i32, Option<Accessible>) {
        (ffi::wxd_AccStatus_WXD_ACC_NOT_IMPLEMENTED, 0, None)
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
    pub fn notify_event(event_type: u32, window: &dyn crate::window::WxWidget, object_type: i32, object_id: i32) {
        unsafe {
            ffi::wxd_Accessible_NotifyEvent(event_type, window.handle_ptr(), object_type, object_id);
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
    status
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
    status
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
    status
}

unsafe extern "C" fn accessible_get_role<T: AccessibleImpl>(
    user_data: *mut c_void,
    child_id: c_int,
    role: *mut ffi::wxd_AccRole,
) -> ffi::wxd_AccStatus {
    let impl_ptr = user_data as *const T;
    let (status, r) = unsafe { (*impl_ptr).get_role(child_id) };
    unsafe { *role = r };
    status
}

unsafe extern "C" fn accessible_get_state<T: AccessibleImpl>(
    user_data: *mut c_void,
    child_id: c_int,
    state: *mut c_long,
) -> ffi::wxd_AccStatus {
    let impl_ptr = user_data as *const T;
    let (status, s) = unsafe { (*impl_ptr).get_state(child_id) };
    unsafe { *state = s as c_long };
    status
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
    status
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
    status
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
    status
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
    status
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
    status
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
    status
}

unsafe extern "C" fn accessible_select<T: AccessibleImpl>(
    user_data: *mut c_void,
    child_id: c_int,
    select_flags: c_int,
) -> ffi::wxd_AccStatus {
    let impl_ptr = user_data as *const T;
    unsafe { (*impl_ptr).select(child_id, select_flags) }
}

unsafe extern "C" fn accessible_get_selections<T: AccessibleImpl>(
    user_data: *mut c_void,
    selections: *mut ffi::wxd_Variant_t,
) -> ffi::wxd_AccStatus {
    let impl_ptr = user_data as *const T;
    let (status, sel) = unsafe { (*impl_ptr).get_selections() };
    if status == ffi::wxd_AccStatus_WXD_ACC_OK && !selections.is_null() {
        unsafe { ffi::wxd_Variant_Assign(selections, sel.as_const_ptr()) };
    }
    status
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
    status
}

unsafe extern "C" fn accessible_do_default_action<T: AccessibleImpl>(
    user_data: *mut c_void,
    child_id: c_int,
) -> ffi::wxd_AccStatus {
    let impl_ptr = user_data as *const T;
    unsafe { (*impl_ptr).do_default_action(child_id) }
}

unsafe extern "C" fn accessible_get_location<T: AccessibleImpl>(
    user_data: *mut c_void,
    child_id: c_int,
    rect: *mut ffi::wxd_Rect,
) -> ffi::wxd_AccStatus {
    let impl_ptr = user_data as *const T;
    let (status, r) = unsafe { (*impl_ptr).get_location(child_id) };
    unsafe { *rect = r.into() };
    status
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
    status
}

unsafe extern "C" fn accessible_navigate<T: AccessibleImpl>(
    user_data: *mut c_void,
    nav_dir: ffi::wxd_NavDir,
    from_id: c_int,
    to_id: *mut c_int,
    to_object: *mut *mut ffi::wxd_Accessible_t,
) -> ffi::wxd_AccStatus {
    let impl_ptr = user_data as *const T;
    let (status, id, acc) = unsafe { (*impl_ptr).navigate(nav_dir, from_id) };
    unsafe { *to_id = id };
    if let Some(a) = acc {
        unsafe { *to_object = a.as_ptr() };
        std::mem::forget(a);
    } else {
        unsafe { *to_object = std::ptr::null_mut() };
    }
    status
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
