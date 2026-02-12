use std::ffi::CString;
use std::os::raw::c_void;
use std::ptr;

use crate::ffi;
use crate::prelude::*;

// --- Traits ---

pub trait Printout {
    fn on_prepare_printing(&mut self, _dc: &GenericDC) {}
    fn on_begin_printing(&mut self, _dc: &GenericDC) {}
    fn on_end_printing(&mut self) {}
    fn on_begin_document(&mut self, _dc: &GenericDC, _start_page: i32, _end_page: i32) -> bool {
        true
    }
    fn on_end_document(&mut self) {}
    fn on_print_page(&mut self, dc: &GenericDC, page_num: i32) -> bool;
    fn has_page(&mut self, page_num: i32) -> bool {
        page_num == 1
    }
    fn get_page_info(&mut self) -> (i32, i32, i32, i32) {
        (1, 32000, 1, 1)
    }
}

// --- Printout Proxy ---

struct PrintoutProxy<T: Printout> {
    inner: T,
    ffi_ptr: *mut ffi::wxd_Printout_t,
}

impl<T: Printout> PrintoutProxy<T> {
    fn new(title: &str, inner: T) -> Box<Self> {
        let title_c = CString::new(title).unwrap();

        let mut proxy = Box::new(Self {
            inner,
            ffi_ptr: ptr::null_mut(),
        });

        let user_data = &mut *proxy as *mut Self as *mut c_void;

        proxy.ffi_ptr = unsafe {
            ffi::wxd_Printout_CreateWithCallbacks(
                title_c.as_ptr(),
                user_data,
                Some(Self::on_prepare_printing_cb),
                Some(Self::on_begin_printing_cb),
                Some(Self::on_end_printing_cb),
                Some(Self::on_begin_document_cb),
                Some(Self::on_end_document_cb),
                Some(Self::on_print_page_cb),
                Some(Self::has_page_cb),
                Some(Self::get_page_info_cb),
            )
        };

        proxy
    }

    unsafe extern "C" fn on_prepare_printing_cb(user_data: *mut c_void) {
        let proxy = unsafe { &mut *(user_data as *mut Self) };
        let dc = proxy.get_dc();
        proxy.inner.on_prepare_printing(&dc);
    }

    unsafe extern "C" fn on_begin_printing_cb(user_data: *mut c_void) {
        let proxy = unsafe { &mut *(user_data as *mut Self) };
        let dc = proxy.get_dc();
        proxy.inner.on_begin_printing(&dc);
    }

    unsafe extern "C" fn on_end_printing_cb(user_data: *mut c_void) {
        let proxy = unsafe { &mut *(user_data as *mut Self) };
        proxy.inner.on_end_printing();
    }

    unsafe extern "C" fn on_begin_document_cb(user_data: *mut c_void, start_page: i32, end_page: i32) {
        let proxy = unsafe { &mut *(user_data as *mut Self) };
        let dc = proxy.get_dc();
        proxy.inner.on_begin_document(&dc, start_page, end_page);
    }

    unsafe extern "C" fn on_end_document_cb(user_data: *mut c_void) {
        let proxy = unsafe { &mut *(user_data as *mut Self) };
        proxy.inner.on_end_document();
    }

    unsafe extern "C" fn on_print_page_cb(user_data: *mut c_void, page_num: i32) -> bool {
        let proxy = unsafe { &mut *(user_data as *mut Self) };
        let dc = proxy.get_dc();
        proxy.inner.on_print_page(&dc, page_num)
    }

    unsafe extern "C" fn has_page_cb(user_data: *mut c_void, page_num: i32) -> bool {
        let proxy = unsafe { &mut *(user_data as *mut Self) };
        proxy.inner.has_page(page_num)
    }

    unsafe extern "C" fn get_page_info_cb(
        user_data: *mut c_void,
        min_page: *mut i32,
        max_page: *mut i32,
        page_from: *mut i32,
        page_to: *mut i32,
    ) {
        let proxy = unsafe { &mut *(user_data as *mut Self) };
        let (min, max, from, to) = proxy.inner.get_page_info();
        unsafe {
            if !min_page.is_null() {
                *min_page = min;
            }
            if !max_page.is_null() {
                *max_page = max;
            }
            if !page_from.is_null() {
                *page_from = from;
            }
            if !page_to.is_null() {
                *page_to = to;
            }
        }
    }
}

impl<T: Printout> Drop for PrintoutProxy<T> {
    fn drop(&mut self) {
        unsafe { ffi::wxd_Printout_Destroy(self.ffi_ptr) };
    }
}

// --- PrintData ---

pub struct PrintData {
    pub(crate) ffi_ptr: *mut ffi::wxd_PrintData_t,
    owned: bool,
}

impl Default for PrintData {
    fn default() -> Self {
        Self::new()
    }
}

impl PrintData {
    pub fn new() -> Self {
        Self {
            ffi_ptr: unsafe { ffi::wxd_PrintData_Create() },
            owned: true,
        }
    }

    pub fn is_ok(&self) -> bool {
        unsafe { ffi::wxd_PrintData_IsOk(self.ffi_ptr) }
    }
}

impl Drop for PrintData {
    fn drop(&mut self) {
        if self.owned {
            unsafe { ffi::wxd_PrintData_Destroy(self.ffi_ptr) };
        }
    }
}

// --- PrintDialogData ---

pub struct PrintDialogData {
    pub(crate) ffi_ptr: *mut ffi::wxd_PrintDialogData_t,
    owned: bool,
}

impl Default for PrintDialogData {
    fn default() -> Self {
        Self::new()
    }
}

impl PrintDialogData {
    pub fn new() -> Self {
        Self {
            ffi_ptr: unsafe { ffi::wxd_PrintDialogData_Create() },
            owned: true,
        }
    }

    pub fn from_data(data: &PrintData) -> Self {
        Self {
            ffi_ptr: unsafe { ffi::wxd_PrintDialogData_CreateFromData(data.ffi_ptr) },
            owned: true,
        }
    }

    pub fn get_print_data(&self) -> PrintData {
        PrintData {
            ffi_ptr: unsafe { ffi::wxd_PrintDialogData_GetPrintData(self.ffi_ptr) },
            owned: false,
        }
    }
}

impl Drop for PrintDialogData {
    fn drop(&mut self) {
        if self.owned {
            unsafe { ffi::wxd_PrintDialogData_Destroy(self.ffi_ptr) };
        }
    }
}

// --- PageSetupDialogData ---

pub struct PageSetupDialogData {
    pub(crate) ffi_ptr: *mut ffi::wxd_PageSetupDialogData_t,
    owned: bool,
}

impl Default for PageSetupDialogData {
    fn default() -> Self {
        Self::new()
    }
}

impl PageSetupDialogData {
    pub fn new() -> Self {
        Self {
            ffi_ptr: unsafe { ffi::wxd_PageSetupDialogData_Create() },
            owned: true,
        }
    }

    pub fn from_data(data: &PrintData) -> Self {
        Self {
            ffi_ptr: unsafe { ffi::wxd_PageSetupDialogData_CreateFromData(data.ffi_ptr) },
            owned: true,
        }
    }

    pub fn get_print_data(&self) -> PrintData {
        PrintData {
            ffi_ptr: unsafe { ffi::wxd_PageSetupDialogData_GetPrintData(self.ffi_ptr) },
            owned: false,
        }
    }
}

impl Drop for PageSetupDialogData {
    fn drop(&mut self) {
        if self.owned {
            unsafe { ffi::wxd_PageSetupDialogData_Destroy(self.ffi_ptr) };
        }
    }
}

// --- Printer ---

pub struct Printer {
    ffi_ptr: *mut ffi::wxd_Printer_t,
}

impl Printer {
    pub fn new(data: Option<&PrintDialogData>) -> Self {
        let ffi_ptr = unsafe { ffi::wxd_Printer_Create(data.map_or(ptr::null_mut(), |d| d.ffi_ptr)) };
        Self { ffi_ptr }
    }

    pub fn print<T: Printout, W: WxWidget>(&mut self, parent: Option<&W>, title: &str, printout: T, prompt: bool) -> bool {
        let proxy = PrintoutProxy::new(title, printout);
        unsafe {
            ffi::wxd_Printer_Print(
                self.ffi_ptr,
                parent.map_or(ptr::null_mut(), |p| p.handle_ptr()),
                proxy.ffi_ptr,
                prompt,
            )
        }
    }

    pub fn get_print_dialog_data(&self) -> PrintDialogData {
        PrintDialogData {
            ffi_ptr: unsafe { ffi::wxd_Printer_GetPrintDialogData(self.ffi_ptr) },
            owned: false,
        }
    }
}

impl Drop for Printer {
    fn drop(&mut self) {
        unsafe { ffi::wxd_Printer_Destroy(self.ffi_ptr) };
    }
}

// --- PrintDialog ---

pub struct PrintDialog {
    ffi_ptr: *mut ffi::wxd_PrintDialog_t,
}

impl PrintDialog {
    pub fn new<W: WxWidget>(parent: Option<&W>, data: Option<&PrintDialogData>) -> Self {
        Self {
            ffi_ptr: unsafe {
                ffi::wxd_PrintDialog_Create(
                    parent.map_or(ptr::null_mut(), |p| p.handle_ptr()),
                    data.map_or(ptr::null_mut(), |d| d.ffi_ptr),
                )
            },
        }
    }

    pub fn show_modal(&self) -> i32 {
        unsafe { ffi::wxd_PrintDialog_ShowModal(self.ffi_ptr) }
    }

    pub fn get_print_dialog_data(&self) -> PrintDialogData {
        PrintDialogData {
            ffi_ptr: unsafe { ffi::wxd_PrintDialog_GetPrintDialogData(self.ffi_ptr) },
            owned: false,
        }
    }

    pub fn get_print_dc(&self) -> Option<GenericDC> {
        let dc_ptr = unsafe { ffi::wxd_PrintDialog_GetPrintDC(self.ffi_ptr) };
        if dc_ptr.is_null() {
            None
        } else {
            // We assume the dialog owns the DC or it's returned as a fresh pointer.
            // In wxWidgets, GetPrintDC() returns a DC that the user SHOULD delete.
            Some(unsafe { GenericDC::from_ffi_ptr(dc_ptr) })
        }
    }
}

impl Drop for PrintDialog {
    fn drop(&mut self) {
        unsafe { ffi::wxd_PrintDialog_Destroy(self.ffi_ptr) };
    }
}

// --- PageSetupDialog ---

pub struct PageSetupDialog {
    ffi_ptr: *mut ffi::wxd_PageSetupDialog_t,
}

impl PageSetupDialog {
    pub fn new<W: WxWidget>(parent: Option<&W>, data: Option<&PageSetupDialogData>) -> Self {
        Self {
            ffi_ptr: unsafe {
                ffi::wxd_PageSetupDialog_Create(
                    parent.map_or(ptr::null_mut(), |p| p.handle_ptr()),
                    data.map_or(ptr::null_mut(), |d| d.ffi_ptr),
                )
            },
        }
    }

    pub fn show_modal(&self) -> i32 {
        unsafe { ffi::wxd_PageSetupDialog_ShowModal(self.ffi_ptr) }
    }

    pub fn get_page_setup_dialog_data(&self) -> PageSetupDialogData {
        PageSetupDialogData {
            ffi_ptr: unsafe { ffi::wxd_PageSetupDialog_GetPageSetupDialogData(self.ffi_ptr) },
            owned: false,
        }
    }
}

impl Drop for PageSetupDialog {
    fn drop(&mut self) {
        unsafe { ffi::wxd_PageSetupDialog_Destroy(self.ffi_ptr) };
    }
}

// --- Printout Utils ---

impl<T: Printout> PrintoutProxy<T> {
    pub fn get_dc(&self) -> GenericDC {
        unsafe { GenericDC::from_ffi_ptr_unowned(ffi::wxd_Printout_GetDC(self.ffi_ptr)) }
    }

    #[allow(dead_code)]
    pub fn get_page_size_pixels(&self) -> (i32, i32) {
        let mut w = 0;
        let mut h = 0;
        unsafe { ffi::wxd_Printout_GetPageSizePixels(self.ffi_ptr, &mut w, &mut h) };
        (w, h)
    }

    #[allow(dead_code)]
    pub fn get_page_size_mm(&self) -> (i32, i32) {
        let mut w = 0;
        let mut h = 0;
        unsafe { ffi::wxd_Printout_GetPageSizeMM(self.ffi_ptr, &mut w, &mut h) };
        (w, h)
    }

    #[allow(dead_code)]
    pub fn get_ppi_screen(&self) -> (i32, i32) {
        let mut x = 0;
        let mut y = 0;
        unsafe { ffi::wxd_Printout_GetPPIScreen(self.ffi_ptr, &mut x, &mut y) };
        (x, y)
    }

    #[allow(dead_code)]
    pub fn get_ppi_printer(&self) -> (i32, i32) {
        let mut x = 0;
        let mut y = 0;
        unsafe { ffi::wxd_Printout_GetPPIPrinter(self.ffi_ptr, &mut x, &mut y) };
        (x, y)
    }

    #[allow(dead_code)]
    pub fn is_preview(&self) -> bool {
        unsafe { ffi::wxd_Printout_IsPreview(self.ffi_ptr) }
    }
}
