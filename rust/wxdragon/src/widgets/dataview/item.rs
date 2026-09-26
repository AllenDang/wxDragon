//! DataViewItem implementation.

use wxdragon_sys as ffi;

/// Represents an item in a DataViewCtrl.
///
/// This struct is a wrapper around a pointer to a C++ wxDataViewItem object.
/// It owns the C++ object when returned from an FFI call that allocates it (e.g., via FromWxDVI helpers).
/// When a DataViewItem is passed from Rust to C++, its internal pointer is used, but C++
/// does not take ownership of the Rust-side `DataViewItem` or the wxDataViewItem it points to.
#[derive(Debug)]
#[repr(C)]
pub struct DataViewItem {
    // inner is non-null when this Rust struct was created from a C wrapper pointer (wxd_DataViewItem_t*)
    // and therefore is responsible for calling wxd_DataViewItem_Release in Drop.
    inner: *const ffi::wxd_DataViewItem_t,
}

impl Default for DataViewItem {
    /// Creates an invalid/empty DataViewItem.
    ///
    /// The default implementation calls `wxd_DataViewItem_Clone` with a null pointer,
    /// which results in an invalid/empty DataViewItem.
    /// Use [`is_ok()`](DataViewItem::is_ok) to check if the item is valid.
    fn default() -> Self {
        let inner = unsafe { ffi::wxd_DataViewItem_Clone(std::ptr::null()) };
        Self { inner }
    }
}

impl DataViewItem {
    /// Creates an owning `DataViewItem` from a raw pointer, taking
    /// responsibility for releasing it.
    ///
    /// # Safety
    ///
    /// `raw` must point to a `wxd_DataViewItem_t` that nothing else will
    /// release, since [`DataViewItem`]'s `Drop` calls
    /// `wxd_DataViewItem_Release` on it. In particular it must not be a pointer
    /// borrowed from a live `DataViewItem` (via its [`Deref`](std::ops::Deref)
    /// or [`AsRef`]) — both wrappers would then release the same item.
    ///
    /// To build an item from a model node identity instead, use
    /// [`DataViewItem::from_id_ptr`], which allocates a fresh item.
    pub unsafe fn from_raw(raw: *const ffi::wxd_DataViewItem_t) -> Self {
        Self { inner: raw }
    }
}

impl AsRef<*const ffi::wxd_DataViewItem_t> for DataViewItem {
    fn as_ref(&self) -> &*const ffi::wxd_DataViewItem_t {
        &self.inner
    }
}

impl std::ops::Deref for DataViewItem {
    type Target = *const ffi::wxd_DataViewItem_t;

    /// Borrows the raw pointer so it can be handed to an FFI call.
    ///
    /// This does not transfer ownership: the pointer stays valid only while
    /// this `DataViewItem` is alive, and feeding it back to
    /// [`DataViewItem::from_raw`] would release the same item twice.
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DataViewItem {
    /// Checks if the inner DataViewItem is valid (non-null and the inner C++ object is valid).
    pub fn is_ok(&self) -> bool {
        unsafe { ffi::wxd_DataViewItem_IsOk(self.inner) }
    }

    /// Returns the ID pointer associated with this DataViewItem, or None if invalid.
    pub fn get_id<T>(&self) -> Option<*const T> {
        if self.is_ok() {
            Some(unsafe { ffi::wxd_DataViewItem_GetID(self.inner) as *const T })
        } else {
            None
        }
    }

    /// Create a DataViewItem from an arbitrary ID pointer.
    ///
    /// This allocates a fresh `wxDataViewItem` naming `raw` as the model node,
    /// so the returned item owns its own allocation and can safely coexist with
    /// other items naming the same node. Custom models use this to hand their
    /// node identities to the control.
    pub fn from_id_ptr<T>(raw: *const T) -> Self {
        let inner = unsafe { ffi::wxd_DataViewItem_CreateFromID(raw as *const std::ffi::c_void) };
        Self { inner }
    }
}

impl Drop for DataViewItem {
    fn drop(&mut self) {
        if !self.inner.is_null() {
            unsafe { ffi::wxd_DataViewItem_Release(self.inner) };
        }
    }
}

// It's important that DataViewItem is not Clone or Copy by default if it manages ownership via Drop.
// If cloning is needed, it would require manual implementation (e.g., an explicit clone method that calls
// a C++ FFI function to duplicate the wxDataViewItem if that's meaningful, or by using Rc/Arc if shared ownership
// within Rust is desired, though that doesn't map directly to the C++ object lifecycle here).
// For now, treating it as a unique owner is safest.

#[cfg(test)]
mod tests {
    use super::DataViewItem;

    #[test]
    fn from_id_ptr_allocates_a_distinct_item_per_call() {
        // Two items naming the same model node are independent allocations, so
        // each one's Drop releases only its own. This is the safe way to build
        // a second handle to a node; adopting a borrowed pointer is not.
        let node = 0x1234_usize;
        let a = DataViewItem::from_id_ptr(node as *const usize);
        let b = DataViewItem::from_id_ptr(node as *const usize);

        assert!(a.is_ok());
        assert!(b.is_ok());
        assert_eq!(a.get_id::<usize>(), b.get_id::<usize>(), "both should name the same node");
        assert!(!std::ptr::eq(*a, *b), "but must be separate wxDataViewItem allocations");

        drop(b);
        // Releasing one must leave the other intact.
        assert!(a.is_ok());
        assert_eq!(a.get_id::<usize>(), Some(node as *const usize));
    }

    #[test]
    fn a_default_item_is_not_ok_and_drops_cleanly() {
        let empty = DataViewItem::default();
        assert!(!empty.is_ok());
        assert_eq!(empty.get_id::<u8>(), None);
    }
}
