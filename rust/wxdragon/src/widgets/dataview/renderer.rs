//! DataViewRenderer implementation.

use super::{DataViewAlign, DataViewCellMode, Variant, VariantType};
use std::ffi::CString;
use wxdragon_sys as ffi;

// Type aliases for custom renderer callbacks to reduce complexity
type GetSizeCallback = Box<dyn Fn(&super::Variant, crate::geometry::Size) -> crate::geometry::Size + 'static>;
// Simple render callback that receives the variant directly
type RenderCallback = Box<dyn Fn(crate::geometry::Rect, &RenderContext, i32, &super::Variant) -> bool + 'static>;
type SetValueCallback = Box<dyn Fn(&super::Variant) -> bool + 'static>;
type GetValueCallback = Box<dyn Fn() -> Option<super::Variant> + 'static>;
type HasEditorCallback = Box<dyn Fn() -> bool + 'static>;
type ActivateCellCallback = Box<dyn Fn(crate::geometry::Rect, i32) -> bool + 'static>;
type CreateEditorCallback =
    Box<dyn Fn(&dyn crate::WxWidget, crate::geometry::Rect, &super::Variant) -> Option<Box<dyn crate::WxWidget>> + 'static>;
type GetValueFromEditorCallback = Box<dyn Fn(&dyn crate::WxWidget) -> Option<super::Variant> + 'static>;

/// Holds the callbacks for a custom DataView renderer
#[repr(C)]
struct CustomRendererCallbacks {
    get_size: Option<GetSizeCallback>,
    render: Option<RenderCallback>,
    set_value: Option<SetValueCallback>,
    get_value: Option<GetValueCallback>,
    has_editor: Option<HasEditorCallback>,
    activate_cell: Option<ActivateCellCallback>,
    create_editor: Option<CreateEditorCallback>,
    get_value_from_editor: Option<GetValueFromEditorCallback>,
    // Store the current value internally in the renderer
    current_value: std::cell::RefCell<super::Variant>,
}

/// Base trait for DataView renderers.
///
/// DataViewRenderer objects are responsible for drawing the data in the columns
/// of a DataViewCtrl and potentially handling user interaction.
pub trait DataViewRenderer {
    /// Gets the raw pointer to the native wxDataViewRenderer.
    fn as_raw(&self) -> *mut ffi::wxd_DataViewRenderer_t;
}

/// A text renderer for DataViewCtrl columns.
///
/// DataViewTextRenderer displays text data in a column.
pub struct DataViewTextRenderer {
    handle: *mut ffi::wxd_DataViewRenderer_t,
}

impl DataViewTextRenderer {
    /// Creates a new text renderer.
    ///
    /// # Parameters
    ///
    /// * `variant_type` - The type of data this renderer can display
    /// * `mode` - The cell mode (inert, activatable, or editable)
    /// * `align` - The text alignment
    pub fn new(variant_type: VariantType, mode: DataViewCellMode, align: DataViewAlign) -> Self {
        let variant_type_str = variant_type.as_str();
        let variant_type_cstr = CString::new(variant_type_str).unwrap();
        let handle = unsafe { ffi::wxd_DataViewTextRenderer_Create(variant_type_cstr.as_ptr(), mode.bits(), align.bits()) };
        Self { handle }
    }
}

impl DataViewRenderer for DataViewTextRenderer {
    fn as_raw(&self) -> *mut ffi::wxd_DataViewRenderer_t {
        self.handle
    }
}

/// A toggle (checkbox) renderer for DataViewCtrl columns.
///
/// DataViewToggleRenderer displays boolean data as a checkbox.
pub struct DataViewToggleRenderer {
    handle: *mut ffi::wxd_DataViewRenderer_t,
}

impl DataViewToggleRenderer {
    /// Creates a new toggle renderer.
    ///
    /// # Parameters
    ///
    /// * `variant_type` - The type of data this renderer can display (typically Bool)
    /// * `mode` - The cell mode (typically activatable for toggles)
    /// * `align` - The alignment of the checkbox
    pub fn new(variant_type: VariantType, mode: DataViewCellMode, align: DataViewAlign) -> Self {
        let variant_type_str = variant_type.as_str();
        let variant_type_cstr = CString::new(variant_type_str).unwrap();
        let handle = unsafe { ffi::wxd_DataViewToggleRenderer_Create(variant_type_cstr.as_ptr(), mode.bits(), align.bits()) };
        Self { handle }
    }
}

impl DataViewRenderer for DataViewToggleRenderer {
    fn as_raw(&self) -> *mut ffi::wxd_DataViewRenderer_t {
        self.handle
    }
}

/// A progress bar renderer for DataViewCtrl columns.
///
/// DataViewProgressRenderer displays numeric data as a progress bar.
pub struct DataViewProgressRenderer {
    handle: *mut ffi::wxd_DataViewRenderer_t,
}

impl DataViewProgressRenderer {
    /// Creates a new progress renderer.
    ///
    /// # Parameters
    ///
    /// * `variant_type` - The type of data this renderer can display (typically Int32)
    /// * `mode` - The cell mode (typically inert for progress bars)
    pub fn new(variant_type: VariantType, mode: DataViewCellMode) -> Self {
        let variant_type_str = variant_type.as_str();
        let variant_type_cstr = CString::new(variant_type_str).unwrap();
        let handle = unsafe {
            ffi::wxd_DataViewProgressRenderer_Create(
                variant_type_cstr.as_ptr(),
                mode.bits(),
                0, // align - ignored for progress renderer
            )
        };
        Self { handle }
    }
}

impl DataViewRenderer for DataViewProgressRenderer {
    fn as_raw(&self) -> *mut ffi::wxd_DataViewRenderer_t {
        self.handle
    }
}

/// An icon-text renderer for DataViewCtrl columns.
///
/// DataViewIconTextRenderer displays an icon followed by text.
pub struct DataViewIconTextRenderer {
    handle: *mut ffi::wxd_DataViewRenderer_t,
}

impl DataViewIconTextRenderer {
    /// Creates a new icon-text renderer.
    ///
    /// # Parameters
    ///
    /// * `variant_type` - The type of data this renderer can display (e.g., String for text part)
    /// * `mode` - The cell mode
    /// * `align` - The alignment
    pub fn new(variant_type: VariantType, mode: DataViewCellMode, align: DataViewAlign) -> Self {
        let variant_type_str = variant_type.as_str();
        let variant_type_cstr = CString::new(variant_type_str).unwrap_or_else(|_| CString::new("string").unwrap());
        let handle = unsafe { ffi::wxd_DataViewIconTextRenderer_Create(variant_type_cstr.as_ptr(), mode.bits(), align.bits()) };
        Self { handle }
    }
}

impl DataViewRenderer for DataViewIconTextRenderer {
    fn as_raw(&self) -> *mut ffi::wxd_DataViewRenderer_t {
        self.handle
    }
}

/// A bitmap renderer for DataViewCtrl columns.
///
/// DataViewBitmapRenderer displays a bitmap in a cell.
pub struct DataViewBitmapRenderer {
    handle: *mut ffi::wxd_DataViewRenderer_t,
}

impl DataViewBitmapRenderer {
    /// Creates a new bitmap renderer.
    ///
    /// # Parameters
    ///
    /// * `mode` - The cell mode
    /// * `align` - The alignment
    pub fn new(mode: DataViewCellMode, align: DataViewAlign) -> Self {
        // Bitmap renderer uses the standard wxVariant type name for bitmaps
        let variant_type_cstr = CString::new("wxBitmap").unwrap();
        let handle = unsafe { ffi::wxd_DataViewBitmapRenderer_Create(variant_type_cstr.as_ptr(), mode.bits(), align.bits()) };
        Self { handle }
    }
}

impl DataViewRenderer for DataViewBitmapRenderer {
    fn as_raw(&self) -> *mut ffi::wxd_DataViewRenderer_t {
        self.handle
    }
}

/// A date renderer for DataViewCtrl columns.
///
/// DataViewDateRenderer displays date values.
pub struct DataViewDateRenderer {
    handle: *mut ffi::wxd_DataViewRenderer_t,
}

impl DataViewDateRenderer {
    /// Creates a new date renderer.
    ///
    /// # Parameters
    ///
    /// * `variant_type` - The type of data this renderer can display (typically DateTime)
    /// * `mode` - The cell mode
    /// * `align` - The alignment
    pub fn new(variant_type: VariantType, mode: DataViewCellMode, align: DataViewAlign) -> Self {
        let variant_type_str = variant_type.as_str();
        let variant_type_cstr = CString::new(variant_type_str).unwrap();
        let handle = unsafe { ffi::wxd_DataViewDateRenderer_Create(variant_type_cstr.as_ptr(), mode.bits(), align.bits()) };
        Self { handle }
    }
}

impl DataViewRenderer for DataViewDateRenderer {
    fn as_raw(&self) -> *mut ffi::wxd_DataViewRenderer_t {
        self.handle
    }
}

/// A spin renderer for DataViewCtrl columns.
///
/// DataViewSpinRenderer displays a spin control for numeric data.
pub struct DataViewSpinRenderer {
    handle: *mut ffi::wxd_DataViewRenderer_t,
}

impl DataViewSpinRenderer {
    /// Creates a new spin renderer.
    ///
    /// # Parameters
    ///
    /// * `variant_type` - The type of data this renderer can display (typically Int32)
    /// * `mode` - The cell mode
    /// * `align` - The alignment
    /// * `min` - Minimum value
    /// * `max` - Maximum value
    /// * `inc` - Increment value
    pub fn new(variant_type: VariantType, mode: DataViewCellMode, align: DataViewAlign, min: i32, max: i32, inc: i32) -> Self {
        let variant_type_str = variant_type.as_str();
        let variant_type_cstr = CString::new(variant_type_str).unwrap();
        let handle =
            unsafe { ffi::wxd_DataViewSpinRenderer_Create(variant_type_cstr.as_ptr(), mode.bits(), align.bits(), min, max, inc) };
        Self { handle }
    }
}

impl DataViewRenderer for DataViewSpinRenderer {
    fn as_raw(&self) -> *mut ffi::wxd_DataViewRenderer_t {
        self.handle
    }
}

/// A choice renderer for DataViewCtrl columns.
///
/// DataViewChoiceRenderer displays a dropdown with choices.
pub struct DataViewChoiceRenderer {
    handle: *mut ffi::wxd_DataViewRenderer_t,
}

impl DataViewChoiceRenderer {
    /// Creates a new choice renderer.
    ///
    /// # Parameters
    ///
    /// * `variant_type` - The type of data this renderer can display (typically String)
    /// * `choices` - A list of choices to display in the dropdown
    /// * `mode` - The cell mode
    /// * `align` - The alignment
    pub fn new(variant_type: VariantType, choices: &[&str], mode: DataViewCellMode, align: DataViewAlign) -> Self {
        // Convert choices to a comma-separated string
        let choices_str = choices.join(",");
        let choices_cstr = CString::new(choices_str).unwrap();

        let variant_type_str = variant_type.as_str();
        let variant_type_cstr = CString::new(variant_type_str).unwrap();

        let handle = unsafe {
            ffi::wxd_DataViewChoiceRenderer_Create(variant_type_cstr.as_ptr(), choices_cstr.as_ptr(), mode.bits(), align.bits())
        };

        Self { handle }
    }
}

impl DataViewRenderer for DataViewChoiceRenderer {
    fn as_raw(&self) -> *mut ffi::wxd_DataViewRenderer_t {
        self.handle
    }
}

/// A check-icon-text renderer for DataViewCtrl columns.
///
/// DataViewCheckIconTextRenderer displays a checkbox, an icon, and text.
pub struct DataViewCheckIconTextRenderer {
    handle: *mut ffi::wxd_DataViewRenderer_t,
}

impl DataViewCheckIconTextRenderer {
    /// Creates a new check-icon-text renderer.
    ///
    /// # Parameters
    ///
    /// * `mode` - The cell mode
    /// * `align` - The alignment
    pub fn new(mode: DataViewCellMode, align: DataViewAlign) -> Self {
        // This renderer uses a special variant type
        let variant_type_cstr = CString::new("wxDataViewCheckIconText").unwrap();
        let handle =
            unsafe { ffi::wxd_DataViewCheckIconTextRenderer_Create(variant_type_cstr.as_ptr(), mode.bits(), align.bits()) };
        Self { handle }
    }
}

impl DataViewRenderer for DataViewCheckIconTextRenderer {
    fn as_raw(&self) -> *mut ffi::wxd_DataViewRenderer_t {
        self.handle
    }
}

/// A custom renderer for DataView controls that allows completely custom drawing and behavior.
///
/// This renderer provides a flexible way to display data in DataView columns with custom
/// appearance and interaction. You can override various aspects like sizing, rendering,
/// editing, and cell activation.
///
/// # Features
///
/// - **Reusable**: Same renderer instance can be used across multiple columns and DataViews
/// - **Thread-safe**: Safe to create from any thread
/// - **Memory safe**: Automatic cleanup when renderer is destroyed
/// - **Flexible**: Support for custom sizing, rendering, editing, and activation
/// - **Type-safe downcasting**: Use `WxWidgetDowncast` trait to safely cast editor widgets
///
/// # Complete Example: Editable Text Renderer
///
/// ```no_run
/// # use wxdragon::prelude::*;
/// # use wxdragon::DeviceContext;
/// # use wxdragon::DataViewColumnFlags;
/// use wxdragon::window::WxWidgetDowncast;
/// use wxdragon::widgets::TextCtrl;
/// use wxdragon::widgets::dataview::{DataViewCustomRenderer, VariantType, DataViewCellMode, DataViewAlign, Variant};
///
/// // Create a custom text renderer with editing support
/// let text_renderer = DataViewCustomRenderer::builder()
///     .variant_type(VariantType::String)
///     .mode(DataViewCellMode::Editable)
///     .align(DataViewAlign::Left)
///     .with_render(|rect, ctx, _state, variant| {
///         if let Some(text) = variant.get_string() {
///             ctx.draw_text(&text, rect.x + 2, rect.y + 2);
///             true
///         } else {
///             false
///         }
///     })
///     .with_has_editor(|| true)
///     .with_create_editor(|parent, rect, variant| {
///         let initial_value = variant.get_string().unwrap_or_default();
///         
///         let text_ctrl = TextCtrl::builder(parent)
///             .with_pos(rect.position())
///             .with_size(rect.size())
///             .with_value(&initial_value)
///             .build();
///         
///         Some(Box::new(text_ctrl))
///     })
///     .with_get_value_from_editor(|editor| {
///         // Use downcasting to get the specific widget type
///         if let Some(text_ctrl) = editor.downcast_ref::<TextCtrl>() {
///             let value = text_ctrl.get_value();
///             Some(Variant::from_string(value))
///         } else {
///             None
///         }
///     })
///     .build();
///
/// // Use in multiple columns
/// let col1 = DataViewColumn::new("Name", &text_renderer, 0, 120, DataViewAlign::Left, DataViewColumnFlags::Resizable);
/// let col2 = DataViewColumn::new("Description", &text_renderer, 1, 200, DataViewAlign::Left, DataViewColumnFlags::Resizable);
/// ```
///
/// # Examples
///
/// ```no_run
/// # use wxdragon::prelude::*;
/// // Create a progress bar renderer
/// let progress_renderer = DataViewCustomRenderer::builder()
///     .variant_type(VariantType::Int32)
///     .mode(DataViewCellMode::Inert)
///     .with_render(|rect, ctx, _state, variant| {
///         if let Some(progress) = variant.get_i32() {
///             // Here you would draw the progress bar using the context and rect.
///             // For example: ctx.draw_progress_bar(rect, progress);
///             true
///         } else {
///             false
///         }
///     })
///     .build();
///
/// // Use in multiple columns
/// let col1 = DataViewColumn::new("Progress 1", &progress_renderer, 1, 120, DataViewAlign::Left, DataViewColumnFlags::Resizable);
/// let col2 = DataViewColumn::new("Progress 2", &progress_renderer, 3, 120, DataViewAlign::Left, DataViewColumnFlags::Resizable);
/// ```
pub struct DataViewCustomRenderer {
    raw: *mut ffi::wxd_DataViewRenderer_t,
}

impl DataViewRenderer for DataViewCustomRenderer {
    fn as_raw(&self) -> *mut ffi::wxd_DataViewRenderer_t {
        self.raw
    }
}

impl DataViewCustomRenderer {
    /// Creates a builder for constructing a custom renderer.
    pub fn builder() -> DataViewCustomRendererBuilder {
        DataViewCustomRendererBuilder::new()
    }
}

/// Builder for creating custom data view renderers.
pub struct DataViewCustomRendererBuilder {
    variant_type: VariantType,
    mode: DataViewCellMode,
    align: DataViewAlign,
    get_size: Option<GetSizeCallback>,
    render: Option<RenderCallback>,
    has_editor: Option<HasEditorCallback>,
    create_editor: Option<CreateEditorCallback>,
    get_value_from_editor: Option<GetValueFromEditorCallback>,
    activate_cell: Option<ActivateCellCallback>,
}

impl DataViewCustomRendererBuilder {
    fn new() -> Self {
        Self {
            variant_type: VariantType::String,
            mode: DataViewCellMode::Inert,
            align: DataViewAlign::Left,
            get_size: None,
            render: None,
            has_editor: None,
            create_editor: None,
            get_value_from_editor: None,
            activate_cell: None,
        }
    }

    /// Sets the variant type for this renderer.
    pub fn variant_type(mut self, variant_type: VariantType) -> Self {
        self.variant_type = variant_type;
        self
    }

    /// Sets the cell mode (inert, activatable, or editable).
    pub fn mode(mut self, mode: DataViewCellMode) -> Self {
        self.mode = mode;
        self
    }

    /// Sets the text alignment.
    pub fn align(mut self, align: DataViewAlign) -> Self {
        self.align = align;
        self
    }

    /// Sets the callback for determining the size needed for rendering.
    ///
    /// The callback receives the variant value and the default cell size,
    /// allowing for more intelligent sizing based on content.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use wxdragon::prelude::*;
    /// use wxdragon::widgets::dataview::{DataViewCustomRenderer, Variant};
    /// use wxdragon::geometry::Size;
    ///
    /// // Provide a plain function (no captures) and pass it to with_get_size
    /// fn size_for_variant(variant: &Variant, default_size: Size) -> Size {
    ///     if let Some(s) = variant.get_string() {
    ///         // Estimate width based on text length
    ///         let char_width = 8; // approximate character width
    ///         Size::new(s.len() as i32 * char_width, default_size.height)
    ///     } else if let Some(_i) = variant.get_i32() {
    ///         // Fixed width for progress bar
    ///         let progress_width = 100;
    ///         Size::new(progress_width, default_size.height)
    ///     } else {
    ///         // Use the default size for other types
    ///         default_size
    ///     }
    /// }
    ///
    /// let _renderer_builder = DataViewCustomRenderer::builder()
    ///     .with_get_size(size_for_variant);
    /// ```
    pub fn with_get_size<F>(mut self, callback: F) -> Self
    where
        F: Fn(&super::Variant, crate::geometry::Size) -> crate::geometry::Size + 'static,
    {
        self.get_size = Some(Box::new(callback));
        self
    }

    /// Sets the render callback. The callback receives the current variant value.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use wxdragon::prelude::*;
    /// use wxdragon::widgets::dataview::{DataViewCustomRenderer, Variant};
    /// use wxdragon::widgets::dataview::renderer::RenderContext;
    ///
    /// // Provide a plain function (no captures) and pass it to with_render
    /// fn render_variant(
    ///     rect: wxdragon::geometry::Rect,
    ///     ctx: &RenderContext,
    ///     state: i32,
    ///     variant: &Variant,
    /// ) -> bool {
    ///     if let Some(text) = variant.get_string() {
    ///         // Render string
    ///         ctx.draw_text(&text, rect.x, rect.y);
    ///         true
    ///     } else if let Some(progress) = variant.get_i32() {
    ///         // Render integer as a simple progress bar using basic DC ops
    ///         // Frame
    ///         ctx.set_pen(colours::BLACK, 1, PenStyle::Solid);
    ///         ctx.set_brush(colours::LIGHT_GRAY, BrushStyle::Solid);
    ///         ctx.draw_rectangle(rect.x, rect.y, rect.width, rect.height);
    ///         // Fill according to progress (0..=100)
    ///         let fill_w = (rect.width * progress / 100).clamp(0, rect.width);
    ///         ctx.set_brush(colours::BLUE, BrushStyle::Solid);
    ///         ctx.draw_rectangle(rect.x + 1, rect.y + 1, (fill_w - 2).max(0), rect.height - 2);
    ///         true
    ///     } else if let Some(checked) = variant.get_bool() {
    ///         // Render boolean as a minimal checkbox: draw a box and an X when checked
    ///         let box_size = rect.height.min(16);
    ///         ctx.set_pen(colours::BLACK, 1, PenStyle::Solid);
    ///         ctx.set_brush(colours::WHITE, BrushStyle::Solid);
    ///         ctx.draw_rectangle(rect.x, rect.y, box_size, box_size);
    ///         if checked {
    ///             ctx.draw_text("X", rect.x + 3, rect.y + 1);
    ///         }
    ///         true
    ///     } else {
    ///         // Handle other variant types or ignore
    ///         false
    ///     }
    /// }
    ///
    /// let _renderer_builder = DataViewCustomRenderer::builder()
    ///     .with_render(render_variant);
    /// ```
    pub fn with_render<F>(mut self, callback: F) -> Self
    where
        F: Fn(crate::geometry::Rect, &RenderContext, i32, &super::Variant) -> bool + 'static,
    {
        self.render = Some(Box::new(callback));
        self
    }

    /// Sets the callback for determining if the cell has an editor.
    pub fn with_has_editor<F>(mut self, callback: F) -> Self
    where
        F: Fn() -> bool + 'static,
    {
        self.has_editor = Some(Box::new(callback));
        self
    }

    /// Sets the callback for creating an editor control.
    ///
    /// The callback receives the parent widget, the cell rectangle, and the current value.
    /// It should return a boxed widget that will be used for editing the cell value.
    ///
    /// # Example
    /// ```no_run
    /// # use wxdragon::prelude::*;
    /// use wxdragon::widgets::TextCtrl;
    /// use wxdragon::widgets::dataview::{DataViewCustomRenderer, Variant};
    ///
    /// // Provide a plain function (no captures) and pass it to with_create_editor
    /// fn create_text_editor(
    ///     parent: &dyn wxdragon::WxWidget,
    ///     rect: wxdragon::geometry::Rect,
    ///     variant: &Variant,
    /// ) -> Option<Box<dyn wxdragon::WxWidget>> {
    ///     let initial_value = variant.get_string().unwrap_or_default();
    ///
    ///     let text_ctrl = TextCtrl::builder(parent)
    ///         .with_pos(rect.position())
    ///         .with_size(rect.size())
    ///         .with_value(&initial_value)
    ///         .build();
    ///
    ///     Some(Box::new(text_ctrl))
    /// }
    ///
    /// let _renderer_builder = DataViewCustomRenderer::builder()
    ///     .with_create_editor(create_text_editor);
    /// ```
    pub fn with_create_editor<F>(mut self, callback: F) -> Self
    where
        F: Fn(&dyn crate::WxWidget, crate::geometry::Rect, &super::Variant) -> Option<Box<dyn crate::WxWidget>> + 'static,
    {
        self.create_editor = Some(Box::new(callback));
        self
    }

    /// Sets the callback for getting the value from an editor control.
    ///
    /// The callback receives the editor widget that was created by `with_create_editor`.
    /// You can downcast it to the specific widget type to extract the value.
    ///
    /// # Example
    /// ```no_run
    /// # use wxdragon::prelude::*;
    /// use wxdragon::window::WxWidgetDowncast;
    /// use wxdragon::widgets::TextCtrl;
    /// use wxdragon::widgets::dataview::{DataViewCustomRenderer, Variant};
    ///
    /// // Provide a plain function (no captures) and pass it to with_get_value_from_editor
    /// fn get_value_from_text_editor(editor: &dyn wxdragon::WxWidget) -> Option<Variant> {
    ///     // Downcast to the specific widget type you created
    ///     if let Some(text_ctrl) = editor.downcast_ref::<TextCtrl>() {
    ///         let value = text_ctrl.get_value();
    ///         Some(Variant::from_string(value))
    ///     } else {
    ///         None
    ///     }
    /// }
    ///
    /// let _renderer_builder = DataViewCustomRenderer::builder()
    ///     .with_get_value_from_editor(get_value_from_text_editor);
    /// ```
    pub fn with_get_value_from_editor<F>(mut self, callback: F) -> Self
    where
        F: Fn(&dyn crate::WxWidget) -> Option<super::Variant> + 'static,
    {
        self.get_value_from_editor = Some(Box::new(callback));
        self
    }

    /// Sets the callback for handling cell activation.
    pub fn with_activate_cell<F>(mut self, callback: F) -> Self
    where
        F: Fn(crate::geometry::Rect, i32) -> bool + 'static,
    {
        self.activate_cell = Some(Box::new(callback));
        self
    }

    /// Build the custom renderer
    pub fn build(self) -> DataViewCustomRenderer {
        let callbacks = Box::new(CustomRendererCallbacks {
            get_size: self.get_size,
            render: self.render,
            set_value: None, // We'll implement this internally
            get_value: None, // We'll implement this internally
            has_editor: self.has_editor,
            activate_cell: self.activate_cell,
            create_editor: self.create_editor,
            get_value_from_editor: self.get_value_from_editor,
            current_value: std::cell::RefCell::new(super::Variant::from_string("")),
        });

        unsafe {
            let raw_callback_data = Box::into_raw(callbacks);
            let variant_type_cstr = CString::new(self.variant_type.as_str()).unwrap();

            let handle = ffi::wxd_DataViewCustomRenderer_Create(
                variant_type_cstr.as_ptr(),
                self.mode.bits(),
                self.align.bits(),
                raw_callback_data as *mut std::ffi::c_void,
                Some(get_size_trampoline),
                Some(render_trampoline),
                Some(set_value_trampoline),
                Some(get_value_trampoline),
                Some(has_editor_trampoline),
                Some(create_editor_trampoline),
                Some(get_value_from_editor_trampoline),
                Some(activate_cell_trampoline),
            );

            if handle.is_null() {
                panic!("Failed to create custom renderer");
            }

            DataViewCustomRenderer { raw: handle }
        }
    }
}

/// Simplified rendering context for custom renderers.
/// This provides a safe wrapper around the wxDC for basic drawing operations.
pub struct RenderContext {
    dc_ptr: *mut wxdragon_sys::wxd_DC_t,
}

impl RenderContext {
    /// Create a new render context from a raw DC pointer.
    ///
    /// # Safety
    /// The caller must ensure the DC pointer is valid for the duration of rendering.
    pub unsafe fn from_raw(dc: *mut std::ffi::c_void) -> Self {
        Self {
            dc_ptr: dc as *mut wxdragon_sys::wxd_DC_t,
        }
    }
}

impl crate::dc::DeviceContext for RenderContext {
    fn dc_ptr(&self) -> *mut wxdragon_sys::wxd_DC_t {
        self.dc_ptr
    }
}

// Trampoline functions that bridge from C++ to Rust
extern "C" fn get_size_trampoline(user_data: *mut std::ffi::c_void) -> ffi::wxd_Size_t {
    if user_data.is_null() {
        return ffi::wxd_Size_t { width: 50, height: 20 };
    }

    let callbacks = unsafe { &*(user_data as *const CustomRendererCallbacks) };
    if let Some(ref callback) = callbacks.get_size {
        // Get the current variant value
        let current_value = callbacks.current_value.borrow();

        // Provide a default cell size - this could be made configurable or
        // derived from the widget's font metrics in a future enhancement
        let default_size = crate::geometry::Size::new(80, 20);

        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| callback(&current_value, default_size)));

        match result {
            Ok(size) => ffi::wxd_Size_t {
                width: size.width,
                height: size.height,
            },
            Err(_) => ffi::wxd_Size_t { width: 50, height: 20 },
        }
    } else {
        ffi::wxd_Size_t { width: 50, height: 20 }
    }
}

extern "C" fn render_trampoline(
    user_data: *mut std::ffi::c_void,
    cell: ffi::wxd_Rect_t,
    dc: *mut std::ffi::c_void,
    state: i32,
) -> bool {
    if user_data.is_null() || dc.is_null() {
        return false;
    }

    let callbacks = unsafe { &*(user_data as *const CustomRendererCallbacks) };
    if let Some(ref callback) = callbacks.render {
        let rect = crate::geometry::Rect::new(cell.x, cell.y, cell.width, cell.height);
        let context = unsafe { RenderContext::from_raw(dc) };

        // Get the current value and pass it directly to the callback
        let current_value = callbacks.current_value.borrow();

        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            callback(rect, &context, state, &current_value)
        }));

        result.unwrap_or(false)
    } else {
        false
    }
}

extern "C" fn set_value_trampoline(user_data: *mut std::ffi::c_void, value: *const ffi::wxd_Variant_t) -> bool {
    if user_data.is_null() || value.is_null() {
        return false;
    }

    let callbacks = unsafe { &*(user_data as *const CustomRendererCallbacks) };
    let variant = Variant::from(value).clone();

    // Store the value internally in the renderer
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        *callbacks.current_value.borrow_mut() = variant;
        true
    }));

    result.unwrap_or(false)
}

extern "C" fn get_value_trampoline(user_data: *mut std::ffi::c_void) -> *mut ffi::wxd_Variant_t {
    if user_data.is_null() {
        return std::ptr::null_mut();
    }

    let callbacks = unsafe { &*(user_data as *const CustomRendererCallbacks) };
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let current_value = callbacks.current_value.borrow();
        let owned = current_value.clone();
        owned.try_into().unwrap()
    }));

    match result {
        Ok(ptr) => ptr,
        Err(_) => super::Variant::from_string("").try_into().unwrap(),
    }
}

extern "C" fn has_editor_trampoline(user_data: *mut std::ffi::c_void) -> bool {
    if user_data.is_null() {
        return false;
    }

    let callbacks = unsafe { &*(user_data as *const CustomRendererCallbacks) };
    if let Some(ref callback) = callbacks.has_editor {
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(callback));
        result.unwrap_or(false)
    } else {
        false
    }
}

extern "C" fn activate_cell_trampoline(
    user_data: *mut std::ffi::c_void,
    cell: ffi::wxd_Rect_t,
    _model: *mut std::ffi::c_void,
    _item: *mut std::ffi::c_void,
    col: u32,
    _mouse_event: *mut std::ffi::c_void,
) -> bool {
    if user_data.is_null() {
        return false;
    }

    let callbacks = unsafe { &*(user_data as *const CustomRendererCallbacks) };
    if let Some(ref callback) = callbacks.activate_cell {
        // Convert parameters properly
        let rect = crate::geometry::Rect::new(cell.x, cell.y, cell.width, cell.height);
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| callback(rect, col as i32)));
        result.unwrap_or(false)
    } else {
        false
    }
}

extern "C" fn create_editor_trampoline(
    user_data: *mut std::ffi::c_void,
    parent: *mut std::ffi::c_void,
    label_rect: ffi::wxd_Rect_t,
    value: *const ffi::wxd_Variant_t,
) -> *mut std::ffi::c_void {
    if user_data.is_null() || parent.is_null() || value.is_null() {
        return std::ptr::null_mut();
    }

    let callbacks = unsafe { &*(user_data as *const CustomRendererCallbacks) };
    if let Some(ref callback) = callbacks.create_editor {
        let rect = crate::geometry::Rect::new(label_rect.x, label_rect.y, label_rect.width, label_rect.height);

        // Just convert the variant, no holding the ownership here
        let variant = Variant::from(value);

        // Create a wrapper for the parent widget
        // Note: This is a simplified implementation. In a full implementation,
        // we would need proper widget type detection and conversion.
        struct ParentWrapper {
            ptr: *mut std::ffi::c_void,
        }

        impl crate::WxWidget for ParentWrapper {
            fn handle_ptr(&self) -> *mut wxdragon_sys::wxd_Window_t {
                self.ptr as *mut wxdragon_sys::wxd_Window_t
            }
        }

        let parent_wrapper = ParentWrapper { ptr: parent };

        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| callback(&parent_wrapper, rect, &variant)));

        match result {
            Ok(Some(editor)) => editor.handle_ptr() as *mut std::ffi::c_void,
            _ => std::ptr::null_mut(),
        }
    } else {
        std::ptr::null_mut()
    }
}

extern "C" fn get_value_from_editor_trampoline(
    user_data: *mut std::ffi::c_void,
    editor: *mut std::ffi::c_void,
) -> *mut ffi::wxd_Variant_t {
    if user_data.is_null() || editor.is_null() {
        return std::ptr::null_mut();
    }

    let callbacks = unsafe { &*(user_data as *const CustomRendererCallbacks) };
    if let Some(ref callback) = callbacks.get_value_from_editor {
        struct EditorWrapper {
            ptr: *mut std::ffi::c_void,
        }

        impl crate::WxWidget for EditorWrapper {
            fn handle_ptr(&self) -> *mut wxdragon_sys::wxd_Window_t {
                self.ptr as *mut wxdragon_sys::wxd_Window_t
            }
        }

        let editor_wrapper = EditorWrapper { ptr: editor };

        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| callback(&editor_wrapper)));

        match result {
            Ok(Some(variant)) => variant.try_into().unwrap_or(std::ptr::null_mut()),
            _ => std::ptr::null_mut(),
        }
    } else {
        std::ptr::null_mut()
    }
}

/// Function called by C++ to drop the Rust callback data.
///
/// # Safety
///
/// This function is called from C++ code and must only be called with a valid pointer
/// that was previously created by `Box::into_raw()` for a `CustomRendererCallbacks` struct.
/// The pointer must not be null and must not have been freed previously. After this
/// function is called, the pointer becomes invalid and must not be used again.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn drop_rust_custom_renderer_callbacks(ptr: *mut std::ffi::c_void) {
    if !ptr.is_null() {
        let _ = unsafe { Box::from_raw(ptr as *mut CustomRendererCallbacks) };
    }
}
