// This file was generated by gir (29b5d65) from gir-files (71d73f0)
// DO NOT EDIT

use Container;
use MenuShell;
use PackDirection;
use Widget;
use ffi;
use gio;
use glib::object::Downcast;
use glib::object::IsA;
use glib::translate::*;

glib_wrapper! {
    pub struct MenuBar(Object<ffi::GtkMenuBar>): MenuShell, Container, Widget;

    match fn {
        get_type => || ffi::gtk_menu_bar_get_type(),
    }
}

impl MenuBar {
    pub fn new() -> MenuBar {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_menu_bar_new()).downcast_unchecked()
        }
    }

    pub fn new_from_model<T: IsA<gio::MenuModel>>(model: &T) -> MenuBar {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_menu_bar_new_from_model(model.to_glib_none().0)).downcast_unchecked()
        }
    }

    pub fn get_child_pack_direction(&self) -> PackDirection {
        unsafe {
            from_glib(ffi::gtk_menu_bar_get_child_pack_direction(self.to_glib_none().0))
        }
    }

    pub fn get_pack_direction(&self) -> PackDirection {
        unsafe {
            from_glib(ffi::gtk_menu_bar_get_pack_direction(self.to_glib_none().0))
        }
    }

    pub fn set_child_pack_direction(&self, child_pack_dir: PackDirection) {
        unsafe {
            ffi::gtk_menu_bar_set_child_pack_direction(self.to_glib_none().0, child_pack_dir.to_glib());
        }
    }

    pub fn set_pack_direction(&self, pack_dir: PackDirection) {
        unsafe {
            ffi::gtk_menu_bar_set_pack_direction(self.to_glib_none().0, pack_dir.to_glib());
        }
    }
}
