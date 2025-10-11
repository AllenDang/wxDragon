//! DataViewModel implementation.

use crate::widgets::dataview::variant::Variant;
use std::any::Any;
use std::ffi::{CStr, CString};
use std::os::raw::c_void;
use wxdragon_sys as ffi;

/// Small helper utilities for DataViewTree models to manage raw pointers and
/// transfer ownership across the FFI boundary in a safer, centralized way.
pub mod tree_helpers {
    use std::ffi::c_void;

    /// Box a Rust value and return it as an opaque userdata pointer for C.
    pub fn box_userdata<T>(v: T) -> *mut c_void {
        Box::into_raw(Box::new(v)) as *mut c_void
    }

    /// # Safety
    /// Convert an opaque userdata pointer back into a raw typed pointer.
    /// Safety: caller must ensure the pointer actually points to T.
    pub unsafe fn userdata_as_mut<T>(ptr: *mut c_void) -> *mut T {
        ptr as *mut T
    }

    /// Convert a typed node pointer into an opaque id for passing to C++.
    pub fn ptr_to_id<T>(p: *mut T) -> *mut c_void {
        p as *mut c_void
    }

    /// # Safety
    /// Convert an opaque id back into a typed node pointer.
    /// Safety: caller must ensure the id was originally created from a pointer to T.
    pub unsafe fn id_to_ptr<T>(id: *mut c_void) -> *mut T {
        id as *mut T
    }

    /// Consume a Vec of node pointers and return a pointer/count pair that C++
    /// can iterate. The function intentionally leaks the Vec; call
    /// `free_children_array` from C/Rust later to reclaim the memory.
    pub fn leak_children_vec<T>(children: Vec<*mut T>) -> (*mut *mut c_void, i32) {
        let mut arr: Vec<*mut c_void> = children.into_iter().map(|p| p as *mut c_void).collect();
        let ptr = arr.as_mut_ptr();
        let count = arr.len() as i32;
        std::mem::forget(arr);
        (ptr, count)
    }

    /// # Safety
    /// Reclaim a leaked children array previously produced by `leak_children_vec`.
    /// Safety: ptr and count must match a previous call that leaked a Vec with the
    /// same length and pointer value.
    pub unsafe fn free_children_array(ptr: *mut *mut c_void, count: i32) {
        if ptr.is_null() || count == 0 {
            return;
        }
        // Recreate the Vec from raw parts and drop it so memory is freed.
        let slice = std::slice::from_raw_parts_mut(ptr, count as usize);
        let _ = Vec::from_raw_parts(slice.as_mut_ptr(), count as usize, count as usize);
    }
}

// Type aliases to reduce complexity
type GetValueCallback = Box<dyn for<'a> Fn(&'a dyn Any, usize, usize) -> Variant>;
type SetValueCallback = Box<dyn for<'a, 'b> Fn(&'a dyn Any, usize, usize, &'b Variant) -> bool>;
type GetAttrCallback = Box<dyn for<'a> Fn(&'a dyn Any, usize, usize) -> Option<DataViewItemAttr>>;
type IsEnabledCallback = Box<dyn for<'a> Fn(&'a dyn Any, usize, usize) -> bool>;

/// DataViewItemAttr represents formatting attributes for a DataViewCtrl cell.
#[derive(Debug, Clone, Default)]
pub struct DataViewItemAttr {
    has_text_colour: bool,
    text_colour_red: u8,
    text_colour_green: u8,
    text_colour_blue: u8,
    text_colour_alpha: u8,

    has_bg_colour: bool,
    bg_colour_red: u8,
    bg_colour_green: u8,
    bg_colour_blue: u8,
    bg_colour_alpha: u8,

    bold: bool,
    italic: bool,
}

impl DataViewItemAttr {
    /// Create a new default attribute set
    pub fn new() -> Self {
        Default::default()
    }

    /// Set the text color
    pub fn with_text_colour(mut self, r: u8, g: u8, b: u8, a: u8) -> Self {
        self.has_text_colour = true;
        self.text_colour_red = r;
        self.text_colour_green = g;
        self.text_colour_blue = b;
        self.text_colour_alpha = a;
        self
    }

    /// Set the background color
    pub fn with_bg_colour(mut self, r: u8, g: u8, b: u8, a: u8) -> Self {
        self.has_bg_colour = true;
        self.bg_colour_red = r;
        self.bg_colour_green = g;
        self.bg_colour_blue = b;
        self.bg_colour_alpha = a;
        self
    }

    /// Set text to bold
    pub fn with_bold(mut self, bold: bool) -> Self {
        self.bold = bold;
        self
    }

    /// Set text to italic
    pub fn with_italic(mut self, italic: bool) -> Self {
        self.italic = italic;
        self
    }

    /// Convert to raw FFI struct
    pub fn to_raw(&self) -> ffi::wxd_DataViewItemAttr_t {
        ffi::wxd_DataViewItemAttr_t {
            has_text_colour: self.has_text_colour,
            text_colour_red: self.text_colour_red,
            text_colour_green: self.text_colour_green,
            text_colour_blue: self.text_colour_blue,
            text_colour_alpha: self.text_colour_alpha,

            has_bg_colour: self.has_bg_colour,
            bg_colour_red: self.bg_colour_red,
            bg_colour_green: self.bg_colour_green,
            bg_colour_blue: self.bg_colour_blue,
            bg_colour_alpha: self.bg_colour_alpha,

            bold: self.bold,
            italic: self.italic,
        }
    }
}

/// A type representing a data model for use with DataViewCtrl
pub trait DataViewModel {
    /// Get the handle to the underlying wxDataViewModel
    fn handle_ptr(&self) -> *mut ffi::wxd_DataViewModel_t;

    /// Get the number of columns in the model
    fn get_column_count(&self) -> usize;

    /// Get the number of rows in the model
    fn get_row_count(&self) -> usize;

    /// Get the value at the specified row and column
    fn get_value(&self, row: usize, col: usize) -> Variant;

    /// Set the value at the specified row and column
    fn set_value(&self, row: usize, col: usize, value: &Variant) -> bool {
        // Default implementation is read-only
        let _ = (row, col, value); // Silence unused variable warnings
        false
    }

    /// Get display attributes for a cell
    fn get_attributes(&self, row: usize, col: usize) -> Option<DataViewItemAttr> {
        let _ = (row, col); // Silence unused variable warnings
        None
    }

    /// Check if a cell is enabled for editing
    fn is_enabled(&self, row: usize, col: usize) -> bool {
        let _ = (row, col); // Silence unused variable warnings
        true
    }
}

/// Raw pointer to a DataViewModel
pub(crate) struct DataViewModelPtr {
    pub(crate) ptr: *mut ffi::wxd_DataViewModel_t,
    owned: bool,
}

impl DataViewModelPtr {
    /// Create a new DataViewModelPtr from a raw pointer
    pub(crate) fn new(ptr: *mut ffi::wxd_DataViewModel_t) -> Self {
        Self { ptr, owned: true }
    }
}

impl Drop for DataViewModelPtr {
    fn drop(&mut self) {
        if self.owned && !self.ptr.is_null() {
            // In wxWidgets, models are typically reference-counted and might be
            // owned by the control. When a DataViewCtrl::associate_model() is called,
            // the control increases the reference count of the model and takes partial
            // ownership.
            //
            // If we were to destroy the model here, it could lead to use-after-free
            // bugs if the control still holds a reference. The C++ side of wxWidgets
            // will handle cleanup when the control itself is destroyed.
            //
            // For complete memory safety, models should be destroyed only after
            // all controls using them are destroyed, or by explicitly calling
            // a detach/disassociate method on the control first.
        }
    }
}

/// A basic list model for DataViewCtrl that stores data in a 2D array
pub struct DataViewListModel {
    ptr: DataViewModelPtr,
}

impl DataViewListModel {
    /// Create a new empty DataViewListModel
    pub fn new() -> Self {
        let ptr = unsafe { ffi::wxd_DataViewListModel_Create() };
        Self {
            ptr: DataViewModelPtr::new(ptr),
        }
    }

    /// Add a new column to the model
    pub fn append_column(&self, name: &str) -> bool {
        let c_name = CString::new(name).unwrap_or_default();
        unsafe { ffi::wxd_DataViewListModel_AppendColumn(self.ptr.ptr, c_name.as_ptr()) }
    }

    /// Add a new row to the model
    pub fn append_row(&self) -> bool {
        unsafe { ffi::wxd_DataViewListModel_AppendRow(self.ptr.ptr) }
    }

    /// Set a value in the model
    pub fn set_value<T: Into<Variant>>(&self, row: usize, col: usize, value: T) -> bool {
        let variant = value.into();
        let variant_ptr = variant.as_raw_mut();
        let result = unsafe {
            ffi::wxd_DataViewListModel_SetValue(self.ptr.ptr, row as u64, col as u64, variant_ptr)
        };
        // The C API consumes the variant, so we don't need to free it
        let _ = variant_ptr;
        result
    }
}

impl DataViewModel for DataViewListModel {
    fn handle_ptr(&self) -> *mut ffi::wxd_DataViewModel_t {
        self.ptr.ptr
    }

    fn get_column_count(&self) -> usize {
        // Call the C API to get the column count if available
        // For now, just return a reasonable default
        // In a more complete implementation, we would track this internally
        // or provide a C API function to get it
        10 // Maximum reasonable number of columns
    }

    fn get_row_count(&self) -> usize {
        // Call the C API to get the row count if available
        // For now, just return a reasonable default
        // In a more complete implementation, we would track this internally
        // or provide a C API function to get it
        0 // Default to empty
    }

    fn get_value(&self, _row: usize, _col: usize) -> Variant {
        // Default implementation returns empty string
        // In real application code, users should set values explicitly with set_value
        Variant::String(String::new())
    }
}

impl Default for DataViewListModel {
    fn default() -> Self {
        Self::new()
    }
}

/// A virtual list model for DataViewCtrl
///
/// This model implementation doesn't store data; it just provides placeholders
/// that should be overridden with your own data retrieval methods.
pub struct DataViewVirtualListModel {
    ptr: DataViewModelPtr,
    size: usize,
}

impl DataViewVirtualListModel {
    /// Create a new virtual list model with the specified initial size
    pub fn new(initial_size: usize) -> Self {
        let ptr = unsafe { ffi::wxd_DataViewVirtualListModel_Create(initial_size as u64) };
        Self {
            ptr: DataViewModelPtr::new(ptr),
            size: initial_size,
        }
    }

    /// Notify that a row has been prepended
    pub fn row_prepended(&mut self) {
        unsafe {
            ffi::wxd_DataViewVirtualListModel_RowPrepended(self.ptr.ptr);
        }
        self.size += 1;
    }

    /// Notify that a row has been inserted
    pub fn row_inserted(&mut self, before: usize) {
        unsafe {
            ffi::wxd_DataViewVirtualListModel_RowInserted(self.ptr.ptr, before as u64);
        }
        self.size += 1;
    }

    /// Notify that a row has been appended
    pub fn row_appended(&mut self) {
        unsafe {
            ffi::wxd_DataViewVirtualListModel_RowAppended(self.ptr.ptr);
        }
        self.size += 1;
    }

    /// Notify that a row has been deleted
    pub fn row_deleted(&mut self, row: usize) {
        unsafe {
            ffi::wxd_DataViewVirtualListModel_RowDeleted(self.ptr.ptr, row as u64);
        }
        if self.size > 0 {
            self.size -= 1;
        }
    }

    /// Notify that multiple rows have been deleted
    pub fn rows_deleted(&mut self, rows: &[i32]) {
        unsafe {
            // The C++ API expects a mutable array, so we'll need to cast away the const
            let rows_ptr = rows.as_ptr() as *mut i32;
            ffi::wxd_DataViewVirtualListModel_RowsDeleted(
                self.ptr.ptr,
                rows_ptr,
                rows.len() as i32,
            );
        }
        if self.size >= rows.len() {
            self.size -= rows.len();
        } else {
            self.size = 0;
        }
    }

    /// Notify that a row has changed
    pub fn row_changed(&self, row: usize) {
        unsafe {
            ffi::wxd_DataViewVirtualListModel_RowChanged(self.ptr.ptr, row as u64);
        }
    }

    /// Notify that a specific cell value has changed
    pub fn row_value_changed(&self, row: usize, col: usize) {
        unsafe {
            ffi::wxd_DataViewVirtualListModel_RowValueChanged(self.ptr.ptr, row as u64, col as u64);
        }
    }

    /// Reset the model with a new size
    pub fn reset(&mut self, new_size: usize) {
        unsafe {
            ffi::wxd_DataViewVirtualListModel_Reset(self.ptr.ptr, new_size as u64);
        }
        self.size = new_size;
    }

    /// Get the native item for a row
    pub fn get_item(&self, row: usize) -> *mut std::ffi::c_void {
        unsafe { ffi::wxd_DataViewVirtualListModel_GetItem(self.ptr.ptr, row as u64) }
    }

    /// Get the row for a native item
    ///
    /// # Safety
    /// The caller must ensure the item pointer is valid and comes from the same model.
    pub unsafe fn get_row(&self, item: *mut std::ffi::c_void) -> usize {
        ffi::wxd_DataViewVirtualListModel_GetRow(self.ptr.ptr, item) as usize
    }

    /// Get the current size of the model
    pub fn size(&self) -> usize {
        self.size
    }
}

impl DataViewModel for DataViewVirtualListModel {
    fn handle_ptr(&self) -> *mut ffi::wxd_DataViewModel_t {
        self.ptr.ptr
    }

    fn get_column_count(&self) -> usize {
        // Virtual list model columns are handled by the control
        0
    }

    fn get_row_count(&self) -> usize {
        self.size
    }

    fn get_value(&self, _row: usize, _col: usize) -> Variant {
        // By default return empty strings, this should be overridden
        Variant::String(String::new())
    }
}

impl Default for DataViewVirtualListModel {
    fn default() -> Self {
        Self::new(0)
    }
}

/// A callback-based DataView tree model wrapper.
pub struct CustomDataViewTreeModel {
    handle: *mut ffi::wxd_DataViewModel_t,
    // Pointer to the FFI callbacks struct allocated and owned by C++.
    _callback_data_ptr: *const ffi::wxd_DataViewTreeModel_Callbacks,
}

impl Drop for CustomDataViewTreeModel {
    fn drop(&mut self) {
        if !self.handle.is_null() {
            unsafe { ffi::wxd_DataViewTreeModel_Release(self.handle) };
        }
    }
}

// Internal representation of the Rust-side callbacks and userdata. This is the
// concrete type we store in `userdata` for the FFI struct.
#[allow(clippy::type_complexity)]
struct OwnedTreeCallbacks {
    userdata: Box<dyn Any>,
    get_parent: Box<dyn Fn(&dyn Any, *mut std::ffi::c_void) -> *mut std::ffi::c_void>,
    is_container: Box<dyn Fn(&dyn Any, *mut std::ffi::c_void) -> bool>,
    get_children: Box<dyn Fn(&dyn Any, *mut std::ffi::c_void) -> Vec<*mut std::ffi::c_void>>,
    // free_children will be provided by the FFI layer; we still keep a placeholder
    get_value: Box<dyn Fn(&dyn Any, *mut std::ffi::c_void, u32) -> Variant>,
    set_value: Option<Box<dyn Fn(&dyn Any, *mut std::ffi::c_void, u32, &Variant) -> bool>>,
    is_enabled: Option<Box<dyn Fn(&dyn Any, *mut std::ffi::c_void, u32) -> bool>>,
    compare: Option<
        Box<dyn Fn(&dyn Any, *mut std::ffi::c_void, *mut std::ffi::c_void, u32, bool) -> i32>,
    >,
}

impl CustomDataViewTreeModel {
    /// Typed constructor where node/item pointers are the concrete `N` type
    /// instead of opaque `*mut c_void`. This prevents callers from working
    /// with raw `*mut c_void` and makes the API safer and clearer.
    ///
    /// The supplied closures use `*mut N` for item pointers; a null pointer
    /// represents the root item (same convention as the C++ API).
    #[allow(clippy::type_complexity, clippy::too_many_arguments)]
    pub fn new<T, N, GP, IC, GC, GV, SV, IE, CMP>(
        data: T,
        get_parent: GP,
        is_container: IC,
        get_children: GC,
        get_value: GV,
        set_value: Option<SV>,
        is_enabled: Option<IE>,
        compare: Option<CMP>,
    ) -> Self
    where
        T: Any + 'static,
        N: Any + 'static,
        // Use HRTB so callbacks can accept references with any short lifetime
        GP: for<'a> Fn(&T, Option<&'a N>) -> Option<*mut N> + 'static,
        IC: for<'a> Fn(&T, Option<&'a N>) -> bool + 'static,
        // get_children returns a vector of raw typed pointers; returning raw
        // pointers avoids complex lifetime issues for collections.
        GC: for<'a> Fn(&T, Option<&'a N>) -> Vec<*mut N> + 'static,
        GV: for<'a> Fn(&T, Option<&'a N>, u32) -> Variant + 'static,
        SV: for<'a> Fn(&T, Option<&'a N>, u32, &Variant) -> bool + 'static,
        IE: for<'a> Fn(&T, Option<&'a N>, u32) -> bool + 'static,
        // compare expects two concrete items (non-root)
        CMP: for<'a> Fn(&T, &'a N, &'a N, u32, bool) -> i32 + 'static,
    {
        // Wrap typed data in a Box<dyn Any>
        let boxed_data: Box<dyn Any> = Box::new(data);

        // Adapt typed closures into closures working with &dyn Any and *mut c_void
        let any_get_parent: Box<
            dyn for<'a> Fn(&dyn Any, *mut std::ffi::c_void) -> *mut std::ffi::c_void,
        > = Box::new(move |any_data, item| {
            let t = any_data.downcast_ref::<T>().unwrap();
            let item_opt: Option<&N> = if item.is_null() {
                None
            } else {
                Some(unsafe { &*(item as *mut N) })
            };
            let ret_opt = get_parent(t, item_opt);
            match ret_opt {
                Some(ptr) => ptr as *mut std::ffi::c_void,
                None => std::ptr::null_mut(),
            }
        });

        let any_is_container: Box<dyn for<'a> Fn(&dyn Any, *mut std::ffi::c_void) -> bool> =
            Box::new(move |any_data, item| {
                let t = any_data.downcast_ref::<T>().unwrap();
                let item_opt: Option<&N> = if item.is_null() {
                    None
                } else {
                    Some(unsafe { &*(item as *mut N) })
                };
                is_container(t, item_opt)
            });
        // Convert typed get_children into an Any-based callback. Note the
        // conversions between `*mut N` and `*mut c_void` (opaque ids used by
        // wxWidgets). We keep these casts explicit and document the invariant:
        //
        // Invariant: the model's callbacks and the control agree that item ids
        // are actually raw pointers to `N` (or null for the root). These ids
        // are treated as opaque by wxWidgets and are not mutated by C++ code.
        // Therefore we only cast between pointer types for transport across the
        // FFI boundary; we never dereference them here except when the caller
        // explicitly provides an `N` reference via the typed `get_children`.
        //
        // The casts are performed inside `unsafe` blocks and are intentionally
        // minimal and local to this closure so the unsafety surface is easy to
        // review and reason about.
        let any_get_children: Box<
            dyn for<'a> Fn(&dyn Any, *mut std::ffi::c_void) -> Vec<*mut std::ffi::c_void>,
        > = Box::new(move |any_data, item| {
            let t = any_data.downcast_ref::<T>().unwrap();
            let item_opt: Option<&N> = if item.is_null() {
                None
            } else {
                // SAFETY: We only cast the opaque id back to `*mut N` for the
                // purpose of calling the user's typed closure. This is safe
                // because the `new` constructor requires the caller to ensure
                // the ids actually originate from pointers to `N` (or null).
                // Dereferencing happens only to create a temporary `&N` which
                // the user's closure observes; we don't mutate through the
                // pointer here.
                Some(unsafe { &*(item as *mut N) })
            };
            let vec_typed: Vec<*mut N> = get_children(t, item_opt);
            vec_typed
                .into_iter()
                .map(|p| p as *mut std::ffi::c_void)
                .collect()
        });
        let any_get_value: Box<dyn for<'a> Fn(&dyn Any, *mut std::ffi::c_void, u32) -> Variant> =
            Box::new(move |any_data, item, col| {
                let t = any_data.downcast_ref::<T>().unwrap();
                let item_opt: Option<&N> = if item.is_null() {
                    None
                } else {
                    Some(unsafe { &*(item as *mut N) })
                };
                get_value(t, item_opt, col)
            });

        let any_set_value = set_value.map(|f| {
            Box::new(
                move |any_data: &dyn Any, item: *mut std::ffi::c_void, col, var: &Variant| {
                    let t = any_data.downcast_ref::<T>().unwrap();
                    let item_opt: Option<&N> = if item.is_null() {
                        None
                    } else {
                        Some(unsafe { &*(item as *mut N) })
                    };
                    f(t, item_opt, col, var)
                },
            )
                as Box<dyn for<'a> Fn(&dyn Any, *mut std::ffi::c_void, u32, &Variant) -> bool>
        });

        let any_is_enabled = is_enabled.map(|f| {
            Box::new(
                move |any_data: &dyn Any, item: *mut std::ffi::c_void, col| {
                    let t = any_data.downcast_ref::<T>().unwrap();
                    let item_opt: Option<&N> = if item.is_null() {
                        None
                    } else {
                        Some(unsafe { &*(item as *mut N) })
                    };
                    f(t, item_opt, col)
                },
            ) as Box<dyn for<'a> Fn(&dyn Any, *mut std::ffi::c_void, u32) -> bool>
        });

        let any_compare = compare.map(|f| {
            Box::new(
                move |any_data: &dyn Any,
                      a: *mut std::ffi::c_void,
                      b: *mut std::ffi::c_void,
                      col: u32,
                      asc: bool| {
                    let t = any_data.downcast_ref::<T>().unwrap();
                    if a.is_null() || b.is_null() {
                        return 0;
                    }
                    let a_ref: &N = unsafe { &*(a as *mut N) };
                    let b_ref: &N = unsafe { &*(b as *mut N) };
                    f(t, a_ref, b_ref, col, asc)
                },
            )
                as Box<
                    dyn for<'a> Fn(
                        &dyn Any,
                        *mut std::ffi::c_void,
                        *mut std::ffi::c_void,
                        u32,
                        bool,
                    ) -> i32,
                >
        });

        // Build OwnedTreeCallbacks and move it to C as userdata.
        let owned = Box::new(OwnedTreeCallbacks {
            userdata: boxed_data,
            get_parent: any_get_parent,
            is_container: any_is_container,
            get_children: any_get_children,
            get_value: any_get_value,
            set_value: any_set_value,
            is_enabled: any_is_enabled,
            compare: any_compare,
        });

        // Raw pointer to OwnedTreeCallbacks which will be stored in the FFI userdata
        let owned_raw = Box::into_raw(owned);

        // Create the FFI callback struct
        let cb = ffi::wxd_DataViewTreeModel_Callbacks {
            userdata: owned_raw as *mut std::ffi::c_void,
            userdata_free: Some(free_owned_tree_callbacks),
            get_parent: Some(trampoline_get_parent),
            is_container: Some(trampoline_is_container),
            get_children: Some(trampoline_get_children),
            free_children: Some(trampoline_free_children),
            get_value: Some(trampoline_get_value),
            set_value: Some(trampoline_set_value),
            is_enabled: Some(trampoline_is_enabled),
            compare: Some(trampoline_compare),
        };

        // Box the FFI struct and hand ownership to C++ by passing a raw pointer
        let cb_box = Box::new(cb);
        let cb_raw = Box::into_raw(cb_box);

        // Create the native model
        let handle = unsafe { ffi::wxd_DataViewTreeModel_CreateWithCallbacks(cb_raw) };

        Self {
            handle,
            _callback_data_ptr: cb_raw,
        }
    }
}

// Extern "C" trampolines and helpers used by the FFI callbacks
extern "C" fn trampoline_free_children(items: *mut *mut std::ffi::c_void, count: i32) {
    unsafe { tree_helpers::free_children_array(items, count) };
}

extern "C" fn trampoline_get_parent(
    userdata: *mut std::ffi::c_void,
    item: *mut std::ffi::c_void,
) -> *mut std::ffi::c_void {
    if userdata.is_null() {
        return std::ptr::null_mut();
    }
    let cb = unsafe { &*(userdata as *mut OwnedTreeCallbacks) };
    (cb.get_parent)(&*cb.userdata, item)
}

extern "C" fn trampoline_is_container(
    userdata: *mut std::ffi::c_void,
    item: *mut std::ffi::c_void,
) -> bool {
    if userdata.is_null() {
        return false;
    }
    let cb = unsafe { &*(userdata as *mut OwnedTreeCallbacks) };
    (cb.is_container)(&*cb.userdata, item)
}

extern "C" fn trampoline_get_children(
    userdata: *mut std::ffi::c_void,
    item: *mut std::ffi::c_void,
    out_items: *mut *mut *mut std::ffi::c_void,
    out_count: *mut i32,
) {
    if userdata.is_null() {
        unsafe { *out_items = std::ptr::null_mut() };
        unsafe { *out_count = 0 };
        return;
    }
    let cb = unsafe { &*(userdata as *mut OwnedTreeCallbacks) };
    let vec = (cb.get_children)(&*cb.userdata, item);
    let (ptr, cnt) = tree_helpers::leak_children_vec(vec);
    // SAFETY: `ptr` is a pointer to a heap-allocated array of `*mut c_void`
    // produced by `leak_children_vec`. The FFI contract expects a
    // `*mut *mut c_void` output parameter; assign `ptr` directly. We keep the
    // cast explicit to highlight that `ptr` is owned by Rust until the C++
    // side calls the corresponding free function which will call
    // `trampoline_free_children` to reclaim it.
    unsafe { *out_items = ptr };
    unsafe { *out_count = cnt };
}

extern "C" fn trampoline_get_value(
    userdata: *mut std::ffi::c_void,
    item: *mut std::ffi::c_void,
    col: u32,
    out_variant: *mut ffi::wxd_Variant_t,
) {
    if out_variant.is_null() || userdata.is_null() {
        return;
    }
    let cb = unsafe { &*(userdata as *mut OwnedTreeCallbacks) };
    let val = (cb.get_value)(&*cb.userdata, item, col);
    let raw = to_raw_variant(&val);
    unsafe { *out_variant = raw };
}

extern "C" fn trampoline_set_value(
    userdata: *mut std::ffi::c_void,
    item: *mut std::ffi::c_void,
    col: u32,
    variant: *const ffi::wxd_Variant_t,
) -> bool {
    if userdata.is_null() || variant.is_null() {
        return false;
    }
    let cb = unsafe { &*(userdata as *mut OwnedTreeCallbacks) };
    if let Some(f) = &cb.set_value {
        // Convert incoming raw variant to Variant
        let v = unsafe { from_raw_variant(variant) };
        f(&*cb.userdata, item, col, &v)
    } else {
        false
    }
}

extern "C" fn trampoline_is_enabled(
    userdata: *mut std::ffi::c_void,
    item: *mut std::ffi::c_void,
    col: u32,
) -> bool {
    if userdata.is_null() {
        return true;
    }
    let cb = unsafe { &*(userdata as *mut OwnedTreeCallbacks) };
    if let Some(f) = &cb.is_enabled {
        f(&*cb.userdata, item, col)
    } else {
        true
    }
}

extern "C" fn trampoline_compare(
    userdata: *mut std::ffi::c_void,
    a: *mut std::ffi::c_void,
    b: *mut std::ffi::c_void,
    col: u32,
    asc: bool,
) -> i32 {
    if userdata.is_null() {
        return 0;
    }
    let cb = unsafe { &*(userdata as *mut OwnedTreeCallbacks) };
    if let Some(f) = &cb.compare {
        f(&*cb.userdata, a, b, col, asc)
    } else {
        0
    }
}

extern "C" fn free_owned_tree_callbacks(ptr: *mut std::ffi::c_void) {
    if ptr.is_null() {
        return;
    }

    // Recreate the Box<OwnedTreeCallbacks> and drop it to run destructors
    let _ = unsafe { Box::from_raw(ptr as *mut OwnedTreeCallbacks) };
}

impl DataViewModel for CustomDataViewTreeModel {
    fn handle_ptr(&self) -> *mut ffi::wxd_DataViewModel_t {
        self.handle
    }

    fn get_column_count(&self) -> usize {
        0
    }
    fn get_row_count(&self) -> usize {
        0
    }
    fn get_value(&self, _row: usize, _col: usize) -> Variant {
        Variant::String(String::new())
    }
}

#[no_mangle]
#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub extern "C" fn wxd_Drop_Rust_DataViewTreeModelCallbacks(
    cb_ptr: *mut ffi::wxd_DataViewTreeModel_Callbacks,
) {
    if cb_ptr.is_null() {
        return;
    }

    let cb_box = unsafe { Box::from_raw(cb_ptr) };
    if let Some(free_fn) = cb_box.userdata_free {
        unsafe { free_fn(cb_box.userdata) };
    }
    drop(cb_box);
}

/// A customizable virtual list model that uses callbacks to provide data.
pub struct CustomDataViewVirtualListModel {
    handle: *mut ffi::wxd_DataViewModel_t,
    size: usize,
    // Box holding the callback data to ensure it lives as long as the model
    callback_data: Box<CustomModelCallbacks>,
}

struct CustomModelCallbacks {
    // The actual user data that will be passed to callbacks
    userdata: Box<dyn Any>,
    // The callbacks
    get_value: GetValueCallback,
    set_value: Option<SetValueCallback>,
    get_attr: Option<GetAttrCallback>,
    is_enabled: Option<IsEnabledCallback>,
}

impl CustomDataViewVirtualListModel {
    /// Creates a new custom virtual list model with the specified data provider.
    pub fn new<T, F, G, H, I>(
        initial_size: usize,
        data: T,
        get_value: F,
        set_value: Option<G>,
        get_attr: Option<H>,
        is_enabled: Option<I>,
    ) -> Self
    where
        T: Any + 'static,
        F: for<'a> Fn(&'a T, usize, usize) -> Variant + 'static,
        G: for<'a, 'b> Fn(&'a T, usize, usize, &'b Variant) -> bool + 'static,
        H: for<'a> Fn(&'a T, usize, usize) -> Option<DataViewItemAttr> + 'static,
        I: for<'a> Fn(&'a T, usize, usize) -> bool + 'static,
    {
        // Wrap the user's data in a Box<dyn Any>
        let any_data = Box::new(data);

        // Convert type-specific callbacks to callbacks that work with Any
        let any_get_value: GetValueCallback = Box::new(move |any_data, row, col| {
            let data = any_data.downcast_ref::<T>().unwrap();
            get_value(data, row, col)
        });

        let any_set_value: Option<SetValueCallback> = if let Some(f) = set_value {
            Some(Box::new(move |any_data: &dyn Any, row, col, value| {
                let data = any_data.downcast_ref::<T>().unwrap();
                f(data, row, col, value)
            }))
        } else {
            None
        };

        let any_get_attr: Option<GetAttrCallback> = if let Some(f) = get_attr {
            Some(Box::new(move |any_data: &dyn Any, row, col| {
                let data = any_data.downcast_ref::<T>().unwrap();
                f(data, row, col)
            }))
        } else {
            None
        };

        let any_is_enabled: Option<IsEnabledCallback> = if let Some(f) = is_enabled {
            Some(Box::new(move |any_data: &dyn Any, row, col| {
                let data = any_data.downcast_ref::<T>().unwrap();
                f(data, row, col)
            }))
        } else {
            None
        };

        // Create callback data struct
        let callback_data = Box::new(CustomModelCallbacks {
            userdata: any_data,
            get_value: any_get_value,
            set_value: any_set_value,
            get_attr: any_get_attr,
            is_enabled: any_is_enabled,
        });

        unsafe {
            // Create C++ callbacks
            extern "C" fn get_value_callback(
                userdata: *mut ::std::os::raw::c_void,
                row: u64,
                col: u64,
                variant: *mut ffi::wxd_Variant_t,
            ) {
                if variant.is_null() {
                    return;
                }

                if userdata.is_null() {
                    unsafe {
                        (*variant).type_ = ffi::WXD_VARIANT_TYPE_STRING as i32;
                        let error_message = "Error: null userdata".to_string();
                        (*variant).data.string_val =
                            CString::new(error_message).unwrap_or_default().into_raw();
                    }
                    return;
                }

                // Safety: This cast should be valid if the userdata was properly created
                let callbacks = unsafe { &*(userdata as *const CustomModelCallbacks) };

                // Call the user's callback
                let value = (callbacks.get_value)(&*callbacks.userdata, row as usize, col as usize);

                // Convert Variant to wxd_Variant_t
                let raw_variant = to_raw_variant(&value);

                // Copy the result to the provided variant
                unsafe {
                    *variant = raw_variant;
                }
            }

            extern "C" fn set_value_callback(
                userdata: *mut ::std::os::raw::c_void,
                variant: *const ffi::wxd_Variant_t,
                row: u64,
                col: u64,
            ) -> bool {
                let callbacks = unsafe { &*(userdata as *const CustomModelCallbacks) };
                if let Some(set_value) = &callbacks.set_value {
                    // Convert wxd_Variant_t to Variant
                    let value = unsafe { from_raw_variant(variant) };

                    // Call the user's callback
                    (set_value)(&*callbacks.userdata, row as usize, col as usize, &value)
                } else {
                    false
                }
            }

            extern "C" fn get_attr_callback(
                userdata: *mut ::std::os::raw::c_void,
                row: u64,
                col: u64,
                attr: *mut ffi::wxd_DataViewItemAttr_t,
            ) -> bool {
                let callbacks = unsafe { &*(userdata as *const CustomModelCallbacks) };
                if let Some(get_attr) = &callbacks.get_attr {
                    if let Some(attrs) =
                        (get_attr)(&*callbacks.userdata, row as usize, col as usize)
                    {
                        // Copy the attributes to the provided struct
                        unsafe {
                            *attr = attrs.to_raw();
                        }
                        true
                    } else {
                        false
                    }
                } else {
                    false
                }
            }

            extern "C" fn is_enabled_callback(
                userdata: *mut ::std::os::raw::c_void,
                row: u64,
                col: u64,
            ) -> bool {
                let callbacks = unsafe { &*(userdata as *const CustomModelCallbacks) };
                if let Some(is_enabled) = &callbacks.is_enabled {
                    (is_enabled)(&*callbacks.userdata, row as usize, col as usize)
                } else {
                    true
                }
            }

            // Create the C++ model with our callbacks
            let raw_callback_data = Box::into_raw(callback_data);
            let handle = ffi::wxd_DataViewVirtualListModel_CreateWithCallbacks(
                initial_size as u64,
                raw_callback_data as *mut ::std::os::raw::c_void,
                Some(get_value_callback),
                Some(set_value_callback),
                Some(get_attr_callback),
                Some(is_enabled_callback),
            );

            if handle.is_null() {
                // If the C++ side failed, reclaim ownership and drop the box properly
                drop(Box::from_raw(raw_callback_data));

                // Create a dummy callback_data for the error case
                let dummy_callback_data = Box::new(CustomModelCallbacks {
                    userdata: Box::new(()),
                    get_value: Box::new(|_, _, _| Variant::String(String::new())),
                    set_value: None,
                    get_attr: None,
                    is_enabled: None,
                });

                return Self {
                    handle: std::ptr::null_mut(),
                    size: 0,
                    callback_data: dummy_callback_data,
                };
            }

            // Create a dummy callback_data for our own Rust-side model
            // The real one is now owned by C++ and will be properly cleaned up
            // when we call wxd_DataViewVirtualListModel_ReleaseCallbacks in Drop
            let dummy_callback_data = Box::new(CustomModelCallbacks {
                userdata: Box::new(()),
                get_value: Box::new(|_, _, _| Variant::String(String::new())),
                set_value: None,
                get_attr: None,
                is_enabled: None,
            });

            Self {
                handle,
                size: initial_size,
                callback_data: dummy_callback_data,
            }
        }
    }

    /// Creates a new simple custom model with defaults.
    pub fn new_simple(initial_size: usize) -> Self {
        Self::new(
            initial_size,
            (),
            |_, row, col| Variant::String(format!("Item ({row}, {col})")),
            None::<fn(&(), usize, usize, &Variant) -> bool>,
            None::<fn(&(), usize, usize) -> Option<DataViewItemAttr>>,
            None::<fn(&(), usize, usize) -> bool>,
        )
    }

    /// Notify that a row has been prepended
    pub fn row_prepended(&mut self) {
        unsafe {
            ffi::wxd_DataViewVirtualListModel_RowPrepended(self.handle);
        }
        self.size += 1;
    }

    /// Notify that a row has been inserted
    pub fn row_inserted(&mut self, before: usize) {
        unsafe {
            ffi::wxd_DataViewVirtualListModel_RowInserted(self.handle, before as u64);
        }
        self.size += 1;
    }

    /// Notify that a row has been appended
    pub fn row_appended(&mut self) {
        unsafe {
            ffi::wxd_DataViewVirtualListModel_RowAppended(self.handle);
        }
        self.size += 1;
    }

    /// Notify that a row has been deleted
    pub fn row_deleted(&mut self, row: usize) {
        unsafe {
            ffi::wxd_DataViewVirtualListModel_RowDeleted(self.handle, row as u64);
        }
        if self.size > 0 {
            self.size -= 1;
        }
    }

    /// Notify that a row has changed
    pub fn row_changed(&self, row: usize) {
        unsafe {
            ffi::wxd_DataViewVirtualListModel_RowChanged(self.handle, row as u64);
        }
    }

    /// Notify that a specific cell value has changed
    pub fn row_value_changed(&self, row: usize, col: usize) {
        unsafe {
            ffi::wxd_DataViewVirtualListModel_RowValueChanged(self.handle, row as u64, col as u64);
        }
    }

    /// Reset the model with a new size
    pub fn reset(&mut self, new_size: usize) {
        unsafe {
            ffi::wxd_DataViewVirtualListModel_Reset(self.handle, new_size as u64);
        }
        self.size = new_size;
    }

    /// Get the current size of the model
    pub fn size(&self) -> usize {
        self.size
    }
}

impl DataViewModel for CustomDataViewVirtualListModel {
    fn handle_ptr(&self) -> *mut ffi::wxd_DataViewModel_t {
        self.handle
    }

    fn get_column_count(&self) -> usize {
        // Virtual list model columns are set externally by the control
        0
    }

    fn get_row_count(&self) -> usize {
        self.size
    }

    fn get_value(&self, row: usize, col: usize) -> Variant {
        (self.callback_data.get_value)(&*self.callback_data.userdata, row, col)
    }

    fn set_value(&self, row: usize, col: usize, value: &Variant) -> bool {
        if let Some(set_value) = &self.callback_data.set_value {
            (set_value)(&*self.callback_data.userdata, row, col, value)
        } else {
            false
        }
    }

    fn get_attributes(&self, row: usize, col: usize) -> Option<DataViewItemAttr> {
        if let Some(get_attr) = &self.callback_data.get_attr {
            (get_attr)(&*self.callback_data.userdata, row, col)
        } else {
            None
        }
    }

    fn is_enabled(&self, row: usize, col: usize) -> bool {
        if let Some(is_enabled) = &self.callback_data.is_enabled {
            (is_enabled)(&*self.callback_data.userdata, row, col)
        } else {
            true
        }
    }
}

impl Drop for CustomDataViewVirtualListModel {
    fn drop(&mut self) {
        if !self.handle.is_null() {
            // IMPORTANT: Do not release callbacks here, as this would cause
            // issues with the model's use in the DataViewCtrl.
            // The callbacks need to remain valid for the lifetime of the model.
            //
            // We only need to ensure the model's reference count is managed correctly
            // by wxWidgets, which handles cleanup when the control is destroyed.
            //
            // ffi::wxd_DataViewVirtualListModel_ReleaseCallbacks(self.handle);

            // Note: We don't free the handle itself because wxWidgets takes ownership of it
            // via AssociateModel, which increases the reference count.
            // The model will be destroyed when the control it's associated with is destroyed.
            // In C++, we call IncRef() when creating the model and wxWidgets calls it again
            // during AssociateModel(), so the reference count is at least 2 at this point.

            // wxWidgets will call DecRef() when the control is destroyed, and we don't need
            // to call it here because we're letting the C++ side handle destruction.
        }
    }
}

/// Converts a Variant to a C wxd_Variant_t
pub fn to_raw_variant(value: &Variant) -> ffi::wxd_Variant_t {
    let mut result = ffi::wxd_Variant_t {
        type_: 0,
        data: unsafe { std::mem::zeroed() },
    };

    match value {
        Variant::Bool(val) => {
            result.type_ = ffi::WXD_VARIANT_TYPE_BOOL as i32;
            result.data.bool_val = *val;
        }
        Variant::Int32(val) => {
            result.type_ = ffi::WXD_VARIANT_TYPE_INT32 as i32;
            result.data.int32_val = *val;
        }
        Variant::Int64(val) => {
            result.type_ = ffi::WXD_VARIANT_TYPE_INT64 as i32;
            result.data.int64_val = *val;
        }
        Variant::Double(val) => {
            result.type_ = ffi::WXD_VARIANT_TYPE_DOUBLE as i32;
            result.data.double_val = *val;
        }
        Variant::String(val) => {
            result.type_ = ffi::WXD_VARIANT_TYPE_STRING as i32;

            // Use proper string duplication to ensure C++ can safely free it
            result.data.string_val = CString::new(val.as_str()).unwrap_or_default().into_raw();
        }
        Variant::DateTime(val) => {
            result.type_ = ffi::WXD_VARIANT_TYPE_DATETIME as i32;
            unsafe {
                result.data.datetime_val = *val.as_ptr();
            }
        }
        Variant::Bitmap(val) => {
            // This path is for an owned Bitmap, uses the FFI-cloned mechanism
            result.type_ = ffi::WXD_VARIANT_TYPE_BITMAP as i32;
            let original_rust_owned_ptr = val.as_ptr();
            if original_rust_owned_ptr.is_null() {
                result.data.bitmap_val = std::ptr::null_mut();
            } else {
                // Ask C++ to clone the bitmap. This new bitmap is on the C++ heap.
                // C++ GetValueByRow will be responsible for Destroying this clone later.
                let cloned_ptr_on_cpp_heap =
                    unsafe { ffi::wxd_Bitmap_Clone(original_rust_owned_ptr) };
                result.data.bitmap_val = cloned_ptr_on_cpp_heap;
            }
        }
        Variant::BitmapBorrowed(borrowed_ptr) => {
            // New path for borrowed bitmap pointer
            result.type_ = ffi::WXD_VARIANT_TYPE_BITMAP_RUST_BORROWED as i32;
            result.data.bitmap_val = *borrowed_ptr; // Pass the borrowed pointer directly
        }
    }

    result
}

/// Converts a C wxd_Variant_t to a Variant
///
/// # Safety
/// The caller must ensure the raw pointer is valid and points to a properly initialized wxd_Variant_t.
pub unsafe fn from_raw_variant(raw: *const ffi::wxd_Variant_t) -> Variant {
    if raw.is_null() {
        return Variant::String(String::new());
    }

    match (*raw).type_ {
        t if t == ffi::WXD_VARIANT_TYPE_BOOL as i32 => Variant::Bool((*raw).data.bool_val),
        t if t == ffi::WXD_VARIANT_TYPE_INT32 as i32 => Variant::Int32((*raw).data.int32_val),
        t if t == ffi::WXD_VARIANT_TYPE_INT64 as i32 => Variant::Int64((*raw).data.int64_val),
        t if t == ffi::WXD_VARIANT_TYPE_DOUBLE as i32 => Variant::Double((*raw).data.double_val),
        t if t == ffi::WXD_VARIANT_TYPE_STRING as i32 => {
            if (*raw).data.string_val.is_null() {
                Variant::String(String::new())
            } else {
                let c_str = CStr::from_ptr((*raw).data.string_val);
                Variant::String(c_str.to_string_lossy().to_string())
            }
        }
        t if t == ffi::WXD_VARIANT_TYPE_DATETIME as i32 => {
            // Create a DateTime from the raw data
            let dt = crate::DateTime::from_raw((*raw).data.datetime_val);
            Variant::DateTime(dt)
        }
        t if t == ffi::WXD_VARIANT_TYPE_BITMAP as i32 => {
            if (*raw).data.bitmap_val.is_null() {
                // Create a minimal 1x1 transparent bitmap as fallback
                match crate::Bitmap::from_rgba(&[0, 0, 0, 0], 1, 1) {
                    Some(bitmap) => Variant::Bitmap(bitmap),
                    None => Variant::String(String::new()), // Last resort fallback
                }
            } else {
                // For bitmaps from C++, we need to clone them as we don't own them
                let ptr = (*raw).data.bitmap_val;
                let cloned_ptr = ffi::wxd_Bitmap_Clone(ptr);
                if !cloned_ptr.is_null() {
                    let bitmap = crate::Bitmap::from_ptr_owned(cloned_ptr);
                    Variant::Bitmap(bitmap)
                } else {
                    // If clone fails, fallback to a small placeholder
                    match crate::Bitmap::from_rgba(&[255, 0, 0, 255], 1, 1) {
                        Some(bitmap) => Variant::Bitmap(bitmap),
                        None => Variant::String(String::new()),
                    }
                }
            }
        }
        _ => {
            // Default for unknown/unsupported types
            Variant::String(String::new())
        }
    }
}

/// Function called by C++ to properly drop CustomModelCallbacks that were allocated with Box::into_raw().
/// This must be used instead of C's free() for Box-allocated callback data.
///
/// # Safety
/// The caller (C++) must ensure `ptr` is a valid pointer obtained from
/// `Box::into_raw()` for a `CustomModelCallbacks` and that it hasn't been freed already.
#[no_mangle]
#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub extern "C" fn wxd_Drop_Rust_CustomModelCallbacks(ptr: *mut c_void) {
    if !ptr.is_null() {
        // Reconstitute the Box and let it drop, properly freeing the memory
        // and running any destructors for the contained data
        unsafe {
            let _callback_box = Box::from_raw(ptr as *mut CustomModelCallbacks);
            // Drop happens automatically when `_callback_box` goes out of scope here.
        }
    }
}
