// This file was generated by gir (https://github.com/gtk-rs/gir @ 6855214)
// from gir-files (https://github.com/gtk-rs/gir-files @ cf55bdc)
// DO NOT EDIT

use Direction;
use Font;
use FontDescription;
use FontFamily;
use FontMap;
use FontMetrics;
use Fontset;
use Gravity;
use GravityHint;
use Language;
use Matrix;
use ffi;
use glib::object::IsA;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::mem;
use std::ptr;

glib_wrapper! {
    pub struct Context(Object<ffi::PangoContext, ffi::PangoContextClass>);

    match fn {
        get_type => || ffi::pango_context_get_type(),
    }
}

impl Context {
    pub fn new() -> Context {
        unsafe {
            from_glib_full(ffi::pango_context_new())
        }
    }
}

impl Default for Context {
    fn default() -> Self {
        Self::new()
    }
}

pub trait ContextExt {
    #[cfg(any(feature = "v1_32_4", feature = "dox"))]
    fn changed(&self);

    fn get_base_dir(&self) -> Direction;

    fn get_base_gravity(&self) -> Gravity;

    fn get_font_description(&self) -> Option<FontDescription>;

    fn get_font_map(&self) -> Option<FontMap>;

    fn get_gravity(&self) -> Gravity;

    fn get_gravity_hint(&self) -> GravityHint;

    fn get_language(&self) -> Option<Language>;

    fn get_matrix(&self) -> Option<Matrix>;

    fn get_metrics<'a, 'b, P: Into<Option<&'a FontDescription>>, Q: Into<Option<&'b Language>>>(&self, desc: P, language: Q) -> Option<FontMetrics>;

    #[cfg(any(feature = "v1_32_4", feature = "dox"))]
    fn get_serial(&self) -> u32;

    fn list_families(&self) -> Vec<FontFamily>;

    fn load_font(&self, desc: &FontDescription) -> Option<Font>;

    fn load_fontset(&self, desc: &FontDescription, language: &Language) -> Option<Fontset>;

    fn set_base_dir(&self, direction: Direction);

    fn set_base_gravity(&self, gravity: Gravity);

    fn set_font_description(&self, desc: &FontDescription);

    fn set_font_map(&self, font_map: &FontMap);

    fn set_gravity_hint(&self, hint: GravityHint);

    fn set_language(&self, language: &Language);

    fn set_matrix<'a, P: Into<Option<&'a Matrix>>>(&self, matrix: P);
}

impl<O: IsA<Context>> ContextExt for O {
    #[cfg(any(feature = "v1_32_4", feature = "dox"))]
    fn changed(&self) {
        unsafe {
            ffi::pango_context_changed(self.to_glib_none().0);
        }
    }

    fn get_base_dir(&self) -> Direction {
        unsafe {
            from_glib(ffi::pango_context_get_base_dir(self.to_glib_none().0))
        }
    }

    fn get_base_gravity(&self) -> Gravity {
        unsafe {
            from_glib(ffi::pango_context_get_base_gravity(self.to_glib_none().0))
        }
    }

    fn get_font_description(&self) -> Option<FontDescription> {
        unsafe {
            from_glib_none(ffi::pango_context_get_font_description(self.to_glib_none().0))
        }
    }

    fn get_font_map(&self) -> Option<FontMap> {
        unsafe {
            from_glib_none(ffi::pango_context_get_font_map(self.to_glib_none().0))
        }
    }

    fn get_gravity(&self) -> Gravity {
        unsafe {
            from_glib(ffi::pango_context_get_gravity(self.to_glib_none().0))
        }
    }

    fn get_gravity_hint(&self) -> GravityHint {
        unsafe {
            from_glib(ffi::pango_context_get_gravity_hint(self.to_glib_none().0))
        }
    }

    fn get_language(&self) -> Option<Language> {
        unsafe {
            from_glib_full(ffi::pango_context_get_language(self.to_glib_none().0))
        }
    }

    fn get_matrix(&self) -> Option<Matrix> {
        unsafe {
            from_glib_none(ffi::pango_context_get_matrix(self.to_glib_none().0))
        }
    }

    fn get_metrics<'a, 'b, P: Into<Option<&'a FontDescription>>, Q: Into<Option<&'b Language>>>(&self, desc: P, language: Q) -> Option<FontMetrics> {
        let desc = desc.into();
        let desc = desc.to_glib_none();
        let language = language.into();
        unsafe {
            from_glib_full(ffi::pango_context_get_metrics(self.to_glib_none().0, desc.0, mut_override(language.to_glib_none().0)))
        }
    }

    #[cfg(any(feature = "v1_32_4", feature = "dox"))]
    fn get_serial(&self) -> u32 {
        unsafe {
            ffi::pango_context_get_serial(self.to_glib_none().0)
        }
    }

    fn list_families(&self) -> Vec<FontFamily> {
        unsafe {
            let mut families = ptr::null_mut();
            let mut n_families = mem::uninitialized();
            ffi::pango_context_list_families(self.to_glib_none().0, &mut families, &mut n_families);
            FromGlibContainer::from_glib_container_num(families, n_families as usize)
        }
    }

    fn load_font(&self, desc: &FontDescription) -> Option<Font> {
        unsafe {
            from_glib_full(ffi::pango_context_load_font(self.to_glib_none().0, desc.to_glib_none().0))
        }
    }

    fn load_fontset(&self, desc: &FontDescription, language: &Language) -> Option<Fontset> {
        unsafe {
            from_glib_full(ffi::pango_context_load_fontset(self.to_glib_none().0, desc.to_glib_none().0, mut_override(language.to_glib_none().0)))
        }
    }

    fn set_base_dir(&self, direction: Direction) {
        unsafe {
            ffi::pango_context_set_base_dir(self.to_glib_none().0, direction.to_glib());
        }
    }

    fn set_base_gravity(&self, gravity: Gravity) {
        unsafe {
            ffi::pango_context_set_base_gravity(self.to_glib_none().0, gravity.to_glib());
        }
    }

    fn set_font_description(&self, desc: &FontDescription) {
        unsafe {
            ffi::pango_context_set_font_description(self.to_glib_none().0, desc.to_glib_none().0);
        }
    }

    fn set_font_map(&self, font_map: &FontMap) {
        unsafe {
            ffi::pango_context_set_font_map(self.to_glib_none().0, font_map.to_glib_none().0);
        }
    }

    fn set_gravity_hint(&self, hint: GravityHint) {
        unsafe {
            ffi::pango_context_set_gravity_hint(self.to_glib_none().0, hint.to_glib());
        }
    }

    fn set_language(&self, language: &Language) {
        unsafe {
            ffi::pango_context_set_language(self.to_glib_none().0, mut_override(language.to_glib_none().0));
        }
    }

    fn set_matrix<'a, P: Into<Option<&'a Matrix>>>(&self, matrix: P) {
        let matrix = matrix.into();
        let matrix = matrix.to_glib_none();
        unsafe {
            ffi::pango_context_set_matrix(self.to_glib_none().0, matrix.0);
        }
    }
}
