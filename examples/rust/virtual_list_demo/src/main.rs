use std::any::Any;
use wxdragon::prelude::*;
use wxdragon::widgets::virtual_list::{
    ItemSizingMode, VirtualList, VirtualListDataSource, VirtualListItemRenderer, VirtualListLayoutMode,
};

// Sample data structure for our virtual list
#[derive(Debug, Clone)]
pub struct ListItem {
    pub title: String,
    pub description: String,
    pub height: i32,
}

// Widget IDs for consistent lookups
const TITLE_ID: i32 = 1001;
const DESCRIPTION_ID: i32 = 1002;
const BUTTON_ID: i32 = 1003;

// Renderer implementation using VirtualListItemRenderer trait
pub struct ListItemRenderer;

impl VirtualListItemRenderer for ListItemRenderer {
    fn create_item(&self, parent: &Panel) -> Panel {
        let panel = Panel::builder(parent).with_style(PanelStyle::TabTraversal).build();

        // Create widgets with specific IDs for later lookup
        let title = StaticText::builder(&panel)
            .with_id(TITLE_ID)
            .with_label("") // Empty initially
            .build();

        let description = StaticText::builder(&panel)
            .with_id(DESCRIPTION_ID)
            .with_label("") // Empty initially
            .build();

        let button = Button::builder(&panel).with_id(BUTTON_ID).with_label("Click me!").build();

        // Set up button click handler ONCE during creation
        // Use find_window_by_id to get current item data at click time
        let panel_for_click = panel.clone();
        button.on_click(move |_event| {
            // Get current data from the panel's text widgets at click time
            let mut item_index = "Unknown".to_string();
            let mut item_title = "Unknown".to_string();
            let mut item_description = "Unknown".to_string();

            // Extract index from button label (format: "Click me! (123)")
            if let Some(button_window) = panel_for_click.find_window_by_id(BUTTON_ID)
                && let Some(button_widget) = button_window.as_widget::<Button>()
            {
                let label = button_widget.get_label();
                if let Some(start) = label.find('(')
                    && let Some(end) = label.find(')')
                {
                    item_index = label[start + 1..end].to_string();
                }
            }

            // Get current title
            if let Some(title_window) = panel_for_click.find_window_by_id(TITLE_ID)
                && let Some(title_widget) = title_window.as_widget::<StaticText>()
            {
                item_title = title_widget.get_label();
            }

            // Get current description
            if let Some(desc_window) = panel_for_click.find_window_by_id(DESCRIPTION_ID)
                && let Some(desc_widget) = desc_window.as_widget::<StaticText>()
            {
                item_description = desc_widget.get_label();
            }

            println!("🎯 Button clicked!");
            println!("   Index: {item_index}");
            println!("   Title: {item_title}");
            println!("   Description: {item_description}");
            println!("   ---");
        });

        // Layout the content using BoxSizer with minimal padding
        let sizer = BoxSizer::builder(Orientation::Vertical).build();

        // Title with minimal padding
        sizer.add(
            &title,
            0,
            SizerFlag::Expand | SizerFlag::Left | SizerFlag::Right | SizerFlag::Top,
            4,
        );

        // Description with minimal padding - let it take only the space it needs
        sizer.add(&description, 0, SizerFlag::Expand | SizerFlag::Left | SizerFlag::Right, 2);

        // Button with minimal bottom padding
        sizer.add(&button, 0, SizerFlag::Left | SizerFlag::Right | SizerFlag::Bottom, 4);

        panel.set_sizer(sizer, true);

        panel
    }

    fn update_item(&self, panel: &Panel, index: usize, data: &dyn Any) {
        // Cast the data back to our ListItem type
        if let Some(item_data) = data.downcast_ref::<ListItem>() {
            // Update text content with responsive wrapping
            if let Some(title_window) = panel.find_window_by_id(TITLE_ID)
                && let Some(title) = title_window.as_widget::<StaticText>()
            {
                let panel_width = panel.get_size().width;
                let wrap_width = if panel_width > 60 { panel_width - 20 } else { 350 };

                // Reset text to remove existing line breaks, then apply new wrap
                title.set_label(&item_data.title);
                title.wrap(wrap_width);
            }

            if let Some(desc_window) = panel.find_window_by_id(DESCRIPTION_ID)
                && let Some(description) = desc_window.as_widget::<StaticText>()
            {
                let panel_width = panel.get_size().width;
                let wrap_width = if panel_width > 60 { panel_width - 20 } else { 400 };

                // Reset text to remove existing line breaks, then apply new wrap
                description.set_label(&item_data.description);
                description.wrap(wrap_width);
            }

            if let Some(button_window) = panel.find_window_by_id(BUTTON_ID)
                && let Some(button) = button_window.as_widget::<Button>()
            {
                // Update button label with current index
                button.set_label(&format!("Click me! ({index})"));

                // Event handler is set up once in create_item()
                // Button now displays current item index for reference
            }
        }
    }
}

// New horizontal renderer with fixed 200px width
pub struct HorizontalListItemRenderer;

impl VirtualListItemRenderer for HorizontalListItemRenderer {
    fn create_item(&self, parent: &Panel) -> Panel {
        let panel = Panel::builder(parent).with_style(PanelStyle::TabTraversal).build();

        // Create widgets with specific IDs for later lookup
        let title = StaticText::builder(&panel)
            .with_id(TITLE_ID)
            .with_label("") // Empty initially
            .build();

        let description = StaticText::builder(&panel)
            .with_id(DESCRIPTION_ID)
            .with_label("") // Empty initially
            .build();

        let button = Button::builder(&panel).with_id(BUTTON_ID).with_label("Click me!").build();

        // Set up button click handler ONCE during creation
        let panel_for_click = panel.clone();
        button.on_click(move |_event| {
            // Get current data from the panel's text widgets at click time
            let mut item_index = "Unknown".to_string();
            let mut item_title = "Unknown".to_string();
            let mut item_description = "Unknown".to_string();

            // Extract index from button label (format: "Click me! (123)")
            if let Some(button_window) = panel_for_click.find_window_by_id(BUTTON_ID)
                && let Some(button_widget) = button_window.as_widget::<Button>()
            {
                let label = button_widget.get_label();
                if let Some(start) = label.find('(')
                    && let Some(end) = label.find(')')
                {
                    item_index = label[start + 1..end].to_string();
                }
            }

            // Get current title
            if let Some(title_window) = panel_for_click.find_window_by_id(TITLE_ID)
                && let Some(title_widget) = title_window.as_widget::<StaticText>()
            {
                item_title = title_widget.get_label();
            }

            // Get current description
            if let Some(desc_window) = panel_for_click.find_window_by_id(DESCRIPTION_ID)
                && let Some(desc_widget) = desc_window.as_widget::<StaticText>()
            {
                item_description = desc_widget.get_label();
            }

            println!("🎯 Horizontal Button clicked!");
            println!("   Index: {item_index}");
            println!("   Title: {item_title}");
            println!("   Description: {item_description}");
            println!("   ---");
        });

        // Layout the content using BoxSizer with vertical arrangement
        let sizer = BoxSizer::builder(Orientation::Vertical).build();

        // Title with minimal padding
        sizer.add(
            &title,
            0,
            SizerFlag::Expand | SizerFlag::Left | SizerFlag::Right | SizerFlag::Top,
            4,
        );

        // Description with minimal padding
        sizer.add(
            &description,
            1, // Allow to expand vertically to fill available space
            SizerFlag::Expand | SizerFlag::Left | SizerFlag::Right,
            2,
        );

        // Button with minimal bottom padding
        sizer.add(&button, 0, SizerFlag::Left | SizerFlag::Right | SizerFlag::Bottom, 4);

        panel.set_sizer(sizer, true);

        panel
    }

    fn update_item(&self, panel: &Panel, index: usize, data: &dyn Any) {
        // Cast the data back to our ListItem type
        if let Some(item_data) = data.downcast_ref::<ListItem>() {
            // Fixed width approach - always wrap to 180px (200px - 20px margin)
            const FIXED_ITEM_WIDTH: i32 = 200;
            const CONTENT_WIDTH: i32 = FIXED_ITEM_WIDTH - 20; // Account for margins

            // Update title with fixed width wrapping
            if let Some(title_window) = panel.find_window_by_id(TITLE_ID)
                && let Some(title) = title_window.as_widget::<StaticText>()
            {
                title.set_label(&item_data.title);
                title.wrap(CONTENT_WIDTH);
            }

            // Update description with fixed width wrapping
            if let Some(desc_window) = panel.find_window_by_id(DESCRIPTION_ID)
                && let Some(description) = desc_window.as_widget::<StaticText>()
            {
                description.set_label(&item_data.description);
                description.wrap(CONTENT_WIDTH);
            }

            // Update button
            if let Some(button_window) = panel.find_window_by_id(BUTTON_ID)
                && let Some(button) = button_window.as_widget::<Button>()
            {
                button.set_label(&format!("Click! ({index})"));
            }

            // Set the panel to fixed width after content update
            // The virtual list will override the height to match viewport height
            panel.set_size(Size::new(FIXED_ITEM_WIDTH, panel.get_size().height));

            // CRITICAL: Set minimum and maximum size to enforce exact 200px width
            panel.set_min_size(Size::new(FIXED_ITEM_WIDTH, -1));
            panel.set_max_size(Size::new(FIXED_ITEM_WIDTH, -1));
        }
    }
}

fn create_sample_data() -> Vec<ListItem> {
    let mut data = Vec::new();
    let descriptions = [
        "Short description.",
        "This is a medium-length description that spans more text and provides more details about this particular item.",
        "This is a very long description that contains a lot of text and will likely span multiple lines when displayed in the virtual list. It demonstrates how the dynamic height calculation works with varying content lengths and word wrapping functionality. The height should automatically adjust based on the content size to prevent clipping and ensure all text is visible to the user.",
    ];

    for i in 0..1000 {
        let desc_template = descriptions[i % descriptions.len()];
        data.push(ListItem {
            title: format!("Item {i}"),
            description: format!("{desc_template} (Item {i})"),
            height: 60, // This will be overridden by dynamic calculation
        });
    }
    data
}

fn main() {
    let _ = wxdragon::main(|_| {
        let frame = Frame::builder()
            .with_title("Virtual List Demo - Vertical & Horizontal Lists")
            .with_size(Size::new(1200, 600))
            .build();

        let main_panel = Panel::builder(&frame).build();

        // Create sample data
        let data = create_sample_data();

        // Create the vertical virtual list widget
        let vertical_list = VirtualList::builder(&main_panel)
            .with_layout_mode(VirtualListLayoutMode::Vertical)
            .build();

        // Create the horizontal virtual list widget
        let horizontal_list = VirtualList::builder(&main_panel)
            .with_layout_mode(VirtualListLayoutMode::Horizontal)
            .build();

        // Set up the data source and renderer for both lists
        let vertical_data_source = ListItemDataSource::new(data.clone());
        let horizontal_data_source = ListItemDataSource::new(data);
        let vertical_renderer = ListItemRenderer;
        let horizontal_renderer = HorizontalListItemRenderer;

        // Configure vertical list
        vertical_list.set_data_source(vertical_data_source);
        vertical_list.set_item_renderer(vertical_renderer);
        vertical_list.set_item_sizing_mode(ItemSizingMode::DynamicSize);

        // Configure horizontal list
        horizontal_list.set_data_source(horizontal_data_source);
        horizontal_list.set_item_renderer(horizontal_renderer);
        horizontal_list.set_item_sizing_mode(ItemSizingMode::FixedSize);

        // Create labels for the lists
        let vertical_label = StaticText::builder(&main_panel).with_label("Vertical Virtual List:").build();

        let horizontal_label = StaticText::builder(&main_panel)
            .with_label("Horizontal Virtual List:")
            .build();

        // Set up the main frame layout with horizontal split
        let main_sizer = BoxSizer::builder(Orientation::Horizontal).build();

        // Left side - vertical list
        let left_sizer = BoxSizer::builder(Orientation::Vertical).build();
        left_sizer.add(&vertical_label, 0, SizerFlag::Expand | SizerFlag::All, 5);
        left_sizer.add(&vertical_list, 1, SizerFlag::Expand | SizerFlag::All, 5);

        // Right side - horizontal list
        let right_sizer = BoxSizer::builder(Orientation::Vertical).build();
        right_sizer.add(&horizontal_label, 0, SizerFlag::Expand | SizerFlag::All, 5);
        right_sizer.add(&horizontal_list, 1, SizerFlag::Expand | SizerFlag::All, 5);

        // Add both sides to main sizer
        main_sizer.add_sizer(&left_sizer, 1, SizerFlag::Expand | SizerFlag::All, 5);
        main_sizer.add_sizer(&right_sizer, 1, SizerFlag::Expand | SizerFlag::All, 5);

        main_panel.set_sizer(main_sizer, true);

        // Apply sizer to frame
        let main_frame_sizer = BoxSizer::builder(Orientation::Vertical).build();
        main_frame_sizer.add(&main_panel, 1, SizerFlag::Expand, 0);
        frame.set_sizer(main_frame_sizer, true);

        frame.show(true);
        frame.centre();
    });
}

// Data source implementation for our ListItem data
pub struct ListItemDataSource {
    items: Vec<ListItem>,
}

impl ListItemDataSource {
    pub fn new(items: Vec<ListItem>) -> Self {
        Self { items }
    }
}

impl VirtualListDataSource for ListItemDataSource {
    fn get_item_count(&self) -> usize {
        self.items.len()
    }

    fn get_item_data(&self, index: usize) -> Box<dyn Any + Send + Sync> {
        if index < self.items.len() {
            Box::new(self.items[index].clone())
        } else {
            Box::new(ListItem {
                title: "Unknown".to_string(),
                description: "Unknown item".to_string(),
                height: 60,
            })
        }
    }
}
