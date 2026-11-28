//! GUI regression for the platform's preferences window. Requires a desktop.
use std::cell::{Cell, RefCell};
use std::rc::Rc;
use wxdragon::prelude::*;

/// Counts how many times a page is built, and, by its strong count, whether its closure was dropped.
type BuildCount = Rc<Cell<u32>>;

fn page<W: WxWidget>(count: &BuildCount, build: impl Fn(&Panel) -> W + 'static) -> impl FnMut(Window) -> Panel + 'static {
    let count = count.clone();
    move |parent| {
        count.set(count.get() + 1);
        let panel = Panel::builder(&parent).build();
        build(&panel);
        panel
    }
}

fn build_editor(general: &BuildCount, custom: &BuildCount) -> PreferencesEditor {
    let editor = PreferencesEditor::new("Settings");
    editor.add_stock_page(
        StockPreferencesPage::General,
        page(general, |panel| {
            CheckBox::builder(panel).with_label("Open the last document").build()
        }),
    );
    let icon = ArtProvider::get_bitmap_bundle(ArtId::Information, ArtClient::Toolbar, None)
        .expect("the art provider should have an information icon");
    editor.add_page(
        "Reading",
        Some(&icon),
        page(custom, |panel| CheckBox::builder(panel).with_label("Read aloud").build()),
    );
    editor
}

fn main() {
    SystemOptions::set_option_by_int("msw.no-manifest-check", 1);
    let finished = Rc::new(Cell::new(false));
    let finished_in_loop = finished.clone();
    let timer_store: Rc<RefCell<Option<Timer<Frame>>>> = Rc::new(RefCell::new(None));
    let timer_in_loop = timer_store.clone();
    wxdragon::main(move |app| {
        let frame = Frame::builder().with_title("Preferences editor").build();
        let general = BuildCount::default();
        let custom = BuildCount::default();
        let editor = Rc::new(RefCell::new(Some(build_editor(&general, &custom))));
        assert_eq!(general.get(), 0, "no page should be built before the editor is shown");
        // Ticks are counted so the editor is dropped on one and its closures checked on the next,
        // since wxWidgets deletes a modeless editor's window, and with it the pages, when idle.
        let ticks = Rc::new(Cell::new(0));
        let timer = Timer::new(&frame);
        let timer_cleanup = timer_in_loop.clone();
        let editor_in_timer = editor.clone();
        let finished_in_timer = finished_in_loop.clone();
        let (general_in_timer, custom_in_timer) = (general.clone(), custom.clone());
        timer.on_tick(move |_| {
            ticks.set(ticks.get() + 1);
            if ticks.get() == 1 {
                if let Some(editor) = editor_in_timer.borrow().as_ref() {
                    editor.dismiss();
                }
                if !PreferencesEditor::shown_modally() {
                    editor_in_timer.borrow_mut().take();
                }
                return;
            }
            timer_cleanup.borrow_mut().take();
            app.exit_main_loop();
            editor_in_timer.borrow_mut().take();
            assert_eq!(
                Rc::strong_count(&general_in_timer),
                1,
                "dropping the editor should drop the General page's closure"
            );
            assert_eq!(
                Rc::strong_count(&custom_in_timer),
                1,
                "dropping the editor should drop the custom page's closure"
            );
            finished_in_timer.set(true);
        });
        timer.start(200, false);
        timer_in_loop.borrow_mut().replace(timer);

        assert_eq!(
            PreferencesEditor::shown_modally(),
            !PreferencesEditor::should_apply_changes_immediately(),
            "a modal editor applies changes with OK, and a modeless one as they're made"
        );
        // A modal editor returns from show only when the timer's first tick dismisses it.
        editor.borrow().as_ref().unwrap().show(&frame);
        assert_eq!(general.get(), 1, "showing the editor should build the first page");
        if PreferencesEditor::shown_modally() {
            assert_eq!(custom.get(), 1, "a modal editor builds every page when shown");
        }
    })
    .expect("could not initialize wxWidgets for preferences editor regressions");
    assert!(finished.get(), "preferences editor assertions did not finish");
}
