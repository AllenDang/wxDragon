// A unified approach to associating custom data with widget items.
//
// This module provides a generic way to associate arbitrary Rust data
// with various wxDragon elements like list items, tree items, etc.

use std::any::Any;
use std::collections::HashMap;
use std::sync::{
    Arc, LazyLock, RwLock,
    atomic::{AtomicU64, Ordering},
};

// Global registry to store item data safely
static ITEM_DATA_REGISTRY: LazyLock<RwLock<HashMap<u64, Arc<dyn Any + Send + Sync>>>> =
    LazyLock::new(|| RwLock::new(HashMap::new()));

// Monotonic counter for item IDs
static NEXT_DATA_ID: AtomicU64 = AtomicU64::new(1);

/// Store an item in the global registry and return a unique ID
pub fn store_item_data<T: Any + Send + Sync + 'static>(data: T) -> u64 {
    let id = NEXT_DATA_ID.fetch_add(1, Ordering::SeqCst);
    ITEM_DATA_REGISTRY.write().unwrap().insert(id, Arc::new(data));
    id
}

/// Retrieve an item from the global registry by its ID
pub fn get_item_data(id: u64) -> Option<Arc<dyn Any + Send + Sync>> {
    if id == 0 {
        return None;
    }
    ITEM_DATA_REGISTRY.read().unwrap().get(&id).cloned()
}

/// Remove an item from the global registry
pub fn remove_item_data(id: u64) -> Option<Arc<dyn Any + Send + Sync>> {
    if id == 0 {
        return None;
    }
    ITEM_DATA_REGISTRY.write().unwrap().remove(&id)
}

/// A safe wrapper around the item data functionality in wxWidgets.
///
/// This allows you to associate arbitrary data with list items, tree items, etc.
/// The ItemData wraps the data item and provides type-safe access methods.
pub struct ItemData<T: Clone + 'static + Send + Sync> {
    data: T,
}

impl<T: Clone + 'static + Send + Sync> ItemData<T> {
    /// Creates a new ItemData with the specified data
    pub fn new(data: T) -> Self {
        Self { data }
    }

    /// Gets an immutable reference to the contained data
    pub fn data(&self) -> &T {
        &self.data
    }

    /// Gets a cloned copy of the contained data
    pub fn to_owned(&self) -> T {
        self.data.clone()
    }
}

/// Trait for widgets that can associate custom data with items
pub trait HasItemData {
    /// Associates custom data with an item
    ///
    /// # Returns
    /// The unique ID of the stored data that can be used for retrieval later,
    /// or 0 if the operation failed
    fn set_custom_data<T: Any + Send + Sync + 'static>(&self, item_id: impl Into<u64>, data: T) -> u64;

    /// Retrieves custom data previously associated with an item
    fn get_custom_data(&self, item_id: impl Into<u64>) -> Option<Arc<dyn Any + Send + Sync>>;

    /// Checks if an item has custom data associated with it
    fn has_custom_data(&self, item_id: impl Into<u64>) -> bool;

    /// Clears any custom data associated with an item
    fn clear_custom_data(&self, item_id: impl Into<u64>) -> bool;

    /// Cleans up all custom data associated with this widget
    ///
    /// This should be called when the widget is being destroyed to prevent memory leaks.
    /// Each implementation should know how to iterate through all items that might have
    /// custom data.
    fn cleanup_all_custom_data(&self);
}
