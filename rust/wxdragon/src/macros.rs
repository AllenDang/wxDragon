// use crate::event::WxEvtHandler; // This might be unused if all macros import it directly.

//! Macros to simplify implementation of common patterns in the codebase.

/// Creates a builder pattern for widgets
///
/// This macro generates a builder struct with standard fields and methods
/// following the WxDragon convention.
///
/// # Parameters
///
/// * `name` - The name of the widget (e.g., `Button`, `CheckBox`)
/// * `parent_type` - The type of the parent parameter (e.g., `&'a dyn WxWidget`)
/// * `style_type` - The type of the style parameter (e.g., `ButtonStyle`)
/// * `fields` - Additional fields for the builder, including label if needed
/// * `build_impl` - A closure that implements the build logic
///
/// # Example
///
/// ```ignore
/// widget_builder!(
///     name: Button,
///     parent_type: &'a dyn WxWidget,
///     style_type: ButtonStyle,
///     fields: {
///         label: String = String::new()
///     },
///     build_impl: |slf| {
///         let parent_ptr = slf.parent.handle_ptr();
///         Button::new_impl(
///             parent_ptr,
///             slf.id,
///             &slf.label,
///             slf.pos,
///             slf.size,
///             slf.style.bits(),
///         )
///     }
/// );
/// ```
#[macro_export]
macro_rules! widget_builder {
    (
        name: $name:ident,
        parent_type: &'a dyn WxWidget,
        style_type: $style_type:ty,
        fields: {
            $(
                $field_name:ident: $field_type:ty $(= $field_default:expr)?
            ),*
        },
        build_impl: |$self_param:ident| $build_impl:block
    ) => {
        paste::paste! {
            #[derive(Clone)]
            pub struct [<$name Builder>]<'a> {
                parent: &'a dyn WxWidget,
                id: Id,
                pos: Point,
                size: Size,
                style: $style_type,
                $(
                    $field_name: $field_type,
                )*
            }

            impl<'a> [<$name Builder>]<'a> {
                pub fn new(parent: &'a dyn WxWidget) -> Self {
                    Self {
                        parent,
                        id: $crate::id::ID_ANY as Id,
                        pos: $crate::geometry::DEFAULT_POSITION,
                        size: $crate::geometry::DEFAULT_SIZE,
                        style: <$style_type>::default(),
                        $(
                            $field_name: $crate::__widget_builder_default!($($field_default)?),
                        )*
                    }
                }

                /// Sets the window identifier.
                pub fn with_id(mut self, id: Id) -> Self {
                    self.id = id;
                    self
                }

                /// Sets the position.
                pub fn with_pos(mut self, pos: Point) -> Self {
                    self.pos = pos;
                    self
                }

                /// Sets the size.
                pub fn with_size(mut self, size: Size) -> Self {
                    self.size = size;
                    self
                }

                /// Sets the window style flags.
                pub fn with_style(mut self, style: $style_type) -> Self {
                    self.style = style;
                    self
                }

                $(
                    $crate::__widget_builder_field_method!($field_name: $field_type);
                )*

                /// Builds the widget.
                pub fn build(self) -> $name {
                    let build_fn = |$self_param: [<$name Builder>]<'a>| $build_impl;
                    build_fn(self)
                }
            }
        }
    };
}

/// Implements common widget traits.
///
/// This macro generates implementations for:
/// - WxWidget trait
/// - Deref/DerefMut to Window
/// - WxEvtHandler trait
/// - Drop implementation with empty body (for child widgets)
///
/// # Parameters
///
/// * `name` - The name of the widget struct
/// * `field` - The name of the field within the struct that is a Window
///
/// # Example
///
/// ```ignore
/// implement_widget_traits!(MyWidget, window_field);
/// ```
#[macro_export]
macro_rules! implement_widget_traits {
    ($widget_name:ident, $window_field:ident) => {
        // Don't import WxEvtHandler here as modules might already have it imported
        // use $crate::event::WxEvtHandler;

        impl $crate::window::WxWidget for $widget_name {
            fn handle_ptr(&self) -> *mut wxdragon_sys::wxd_Window_t {
                self.$window_field.handle_ptr()
            }
        }

        impl std::ops::Deref for $widget_name {
            type Target = $crate::window::Window;
            fn deref(&self) -> &Self::Target {
                &self.$window_field
            }
        }

        impl std::ops::DerefMut for $widget_name {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.$window_field
            }
        }

        impl $crate::event::WxEvtHandler for $widget_name {
            unsafe fn get_event_handler_ptr(&self) -> *mut wxdragon_sys::wxd_EvtHandler_t {
                self.$window_field.get_event_handler_ptr()
            }
        }

        impl Drop for $widget_name {
            fn drop(&mut self) {
                // Child windows are managed by their parent in wxWidgets,
                // so no explicit cleanup is needed here.
            }
        }
    };
}

/// Implements common widget traits with a custom target type for Deref
///
/// This macro generates implementations for:
/// - WxWidget trait
/// - Deref/DerefMut to the specified target type
/// - WxEvtHandler trait
/// - Drop implementation with empty body (for child widgets)
/// - Clone trait (clones the target field; other fields must be Copy or have manual Clone)
/// - MenuEvents trait (for context menus and menu handling)
/// - WindowEvents trait (for standard window events)
///
/// # Parameters
///
/// * `name` - The name of the widget struct
/// * `field` - The name of the field within the struct that implements WxWidget
/// * `target_type` - The target type for Deref/DerefMut implementations
///
/// # Note on Clone
///
/// The generated Clone implementation only explicitly clones the target field.
/// Other fields in the struct must either:
/// - Be Copy types (like raw pointers, PhantomData)
/// - Or the struct should manually implement Clone
///
/// # Example
///
/// ```ignore
/// // For a widget that wraps a Window:
/// implement_widget_traits_with_target!(MyWidget, window, Window);
///
/// // For a widget that wraps another widget type:
/// implement_widget_traits_with_target!(MyCompositeWidget, inner_widget, OtherWidgetType);
/// ```
#[macro_export]
macro_rules! implement_widget_traits_with_target {
    ($widget_name:ident, $window_field:ident, $target_ty:ty) => {
        // Don't import WxEvtHandler here as modules might already have it imported
        // use $crate::event::WxEvtHandler;

        impl $crate::window::WxWidget for $widget_name {
            fn handle_ptr(&self) -> *mut wxdragon_sys::wxd_Window_t {
                self.$window_field.handle_ptr()
            }
        }

        impl std::ops::Deref for $widget_name {
            type Target = $target_ty;
            fn deref(&self) -> &Self::Target {
                &self.$window_field
            }
        }

        impl std::ops::DerefMut for $widget_name {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.$window_field
            }
        }

        impl $crate::event::WxEvtHandler for $widget_name {
            unsafe fn get_event_handler_ptr(&self) -> *mut wxdragon_sys::wxd_EvtHandler_t {
                self.$window_field.get_event_handler_ptr()
            }
        }

        impl Drop for $widget_name {
            fn drop(&mut self) {
                // Child windows are managed by their parent in wxWidgets,
                // so no explicit cleanup is needed here.
            }
        }

        // Auto-implement common event traits that all Window-based widgets support
        impl $crate::event::MenuEvents for $widget_name {}
        impl $crate::event::WindowEvents for $widget_name {}
    };
}

/// Defines a style enum for a widget with standard implementations
///
/// This macro creates a style enum with variants from wxWidgets constants
/// and implements common traits like Default, BitOr, BitOrAssign, and bits().
///
/// # Parameters
///
/// * `name` - The name of the style enum (e.g., `ButtonStyle`, `RadioBoxStyle`)
/// * `doc` - Documentation for the enum (e.g., "Style flags for Button")
/// * `variants` - A list of variants with their names, wxWidgets constants, and docs
/// * `default_variant` - The name of the variant to use as Default
///
/// # Example
///
/// ```ignore
/// widget_style_enum!(
///     name: ButtonStyle,
///     doc: "Style flags for Button.",
///     variants: {
///         Default: 0, "Default style (no specific alignment).",
///         Left: ffi::WXD_BU_LEFT, "Align label to the left.",
///         Right: ffi::WXD_BU_RIGHT, "Align label to the right."
///     },
///     default_variant: Default
/// );
/// ```
#[macro_export]
macro_rules! widget_style_enum {
    (
        name: $name:ident,
        doc: $doc:expr,
        variants: {
            $(
                $variant:ident: $value:expr, $variant_doc:expr
            ),+
        },
        default_variant: $default:ident
    ) => {
        bitflags::bitflags! {
            #[doc = $doc]
            #[doc = "\n\nThese flags can be combined using the bitwise OR operator (`|`)."]
            #[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
            pub struct $name : i64 {
                $(
                    #[doc = $variant_doc]
                    const $variant = $value;
                )+
            }
        }

        impl Default for $name {
            fn default() -> Self {
                $name::$default
            }
        }
    };
}

// Helper macro for generating appropriate setter methods based on field type
#[doc(hidden)]
#[macro_export]
macro_rules! __widget_builder_field_method {
    // Special case for String fields - allow &str parameters and convert to String
    // This applies to any field with type String, regardless of name
    ($field_name:ident: String) => {
        paste::paste! {
            /// Sets the $field_name.
            #[allow(non_snake_case)]
            pub fn [<with_ $field_name>](mut self, $field_name: &str) -> Self {
                self.$field_name = $field_name.to_string();
                self
            }
        }
    };
    // Default case for all other fields - generate a method that takes the same type as the field
    ($field_name:ident: $field_type:ty) => {
        paste::paste! {
            /// Sets the $field_name.
            #[allow(non_snake_case)]
            pub fn [<with_ $field_name>](mut self, $field_name: $field_type) -> Self {
                self.$field_name = $field_name;
                self
            }
        }
    };
}

// Helper macro for default values in fields
#[doc(hidden)]
#[macro_export]
macro_rules! __widget_builder_default {
    () => {
        Default::default()
    };
    ($expr:expr) => {
        $expr
    };
}

/// Implements XrcSupport trait for a widget with custom field initialization
///
/// This macro generates the XrcSupport implementation for widgets, handling
/// different field patterns that widgets may have.
///
/// # Parameters
///
/// * `widget_name` - The name of the widget struct
/// * `fields` - Block containing field initialization expressions
///
/// # Example
///
/// ```ignore
/// // For widgets with just a window field:
/// impl_xrc_support!(TextCtrl, {
///     window
/// });
///
/// // For widgets with window and parent_ptr fields:
/// impl_xrc_support!(Button, {
///     window,
///     parent_ptr: std::ptr::null_mut()
/// });
/// ```
#[macro_export]
#[cfg(feature = "xrc")]
macro_rules! impl_xrc_support {
    // Handle the simple case where only window field is specified
    ($widget_name:ident, { window }) => {
        #[cfg(feature = "xrc")]
        impl $crate::xrc::XrcSupport for $widget_name {
            unsafe fn from_xrc_ptr(ptr: *mut wxdragon_sys::wxd_Window_t) -> Self {
                let window = unsafe { $crate::window::Window::from_ptr(ptr) };
                Self { window }
            }
        }
    };
    // Handle the more complex case with additional fields
    ($widget_name:ident, { window, $($field_name:ident: $field_value:expr),* $(,)? }) => {
        #[cfg(feature = "xrc")]
        impl $crate::xrc::XrcSupport for $widget_name {
            unsafe fn from_xrc_ptr(ptr: *mut wxdragon_sys::wxd_Window_t) -> Self {
                let window = unsafe { $crate::window::Window::from_ptr(ptr) };
                Self {
                    window,
                    $(
                        $field_name: $field_value,
                    )*
                }
            }
        }
    };
}

// Empty version when xrc feature is disabled
#[macro_export]
#[cfg(not(feature = "xrc"))]
macro_rules! impl_xrc_support {
    // Accept the same patterns but generate no code
    ($widget_name:ident, { window }) => {};
    ($widget_name:ident, { window, $($field_name:ident: $field_value:expr),* $(,)? }) => {};
}

/// Creates a custom widget with builder pattern based on Panel
///
/// This macro simplifies custom widget development by:
/// - Creating a Panel-based widget struct
/// - Generating a builder pattern with customizable fields
/// - Automatically implementing WxWidget, Deref, DerefMut, and WxEvtHandler traits
/// - Handling memory management properly
///
/// # Parameters
///
/// * `name` - The name of the custom widget (e.g., `AniFillButton`)
/// * `fields` - Additional fields for configuration with default values
/// * `setup_impl` - Implementation block for widget setup (receives config and panel)
///
/// # Example
///
/// ```ignore
/// custom_widget!(
///     name: AniFillButton,
///     fields: {
///         text: String = "Button".to_string(),
///         border_color: Colour = Colour::new(100, 100, 100, 255),
///         border_width: i32 = 2
///     },
///     setup_impl: |config, panel| {
///         // Your setup code here
///         // config contains all the field values
///         // panel is the underlying Panel widget
///     }
/// );
/// ```
#[macro_export]
macro_rules! custom_widget {
    (
        name: $name:ident,
        fields: {
            $(
                $field_name:ident: $field_type:ty = $field_default:expr
            ),* $(,)?
        },
        setup_impl: |$config_param:ident, $panel_param:ident| $setup_impl:block
    ) => {
        paste::paste! {
            /// Configuration struct for the custom widget
            #[derive(Debug, Clone)]
            pub struct [<$name Config>] {
                $(
                    pub $field_name: $field_type,
                )*
            }

            impl Default for [<$name Config>] {
                fn default() -> Self {
                    Self {
                        $(
                            $field_name: $field_default,
                        )*
                    }
                }
            }

            /// Builder for the custom widget
            #[derive(Clone)]
            pub struct [<$name Builder>]<'a> {
                parent: &'a dyn $crate::window::WxWidget,
                size: $crate::geometry::Size,
                $(
                    $field_name: $field_type,
                )*
            }

            impl<'a> [<$name Builder>]<'a> {
                /// Creates a new builder
                pub fn new(parent: &'a dyn $crate::window::WxWidget) -> Self {
                    Self {
                        parent,
                        size: $crate::geometry::DEFAULT_SIZE,
                        $(
                            $field_name: $field_default,
                        )*
                    }
                }

                /// Sets the widget size
                pub fn with_size(mut self, size: $crate::geometry::Size) -> Self {
                    self.size = size;
                    self
                }

                $(
                    paste::paste! {
                        /// Sets the field value
                        pub fn [<with_ $field_name>](mut self, $field_name: $field_type) -> Self {
                            self.$field_name = $field_name;
                            self
                        }
                    }
                )*

                /// Builds the custom widget
                pub fn build(self) -> $name {
                    let panel = $crate::widgets::panel::Panel::builder(self.parent)
                        .with_size(self.size)
                        .build();

                    let config = [<$name Config>] {
                        $(
                            $field_name: self.$field_name,
                        )*
                    };

                    let widget = $name {
                        panel: panel.clone(),
                        config: config.clone(),
                    };

                    // Call the setup implementation
                    let setup_fn = |$config_param: [<$name Config>], $panel_param: $crate::widgets::panel::Panel| $setup_impl;
                    setup_fn(config, panel);

                    widget
                }
            }

            /// The custom widget struct
            pub struct $name {
                panel: $crate::widgets::panel::Panel,
                config: [<$name Config>],
            }

            impl $name {
                /// Returns a reference to the configuration
                pub fn config(&self) -> &[<$name Config>] {
                    &self.config
                }

                /// Creates a new builder for this widget type
                pub fn builder(parent: &dyn $crate::window::WxWidget) -> [<$name Builder>]<'_> {
                    [<$name Builder>]::new(parent)
                }
            }

            // Implement the necessary traits
            impl $crate::window::WxWidget for $name {
                fn handle_ptr(&self) -> *mut $crate::ffi::wxd_Window_t {
                    self.panel.handle_ptr()
                }
            }

            impl std::ops::Deref for $name {
                type Target = $crate::widgets::panel::Panel;
                fn deref(&self) -> &Self::Target {
                    &self.panel
                }
            }

            impl std::ops::DerefMut for $name {
                fn deref_mut(&mut self) -> &mut Self::Target {
                    &mut self.panel
                }
            }

            impl $crate::event::WxEvtHandler for $name {
                unsafe fn get_event_handler_ptr(&self) -> *mut $crate::ffi::wxd_EvtHandler_t {
                    self.panel.get_event_handler_ptr()
                }
            }

            // Let the underlying Panel handle memory management
        }
    };
}

/// Implements widget casting capability for a widget type.
///
/// This macro generates the FromWindowWithClassName trait implementation
/// for a widget, enabling it to be safely cast from a generic Window
/// using wxWidgets' built-in RTTI system.
///
/// # Parameters
/// * `widget` - The widget type name (e.g., StaticText, Button)
/// * `class_name` - The corresponding wxWidgets class name (e.g., "wxStaticText")
/// * `construction` - How to construct the widget from a Window:
///   - `{ window }` - For widgets that only need a window field (like StaticText)
///   - `composition` - For widgets that use new_from_composition (like Button)
///
/// # Examples
/// ```ignore
/// // For StaticText (simple window field pattern):
/// impl_widget_cast!(StaticText, "wxStaticText", { window });
///
/// // For Button (composition pattern):
/// impl_widget_cast!(Button, "wxButton", composition);
///
/// // Usage:
/// if let Some(text) = window.as_widget::<StaticText>() {
///     text.set_label("Hello");
/// }
/// ```
#[macro_export]
macro_rules! impl_widget_cast {
    // Pattern for widgets that use new_from_composition
    ($widget:ident, $class_name:literal, composition) => {
        impl $crate::window::FromWindowWithClassName for $widget {
            fn class_name() -> &'static str {
                $class_name
            }

            unsafe fn from_ptr(ptr: *mut wxdragon_sys::wxd_Window_t) -> Self {
                let window = $crate::window::Window::from_ptr(ptr);
                Self::new_from_composition(window, std::ptr::null_mut())
            }
        }
    };

    // Pattern for widgets that only need { window }
    ($widget:ident, $class_name:literal, { $field:ident }) => {
        impl $crate::window::FromWindowWithClassName for $widget {
            fn class_name() -> &'static str {
                $class_name
            }

            unsafe fn from_ptr(ptr: *mut wxdragon_sys::wxd_Window_t) -> Self {
                let $field = $crate::window::Window::from_ptr(ptr);
                Self { $field }
            }
        }
    };
}
