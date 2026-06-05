use std::cell::RefCell;
use std::rc::Rc;
use wxdragon::prelude::*;

#[derive(Clone)]
struct Row {
    id: usize,
    title: String,
    category: String,
}

fn main() {
    SystemOptions::set_option_by_int("msw.no-manifest-check", 1);

    let _ = wxdragon::main(|_| {
        let rows = Rc::new(build_rows(100_000));
        let filtered_indices = Rc::new(RefCell::new((0..rows.len()).collect::<Vec<_>>()));

        let frame = Frame::builder()
            .with_title("Virtual wxListCtrl")
            .with_size(Size::new(760, 520))
            .build();

        let panel = Panel::builder(&frame).build();
        let search = SearchCtrl::builder(&panel).with_style(SearchCtrlStyle::ProcessEnter).build();
        search.show_search_button(true);
        search.show_cancel_button(true);

        let status = StaticText::builder(&panel)
            .with_label(&format!("{} rows", rows.len()))
            .build();

        let list = ListCtrl::builder(&panel)
            .with_style(ListCtrlStyle::Report | ListCtrlStyle::Virtual | ListCtrlStyle::VRules | ListCtrlStyle::HRules)
            .build();

        list.insert_column(0, "ID", ListColumnFormat::Right, 90);
        list.insert_column(1, "Title", ListColumnFormat::Left, 420);
        list.insert_column(2, "Category", ListColumnFormat::Left, 180);
        list.set_item_count(rows.len() as i64);

        let callback_rows = rows.clone();
        let callback_indices = filtered_indices.clone();
        assert!(list.set_virtual_text_callback(move |item, col| {
            let Some(row_index) = callback_indices.borrow().get(item as usize).copied() else {
                return String::new();
            };
            let Some(row) = callback_rows.get(row_index) else {
                return String::new();
            };

            match col {
                0 => row.id.to_string(),
                1 => row.title.clone(),
                2 => row.category.clone(),
                _ => String::new(),
            }
        }));

        let top_sizer = BoxSizer::builder(Orientation::Vertical).build();
        let search_sizer = BoxSizer::builder(Orientation::Horizontal).build();
        search_sizer.add(&search, 1, SizerFlag::Expand | SizerFlag::Right, 8);
        search_sizer.add(&status, 0, SizerFlag::AlignCenterVertical, 0);
        top_sizer.add_sizer(&search_sizer, 0, SizerFlag::Expand | SizerFlag::All, 8);
        top_sizer.add(
            &list,
            1,
            SizerFlag::Expand | SizerFlag::Left | SizerFlag::Right | SizerFlag::Bottom,
            8,
        );
        panel.set_sizer(top_sizer, true);

        let filter_rows = rows.clone();
        let filter_indices = filtered_indices.clone();
        let filter_list = list;
        let filter_status = status;
        search.on_text_updated(move |event| {
            let query = event.get_string().unwrap_or_default().to_lowercase();
            let mut indices = filter_indices.borrow_mut();
            indices.clear();

            if query.is_empty() {
                indices.extend(0..filter_rows.len());
            } else {
                indices.extend(filter_rows.iter().enumerate().filter_map(|(index, row)| {
                    let matches = row.title.to_lowercase().contains(&query)
                        || row.category.to_lowercase().contains(&query)
                        || row.id.to_string().contains(&query);
                    matches.then_some(index)
                }));
            }

            let count = indices.len();
            filter_list.set_item_count(count as i64);
            if count > 0 {
                filter_list.refresh_items(0, count as i64 - 1);
            }
            filter_status.set_label(&format!("{count} rows"));
        });

        let cancel_search = search;
        search.on_cancel_button_clicked(move |_| {
            cancel_search.set_value("");
        });

        frame.show(true);
        frame.centre();
    });
}

fn build_rows(count: usize) -> Vec<Row> {
    let categories = ["Build", "Runtime", "Docs", "UI", "Testing", "Release"];
    (0..count)
        .map(|id| Row {
            id,
            title: format!("Issue candidate {id:06}"),
            category: categories[id % categories.len()].to_string(),
        })
        .collect()
}
