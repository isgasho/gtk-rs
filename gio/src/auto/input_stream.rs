// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use gio_sys;
use glib;
use glib::object::IsA;
use glib::translate::*;
use glib_sys;
use gobject_sys;
use std::boxed::Box as Box_;
use std::fmt;
use std::pin::Pin;
use std::ptr;
use Cancellable;

glib_wrapper! {
    pub struct InputStream(Object<gio_sys::GInputStream, gio_sys::GInputStreamClass, InputStreamClass>);

    match fn {
        get_type => || gio_sys::g_input_stream_get_type(),
    }
}

pub const NONE_INPUT_STREAM: Option<&InputStream> = None;

pub trait InputStreamExt: 'static {
    fn clear_pending(&self);

    fn close<P: IsA<Cancellable>>(&self, cancellable: Option<&P>) -> Result<(), glib::Error>;

    fn close_async<P: IsA<Cancellable>, Q: FnOnce(Result<(), glib::Error>) + Send + 'static>(
        &self,
        io_priority: glib::Priority,
        cancellable: Option<&P>,
        callback: Q,
    );

    fn close_async_future(
        &self,
        io_priority: glib::Priority,
    ) -> Pin<Box_<dyn std::future::Future<Output = Result<(), glib::Error>> + 'static>>;

    fn has_pending(&self) -> bool;

    fn is_closed(&self) -> bool;

    fn read_bytes<P: IsA<Cancellable>>(
        &self,
        count: usize,
        cancellable: Option<&P>,
    ) -> Result<glib::Bytes, glib::Error>;

    fn read_bytes_async<
        P: IsA<Cancellable>,
        Q: FnOnce(Result<glib::Bytes, glib::Error>) + Send + 'static,
    >(
        &self,
        count: usize,
        io_priority: glib::Priority,
        cancellable: Option<&P>,
        callback: Q,
    );

    fn read_bytes_async_future(
        &self,
        count: usize,
        io_priority: glib::Priority,
    ) -> Pin<Box_<dyn std::future::Future<Output = Result<glib::Bytes, glib::Error>> + 'static>>;

    fn set_pending(&self) -> Result<(), glib::Error>;

    fn skip<P: IsA<Cancellable>>(
        &self,
        count: usize,
        cancellable: Option<&P>,
    ) -> Result<isize, glib::Error>;

    fn skip_async<P: IsA<Cancellable>, Q: FnOnce(Result<isize, glib::Error>) + Send + 'static>(
        &self,
        count: usize,
        io_priority: glib::Priority,
        cancellable: Option<&P>,
        callback: Q,
    );

    fn skip_async_future(
        &self,
        count: usize,
        io_priority: glib::Priority,
    ) -> Pin<Box_<dyn std::future::Future<Output = Result<isize, glib::Error>> + 'static>>;
}

impl<O: IsA<InputStream>> InputStreamExt for O {
    fn clear_pending(&self) {
        unsafe {
            gio_sys::g_input_stream_clear_pending(self.as_ref().to_glib_none().0);
        }
    }

    fn close<P: IsA<Cancellable>>(&self, cancellable: Option<&P>) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = gio_sys::g_input_stream_close(
                self.as_ref().to_glib_none().0,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn close_async<P: IsA<Cancellable>, Q: FnOnce(Result<(), glib::Error>) + Send + 'static>(
        &self,
        io_priority: glib::Priority,
        cancellable: Option<&P>,
        callback: Q,
    ) {
        let user_data: Box_<Q> = Box_::new(callback);
        unsafe extern "C" fn close_async_trampoline<
            Q: FnOnce(Result<(), glib::Error>) + Send + 'static,
        >(
            _source_object: *mut gobject_sys::GObject,
            res: *mut gio_sys::GAsyncResult,
            user_data: glib_sys::gpointer,
        ) {
            let mut error = ptr::null_mut();
            let _ = gio_sys::g_input_stream_close_finish(_source_object as *mut _, res, &mut error);
            let result = if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            };
            let callback: Box_<Q> = Box_::from_raw(user_data as *mut _);
            callback(result);
        }
        let callback = close_async_trampoline::<Q>;
        unsafe {
            gio_sys::g_input_stream_close_async(
                self.as_ref().to_glib_none().0,
                io_priority.to_glib(),
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                Some(callback),
                Box_::into_raw(user_data) as *mut _,
            );
        }
    }

    fn close_async_future(
        &self,
        io_priority: glib::Priority,
    ) -> Pin<Box_<dyn std::future::Future<Output = Result<(), glib::Error>> + 'static>> {
        Box_::pin(crate::GioFuture::new(self, move |obj, send| {
            let cancellable = Cancellable::new();
            obj.close_async(io_priority, Some(&cancellable), move |res| {
                send.resolve(res);
            });

            cancellable
        }))
    }

    fn has_pending(&self) -> bool {
        unsafe {
            from_glib(gio_sys::g_input_stream_has_pending(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn is_closed(&self) -> bool {
        unsafe {
            from_glib(gio_sys::g_input_stream_is_closed(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn read_bytes<P: IsA<Cancellable>>(
        &self,
        count: usize,
        cancellable: Option<&P>,
    ) -> Result<glib::Bytes, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = gio_sys::g_input_stream_read_bytes(
                self.as_ref().to_glib_none().0,
                count,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn read_bytes_async<
        P: IsA<Cancellable>,
        Q: FnOnce(Result<glib::Bytes, glib::Error>) + Send + 'static,
    >(
        &self,
        count: usize,
        io_priority: glib::Priority,
        cancellable: Option<&P>,
        callback: Q,
    ) {
        let user_data: Box_<Q> = Box_::new(callback);
        unsafe extern "C" fn read_bytes_async_trampoline<
            Q: FnOnce(Result<glib::Bytes, glib::Error>) + Send + 'static,
        >(
            _source_object: *mut gobject_sys::GObject,
            res: *mut gio_sys::GAsyncResult,
            user_data: glib_sys::gpointer,
        ) {
            let mut error = ptr::null_mut();
            let ret = gio_sys::g_input_stream_read_bytes_finish(
                _source_object as *mut _,
                res,
                &mut error,
            );
            let result = if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            };
            let callback: Box_<Q> = Box_::from_raw(user_data as *mut _);
            callback(result);
        }
        let callback = read_bytes_async_trampoline::<Q>;
        unsafe {
            gio_sys::g_input_stream_read_bytes_async(
                self.as_ref().to_glib_none().0,
                count,
                io_priority.to_glib(),
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                Some(callback),
                Box_::into_raw(user_data) as *mut _,
            );
        }
    }

    fn read_bytes_async_future(
        &self,
        count: usize,
        io_priority: glib::Priority,
    ) -> Pin<Box_<dyn std::future::Future<Output = Result<glib::Bytes, glib::Error>> + 'static>>
    {
        Box_::pin(crate::GioFuture::new(self, move |obj, send| {
            let cancellable = Cancellable::new();
            obj.read_bytes_async(count, io_priority, Some(&cancellable), move |res| {
                send.resolve(res);
            });

            cancellable
        }))
    }

    fn set_pending(&self) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = gio_sys::g_input_stream_set_pending(self.as_ref().to_glib_none().0, &mut error);
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn skip<P: IsA<Cancellable>>(
        &self,
        count: usize,
        cancellable: Option<&P>,
    ) -> Result<isize, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = gio_sys::g_input_stream_skip(
                self.as_ref().to_glib_none().0,
                count,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(ret)
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn skip_async<P: IsA<Cancellable>, Q: FnOnce(Result<isize, glib::Error>) + Send + 'static>(
        &self,
        count: usize,
        io_priority: glib::Priority,
        cancellable: Option<&P>,
        callback: Q,
    ) {
        let user_data: Box_<Q> = Box_::new(callback);
        unsafe extern "C" fn skip_async_trampoline<
            Q: FnOnce(Result<isize, glib::Error>) + Send + 'static,
        >(
            _source_object: *mut gobject_sys::GObject,
            res: *mut gio_sys::GAsyncResult,
            user_data: glib_sys::gpointer,
        ) {
            let mut error = ptr::null_mut();
            let ret =
                gio_sys::g_input_stream_skip_finish(_source_object as *mut _, res, &mut error);
            let result = if error.is_null() {
                Ok(ret)
            } else {
                Err(from_glib_full(error))
            };
            let callback: Box_<Q> = Box_::from_raw(user_data as *mut _);
            callback(result);
        }
        let callback = skip_async_trampoline::<Q>;
        unsafe {
            gio_sys::g_input_stream_skip_async(
                self.as_ref().to_glib_none().0,
                count,
                io_priority.to_glib(),
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                Some(callback),
                Box_::into_raw(user_data) as *mut _,
            );
        }
    }

    fn skip_async_future(
        &self,
        count: usize,
        io_priority: glib::Priority,
    ) -> Pin<Box_<dyn std::future::Future<Output = Result<isize, glib::Error>> + 'static>> {
        Box_::pin(crate::GioFuture::new(self, move |obj, send| {
            let cancellable = Cancellable::new();
            obj.skip_async(count, io_priority, Some(&cancellable), move |res| {
                send.resolve(res);
            });

            cancellable
        }))
    }
}

impl fmt::Display for InputStream {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "InputStream")
    }
}