// This file was generated by gir (9261d77) from gir-files (469db10)
// DO NOT EDIT

use Cancellable;
use DataStreamByteOrder;
use Error;
use FilterOutputStream;
use OutputStream;
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
    pub struct DataOutputStream(Object<ffi::GDataOutputStream, ffi::GDataOutputStreamClass>): FilterOutputStream, OutputStream;

    match fn {
        get_type => || ffi::g_data_output_stream_get_type(),
    }
}

impl DataOutputStream {
    pub fn new<P: IsA<OutputStream>>(base_stream: &P) -> DataOutputStream {
        unsafe {
            from_glib_full(ffi::g_data_output_stream_new(base_stream.to_glib_none().0))
        }
    }
}

pub trait DataOutputStreamExt {
    fn get_byte_order(&self) -> DataStreamByteOrder;

    fn put_byte<'a, P: Into<Option<&'a Cancellable>>>(&self, data: u8, cancellable: P) -> Result<(), Error>;

    fn put_int16<'a, P: Into<Option<&'a Cancellable>>>(&self, data: i16, cancellable: P) -> Result<(), Error>;

    fn put_int32<'a, P: Into<Option<&'a Cancellable>>>(&self, data: i32, cancellable: P) -> Result<(), Error>;

    fn put_int64<'a, P: Into<Option<&'a Cancellable>>>(&self, data: i64, cancellable: P) -> Result<(), Error>;

    fn put_string<'a, P: Into<Option<&'a Cancellable>>>(&self, str: &str, cancellable: P) -> Result<(), Error>;

    fn put_uint16<'a, P: Into<Option<&'a Cancellable>>>(&self, data: u16, cancellable: P) -> Result<(), Error>;

    fn put_uint32<'a, P: Into<Option<&'a Cancellable>>>(&self, data: u32, cancellable: P) -> Result<(), Error>;

    fn put_uint64<'a, P: Into<Option<&'a Cancellable>>>(&self, data: u64, cancellable: P) -> Result<(), Error>;

    fn set_byte_order(&self, order: DataStreamByteOrder);

    fn connect_property_byte_order_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<DataOutputStream> + IsA<glib::object::Object>> DataOutputStreamExt for O {
    fn get_byte_order(&self) -> DataStreamByteOrder {
        unsafe {
            from_glib(ffi::g_data_output_stream_get_byte_order(self.to_glib_none().0))
        }
    }

    fn put_byte<'a, P: Into<Option<&'a Cancellable>>>(&self, data: u8, cancellable: P) -> Result<(), Error> {
        let cancellable = cancellable.into();
        let cancellable = cancellable.to_glib_none();
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::g_data_output_stream_put_byte(self.to_glib_none().0, data, cancellable.0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    fn put_int16<'a, P: Into<Option<&'a Cancellable>>>(&self, data: i16, cancellable: P) -> Result<(), Error> {
        let cancellable = cancellable.into();
        let cancellable = cancellable.to_glib_none();
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::g_data_output_stream_put_int16(self.to_glib_none().0, data, cancellable.0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    fn put_int32<'a, P: Into<Option<&'a Cancellable>>>(&self, data: i32, cancellable: P) -> Result<(), Error> {
        let cancellable = cancellable.into();
        let cancellable = cancellable.to_glib_none();
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::g_data_output_stream_put_int32(self.to_glib_none().0, data, cancellable.0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    fn put_int64<'a, P: Into<Option<&'a Cancellable>>>(&self, data: i64, cancellable: P) -> Result<(), Error> {
        let cancellable = cancellable.into();
        let cancellable = cancellable.to_glib_none();
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::g_data_output_stream_put_int64(self.to_glib_none().0, data, cancellable.0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    fn put_string<'a, P: Into<Option<&'a Cancellable>>>(&self, str: &str, cancellable: P) -> Result<(), Error> {
        let cancellable = cancellable.into();
        let cancellable = cancellable.to_glib_none();
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::g_data_output_stream_put_string(self.to_glib_none().0, str.to_glib_none().0, cancellable.0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    fn put_uint16<'a, P: Into<Option<&'a Cancellable>>>(&self, data: u16, cancellable: P) -> Result<(), Error> {
        let cancellable = cancellable.into();
        let cancellable = cancellable.to_glib_none();
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::g_data_output_stream_put_uint16(self.to_glib_none().0, data, cancellable.0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    fn put_uint32<'a, P: Into<Option<&'a Cancellable>>>(&self, data: u32, cancellable: P) -> Result<(), Error> {
        let cancellable = cancellable.into();
        let cancellable = cancellable.to_glib_none();
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::g_data_output_stream_put_uint32(self.to_glib_none().0, data, cancellable.0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    fn put_uint64<'a, P: Into<Option<&'a Cancellable>>>(&self, data: u64, cancellable: P) -> Result<(), Error> {
        let cancellable = cancellable.into();
        let cancellable = cancellable.to_glib_none();
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::g_data_output_stream_put_uint64(self.to_glib_none().0, data, cancellable.0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    fn set_byte_order(&self, order: DataStreamByteOrder) {
        unsafe {
            ffi::g_data_output_stream_set_byte_order(self.to_glib_none().0, order.to_glib());
        }
    }

    fn connect_property_byte_order_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::byte-order",
                transmute(notify_byte_order_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn notify_byte_order_trampoline<P>(this: *mut ffi::GDataOutputStream, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DataOutputStream> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DataOutputStream::from_glib_borrow(this).downcast_unchecked())
}
