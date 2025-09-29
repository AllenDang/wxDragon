use proc_macro::TokenStream;
use quote::quote;
use std::collections::HashMap;
use syn::{parse_macro_input, Error, Ident, LitStr, Token};

/// A procedural macro that generates a Rust struct for XRC-defined UI with all named widgets.
///
/// This macro reads an XRC file at compile time, parses it to extract all named widgets,
/// and generates a Rust struct with typed fields for accessing all UI components.
/// The root object (Frame, Dialog, or Panel) is automatically detected from the XRC file.
///
/// # Syntax
///
/// ```ignore
/// include_xrc!("path/to/file.xrc", StructName);
/// ```
///
/// # Arguments
///
/// * `path` - Path to the XRC file relative to the current crate root
/// * `struct_name` - Name for the generated Rust struct  
///
/// # Generated Code
///
/// The macro generates a struct with:
/// - A field for the root object (Frame, Dialog, or Panel) - automatically detected
/// - Fields for all named child widgets found in the XRC
/// - A `new()` method that loads the XRC and initializes all fields
/// - An `xrc_id()` helper method for getting XRC IDs
///
/// # Example
///
/// Given an XRC file `dialog.xrc`:
/// ```xml
/// <?xml version="1.0" encoding="UTF-8"?>
/// <resource>
///   <object class="wxFrame" name="main_frame">
///     <title>My Frame</title>
///     <object class="wxPanel" name="main_panel">
///       <object class="wxButton" name="test_button">
///         <label>Click Me!</label>
///       </object>
///       <object class="wxTextCtrl" name="input_field">
///       </object>
///     </object>
///   </object>
/// </resource>
/// ```
///
/// Usage:
/// ```ignore
/// include_xrc!("dialog.xrc", MyFrameUI);
///
/// // This generates:
/// pub struct MyFrameUI {
///     pub main_frame: Frame,      // Root object - auto-detected
///     pub main_panel: Panel,
///     pub test_button: Button,
///     pub input_field: TextCtrl,
///     _resource: XmlResource,
/// }
///
/// impl MyFrameUI {
///     pub const XRC_DATA: &'static str = "..."; // Embedded XRC content
///     
///     pub fn new(parent: Option<&dyn WxWidget>) -> Self {
///         // Implementation that loads XRC and finds all widgets
///     }
///     
///     pub fn xrc_id(name: &str) -> i32 {
///         // Helper to get XRC IDs
///     }
/// }
/// ```
#[proc_macro]
pub fn include_xrc(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as XrcMacroInput);

    match generate_xrc_struct(input) {
        Ok(tokens) => tokens.into(),
        Err(err) => err.to_compile_error().into(),
    }
}

/// Parsed input for the include_xrc macro
struct XrcMacroInput {
    xrc_path: String,
    struct_name: Ident,
}

impl syn::parse::Parse for XrcMacroInput {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        // Parse "path/to/file.xrc"
        let xrc_path: LitStr = input.parse()?;
        input.parse::<Token![,]>()?;

        // Parse StructName
        let struct_name: Ident = input.parse()?;

        // No more root parameter - we'll auto-detect it

        Ok(XrcMacroInput {
            xrc_path: xrc_path.value(),
            struct_name,
        })
    }
}

/// XRC object information extracted from XML
#[derive(Debug, Clone)]
struct XrcObject {
    name: String,
    class: String,
    children: Vec<XrcObject>,
}

/// Mapping from XRC class names to wxDragon Rust types
fn get_class_mapping() -> HashMap<&'static str, &'static str> {
    let mut map = HashMap::new();

    // Window types
    map.insert("wxDialog", "wxdragon::dialogs::Dialog");
    map.insert("wxFrame", "wxdragon::widgets::Frame");
    map.insert("wxPanel", "wxdragon::widgets::Panel");

    // Controls
    map.insert("wxButton", "wxdragon::widgets::Button");
    map.insert("wxCheckBox", "wxdragon::widgets::CheckBox");
    map.insert("wxComboBox", "wxdragon::widgets::ComboBox");
    map.insert("wxTextCtrl", "wxdragon::widgets::TextCtrl");
    map.insert("wxStaticText", "wxdragon::widgets::StaticText");
    map.insert("wxListBox", "wxdragon::widgets::ListBox");
    map.insert("wxChoice", "wxdragon::widgets::Choice");
    map.insert("wxSlider", "wxdragon::widgets::Slider");
    map.insert("wxGauge", "wxdragon::widgets::Gauge");
    map.insert("wxSpinCtrl", "wxdragon::widgets::SpinCtrl");
    map.insert("wxSpinCtrlDouble", "wxdragon::widgets::SpinCtrlDouble");
    map.insert("wxSpinButton", "wxdragon::widgets::SpinButton");
    map.insert("wxTreeCtrl", "wxdragon::widgets::TreeCtrl");
    map.insert("wxTreeListCtrl", "wxdragon::widgets::TreeListCtrl");
    map.insert("wxNotebook", "wxdragon::widgets::Notebook");
    map.insert("wxSimplebook", "wxdragon::widgets::SimpleBook");
    map.insert("wxRadioButton", "wxdragon::widgets::RadioButton");
    map.insert("wxRadioBox", "wxdragon::widgets::RadioBox");
    map.insert("wxToggleButton", "wxdragon::widgets::ToggleButton");
    map.insert("wxBitmapButton", "wxdragon::widgets::BitmapButton");

    // StaticBitmap uses platform-aware XRC handler at C++ level
    // On Windows: creates wxGenericStaticBitmap, but we still treat it as StaticBitmap in Rust
    // On other platforms: creates native wxStaticBitmap
    // Both have the same interface, so we always use StaticBitmap wrapper
    map.insert("wxStaticBitmap", "wxdragon::widgets::StaticBitmap");

    map.insert("wxStaticLine", "wxdragon::widgets::StaticLine");
    map.insert("wxStaticBox", "wxdragon::widgets::StaticBox");
    map.insert("wxScrollBar", "wxdragon::widgets::ScrollBar");
    map.insert("wxScrolledWindow", "wxdragon::widgets::ScrolledWindow");
    map.insert("wxSplitterWindow", "wxdragon::widgets::SplitterWindow");
    map.insert("wxCollapsiblePane", "wxdragon::widgets::CollapsiblePane");
    map.insert("wxCheckListBox", "wxdragon::widgets::CheckListBox");
    map.insert("wxRearrangeList", "wxdragon::widgets::RearrangeList");
    map.insert("wxTreebook", "wxdragon::widgets::Treebook");
    map.insert("wxListCtrl", "wxdragon::widgets::ListCtrl");
    map.insert("wxHyperlinkCtrl", "wxdragon::widgets::HyperlinkCtrl");
    map.insert("wxSearchCtrl", "wxdragon::widgets::SearchCtrl");
    map.insert("wxStyledTextCtrl", "wxdragon::widgets::StyledTextCtrl");
    map.insert(
        "wxActivityIndicator",
        "wxdragon::widgets::ActivityIndicator",
    );
    map.insert("wxAnimationCtrl", "wxdragon::widgets::AnimationCtrl");
    map.insert("wxBitmapComboBox", "wxdragon::widgets::BitmapComboBox");
    map.insert("wxCalendarCtrl", "wxdragon::widgets::CalendarCtrl");
    map.insert("wxColourPickerCtrl", "wxdragon::widgets::ColourPickerCtrl");
    map.insert(
        "wxCommandLinkButton",
        "wxdragon::widgets::CommandLinkButton",
    );
    map.insert("wxDatePickerCtrl", "wxdragon::widgets::DatePickerCtrl");
    map.insert("wxDirPickerCtrl", "wxdragon::widgets::DirPickerCtrl");
    map.insert("wxEditableListBox", "wxdragon::widgets::EditableListBox");
    map.insert("wxFileCtrl", "wxdragon::widgets::FileCtrl");
    map.insert("wxFilePickerCtrl", "wxdragon::widgets::FilePickerCtrl");
    map.insert("wxFontPickerCtrl", "wxdragon::widgets::FontPickerCtrl");
    map.insert("wxMediaCtrl", "wxdragon::widgets::MediaCtrl");
    map.insert("wxTimePickerCtrl", "wxdragon::widgets::TimePickerCtrl");
    map.insert("wxRichTextCtrl", "wxdragon::widgets::RichTextCtrl");

    // AUI widgets
    map.insert("wxAuiManager", "wxdragon::widgets::AuiManager");
    map.insert("wxAuiNotebook", "wxdragon::widgets::AuiNotebook");
    map.insert("wxAuiToolBar", "wxdragon::widgets::AuiToolBar");
    map.insert(
        "wxAuiMDIParentFrame",
        "wxdragon::widgets::AuiMDIParentFrame",
    );
    map.insert("wxAuiMDIChildFrame", "wxdragon::widgets::AuiMDIChildFrame");

    // Frame-related widgets
    map.insert("wxToolBar", "wxdragon::widgets::ToolBar");
    map.insert("wxStatusBar", "wxdragon::widgets::StatusBar");

    // Toolbar tools
    map.insert("tool", "wxdragon::widgets::Tool");

    // Menu system
    map.insert("wxMenuBar", "wxdragon::menus::MenuBar");
    map.insert("wxMenu", "wxdragon::menus::Menu");
    map.insert("wxMenuItem", "wxdragon::menus::MenuItem");

    // Fallback for unknown types - treat as Window
    map.insert("unknown", "wxdragon::window::Window");

    map
}

/// Generate the complete XRC struct implementation
fn generate_xrc_struct(input: XrcMacroInput) -> syn::Result<proc_macro2::TokenStream> {
    // Read and parse the XRC file for widget analysis
    let xrc_content = read_xrc_file(&input.xrc_path)?;
    let xrc_objects = parse_xrc_content(&xrc_content)?;

    // Find root object
    let root_object = find_root_object(&xrc_objects)?;

    // Collect all named objects for field generation
    let mut all_objects = Vec::new();
    collect_named_objects(root_object, &mut all_objects);

    // Filter out sizers and other non-widget objects that don't support XRC
    let widget_objects: Vec<_> = all_objects
        .into_iter()
        .filter(|obj| {
            !obj.class.contains("Sizer")
                && !obj.class.contains("sizeritem")
                && !obj.class.contains("spacer")
                && obj.class != "wxMenu" // Skip Menu objects - they're part of MenuBar
        })
        .collect();

    // Separate tools and menu items from other widgets for special handling
    let (tool_objects, remaining_objects): (Vec<_>, Vec<_>) =
        widget_objects.iter().partition(|obj| obj.class == "tool");

    let (menu_item_objects, remaining_objects2): (Vec<_>, Vec<_>) = remaining_objects
        .into_iter()
        .partition(|obj| obj.class == "wxMenuItem");

    let (menubar_objects, non_special_objects): (Vec<_>, Vec<_>) = remaining_objects2
        .into_iter()
        .partition(|obj| obj.class == "wxMenuBar");

    // Generate the struct and implementation
    let struct_name = &input.struct_name;
    let class_mapping = get_class_mapping();

    // Generate struct fields for all named objects
    let struct_fields = widget_objects.iter().map(|obj| {
        let field_name = Ident::new(&obj.name, proc_macro2::Span::call_site());
        let type_str = class_mapping
            .get(obj.class.as_str())
            .unwrap_or(&"wxdragon::window::Window");

        let field_type: syn::Type = syn::parse_str(type_str).unwrap();
        quote! { pub #field_name: #field_type }
    });

    // Generate field initialization in new() method
    let root_load_method = match root_object.class.as_str() {
        "wxDialog" => quote! { load_dialog },
        "wxFrame" => quote! { load_frame },
        "wxPanel" => quote! { load_panel },
        _ => quote! { load_frame }, // Default to frame
    };

    let root_field_name = Ident::new(&root_object.name, proc_macro2::Span::call_site());
    let xrc_path = &input.xrc_path;

    // Generate initialization for regular widgets first
    let non_special_initializers = non_special_objects.iter().map(|obj| {
        let field_name = Ident::new(&obj.name, proc_macro2::Span::call_site());
        let obj_name_lit = &obj.name;

        if obj.name == root_object.name {
            // Root object is loaded directly
            quote! {
                let #field_name = resource.#root_load_method(parent, #obj_name_lit)
                    .unwrap_or_else(|| panic!("Failed to load XRC root object: {}", #obj_name_lit));
            }
        } else {
            // Regular widgets are found within the root - explicitly specify the widget type
            let type_str = class_mapping
                .get(obj.class.as_str())
                .unwrap_or(&"wxdragon::window::Window");
            let widget_type: syn::Type = syn::parse_str(type_str).unwrap();

            quote! {
                let #field_name = #root_field_name
                    .find_child_by_xrc_name::<#widget_type>(#obj_name_lit)
                    .unwrap_or_else(|| panic!("Failed to find XRC child: {}", #obj_name_lit));
            }
        }
    });

    // Generate initialization for MenuBars (they're loaded separately, not as child windows)
    let menubar_initializers = menubar_objects.iter().map(|menubar_obj| {
        let field_name = Ident::new(&menubar_obj.name, proc_macro2::Span::call_site());
        let _menubar_name_lit = &menubar_obj.name;

        quote! {
            let #field_name = #root_field_name.get_menu_bar()
                .unwrap_or_else(|| panic!("Failed to get MenuBar from Frame"));
        }
    });

    // Generate initialization for tools after toolbars are loaded
    let tool_initializers = tool_objects.iter().map(|tool_obj| {
        let field_name = Ident::new(&tool_obj.name, proc_macro2::Span::call_site());
        let tool_name_lit = &tool_obj.name;

        // Find the parent toolbar - for now assume it's "main_toolbar"
        // TODO: Implement proper parent detection from XRC hierarchy
        quote! {
            let #field_name = main_toolbar.get_tool_by_name(#tool_name_lit)
                .unwrap_or_else(|| panic!("Failed to find tool: {}", #tool_name_lit));
        }
    });

    // Generate initialization for menu items after the root frame is loaded
    let menu_item_initializers = menu_item_objects.iter().map(|menu_item_obj| {
        let field_name = Ident::new(&menu_item_obj.name, proc_macro2::Span::call_site());
        let menu_item_name_lit = &menu_item_obj.name;

        quote! {
            let #field_name = wxdragon::menus::MenuItem::from_xrc_name(&#root_field_name, #menu_item_name_lit)
                .unwrap_or_else(|| panic!("Failed to find menu item: {}", #menu_item_name_lit));
        }
    });

    let field_assignments = widget_objects.iter().map(|obj| {
        let field_name = Ident::new(&obj.name, proc_macro2::Span::call_site());
        quote! { #field_name }
    });

    let generated = quote! {
        #[allow(non_snake_case)]
        pub struct #struct_name {
            #(#struct_fields,)*
            _resource: wxdragon::xrc::XmlResource,
        }

        impl #struct_name {
            /// The embedded XRC data from the file
            pub const XRC_DATA: &'static str = include_str!(#xrc_path);

            /// Create a new instance by loading the embedded XRC
            pub fn new(parent: Option<&dyn wxdragon::window::WxWidget>) -> Self {
                let resource = wxdragon::xrc::XmlResource::get();

                // Initialize platform-aware StaticBitmap handler BEFORE default handlers
                // to ensure it gets registered first
                resource.init_platform_aware_staticbitmap_handler();

                resource.init_all_handlers();

                resource.load_from_string(Self::XRC_DATA)
                    .unwrap_or_else(|err| panic!("Failed to load XRC data: {}", err));

                #(#non_special_initializers)*

                // Initialize MenuBars (loaded separately from XRC)
                #(#menubar_initializers)*

                // Initialize tools after toolbars are loaded
                #(#tool_initializers)*

                // Initialize menu items after the root frame is loaded
                #(#menu_item_initializers)*

                Self {
                    #(#field_assignments,)*
                    _resource: resource,
                }
            }

            /// Get XRC ID for a control name
            pub fn xrc_id(name: &str) -> i32 {
                wxdragon::xrc::XmlResource::get_xrc_id(name)
            }
        }
    };

    Ok(generated)
}

/// Read XRC file content from the filesystem during macro expansion
fn read_xrc_file(path: &str) -> syn::Result<String> {
    // Try to resolve the file the same way include_str! would
    // include_str! looks for files relative to the current source file

    // For procedural macros, we need to handle the fact that CARGO_MANIFEST_DIR
    // might refer to the macro crate, not the invoking crate. We use multiple strategies.

    let possible_paths = vec![
        // 1. Try path as-is from current working directory
        std::path::PathBuf::from(path),
        // 2. Try from the current working directory (which should be the invoking crate)
        std::env::current_dir()
            .map(|cwd| cwd.join(path))
            .unwrap_or_else(|_| std::path::PathBuf::from(path)),
        // 3. For "../" patterns, try resolving from current working dir + "src"
        if path.starts_with("../") {
            std::env::current_dir()
                .map(|cwd| cwd.join("src").join(path))
                .unwrap_or_else(|_| std::path::PathBuf::from(path))
        } else {
            std::path::PathBuf::from(path)
        },
        // 4. Try CARGO_MANIFEST_DIR if available (invoking crate's manifest dir)
        std::env::var("CARGO_MANIFEST_DIR")
            .map(|manifest_dir| std::path::PathBuf::from(manifest_dir).join(path))
            .unwrap_or_else(|_| std::path::PathBuf::from(path)),
        // 5. For "../" from src directory pattern (most common case)
        std::env::var("CARGO_MANIFEST_DIR")
            .map(|manifest_dir| {
                if path.starts_with("../") {
                    std::path::PathBuf::from(manifest_dir).join(path.trim_start_matches("../"))
                } else {
                    std::path::PathBuf::from(manifest_dir)
                        .join("src")
                        .join(path)
                }
            })
            .unwrap_or_else(|_| std::path::PathBuf::from(path)),
    ];

    for full_path in &possible_paths {
        if let Ok(content) = std::fs::read_to_string(full_path) {
            return Ok(content);
        }
    }

    // If none worked, give a helpful error
    Err(Error::new(
        proc_macro2::Span::call_site(),
        format!(
            "Failed to read XRC file '{}' for macro analysis. \
             The file will be embedded using include_str! at compile time, \
             but the macro needs to read it now to generate widget fields. \
             \nTried paths: {:?} \
             \n\nNote: Use paths relative to your crate root or source file, just like include_str!",
            path,
            possible_paths.iter().map(|p| p.display().to_string()).collect::<Vec<_>>()
        )
    ))
}

/// Parse XRC XML content to extract object hierarchy
fn parse_xrc_content(content: &str) -> syn::Result<Vec<XrcObject>> {
    use quick_xml::events::Event;
    use quick_xml::Reader;

    let mut reader = Reader::from_str(content);
    reader.config_mut().trim_text(true);

    let mut objects = Vec::new();
    let mut stack = Vec::new();

    loop {
        match reader.read_event() {
            Ok(Event::Start(ref e)) if e.name().as_ref() == b"object" => {
                let mut obj = XrcObject {
                    name: String::new(),
                    class: String::new(),
                    children: Vec::new(),
                };

                // Parse attributes
                for attr in e.attributes() {
                    let attr = attr.map_err(|e| {
                        Error::new(
                            proc_macro2::Span::call_site(),
                            format!("XML parsing error: {e}"),
                        )
                    })?;

                    match attr.key.as_ref() {
                        b"name" => obj.name = String::from_utf8_lossy(&attr.value).into_owned(),
                        b"class" => obj.class = String::from_utf8_lossy(&attr.value).into_owned(),
                        _ => {}
                    }
                }

                stack.push(obj);
            }
            Ok(Event::End(ref e)) if e.name().as_ref() == b"object" => {
                if let Some(obj) = stack.pop() {
                    if let Some(parent) = stack.last_mut() {
                        // Always add to parent, even if object doesn't have a name
                        // This preserves the hierarchy for unnamed intermediate objects
                        parent.children.push(obj);
                    } else {
                        // Top-level object - only add if it has a name or children
                        if !obj.name.is_empty() || !obj.children.is_empty() {
                            objects.push(obj);
                        }
                    }
                }
            }
            Ok(Event::Eof) => break,
            Err(e) => {
                return Err(Error::new(
                    proc_macro2::Span::call_site(),
                    format!("XML parsing error: {e}"),
                ))
            }
            _ => {}
        }
    }

    Ok(objects)
}

/// Find the root object to load (automatically detect Frame, Dialog, or Panel)
fn find_root_object(objects: &[XrcObject]) -> syn::Result<&XrcObject> {
    // Look for the first Frame, Dialog, or Panel object
    objects
        .iter()
        .find(|obj| {
            obj.class == "wxFrame" || obj.class == "wxDialog" || obj.class == "wxPanel"
        })
        .ok_or_else(|| {
            Error::new(
                proc_macro2::Span::call_site(),
                "No root Frame, Dialog, or Panel object found in XRC. Make sure your XRC file contains a top-level wxFrame, wxDialog, or wxPanel object.",
            )
        })
}

/// Recursively collect all named objects from the hierarchy
fn collect_named_objects(obj: &XrcObject, result: &mut Vec<XrcObject>) {
    if !obj.name.is_empty() {
        result.push(obj.clone());
    }

    for child in &obj.children {
        collect_named_objects(child, result);
    }
}
