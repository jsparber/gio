// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use Icon;
use LoadableIcon;
use ffi;
#[cfg(any(feature = "v2_38", feature = "dox"))]
use glib;
use glib::object::IsA;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::fmt;
use std::mem;
use std::ptr;

glib_wrapper! {
    pub struct BytesIcon(Object<ffi::GBytesIcon>): Icon, LoadableIcon;

    match fn {
        get_type => || ffi::g_bytes_icon_get_type(),
    }
}

impl BytesIcon {
    #[cfg(any(feature = "v2_38", feature = "dox"))]
    pub fn new(bytes: &glib::Bytes) -> BytesIcon {
        unsafe {
            from_glib_full(ffi::g_bytes_icon_new(bytes.to_glib_none().0))
        }
    }
}

pub trait BytesIconExt {
    #[cfg(any(feature = "v2_38", feature = "dox"))]
    fn get_bytes(&self) -> Option<glib::Bytes>;
}

impl<O: IsA<BytesIcon>> BytesIconExt for O {
    #[cfg(any(feature = "v2_38", feature = "dox"))]
    fn get_bytes(&self) -> Option<glib::Bytes> {
        unsafe {
            from_glib_none(ffi::g_bytes_icon_get_bytes(self.to_glib_none().0))
        }
    }
}

impl fmt::Display for BytesIcon {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "BytesIcon")
    }
}
