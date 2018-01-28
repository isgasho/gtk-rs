// This file was generated by gir (9261d77) from gir-files (469db10)
// DO NOT EDIT

use BufferedInputStream;
use Cancellable;
use DataStreamByteOrder;
use DataStreamNewlineType;
use Error;
use FilterInputStream;
use InputStream;
use ffi;
use glib;
use glib::object::Downcast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::boxed::Box as Box_;
use std::mem;
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct DataInputStream(Object<ffi::GDataInputStream, ffi::GDataInputStreamClass>): BufferedInputStream, FilterInputStream, InputStream;

    match fn {
        get_type => || ffi::g_data_input_stream_get_type(),
    }
}

impl DataInputStream {
    pub fn new<P: IsA<InputStream>>(base_stream: &P) -> DataInputStream {
        unsafe {
            from_glib_full(ffi::g_data_input_stream_new(base_stream.to_glib_none().0))
        }
    }
}

pub trait DataInputStreamExt {
    fn get_byte_order(&self) -> DataStreamByteOrder;

    fn get_newline_type(&self) -> DataStreamNewlineType;

    fn read_byte<'a, P: Into<Option<&'a Cancellable>>>(&self, cancellable: P) -> Result<u8, Error>;

    fn read_int16<'a, P: Into<Option<&'a Cancellable>>>(&self, cancellable: P) -> Result<i16, Error>;

    fn read_int32<'a, P: Into<Option<&'a Cancellable>>>(&self, cancellable: P) -> Result<i32, Error>;

    fn read_int64<'a, P: Into<Option<&'a Cancellable>>>(&self, cancellable: P) -> Result<i64, Error>;

    fn read_line_async<'a, P: Into<Option<&'a Cancellable>>, Q: FnOnce(Result<usize, Error>) + Send + 'static>(&self, io_priority: i32, cancellable: P, callback: Q);

    //fn read_line_finish_utf8<P: IsA</*Ignored*/AsyncResult>>(&self, result: &P) -> Result<(Option<String>, usize), Error>;

    fn read_line_utf8<'a, P: Into<Option<&'a Cancellable>>>(&self, cancellable: P) -> Result<(Option<String>, usize), Error>;

    fn read_uint16<'a, P: Into<Option<&'a Cancellable>>>(&self, cancellable: P) -> Result<u16, Error>;

    fn read_uint32<'a, P: Into<Option<&'a Cancellable>>>(&self, cancellable: P) -> Result<u32, Error>;

    fn read_uint64<'a, P: Into<Option<&'a Cancellable>>>(&self, cancellable: P) -> Result<u64, Error>;

    fn read_until<'a, P: Into<Option<&'a Cancellable>>>(&self, stop_chars: &str, cancellable: P) -> Result<(String, usize), Error>;

    fn read_until_async<'a, P: Into<Option<&'a Cancellable>>, Q: FnOnce(Result<usize, Error>) + Send + 'static>(&self, stop_chars: &str, io_priority: i32, cancellable: P, callback: Q);

    fn read_upto<'a, P: Into<Option<&'a Cancellable>>>(&self, stop_chars: &str, cancellable: P) -> Result<(String, usize), Error>;

    fn read_upto_async<'a, P: Into<Option<&'a Cancellable>>, Q: FnOnce(Result<usize, Error>) + Send + 'static>(&self, stop_chars: &str, io_priority: i32, cancellable: P, callback: Q);

    fn set_byte_order(&self, order: DataStreamByteOrder);

    fn set_newline_type(&self, type_: DataStreamNewlineType);

    fn connect_property_byte_order_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_newline_type_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<DataInputStream> + IsA<glib::object::Object>> DataInputStreamExt for O {
    fn get_byte_order(&self) -> DataStreamByteOrder {
        unsafe {
            from_glib(ffi::g_data_input_stream_get_byte_order(self.to_glib_none().0))
        }
    }

    fn get_newline_type(&self) -> DataStreamNewlineType {
        unsafe {
            from_glib(ffi::g_data_input_stream_get_newline_type(self.to_glib_none().0))
        }
    }

    fn read_byte<'a, P: Into<Option<&'a Cancellable>>>(&self, cancellable: P) -> Result<u8, Error> {
        let cancellable = cancellable.into();
        let cancellable = cancellable.to_glib_none();
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::g_data_input_stream_read_byte(self.to_glib_none().0, cancellable.0, &mut error);
            if error.is_null() { Ok(ret) } else { Err(from_glib_full(error)) }
        }
    }

    fn read_int16<'a, P: Into<Option<&'a Cancellable>>>(&self, cancellable: P) -> Result<i16, Error> {
        let cancellable = cancellable.into();
        let cancellable = cancellable.to_glib_none();
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::g_data_input_stream_read_int16(self.to_glib_none().0, cancellable.0, &mut error);
            if error.is_null() { Ok(ret) } else { Err(from_glib_full(error)) }
        }
    }

    fn read_int32<'a, P: Into<Option<&'a Cancellable>>>(&self, cancellable: P) -> Result<i32, Error> {
        let cancellable = cancellable.into();
        let cancellable = cancellable.to_glib_none();
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::g_data_input_stream_read_int32(self.to_glib_none().0, cancellable.0, &mut error);
            if error.is_null() { Ok(ret) } else { Err(from_glib_full(error)) }
        }
    }

    fn read_int64<'a, P: Into<Option<&'a Cancellable>>>(&self, cancellable: P) -> Result<i64, Error> {
        let cancellable = cancellable.into();
        let cancellable = cancellable.to_glib_none();
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::g_data_input_stream_read_int64(self.to_glib_none().0, cancellable.0, &mut error);
            if error.is_null() { Ok(ret) } else { Err(from_glib_full(error)) }
        }
    }

    fn read_line_async<'a, P: Into<Option<&'a Cancellable>>, Q: FnOnce(Result<usize, Error>) + Send + 'static>(&self, io_priority: i32, cancellable: P, callback: Q) {
        let cancellable = cancellable.into();
        let cancellable = cancellable.to_glib_none();
        let user_data: Box<Box<Q>> = Box::new(Box::new(callback));
        unsafe extern "C" fn read_line_async_trampoline<Q: FnOnce(Result<usize, Error>) + Send + 'static>(_source_object: *mut gobject_ffi::GObject, res: *mut ffi::GAsyncResult, user_data: glib_ffi::gpointer)
        {
            callback_guard!();
            let mut error = ptr::null_mut();
            let mut length = mem::uninitialized();
            let _ = ffi::g_data_input_stream_read_line_finish(_source_object as *mut _, res, &mut length, &mut error);
            let result = if error.is_null() { Ok(length) } else { Err(from_glib_full(error)) };
            let callback: Box<Box<Q>> = Box::from_raw(user_data as *mut _);
            callback(result);
        }
        let callback = read_line_async_trampoline::<Q>;
        unsafe {
            ffi::g_data_input_stream_read_line_async(self.to_glib_none().0, io_priority, cancellable.0, Some(callback), Box::into_raw(user_data) as *mut _);
        }
    }

    //fn read_line_finish_utf8<P: IsA</*Ignored*/AsyncResult>>(&self, result: &P) -> Result<(Option<String>, usize), Error> {
    //    unsafe { TODO: call ffi::g_data_input_stream_read_line_finish_utf8() }
    //}

    fn read_line_utf8<'a, P: Into<Option<&'a Cancellable>>>(&self, cancellable: P) -> Result<(Option<String>, usize), Error> {
        let cancellable = cancellable.into();
        let cancellable = cancellable.to_glib_none();
        unsafe {
            let mut length = mem::uninitialized();
            let mut error = ptr::null_mut();
            let ret = ffi::g_data_input_stream_read_line_utf8(self.to_glib_none().0, &mut length, cancellable.0, &mut error);
            if error.is_null() { Ok((from_glib_full(ret), length)) } else { Err(from_glib_full(error)) }
        }
    }

    fn read_uint16<'a, P: Into<Option<&'a Cancellable>>>(&self, cancellable: P) -> Result<u16, Error> {
        let cancellable = cancellable.into();
        let cancellable = cancellable.to_glib_none();
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::g_data_input_stream_read_uint16(self.to_glib_none().0, cancellable.0, &mut error);
            if error.is_null() { Ok(ret) } else { Err(from_glib_full(error)) }
        }
    }

    fn read_uint32<'a, P: Into<Option<&'a Cancellable>>>(&self, cancellable: P) -> Result<u32, Error> {
        let cancellable = cancellable.into();
        let cancellable = cancellable.to_glib_none();
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::g_data_input_stream_read_uint32(self.to_glib_none().0, cancellable.0, &mut error);
            if error.is_null() { Ok(ret) } else { Err(from_glib_full(error)) }
        }
    }

    fn read_uint64<'a, P: Into<Option<&'a Cancellable>>>(&self, cancellable: P) -> Result<u64, Error> {
        let cancellable = cancellable.into();
        let cancellable = cancellable.to_glib_none();
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::g_data_input_stream_read_uint64(self.to_glib_none().0, cancellable.0, &mut error);
            if error.is_null() { Ok(ret) } else { Err(from_glib_full(error)) }
        }
    }

    fn read_until<'a, P: Into<Option<&'a Cancellable>>>(&self, stop_chars: &str, cancellable: P) -> Result<(String, usize), Error> {
        let cancellable = cancellable.into();
        let cancellable = cancellable.to_glib_none();
        unsafe {
            let mut length = mem::uninitialized();
            let mut error = ptr::null_mut();
            let ret = ffi::g_data_input_stream_read_until(self.to_glib_none().0, stop_chars.to_glib_none().0, &mut length, cancellable.0, &mut error);
            if error.is_null() { Ok((from_glib_full(ret), length)) } else { Err(from_glib_full(error)) }
        }
    }

    fn read_until_async<'a, P: Into<Option<&'a Cancellable>>, Q: FnOnce(Result<usize, Error>) + Send + 'static>(&self, stop_chars: &str, io_priority: i32, cancellable: P, callback: Q) {
        let cancellable = cancellable.into();
        let cancellable = cancellable.to_glib_none();
        let user_data: Box<Box<Q>> = Box::new(Box::new(callback));
        unsafe extern "C" fn read_until_async_trampoline<Q: FnOnce(Result<usize, Error>) + Send + 'static>(_source_object: *mut gobject_ffi::GObject, res: *mut ffi::GAsyncResult, user_data: glib_ffi::gpointer)
        {
            callback_guard!();
            let mut error = ptr::null_mut();
            let mut length = mem::uninitialized();
            let _ = ffi::g_data_input_stream_read_until_finish(_source_object as *mut _, res, &mut length, &mut error);
            let result = if error.is_null() { Ok(length) } else { Err(from_glib_full(error)) };
            let callback: Box<Box<Q>> = Box::from_raw(user_data as *mut _);
            callback(result);
        }
        let callback = read_until_async_trampoline::<Q>;
        unsafe {
            ffi::g_data_input_stream_read_until_async(self.to_glib_none().0, stop_chars.to_glib_none().0, io_priority, cancellable.0, Some(callback), Box::into_raw(user_data) as *mut _);
        }
    }

    fn read_upto<'a, P: Into<Option<&'a Cancellable>>>(&self, stop_chars: &str, cancellable: P) -> Result<(String, usize), Error> {
        let cancellable = cancellable.into();
        let cancellable = cancellable.to_glib_none();
        let stop_chars_len = stop_chars.len() as isize;
        unsafe {
            let mut length = mem::uninitialized();
            let mut error = ptr::null_mut();
            let ret = ffi::g_data_input_stream_read_upto(self.to_glib_none().0, stop_chars.to_glib_none().0, stop_chars_len, &mut length, cancellable.0, &mut error);
            if error.is_null() { Ok((from_glib_full(ret), length)) } else { Err(from_glib_full(error)) }
        }
    }

    fn read_upto_async<'a, P: Into<Option<&'a Cancellable>>, Q: FnOnce(Result<usize, Error>) + Send + 'static>(&self, stop_chars: &str, io_priority: i32, cancellable: P, callback: Q) {
        let cancellable = cancellable.into();
        let cancellable = cancellable.to_glib_none();
        let stop_chars_len = stop_chars.len() as isize;
        let user_data: Box<Box<Q>> = Box::new(Box::new(callback));
        unsafe extern "C" fn read_upto_async_trampoline<Q: FnOnce(Result<usize, Error>) + Send + 'static>(_source_object: *mut gobject_ffi::GObject, res: *mut ffi::GAsyncResult, user_data: glib_ffi::gpointer)
        {
            callback_guard!();
            let mut error = ptr::null_mut();
            let mut length = mem::uninitialized();
            let _ = ffi::g_data_input_stream_read_upto_finish(_source_object as *mut _, res, &mut length, &mut error);
            let result = if error.is_null() { Ok(length) } else { Err(from_glib_full(error)) };
            let callback: Box<Box<Q>> = Box::from_raw(user_data as *mut _);
            callback(result);
        }
        let callback = read_upto_async_trampoline::<Q>;
        unsafe {
            ffi::g_data_input_stream_read_upto_async(self.to_glib_none().0, stop_chars.to_glib_none().0, stop_chars_len, io_priority, cancellable.0, Some(callback), Box::into_raw(user_data) as *mut _);
        }
    }

    fn set_byte_order(&self, order: DataStreamByteOrder) {
        unsafe {
            ffi::g_data_input_stream_set_byte_order(self.to_glib_none().0, order.to_glib());
        }
    }

    fn set_newline_type(&self, type_: DataStreamNewlineType) {
        unsafe {
            ffi::g_data_input_stream_set_newline_type(self.to_glib_none().0, type_.to_glib());
        }
    }

    fn connect_property_byte_order_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::byte-order",
                transmute(notify_byte_order_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_newline_type_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::newline-type",
                transmute(notify_newline_type_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn notify_byte_order_trampoline<P>(this: *mut ffi::GDataInputStream, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DataInputStream> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DataInputStream::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_newline_type_trampoline<P>(this: *mut ffi::GDataInputStream, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DataInputStream> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DataInputStream::from_glib_borrow(this).downcast_unchecked())
}
