use wxdragon::prelude::*;

fn main() {
    wxdragon::main(|_app| {
        let parent_frame = MDIParentFrame::builder().with_title("wxDragon MDI Demo").build();

        let file_menu = Menu::builder()
            .with_title("&File")
            .append_item(ID_HIGHEST + 1, "&New Child\tCtrl+N", "Create a new MDI child frame")
            .append_separator()
            .append_item(ID_EXIT, "E&xit\tAlt+X", "Exit the demo")
            .build();

        let menu_bar = MenuBar::builder().append(file_menu, "&File").build();

        parent_frame.set_menu_bar(menu_bar);
        parent_frame.create_status_bar(1, 0, ID_ANY as i32, "StatusBar");

        let p_frame = parent_frame;
        parent_frame.on_menu(move |event| match event.get_id() {
            id if id == ID_HIGHEST + 1 => {
                let child = MDIChildFrame::builder(&p_frame).with_title("Child Frame").build();

                let panel = Panel::builder(&child).build();
                let sizer = BoxSizer::builder(Orientation::Vertical).build();
                let text = StaticText::builder(&panel).with_label("This is an MDI child frame.").build();
                sizer.add(&text, 0, SizerFlag::all(), 10);
                panel.set_sizer_and_fit(sizer, true);

                child.show(true);
            }
            ID_EXIT => {
                p_frame.close(false);
            }
            _ => {}
        });

        parent_frame.show(true);
    })
    .unwrap();
}
