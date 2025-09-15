//! Target for drop operations.

use crate::dnd::DragResult;
use crate::prelude::WxWidget;
use std::boxed::Box;
use std::ffi::{c_void, CStr};
use std::os::raw::c_char;
use wxdragon_sys as ffi;

// Type aliases to reduce complexity warnings
type DragCallback = Box<dyn FnMut(i32, i32, DragResult) -> DragResult + 'static>;
type LeaveCallback = Box<dyn FnMut() + 'static>;
type DropCallback = Box<dyn FnMut(i32, i32) -> bool + 'static>;
type DropTextCallback = Box<dyn FnMut(&str, i32, i32) -> bool + 'static>;
type DropFilesCallback = Box<dyn FnMut(Vec<String>, i32, i32) -> bool + 'static>;

/// Callback handlers for a text drop target.
struct TextDropTargetCallbacks {
    on_enter: Option<DragCallback>,
    on_drag_over: Option<DragCallback>,
    on_leave: Option<LeaveCallback>,
    on_drop: Option<DropCallback>,
    on_data: Option<DragCallback>,
    on_drop_text: DropTextCallback,
}

/// A drop target handles text data dropped via drag and drop.
pub struct TextDropTarget {}

/// Builder for TextDropTarget with full callback support
pub struct TextDropTargetBuilder<'a, W: WxWidget> {
    window: &'a W,
    on_enter: Option<DragCallback>,
    on_drag_over: Option<DragCallback>,
    on_leave: Option<LeaveCallback>,
    on_drop: Option<DropCallback>,
    on_data: Option<DragCallback>,
    on_drop_text: Option<DropTextCallback>,
}

impl<'a, W: WxWidget> TextDropTargetBuilder<'a, W> {
    /// Create a new builder for TextDropTarget.
    fn new(window: &'a W) -> Self {
        Self {
            window,
            on_enter: None,
            on_drag_over: None,
            on_leave: None,
            on_drop: None,
            on_data: None,
            on_drop_text: None,
        }
    }

    /// Set the callback for when the cursor enters the drop target.
    pub fn with_on_enter<F>(mut self, callback: F) -> Self
    where
        F: FnMut(i32, i32, DragResult) -> DragResult + 'static,
    {
        self.on_enter = Some(Box::new(callback));
        self
    }

    /// Set the callback for when the cursor is dragged over the drop target.
    pub fn with_on_drag_over<F>(mut self, callback: F) -> Self
    where
        F: FnMut(i32, i32, DragResult) -> DragResult + 'static,
    {
        self.on_drag_over = Some(Box::new(callback));
        self
    }

    /// Set the callback for when the cursor leaves the drop target.
    pub fn with_on_leave<F>(mut self, callback: F) -> Self
    where
        F: FnMut() + 'static,
    {
        self.on_leave = Some(Box::new(callback));
        self
    }

    /// Set the callback for when the user drops data on the drop target.
    pub fn with_on_drop<F>(mut self, callback: F) -> Self
    where
        F: FnMut(i32, i32) -> bool + 'static,
    {
        self.on_drop = Some(Box::new(callback));
        self
    }

    /// Set the callback for when data is available after a drop.
    pub fn with_on_data<F>(mut self, callback: F) -> Self
    where
        F: FnMut(i32, i32, DragResult) -> DragResult + 'static,
    {
        self.on_data = Some(Box::new(callback));
        self
    }

    /// Set the callback for when text is dropped on the drop target.
    /// This callback is required.
    pub fn with_on_drop_text<F>(mut self, callback: F) -> Self
    where
        F: FnMut(&str, i32, i32) -> bool + 'static,
    {
        self.on_drop_text = Some(Box::new(callback));
        self
    }

    /// Create the TextDropTarget with the configured callbacks.
    pub fn build(self) -> TextDropTarget {
        // Ensure we have a text drop callback
        let on_drop_text = self
            .on_drop_text
            .expect("on_drop_text callback is required");

        // Create a struct to hold all our callbacks
        let callbacks = TextDropTargetCallbacks {
            on_enter: self.on_enter,
            on_drag_over: self.on_drag_over,
            on_leave: self.on_leave,
            on_drop: self.on_drop,
            on_data: self.on_data,
            on_drop_text,
        };

        // Create boxed data with callbacks
        let data = Box::new(callbacks);
        let data_ptr = Box::into_raw(data);
        let user_data = data_ptr as *mut c_void;

        // Create the drop target with our callback trampolines
        unsafe {
            ffi::wxd_TextDropTarget_CreateFull(
                self.window.handle_ptr(),
                Some(text_on_enter_trampoline),
                Some(text_on_drag_over_trampoline),
                Some(text_on_leave_trampoline),
                Some(text_on_drop_trampoline),
                Some(text_on_data_trampoline),
                Some(text_on_drop_text_trampoline),
                user_data,
            )
        };

        // The C++ side now owns the drop target and callback data, so we don't need to keep track of them
        TextDropTarget {}
    }
}

impl TextDropTarget {
    /// Creates a builder for a text drop target.
    pub fn builder<W: WxWidget>(window: &W) -> TextDropTargetBuilder<'_, W> {
        TextDropTargetBuilder::new(window)
    }
}

/// Callback handlers for a file drop target.
struct FileDropTargetCallbacks {
    on_enter: Option<DragCallback>,
    on_drag_over: Option<DragCallback>,
    on_leave: Option<LeaveCallback>,
    on_drop: Option<DropCallback>,
    on_data: Option<DragCallback>,
    on_drop_files: DropFilesCallback,
}

/// A drop target handles file data dropped via drag and drop.
pub struct FileDropTarget {}

/// Builder for FileDropTarget to allow setting optional callbacks.
pub struct FileDropTargetBuilder<'a, W: WxWidget> {
    window: &'a W,
    on_enter: Option<DragCallback>,
    on_drag_over: Option<DragCallback>,
    on_leave: Option<LeaveCallback>,
    on_drop: Option<DropCallback>,
    on_data: Option<DragCallback>,
    on_drop_files: Option<DropFilesCallback>,
}

impl<'a, W: WxWidget> FileDropTargetBuilder<'a, W> {
    /// Create a new builder for FileDropTarget.
    fn new(window: &'a W) -> Self {
        Self {
            window,
            on_enter: None,
            on_drag_over: None,
            on_leave: None,
            on_drop: None,
            on_data: None,
            on_drop_files: None,
        }
    }

    /// Set the callback for when the cursor enters the drop target.
    pub fn with_on_enter<F>(mut self, callback: F) -> Self
    where
        F: FnMut(i32, i32, DragResult) -> DragResult + 'static,
    {
        self.on_enter = Some(Box::new(callback));
        self
    }

    /// Set the callback for when the cursor is dragged over the drop target.
    pub fn with_on_drag_over<F>(mut self, callback: F) -> Self
    where
        F: FnMut(i32, i32, DragResult) -> DragResult + 'static,
    {
        self.on_drag_over = Some(Box::new(callback));
        self
    }

    /// Set the callback for when the cursor leaves the drop target.
    pub fn with_on_leave<F>(mut self, callback: F) -> Self
    where
        F: FnMut() + 'static,
    {
        self.on_leave = Some(Box::new(callback));
        self
    }

    /// Set the callback for when the user drops data on the drop target.
    pub fn with_on_drop<F>(mut self, callback: F) -> Self
    where
        F: FnMut(i32, i32) -> bool + 'static,
    {
        self.on_drop = Some(Box::new(callback));
        self
    }

    /// Set the callback for when data is available after a drop.
    pub fn with_on_data<F>(mut self, callback: F) -> Self
    where
        F: FnMut(i32, i32, DragResult) -> DragResult + 'static,
    {
        self.on_data = Some(Box::new(callback));
        self
    }

    /// Set the callback for when files are dropped on the drop target.
    /// This callback is required.
    pub fn with_on_drop_files<F>(mut self, callback: F) -> Self
    where
        F: FnMut(Vec<String>, i32, i32) -> bool + 'static,
    {
        self.on_drop_files = Some(Box::new(callback));
        self
    }

    /// Create the FileDropTarget with the configured callbacks.
    pub fn build(self) -> FileDropTarget {
        // Ensure we have a files drop callback
        let on_drop_files = self
            .on_drop_files
            .expect("on_drop_files callback is required");

        // Create a struct to hold all our callbacks
        let callbacks = FileDropTargetCallbacks {
            on_enter: self.on_enter,
            on_drag_over: self.on_drag_over,
            on_leave: self.on_leave,
            on_drop: self.on_drop,
            on_data: self.on_data,
            on_drop_files,
        };

        // Create boxed data with callbacks
        let data = Box::new(callbacks);
        let data_ptr = Box::into_raw(data);
        let user_data = data_ptr as *mut c_void;

        // Create the drop target with our callback trampolines
        unsafe {
            ffi::wxd_FileDropTarget_CreateFull(
                self.window.handle_ptr(),
                Some(file_on_enter_trampoline),
                Some(file_on_drag_over_trampoline),
                Some(file_on_leave_trampoline),
                Some(file_on_drop_trampoline),
                Some(file_on_data_trampoline),
                Some(file_on_drop_files_trampoline),
                user_data,
            )
        };

        // The C++ side now owns the drop target and callback data, so we don't need to keep track of them
        FileDropTarget {}
    }
}

impl FileDropTarget {
    /// Creates a builder for a file drop target.
    pub fn builder<W: WxWidget>(window: &W) -> FileDropTargetBuilder<'_, W> {
        FileDropTargetBuilder::new(window)
    }
}

// --- Callback trampolines for TextDropTarget ---

extern "C" fn text_on_enter_trampoline(
    x: i32,
    y: i32,
    def_result: ffi::wxd_DragResult,
    data_ptr: *mut c_void,
) -> ffi::wxd_DragResult {
    if data_ptr.is_null() {
        return def_result;
    }

    let callbacks = unsafe { &mut *(data_ptr as *mut TextDropTargetCallbacks) };

    if let Some(ref mut callback) = callbacks.on_enter {
        callback(x, y, DragResult::from(def_result)).into()
    } else {
        def_result
    }
}

extern "C" fn text_on_drag_over_trampoline(
    x: i32,
    y: i32,
    def_result: ffi::wxd_DragResult,
    data_ptr: *mut c_void,
) -> ffi::wxd_DragResult {
    if data_ptr.is_null() {
        return def_result;
    }

    let callbacks = unsafe { &mut *(data_ptr as *mut TextDropTargetCallbacks) };

    if let Some(ref mut callback) = callbacks.on_drag_over {
        callback(x, y, DragResult::from(def_result)).into()
    } else {
        def_result
    }
}

extern "C" fn text_on_leave_trampoline(data_ptr: *mut c_void) {
    if data_ptr.is_null() {
        return;
    }

    let callbacks = unsafe { &mut *(data_ptr as *mut TextDropTargetCallbacks) };

    if let Some(ref mut callback) = callbacks.on_leave {
        callback();
    }
}

extern "C" fn text_on_drop_trampoline(x: i32, y: i32, data_ptr: *mut c_void) -> bool {
    if data_ptr.is_null() {
        return false;
    }

    let callbacks = unsafe { &mut *(data_ptr as *mut TextDropTargetCallbacks) };

    if let Some(ref mut callback) = callbacks.on_drop {
        callback(x, y)
    } else {
        true // Default to accepting the drop
    }
}

extern "C" fn text_on_data_trampoline(
    x: i32,
    y: i32,
    def_result: ffi::wxd_DragResult,
    data_ptr: *mut c_void,
) -> ffi::wxd_DragResult {
    if data_ptr.is_null() {
        return def_result;
    }

    let callbacks = unsafe { &mut *(data_ptr as *mut TextDropTargetCallbacks) };

    if let Some(ref mut callback) = callbacks.on_data {
        callback(x, y, DragResult::from(def_result)).into()
    } else {
        def_result
    }
}

extern "C" fn text_on_drop_text_trampoline(
    text: *const c_char,
    x: i32,
    y: i32,
    data_ptr: *mut c_void,
) -> bool {
    if text.is_null() || data_ptr.is_null() {
        return false;
    }

    let text_str = unsafe { CStr::from_ptr(text).to_string_lossy().into_owned() };
    let callbacks = unsafe { &mut *(data_ptr as *mut TextDropTargetCallbacks) };

    (callbacks.on_drop_text)(&text_str, x, y)
}

// --- Callback trampolines for FileDropTarget ---

extern "C" fn file_on_enter_trampoline(
    x: i32,
    y: i32,
    def_result: ffi::wxd_DragResult,
    data_ptr: *mut c_void,
) -> ffi::wxd_DragResult {
    if data_ptr.is_null() {
        return def_result;
    }

    let callbacks = unsafe { &mut *(data_ptr as *mut FileDropTargetCallbacks) };

    if let Some(ref mut callback) = callbacks.on_enter {
        callback(x, y, DragResult::from(def_result)).into()
    } else {
        def_result
    }
}

extern "C" fn file_on_drag_over_trampoline(
    x: i32,
    y: i32,
    def_result: ffi::wxd_DragResult,
    data_ptr: *mut c_void,
) -> ffi::wxd_DragResult {
    if data_ptr.is_null() {
        return def_result;
    }

    let callbacks = unsafe { &mut *(data_ptr as *mut FileDropTargetCallbacks) };

    if let Some(ref mut callback) = callbacks.on_drag_over {
        callback(x, y, DragResult::from(def_result)).into()
    } else {
        def_result
    }
}

extern "C" fn file_on_leave_trampoline(data_ptr: *mut c_void) {
    if data_ptr.is_null() {
        return;
    }

    let callbacks = unsafe { &mut *(data_ptr as *mut FileDropTargetCallbacks) };

    if let Some(ref mut callback) = callbacks.on_leave {
        callback();
    }
}

extern "C" fn file_on_drop_trampoline(x: i32, y: i32, data_ptr: *mut c_void) -> bool {
    if data_ptr.is_null() {
        return false;
    }

    let callbacks = unsafe { &mut *(data_ptr as *mut FileDropTargetCallbacks) };

    if let Some(ref mut callback) = callbacks.on_drop {
        callback(x, y)
    } else {
        true // Default to accepting the drop
    }
}

extern "C" fn file_on_data_trampoline(
    x: i32,
    y: i32,
    def_result: ffi::wxd_DragResult,
    data_ptr: *mut c_void,
) -> ffi::wxd_DragResult {
    if data_ptr.is_null() {
        return def_result;
    }

    let callbacks = unsafe { &mut *(data_ptr as *mut FileDropTargetCallbacks) };

    if let Some(ref mut callback) = callbacks.on_data {
        callback(x, y, DragResult::from(def_result)).into()
    } else {
        def_result
    }
}

extern "C" fn file_on_drop_files_trampoline(
    filenames_ptr: *const ffi::wxd_ArrayString_t,
    x: i32,
    y: i32,
    data_ptr: *mut c_void,
) -> bool {
    if filenames_ptr.is_null() || data_ptr.is_null() {
        return false;
    }

    // Extract filenames from wxArrayString
    let mut filenames = Vec::<String>::new();
    unsafe {
        let count = ffi::wxd_ArrayString_GetCount(filenames_ptr as *mut _);
        filenames.reserve(count as usize);

        for i in 0..count {
            let mut buffer = vec![0u8; 2048]; // Buffer for path
            let len = ffi::wxd_ArrayString_GetString(
                filenames_ptr as *mut _,
                i,
                buffer.as_mut_ptr() as *mut i8,
                buffer.len() as i32,
            );

            if len > 0 {
                buffer.truncate(len as usize);
                // Convert to UTF-8 String
                if let Ok(s) = String::from_utf8(buffer) {
                    filenames.push(s);
                }
            }
        }
    }

    let callbacks = unsafe { &mut *(data_ptr as *mut FileDropTargetCallbacks) };

    (callbacks.on_drop_files)(filenames, x, y)
}
