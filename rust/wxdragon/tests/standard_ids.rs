use wxdragon::prelude::*;

#[test]
fn standard_dialog_button_ids_are_exported() {
    let ids: [Id; 5] = [ID_OK, ID_HELP, ID_SAVE, ID_CLOSE, ID_CONTEXT_HELP];

    assert_eq!(ids.len(), 5);
    assert_eq!(ID_SAVE, wxdragon::ffi::WXD_ID_SAVE as Id);
    assert_eq!(ID_CLOSE, wxdragon::ffi::WXD_ID_CLOSE as Id);
    assert_eq!(ID_CONTEXT_HELP, wxdragon::ffi::WXD_ID_CONTEXT_HELP as Id);
}
