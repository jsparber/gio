// This file was generated by gir (746446b) from gir-files (469db10)
// DO NOT EDIT

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
    pub struct FilterOutputStream(Object<ffi::GFilterOutputStream, ffi::GFilterOutputStreamClass>): OutputStream;

    match fn {
        get_type => || ffi::g_filter_output_stream_get_type(),
    }
}

pub trait FilterOutputStreamExt {
    fn get_base_stream(&self) -> Option<OutputStream>;

    fn get_close_base_stream(&self) -> bool;

    fn set_close_base_stream(&self, close_base: bool);

    fn connect_property_close_base_stream_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<FilterOutputStream> + IsA<glib::object::Object>> FilterOutputStreamExt for O {
    fn get_base_stream(&self) -> Option<OutputStream> {
        unsafe {
            from_glib_none(ffi::g_filter_output_stream_get_base_stream(self.to_glib_none().0))
        }
    }

    fn get_close_base_stream(&self) -> bool {
        unsafe {
            from_glib(ffi::g_filter_output_stream_get_close_base_stream(self.to_glib_none().0))
        }
    }

    fn set_close_base_stream(&self, close_base: bool) {
        unsafe {
            ffi::g_filter_output_stream_set_close_base_stream(self.to_glib_none().0, close_base.to_glib());
        }
    }

    fn connect_property_close_base_stream_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::close-base-stream",
                transmute(notify_close_base_stream_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn notify_close_base_stream_trampoline<P>(this: *mut ffi::GFilterOutputStream, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<FilterOutputStream> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&FilterOutputStream::from_glib_borrow(this).downcast_unchecked())
}
