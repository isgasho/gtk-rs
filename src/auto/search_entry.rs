// This file was generated by gir (29b5d65) from gir-files (71d73f0)
// DO NOT EDIT

use CellEditable;
use Editable;
use Entry;
use Widget;
use ffi;
#[cfg(feature = "v3_16")]
use gdk;
use glib::object::Downcast;
#[cfg(feature = "v3_10")]
use glib::signal::connect;
use glib::translate::*;
#[cfg(feature = "v3_10")]
use glib_ffi;
#[cfg(feature = "v3_10")]
use std::boxed::Box as Box_;
#[cfg(feature = "v3_10")]
use std::mem::transmute;

glib_wrapper! {
    pub struct SearchEntry(Object<ffi::GtkSearchEntry>): Entry, Widget, CellEditable, Editable;

    match fn {
        get_type => || ffi::gtk_search_entry_get_type(),
    }
}

impl SearchEntry {
    #[cfg(feature = "v3_6")]
    pub fn new() -> SearchEntry {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_search_entry_new()).downcast_unchecked()
        }
    }

    #[cfg(feature = "v3_16")]
    pub fn handle_event(&self, event: &gdk::Event) -> bool {
        unsafe {
            from_glib(ffi::gtk_search_entry_handle_event(self.to_glib_none().0, mut_override(event.to_glib_none().0)))
        }
    }

    #[cfg(feature = "v3_16")]
    pub fn connect_next_match<F: Fn(&SearchEntry) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&SearchEntry) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "next-match",
                transmute(next_match_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }

    #[cfg(feature = "v3_16")]
    pub fn connect_previous_match<F: Fn(&SearchEntry) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&SearchEntry) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "previous-match",
                transmute(previous_match_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }

    #[cfg(feature = "v3_10")]
    pub fn connect_search_changed<F: Fn(&SearchEntry) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&SearchEntry) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "search-changed",
                transmute(search_changed_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }

    #[cfg(feature = "v3_16")]
    pub fn connect_stop_search<F: Fn(&SearchEntry) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&SearchEntry) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "stop-search",
                transmute(stop_search_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

#[cfg(feature = "v3_16")]
unsafe extern "C" fn next_match_trampoline(this: *mut ffi::GtkSearchEntry, f: glib_ffi::gpointer) {
    callback_guard!();
    let f: &Box_<Fn(&SearchEntry) + 'static> = transmute(f);
    f(&from_glib_none(this))
}

#[cfg(feature = "v3_16")]
unsafe extern "C" fn previous_match_trampoline(this: *mut ffi::GtkSearchEntry, f: glib_ffi::gpointer) {
    callback_guard!();
    let f: &Box_<Fn(&SearchEntry) + 'static> = transmute(f);
    f(&from_glib_none(this))
}

#[cfg(feature = "v3_10")]
unsafe extern "C" fn search_changed_trampoline(this: *mut ffi::GtkSearchEntry, f: glib_ffi::gpointer) {
    callback_guard!();
    let f: &Box_<Fn(&SearchEntry) + 'static> = transmute(f);
    f(&from_glib_none(this))
}

#[cfg(feature = "v3_16")]
unsafe extern "C" fn stop_search_trampoline(this: *mut ffi::GtkSearchEntry, f: glib_ffi::gpointer) {
    callback_guard!();
    let f: &Box_<Fn(&SearchEntry) + 'static> = transmute(f);
    f(&from_glib_none(this))
}
