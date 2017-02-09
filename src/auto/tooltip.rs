// This file was generated by gir (29b5d65) from gir-files (71d73f0)
// DO NOT EDIT

use Widget;
use ffi;
use gdk;
use gdk_pixbuf;
use glib::object::IsA;
use glib::translate::*;

glib_wrapper! {
    pub struct Tooltip(Object<ffi::GtkTooltip>);

    match fn {
        get_type => || ffi::gtk_tooltip_get_type(),
    }
}

impl Tooltip {
    pub fn set_custom<T: IsA<Widget>>(&self, custom_widget: Option<&T>) {
        unsafe {
            ffi::gtk_tooltip_set_custom(self.to_glib_none().0, custom_widget.to_glib_none().0);
        }
    }

    pub fn set_icon(&self, pixbuf: Option<&gdk_pixbuf::Pixbuf>) {
        unsafe {
            ffi::gtk_tooltip_set_icon(self.to_glib_none().0, pixbuf.to_glib_none().0);
        }
    }

    //pub fn set_icon_from_gicon<T: IsA</*Ignored*/gio::Icon>>(&self, gicon: Option<&T>, size: i32) {
    //    unsafe { TODO: call ffi::gtk_tooltip_set_icon_from_gicon() }
    //}

    pub fn set_icon_from_icon_name<'a, T: Into<Option<&'a str>>>(&self, icon_name: T, size: i32) {
        unsafe {
            ffi::gtk_tooltip_set_icon_from_icon_name(self.to_glib_none().0, icon_name.into().to_glib_none().0, size);
        }
    }

    pub fn set_icon_from_stock<'a, T: Into<Option<&'a str>>>(&self, stock_id: T, size: i32) {
        unsafe {
            ffi::gtk_tooltip_set_icon_from_stock(self.to_glib_none().0, stock_id.into().to_glib_none().0, size);
        }
    }

    pub fn set_markup<'a, T: Into<Option<&'a str>>>(&self, markup: T) {
        unsafe {
            ffi::gtk_tooltip_set_markup(self.to_glib_none().0, markup.into().to_glib_none().0);
        }
    }

    pub fn set_text<'a, T: Into<Option<&'a str>>>(&self, text: T) {
        unsafe {
            ffi::gtk_tooltip_set_text(self.to_glib_none().0, text.into().to_glib_none().0);
        }
    }

    pub fn set_tip_area(&self, rect: &gdk::Rectangle) {
        unsafe {
            ffi::gtk_tooltip_set_tip_area(self.to_glib_none().0, rect.to_glib_none().0);
        }
    }

    pub fn trigger_tooltip_query(display: &gdk::Display) {
        assert_initialized_main_thread!();
        unsafe {
            ffi::gtk_tooltip_trigger_tooltip_query(display.to_glib_none().0);
        }
    }
}
