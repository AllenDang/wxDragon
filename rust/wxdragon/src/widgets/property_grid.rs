//! Safe Rust bindings for [`wxPropertyGrid`](https://docs.wxwidgets.org/latest/classwx_property_grid.html).
//!
//! A property grid presents labelled, typed values using editors appropriate
//! for each value. Properties are addressed by unique string names instead of
//! native pointers, so deleting a property or clearing the grid cannot leave a
//! dangling Rust handle.
//!
//! # Quick start
//!
//! ```no_run
//! use wxdragon::prelude::*;
//!
//! let _ = wxdragon::main(|_| {
//!     let frame = Frame::builder().with_title("Settings").build();
//!     let grid = PropertyGrid::builder(&frame)
//!         .with_style(PropertyGridStyle::BoldModified | PropertyGridStyle::Tooltips)
//!         .build();
//!
//!     let appearance = grid
//!         .append(Property::category("Appearance", "appearance"))
//!         .expect("unique property name");
//!     grid.append(Property::string("Title", "title", "My application").under(&appearance));
//!     grid.append(Property::boolean("Visible", "visible", true).under(&appearance));
//!
//!     grid.on_changed(|event| {
//!         println!("changed: {:?}", event.property_name());
//!     });
//!
//!     frame.show(true);
//! });
//! ```

use crate::event::{Event, EventType, WxEvtHandler};
use crate::geometry::{Point, Size};
use crate::id::Id;
use crate::widgets::dataview::Variant;
use crate::window::{WindowHandle, WxWidget};
use std::ffi::{CStr, CString};
use std::fmt;
use std::os::raw::c_char;
use wxdragon_sys as ffi;

widget_style_enum!(
    name: PropertyGridStyle,
    doc: "Style flags controlling PropertyGrid layout and editing behavior.\n\nFlags can be combined with `|` and supplied through `PropertyGridBuilder::with_style`.\n\n# Example\n\n```no_run\nuse wxdragon::prelude::*;\n\nlet _ = wxdragon::main(|_| {\n    let frame = Frame::builder().build();\n    let grid = PropertyGrid::builder(&frame)\n        .with_style(PropertyGridStyle::AutoSort | PropertyGridStyle::BoldModified | PropertyGridStyle::Tooltips)\n        .build();\n    frame.show(true);\n});\n```",
    variants: {
        Default: ffi::WXD_PG_DEFAULT_STYLE as i64, "Default property-grid style.",
        AutoSort: ffi::WXD_PG_AUTO_SORT as i64, "Automatically sort properties after insertion.",
        HideCategories: ffi::WXD_PG_HIDE_CATEGORIES as i64, "Hide category rows.",
        AlphabeticMode: ffi::WXD_PG_ALPHABETIC_MODE as i64, "Hide categories and sort properties alphabetically.",
        BoldModified: ffi::WXD_PG_BOLD_MODIFIED as i64, "Render modified values in bold.",
        SplitterAutoCenter: ffi::WXD_PG_SPLITTER_AUTO_CENTER as i64, "Keep the splitter centered while resizing.",
        Tooltips: ffi::WXD_PG_TOOLTIPS as i64, "Show tooltips for clipped cell text.",
        HideMargin: ffi::WXD_PG_HIDE_MARGIN as i64, "Hide the margin and expand/collapse buttons.",
        StaticSplitter: ffi::WXD_PG_STATIC_SPLITTER as i64, "Prevent users from moving the splitter.",
        StaticLayout: ffi::WXD_PG_STATIC_LAYOUT as i64, "Use a fixed margin and splitter layout.",
        LimitedEditing: ffi::WXD_PG_LIMITED_EDITING as i64, "Disable free-form text editors where another editor is available."
    },
    default_variant: Default
);

/// Stable, name-based identity for a property.
///
/// wxWidgets owns native property objects and may destroy them during
/// `delete_property()` or `clear()`. Keeping only the unique name avoids
/// exposing a pointer that could become dangling.
///
/// `PropertyId` implements [`AsRef<str>`], so it can be passed directly to
/// methods such as [`PropertyGrid::get_value`] and [`PropertyGrid::set`].
///
/// # Example
///
/// ```
/// use wxdragon::prelude::PropertyId;
///
/// let id = PropertyId::new("window.width");
/// assert_eq!(id.as_str(), "window.width");
/// assert_eq!(id.to_string(), "window.width");
/// ```
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PropertyId(String);

impl PropertyId {
    /// Creates an identifier from a property name.
    ///
    /// A useful identifier must be non-empty and must match the unique name
    /// supplied when the property was appended. [`PropertyGrid::append`]
    /// constructs identifiers only after successful insertion.
    pub fn new(name: impl Into<String>) -> Self {
        Self(name.into())
    }

    /// Returns the property name represented by this identifier.
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consumes the identifier and returns its owned property name.
    pub fn into_string(self) -> String {
        self.0
    }
}

impl AsRef<str> for PropertyId {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for PropertyId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl From<PropertyId> for String {
    fn from(value: PropertyId) -> Self {
        value.into_string()
    }
}

/// One labelled numeric entry in an enum or flags property.
///
/// The label is displayed to the user while `value` is stored by wxWidgets.
/// Tuples such as `("Dark", 2)` convert to `PropertyChoice` automatically.
///
/// # Example
///
/// ```
/// use wxdragon::prelude::PropertyChoice;
///
/// let choice = PropertyChoice::new("Dark", 2);
/// assert_eq!(choice.label, "Dark");
/// assert_eq!(choice.value, 2);
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PropertyChoice {
    /// Text shown in the property's choice editor.
    pub label: String,
    /// Numeric value associated with the label.
    pub value: i32,
}

impl PropertyChoice {
    /// Creates a choice from a display label and stored numeric value.
    pub fn new(label: impl Into<String>, value: i32) -> Self {
        Self {
            label: label.into(),
            value,
        }
    }
}

impl<S: Into<String>> From<(S, i32)> for PropertyChoice {
    fn from((label, value): (S, i32)) -> Self {
        Self::new(label, value)
    }
}

/// Standard wxPropertyGrid property types supported by [`Property`].
///
/// Most callers should use the named constructors on [`Property`] instead of
/// constructing this enum directly. It remains public for inspection and for
/// code that generates property descriptions dynamically.
#[derive(Debug, Clone, PartialEq)]
pub enum PropertyKind {
    /// A visual category that groups other properties.
    Category,
    /// An editable UTF-8 string.
    String(String),
    /// A signed 64-bit integer.
    Int(i64),
    /// An unsigned 64-bit integer.
    UInt(u64),
    /// A double-precision floating-point number.
    Float(f64),
    /// A boolean value.
    Bool(bool),
    /// One selected value from a fixed list.
    Enum {
        /// Labels and stored values available in the choice editor.
        choices: Vec<PropertyChoice>,
        /// Initially selected stored value.
        value: i32,
    },
    /// A bit set edited using labelled flag choices.
    Flags {
        /// Labels and bit values available in the flags editor.
        choices: Vec<PropertyChoice>,
        /// Initial bitwise combination of the choice values.
        value: i32,
    },
    /// A path edited with a file chooser.
    File(String),
    /// A path edited with a directory chooser.
    Dir(String),
}

/// Description of a property to append to a [`PropertyGrid`].
///
/// A property has a user-facing `label`, a unique programmatic `name`, an
/// optional parent, and a [`PropertyKind`]. Constructors cover all property
/// kinds supported by the binding. Use [`Property::under`] to place a property
/// inside a category returned by [`PropertyGrid::append`].
///
/// # Example
///
/// ```
/// use wxdragon::prelude::*;
///
/// let width = Property::int("Width", "width", 1280).under("geometry");
/// assert_eq!(width.label(), "Width");
/// assert_eq!(width.name(), "width");
/// assert_eq!(width.parent(), Some("geometry"));
/// assert_eq!(width.kind(), &PropertyKind::Int(1280));
///
/// let theme = Property::enumeration(
///     "Theme",
///     "theme",
///     [("System", 0), ("Light", 1), ("Dark", 2)],
///     0,
/// );
/// assert!(matches!(theme.kind(), PropertyKind::Enum { .. }));
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct Property {
    label: String,
    name: String,
    parent: Option<String>,
    kind: PropertyKind,
}

impl Property {
    fn new(label: impl Into<String>, name: impl Into<String>, kind: PropertyKind) -> Self {
        Self {
            label: label.into(),
            name: name.into(),
            parent: None,
            kind,
        }
    }

    /// Creates a category used to group child properties.
    pub fn category(label: impl Into<String>, name: impl Into<String>) -> Self {
        Self::new(label, name, PropertyKind::Category)
    }

    /// Creates an editable string property.
    pub fn string(label: impl Into<String>, name: impl Into<String>, value: impl Into<String>) -> Self {
        Self::new(label, name, PropertyKind::String(value.into()))
    }

    /// Creates a signed 64-bit integer property.
    ///
    /// wxWidgets' `wxLongLong` constructor is used so the range is consistent
    /// on Windows, Linux, and macOS.
    pub fn int(label: impl Into<String>, name: impl Into<String>, value: i64) -> Self {
        Self::new(label, name, PropertyKind::Int(value))
    }

    /// Creates an unsigned 64-bit integer property.
    pub fn uint(label: impl Into<String>, name: impl Into<String>, value: u64) -> Self {
        Self::new(label, name, PropertyKind::UInt(value))
    }

    /// Creates a double-precision floating-point property.
    pub fn float(label: impl Into<String>, name: impl Into<String>, value: f64) -> Self {
        Self::new(label, name, PropertyKind::Float(value))
    }

    /// Creates a boolean property.
    pub fn boolean(label: impl Into<String>, name: impl Into<String>, value: bool) -> Self {
        Self::new(label, name, PropertyKind::Bool(value))
    }

    /// Creates a single-selection property from labelled numeric choices.
    ///
    /// `value` is the stored value to select initially, not necessarily the
    /// zero-based position of the choice.
    pub fn enumeration<I, C>(label: impl Into<String>, name: impl Into<String>, choices: I, value: i32) -> Self
    where
        I: IntoIterator<Item = C>,
        C: Into<PropertyChoice>,
    {
        Self::new(
            label,
            name,
            PropertyKind::Enum {
                choices: choices.into_iter().map(Into::into).collect(),
                value,
            },
        )
    }

    /// Creates a bit-flags property from labelled bit values.
    ///
    /// `value` is the initial bitwise combination. Flag values use `i32`
    /// because the native `wxFlagsProperty` stores a Windows-compatible
    /// `long` value.
    pub fn flags<I, C>(label: impl Into<String>, name: impl Into<String>, choices: I, value: i32) -> Self
    where
        I: IntoIterator<Item = C>,
        C: Into<PropertyChoice>,
    {
        Self::new(
            label,
            name,
            PropertyKind::Flags {
                choices: choices.into_iter().map(Into::into).collect(),
                value,
            },
        )
    }

    /// Creates a string path property with a file chooser editor.
    pub fn file(label: impl Into<String>, name: impl Into<String>, value: impl Into<String>) -> Self {
        Self::new(label, name, PropertyKind::File(value.into()))
    }

    /// Creates a string path property with a directory chooser editor.
    pub fn dir(label: impl Into<String>, name: impl Into<String>, value: impl Into<String>) -> Self {
        Self::new(label, name, PropertyKind::Dir(value.into()))
    }

    /// Sets the category or property below which this property is appended.
    ///
    /// The parent must already exist when [`PropertyGrid::append`] is called.
    /// Passing the [`PropertyId`] returned when a category was appended avoids
    /// spelling its name twice.
    pub fn under(mut self, parent: impl AsRef<str>) -> Self {
        self.parent = Some(parent.as_ref().to_owned());
        self
    }

    /// Returns the user-facing label.
    pub fn label(&self) -> &str {
        &self.label
    }

    /// Returns the unique programmatic name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the configured parent name, or `None` for a root property.
    pub fn parent(&self) -> Option<&str> {
        self.parent.as_deref()
    }

    /// Returns the property's type and initial value.
    pub fn kind(&self) -> &PropertyKind {
        &self.kind
    }
}

/// Events emitted by [`PropertyGrid`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PropertyGridEvent {
    /// The current property selection changed.
    Selected,
    /// A property value is about to change and may be vetoed.
    Changing,
    /// A property value changed successfully.
    Changed,
    /// The mouse highlighted a property.
    Highlighted,
    /// A property was right-clicked.
    RightClick,
    /// The active page changed in a property-grid manager.
    PageChanged,
    /// A property or category was collapsed.
    ItemCollapsed,
    /// A property or category was expanded.
    ItemExpanded,
    /// A property was double-clicked.
    DoubleClick,
    /// Label editing is about to begin.
    LabelEditBegin,
    /// Label editing is about to finish.
    LabelEditEnding,
    /// The user began dragging a column splitter.
    ColumnBeginDrag,
    /// A column splitter is being dragged.
    ColumnDragging,
    /// The user finished dragging a column splitter.
    ColumnEndDrag,
}

/// Data delivered to PropertyGrid event handlers.
///
/// Event data is transient and should only be used during the callback. The
/// [`PropertyGridEventData::property_name`] method returns an owned `String`,
/// and [`PropertyGridEventData::value`] returns an owned [`Variant`], so those
/// values may safely be retained after the callback returns.
///
/// # Validation example
///
/// ```no_run
/// # use wxdragon::prelude::*;
/// # let _ = wxdragon::main(|_| {
/// # let frame = Frame::builder().build();
/// let grid = PropertyGrid::builder(&frame).build();
/// grid.append(Property::int("Width", "width", 640));
///
/// grid.on_changing(|event| {
///     let invalid_width = event.property_name().as_deref() == Some("width")
///         && event.value().and_then(|value| value.get_i64()).is_some_and(|value| value < 100);
///     if invalid_width && event.can_veto() {
///         event.veto(true);
///     }
/// });
/// # });
/// ```
#[derive(Debug)]
pub struct PropertyGridEventData {
    event: Event,
}

impl PropertyGridEventData {
    /// Wraps a generic wxDragon event as PropertyGrid event data.
    ///
    /// Applications normally receive this type through methods such as
    /// [`PropertyGrid::on_changed`] instead of constructing it directly.
    pub fn new(event: Event) -> Self {
        Self { event }
    }

    /// Returns the identifier of the control that emitted the event.
    pub fn get_id(&self) -> i32 {
        self.event.get_id()
    }

    /// Controls whether wxWidgets continues processing the event.
    pub fn skip(&self, skip: bool) {
        self.event.skip(skip);
    }

    /// Returns the name of the property associated with this event.
    ///
    /// Some events, such as column dragging, may not identify a property and
    /// therefore return `None`.
    pub fn property_name(&self) -> Option<String> {
        if self.event.is_null() {
            return None;
        }
        read_ffi_string(|out, out_len| unsafe { ffi::wxd_PropertyGridEvent_GetPropertyName(self.event.0, out, out_len) })
    }

    /// Returns the current value, or the pending value for `Changing`.
    ///
    /// The returned [`Variant`] owns a native clone and remains valid after the
    /// event callback completes.
    pub fn value(&self) -> Option<Variant> {
        if self.event.is_null() {
            return None;
        }
        let ptr = unsafe { ffi::wxd_PropertyGridEvent_GetValue(self.event.0) };
        if ptr.is_null() { None } else { Some(Variant::from(ptr)) }
    }

    /// Returns the associated column index for column-related events.
    pub fn column(&self) -> u32 {
        if self.event.is_null() {
            return 0;
        }
        unsafe { ffi::wxd_PropertyGridEvent_GetColumn(self.event.0) }
    }

    /// Returns whether this event represents an operation that can be vetoed.
    pub fn can_veto(&self) -> bool {
        !self.event.is_null() && unsafe { ffi::wxd_PropertyGridEvent_CanVeto(self.event.0) }
    }

    /// Vetoes or restores the pending operation.
    ///
    /// This is currently meaningful for `Changing` events. Passing `true`
    /// rejects the proposed value; passing `false` removes an earlier veto.
    pub fn veto(&self, veto: bool) {
        if !self.event.is_null() {
            unsafe { ffi::wxd_PropertyGridEvent_Veto(self.event.0, veto) }
        }
    }

    /// Returns whether a handler has vetoed this event.
    pub fn was_vetoed(&self) -> bool {
        !self.event.is_null() && unsafe { ffi::wxd_PropertyGridEvent_WasVetoed(self.event.0) }
    }
}

/// A two-column editor for named, typed properties.
///
/// `PropertyGrid` is a lightweight, copyable widget handle. The native window
/// is owned by its wxWidgets parent. When that window is destroyed, the shared
/// [`WindowHandle`] is invalidated and subsequent methods return failure values
/// or become no-ops.
///
/// Properties are appended from [`Property`] descriptions and addressed by
/// unique names. [`PropertyGrid::append`] returns a [`PropertyId`] only after
/// wxWidgets accepts the property. The identifier contains no native pointer,
/// so it remains memory-safe even if the property is later deleted.
///
/// # Building and populating a grid
///
/// ```no_run
/// use wxdragon::prelude::*;
///
/// let _ = wxdragon::main(|_| {
///     let frame = Frame::builder()
///         .with_title("Application settings")
///         .with_size(Size::new(600, 450))
///         .build();
///     let panel = Panel::builder(&frame).build();
///     let sizer = BoxSizer::builder(Orientation::Vertical).build();
///
///     let grid = PropertyGrid::builder(&panel)
///         .with_style(PropertyGridStyle::BoldModified | PropertyGridStyle::Tooltips)
///         .build();
///
///     let general = grid
///         .append(Property::category("General", "general"))
///         .expect("failed to append category");
///     grid.append(Property::string("Name", "name", "Example").under(&general));
///     grid.append(Property::boolean("Enabled", "enabled", true).under(&general));
///     grid.append(
///         Property::enumeration(
///             "Theme",
///             "theme",
///             [("System", 0), ("Light", 1), ("Dark", 2)],
///             0,
///         )
///         .under(&general),
///     );
///
///     grid.set_help_string("name", "Name shown in the application title bar.");
///     grid.on_changed(|event| {
///         if let Some(name) = event.property_name() {
///             println!("{name} changed to {:?}", event.value());
///         }
///     });
///
///     sizer.add(&grid, 1, SizerFlag::Expand | SizerFlag::All, 8);
///     panel.set_sizer(sizer, true);
///     frame.show(true);
/// });
/// ```
///
/// # Programmatic values
///
/// [`PropertyGrid::set`] updates a value silently. [`PropertyGrid::change`]
/// runs validation and emits the normal changing/changed event sequence.
/// Values are represented by the shared [`Variant`] type.
#[derive(Clone, Copy)]
pub struct PropertyGrid {
    handle: WindowHandle,
}

impl PropertyGrid {
    /// Creates a builder for a PropertyGrid owned by `parent`.
    pub fn builder(parent: &dyn WxWidget) -> PropertyGridBuilder<'_> {
        PropertyGridBuilder::new(parent)
    }

    fn new_impl(parent: *mut ffi::wxd_Window_t, id: Id, pos: Point, size: Size, style: i64) -> Self {
        assert!(!parent.is_null(), "PropertyGrid requires a parent");
        let ptr = unsafe { ffi::wxd_PropertyGrid_Create(parent, id, pos.into(), size.into(), style) };
        assert!(!ptr.is_null(), "Failed to create PropertyGrid: FFI returned null");
        Self {
            handle: WindowHandle::new(ptr.cast()),
        }
    }

    #[inline]
    fn property_grid_ptr(&self) -> *mut ffi::wxd_PropertyGrid_t {
        self.handle.get_ptr().map(|ptr| ptr.cast()).unwrap_or(std::ptr::null_mut())
    }

    /// Returns the shared handle used to track the native window's lifetime.
    pub fn window_handle(&self) -> WindowHandle {
        self.handle
    }

    /// Returns whether a property with the exact, case-sensitive name exists.
    pub fn contains(&self, name: impl AsRef<str>) -> bool {
        let Some(name) = to_cstring(name.as_ref()) else {
            return false;
        };
        let ptr = self.property_grid_ptr();
        !ptr.is_null() && unsafe { ffi::wxd_PropertyGrid_Contains(ptr, name.as_ptr()) }
    }

    /// Appends a property and returns its name-based identifier on success.
    ///
    /// This returns `None` if the widget was destroyed, the property name is
    /// empty or contains an interior NUL, the name is already in use, the
    /// configured parent does not exist, or wxWidgets rejects the property.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use wxdragon::prelude::*;
    /// # let _ = wxdragon::main(|_| {
    /// # let frame = Frame::builder().build();
    /// let grid = PropertyGrid::builder(&frame).build();
    /// let category = grid.append(Property::category("Network", "network")).unwrap();
    /// let host = grid
    ///     .append(Property::string("Host", "host", "localhost").under(&category))
    ///     .unwrap();
    /// assert!(grid.contains(&host));
    /// # });
    /// ```
    pub fn append(&self, property: Property) -> Option<PropertyId> {
        let ptr = self.property_grid_ptr();
        if ptr.is_null() || property.name.is_empty() {
            return None;
        }

        let label = to_cstring(&property.label)?;
        let name = to_cstring(&property.name)?;
        let parent = match property.parent.as_deref() {
            Some(parent) => Some(to_cstring(parent)?),
            None => None,
        };
        let parent_ptr = parent.as_ref().map_or(std::ptr::null(), |value| value.as_ptr());

        let appended = match &property.kind {
            PropertyKind::Category => unsafe {
                ffi::wxd_PropertyGrid_AppendCategory(ptr, parent_ptr, label.as_ptr(), name.as_ptr())
            },
            PropertyKind::String(value) => {
                let value = to_cstring(value)?;
                unsafe { ffi::wxd_PropertyGrid_AppendString(ptr, parent_ptr, label.as_ptr(), name.as_ptr(), value.as_ptr()) }
            }
            PropertyKind::Int(value) => unsafe {
                ffi::wxd_PropertyGrid_AppendInt(ptr, parent_ptr, label.as_ptr(), name.as_ptr(), *value)
            },
            PropertyKind::UInt(value) => unsafe {
                ffi::wxd_PropertyGrid_AppendUInt(ptr, parent_ptr, label.as_ptr(), name.as_ptr(), *value)
            },
            PropertyKind::Float(value) => unsafe {
                ffi::wxd_PropertyGrid_AppendFloat(ptr, parent_ptr, label.as_ptr(), name.as_ptr(), *value)
            },
            PropertyKind::Bool(value) => unsafe {
                ffi::wxd_PropertyGrid_AppendBool(ptr, parent_ptr, label.as_ptr(), name.as_ptr(), *value)
            },
            PropertyKind::Enum { choices, value } => unsafe {
                append_choices(ptr, parent_ptr, &label, &name, choices, *value, false)?
            },
            PropertyKind::Flags { choices, value } => unsafe {
                append_choices(ptr, parent_ptr, &label, &name, choices, *value, true)?
            },
            PropertyKind::File(value) => {
                let value = to_cstring(value)?;
                unsafe { ffi::wxd_PropertyGrid_AppendFile(ptr, parent_ptr, label.as_ptr(), name.as_ptr(), value.as_ptr()) }
            }
            PropertyKind::Dir(value) => {
                let value = to_cstring(value)?;
                unsafe { ffi::wxd_PropertyGrid_AppendDir(ptr, parent_ptr, label.as_ptr(), name.as_ptr(), value.as_ptr()) }
            }
        };

        appended.then(|| PropertyId::new(property.name))
    }

    /// Returns an owned copy of a property's value.
    ///
    /// `None` means the widget or property does not exist. An unspecified
    /// property value is represented by a null [`Variant`].
    pub fn get_value(&self, name: impl AsRef<str>) -> Option<Variant> {
        let name = to_cstring(name.as_ref())?;
        let ptr = self.property_grid_ptr();
        if ptr.is_null() {
            return None;
        }
        let value = unsafe { ffi::wxd_PropertyGrid_GetValue(ptr, name.as_ptr()) };
        (!value.is_null()).then(|| Variant::from(value))
    }

    /// Sets a value without validation or property change events.
    ///
    /// Returns `false` when the widget or named property does not exist. The
    /// caller is responsible for supplying a value compatible with the
    /// property's kind.
    pub fn set_value(&self, name: impl AsRef<str>, value: &Variant) -> bool {
        let Some(name) = to_cstring(name.as_ref()) else {
            return false;
        };
        let ptr = self.property_grid_ptr();
        !ptr.is_null() && unsafe { ffi::wxd_PropertyGrid_SetValue(ptr, name.as_ptr(), value.as_const_ptr()) }
    }

    /// Converts and sets a value without validation or change events.
    ///
    /// This convenience method accepts values supported by [`Variant`], such
    /// as `bool`, `i32`, `i64`, `u64`, `f64`, `String`, and `&str`.
    ///
    /// ```no_run
    /// # use wxdragon::prelude::*;
    /// # let _ = wxdragon::main(|_| {
    /// # let frame = Frame::builder().build();
    /// let grid = PropertyGrid::builder(&frame).build();
    /// grid.append(Property::int("Width", "width", 640));
    /// assert!(grid.set("width", 800_i64));
    /// # });
    /// ```
    pub fn set<V: Into<Variant>>(&self, name: impl AsRef<str>, value: V) -> bool {
        let value = value.into();
        self.set_value(name, &value)
    }

    /// Sets a value through validation and emits changing/changed events.
    ///
    /// Returns whether wxWidgets accepted the proposed value. A handler
    /// registered with [`PropertyGrid::on_changing`] may veto the update.
    pub fn change_value(&self, name: impl AsRef<str>, value: &Variant) -> bool {
        let Some(name) = to_cstring(name.as_ref()) else {
            return false;
        };
        let ptr = self.property_grid_ptr();
        !ptr.is_null() && unsafe { ffi::wxd_PropertyGrid_ChangeValue(ptr, name.as_ptr(), value.as_const_ptr()) }
    }

    /// Converts a value, validates it, and emits changing/changed events.
    pub fn change<V: Into<Variant>>(&self, name: impl AsRef<str>, value: V) -> bool {
        let value = value.into();
        self.change_value(name, &value)
    }

    /// Marks a property's value as unspecified/null.
    ///
    /// Returns `false` when the widget or property does not exist.
    pub fn clear_value(&self, name: impl AsRef<str>) -> bool {
        self.call_name_bool(name, |ptr, name| unsafe { ffi::wxd_PropertyGrid_ClearValue(ptr, name) })
    }

    /// Returns the value formatted exactly as the property displays it.
    pub fn get_value_as_string(&self, name: impl AsRef<str>) -> Option<String> {
        let name = to_cstring(name.as_ref())?;
        let ptr = self.property_grid_ptr();
        if ptr.is_null() {
            return None;
        }
        read_ffi_string(|out, out_len| unsafe { ffi::wxd_PropertyGrid_GetValueAsString(ptr, name.as_ptr(), out, out_len) })
    }

    /// Returns a property's user-facing label.
    pub fn get_label(&self, name: impl AsRef<str>) -> Option<String> {
        let name = to_cstring(name.as_ref())?;
        let ptr = self.property_grid_ptr();
        if ptr.is_null() {
            return None;
        }
        read_ffi_string(|out, out_len| unsafe { ffi::wxd_PropertyGrid_GetLabel(ptr, name.as_ptr(), out, out_len) })
    }

    /// Changes a property's user-facing label without changing its unique name.
    pub fn set_label(&self, name: impl AsRef<str>, label: &str) -> bool {
        let (Some(name), Some(label)) = (to_cstring(name.as_ref()), to_cstring(label)) else {
            return false;
        };
        let ptr = self.property_grid_ptr();
        !ptr.is_null() && unsafe { ffi::wxd_PropertyGrid_SetLabel(ptr, name.as_ptr(), label.as_ptr()) }
    }

    /// Returns the descriptive help text associated with a property.
    pub fn get_help_string(&self, name: impl AsRef<str>) -> Option<String> {
        let name = to_cstring(name.as_ref())?;
        let ptr = self.property_grid_ptr();
        if ptr.is_null() {
            return None;
        }
        read_ffi_string(|out, out_len| unsafe { ffi::wxd_PropertyGrid_GetHelpString(ptr, name.as_ptr(), out, out_len) })
    }

    /// Sets the descriptive help text associated with a property.
    ///
    /// Depending on grid style, wxWidgets may show this text as a tooltip or
    /// in a property-grid manager's description area.
    pub fn set_help_string(&self, name: impl AsRef<str>, help: &str) -> bool {
        let (Some(name), Some(help)) = (to_cstring(name.as_ref()), to_cstring(help)) else {
            return false;
        };
        let ptr = self.property_grid_ptr();
        !ptr.is_null() && unsafe { ffi::wxd_PropertyGrid_SetHelpString(ptr, name.as_ptr(), help.as_ptr()) }
    }

    /// Sets a native wxPropertyGrid attribute on one property.
    ///
    /// Attribute names and expected value types are property-specific. Common
    /// examples include `"Min"`, `"Max"`, `"Step"`, and `"Precision"`.
    /// Returns `false` if the widget, property, attribute name, or value is
    /// invalid at the binding boundary.
    pub fn set_attribute(&self, name: impl AsRef<str>, attribute: &str, value: &Variant) -> bool {
        let (Some(name), Some(attribute)) = (to_cstring(name.as_ref()), to_cstring(attribute)) else {
            return false;
        };
        let ptr = self.property_grid_ptr();
        !ptr.is_null()
            && unsafe { ffi::wxd_PropertyGrid_SetAttribute(ptr, name.as_ptr(), attribute.as_ptr(), value.as_const_ptr()) }
    }

    /// Enables or disables editing and interaction for a property.
    pub fn enable_property(&self, name: impl AsRef<str>, enable: bool) -> bool {
        self.call_name_bool(name, |ptr, name| unsafe {
            ffi::wxd_PropertyGrid_EnableProperty(ptr, name, enable)
        })
    }

    /// Hides or shows a property and its children.
    pub fn hide_property(&self, name: impl AsRef<str>, hide: bool) -> bool {
        self.call_name_bool(name, |ptr, name| unsafe {
            ffi::wxd_PropertyGrid_HideProperty(ptr, name, hide)
        })
    }

    /// Sets or clears the read-only state for a property and its children.
    pub fn set_property_read_only(&self, name: impl AsRef<str>, read_only: bool) -> bool {
        self.call_name_bool(name, |ptr, name| unsafe {
            ffi::wxd_PropertyGrid_SetPropertyReadOnly(ptr, name, read_only)
        })
    }

    /// Returns whether a property exists and is enabled.
    pub fn is_property_enabled(&self, name: impl AsRef<str>) -> bool {
        self.call_name_bool(name, |ptr, name| unsafe {
            ffi::wxd_PropertyGrid_IsPropertyEnabled(ptr, name)
        })
    }

    /// Returns whether a property exists and is hidden.
    pub fn is_property_hidden(&self, name: impl AsRef<str>) -> bool {
        self.call_name_bool(name, |ptr, name| unsafe { ffi::wxd_PropertyGrid_IsPropertyHidden(ptr, name) })
    }

    /// Returns whether a property exists and its child rows are expanded.
    pub fn is_property_expanded(&self, name: impl AsRef<str>) -> bool {
        self.call_name_bool(name, |ptr, name| unsafe {
            ffi::wxd_PropertyGrid_IsPropertyExpanded(ptr, name)
        })
    }

    /// Returns whether the named item is a category.
    pub fn is_property_category(&self, name: impl AsRef<str>) -> bool {
        self.call_name_bool(name, |ptr, name| unsafe {
            ffi::wxd_PropertyGrid_IsPropertyCategory(ptr, name)
        })
    }

    /// Returns whether wxWidgets has marked the property's value as modified.
    pub fn is_property_modified(&self, name: impl AsRef<str>) -> bool {
        self.call_name_bool(name, |ptr, name| unsafe {
            ffi::wxd_PropertyGrid_IsPropertyModified(ptr, name)
        })
    }

    /// Expands one category or property with children.
    ///
    /// Returns `true` only when the item actually changes from collapsed to
    /// expanded.
    pub fn expand(&self, name: impl AsRef<str>) -> bool {
        self.call_name_bool(name, |ptr, name| unsafe { ffi::wxd_PropertyGrid_Expand(ptr, name) })
    }

    /// Collapses one category or property with children.
    ///
    /// Returns `true` only when the item actually changes from expanded to
    /// collapsed.
    pub fn collapse(&self, name: impl AsRef<str>) -> bool {
        self.call_name_bool(name, |ptr, name| unsafe { ffi::wxd_PropertyGrid_Collapse(ptr, name) })
    }

    /// Expands all expandable items when `expand` is true, or collapses them otherwise.
    ///
    /// The operation can fail if an active editor contains an invalid value.
    pub fn expand_all(&self, expand: bool) -> bool {
        let ptr = self.property_grid_ptr();
        !ptr.is_null() && unsafe { ffi::wxd_PropertyGrid_ExpandAll(ptr, expand) }
    }

    /// Selects a property and optionally focuses its editor.
    ///
    /// Returns whether selection succeeded. Selection may fail if the current
    /// editor cannot commit its value.
    pub fn select_property(&self, name: impl AsRef<str>, focus: bool) -> bool {
        self.call_name_bool(name, |ptr, name| unsafe {
            ffi::wxd_PropertyGrid_SelectProperty(ptr, name, focus)
        })
    }

    /// Returns the unique name of the currently selected property.
    pub fn get_selected_property_name(&self) -> Option<String> {
        let ptr = self.property_grid_ptr();
        if ptr.is_null() {
            return None;
        }
        read_ffi_string(|out, out_len| unsafe { ffi::wxd_PropertyGrid_GetSelectedPropertyName(ptr, out, out_len) })
    }

    /// Removes and destroys a property and all of its children.
    ///
    /// Existing [`PropertyId`] values remain ordinary strings but no longer
    /// resolve until a property with the same name is appended again.
    pub fn delete_property(&self, name: impl AsRef<str>) -> bool {
        self.call_name_bool(name, |ptr, name| unsafe { ffi::wxd_PropertyGrid_DeleteProperty(ptr, name) })
    }

    /// Removes and destroys every property in the grid.
    pub fn clear(&self) {
        let ptr = self.property_grid_ptr();
        if !ptr.is_null() {
            unsafe { ffi::wxd_PropertyGrid_Clear(ptr) }
        }
    }

    /// Clears the modified marker from all properties.
    pub fn clear_modified_status(&self) {
        let ptr = self.property_grid_ptr();
        if !ptr.is_null() {
            unsafe { ffi::wxd_PropertyGrid_ClearModifiedStatus(ptr) }
        }
    }

    /// Returns the x-coordinate of a column splitter, or `-1` for an invalid grid.
    ///
    /// For the normal two-column layout, pass `0`.
    pub fn get_splitter_position(&self, column: u32) -> i32 {
        let ptr = self.property_grid_ptr();
        if ptr.is_null() {
            -1
        } else {
            unsafe { ffi::wxd_PropertyGrid_GetSplitterPosition(ptr, column) }
        }
    }

    /// Moves a column splitter to the requested x-coordinate.
    ///
    /// For the normal two-column layout, pass `0` as `column`.
    pub fn set_splitter_position(&self, position: i32, column: u32) {
        let ptr = self.property_grid_ptr();
        if !ptr.is_null() {
            unsafe { ffi::wxd_PropertyGrid_SetSplitterPosition(ptr, position, column) }
        }
    }

    /// Returns the automatic resize proportion for a column.
    ///
    /// A return value of `-1` indicates an invalid grid or column.
    pub fn get_column_proportion(&self, column: u32) -> i32 {
        let ptr = self.property_grid_ptr();
        if ptr.is_null() {
            -1
        } else {
            unsafe { ffi::wxd_PropertyGrid_GetColumnProportion(ptr, column) }
        }
    }

    /// Sets a column's automatic resize proportion.
    ///
    /// This is primarily useful with [`PropertyGridStyle::SplitterAutoCenter`].
    pub fn set_column_proportion(&self, column: u32, proportion: i32) -> bool {
        let ptr = self.property_grid_ptr();
        !ptr.is_null() && unsafe { ffi::wxd_PropertyGrid_SetColumnProportion(ptr, column, proportion) }
    }

    /// Centers the primary splitter.
    ///
    /// `enable_auto_resizing` keeps proportional resizing enabled afterward
    /// when the corresponding style is active.
    pub fn center_splitter(&self, enable_auto_resizing: bool) {
        let ptr = self.property_grid_ptr();
        if !ptr.is_null() {
            unsafe { ffi::wxd_PropertyGrid_CenterSplitter(ptr, enable_auto_resizing) }
        }
    }

    /// Schedules the grid window to be repainted.
    pub fn refresh_grid(&self) {
        let ptr = self.property_grid_ptr();
        if !ptr.is_null() {
            unsafe { ffi::wxd_PropertyGrid_Refresh(ptr) }
        }
    }

    fn call_name_bool(
        &self,
        name: impl AsRef<str>,
        call: impl FnOnce(*mut ffi::wxd_PropertyGrid_t, *const c_char) -> bool,
    ) -> bool {
        let Some(name) = to_cstring(name.as_ref()) else {
            return false;
        };
        let ptr = self.property_grid_ptr();
        !ptr.is_null() && call(ptr, name.as_ptr())
    }
}

impl WxWidget for PropertyGrid {
    fn handle_ptr(&self) -> *mut ffi::wxd_Window_t {
        self.handle.get_ptr().unwrap_or(std::ptr::null_mut())
    }

    fn is_valid(&self) -> bool {
        self.handle.is_valid()
    }
}

impl WxEvtHandler for PropertyGrid {
    unsafe fn get_event_handler_ptr(&self) -> *mut ffi::wxd_EvtHandler_t {
        self.handle.get_ptr().unwrap_or(std::ptr::null_mut()).cast()
    }
}

widget_builder!(
    name: PropertyGrid,
    parent_type: &'a dyn WxWidget,
    style_type: PropertyGridStyle,
    fields: {},
    build_impl: |slf| {
        PropertyGrid::new_impl(
            slf.parent.handle_ptr(),
            slf.id,
            slf.pos,
            slf.size,
            slf.style.bits(),
        )
    }
);

crate::implement_widget_local_event_handlers!(
    PropertyGrid,
    PropertyGridEvent,
    PropertyGridEventData,
    Selected => selected, EventType::PG_SELECTED,
    Changing => changing, EventType::PG_CHANGING,
    Changed => changed, EventType::PG_CHANGED,
    Highlighted => highlighted, EventType::PG_HIGHLIGHTED,
    RightClick => right_click, EventType::PG_RIGHT_CLICK,
    PageChanged => page_changed, EventType::PG_PAGE_CHANGED,
    ItemCollapsed => item_collapsed, EventType::PG_ITEM_COLLAPSED,
    ItemExpanded => item_expanded, EventType::PG_ITEM_EXPANDED,
    DoubleClick => double_click, EventType::PG_DOUBLE_CLICK,
    LabelEditBegin => label_edit_begin, EventType::PG_LABEL_EDIT_BEGIN,
    LabelEditEnding => label_edit_ending, EventType::PG_LABEL_EDIT_ENDING,
    ColumnBeginDrag => column_begin_drag, EventType::PG_COL_BEGIN_DRAG,
    ColumnDragging => column_dragging, EventType::PG_COL_DRAGGING,
    ColumnEndDrag => column_end_drag, EventType::PG_COL_END_DRAG
);

impl crate::window::FromWindowWithClassName for PropertyGrid {
    fn class_name() -> &'static str {
        "wxPropertyGrid"
    }

    unsafe fn from_ptr(ptr: *mut ffi::wxd_Window_t) -> Self {
        Self {
            handle: WindowHandle::new(ptr),
        }
    }
}

#[cfg(feature = "xrc")]
impl crate::xrc::XrcSupport for PropertyGrid {
    unsafe fn from_xrc_ptr(ptr: *mut ffi::wxd_Window_t) -> Self {
        Self {
            handle: WindowHandle::new(ptr),
        }
    }
}

fn to_cstring(value: &str) -> Option<CString> {
    CString::new(value).ok()
}

fn read_ffi_string(reader: impl Fn(*mut c_char, usize) -> i32) -> Option<String> {
    let needed = reader(std::ptr::null_mut(), 0);
    if needed < 0 {
        return None;
    }
    let mut buffer = vec![0; needed as usize + 1];
    if reader(buffer.as_mut_ptr(), buffer.len()) < 0 {
        return None;
    }
    Some(unsafe { CStr::from_ptr(buffer.as_ptr()) }.to_string_lossy().into_owned())
}

unsafe fn append_choices(
    grid: *mut ffi::wxd_PropertyGrid_t,
    parent: *const c_char,
    label: &CString,
    name: &CString,
    choices: &[PropertyChoice],
    value: i32,
    flags: bool,
) -> Option<bool> {
    let labels: Option<Vec<CString>> = choices.iter().map(|choice| to_cstring(&choice.label)).collect();
    let labels = labels?;
    let label_ptrs: Vec<*const c_char> = labels.iter().map(|label| label.as_ptr()).collect();
    let values: Vec<i32> = choices.iter().map(|choice| choice.value).collect();

    if flags {
        Some(unsafe {
            ffi::wxd_PropertyGrid_AppendFlags(
                grid,
                parent,
                label.as_ptr(),
                name.as_ptr(),
                label_ptrs.as_ptr(),
                values.as_ptr(),
                choices.len(),
                value,
            )
        })
    } else {
        Some(unsafe {
            ffi::wxd_PropertyGrid_AppendEnum(
                grid,
                parent,
                label.as_ptr(),
                name.as_ptr(),
                label_ptrs.as_ptr(),
                values.as_ptr(),
                choices.len(),
                value,
            )
        })
    }
}
