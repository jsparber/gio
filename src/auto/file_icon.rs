// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use File;
use Icon;
use LoadableIcon;
use ffi;
use glib::object::IsA;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::fmt;
use std::mem;
use std::ptr;

glib_wrapper! {
    pub struct FileIcon(Object<ffi::GFileIcon, ffi::GFileIconClass>): Icon, LoadableIcon;

    match fn {
        get_type => || ffi::g_file_icon_get_type(),
    }
}

impl FileIcon {
    pub fn new<P: IsA<File>>(file: &P) -> FileIcon {
        unsafe {
            from_glib_full(ffi::g_file_icon_new(file.to_glib_none().0))
        }
    }
}

pub trait FileIconExt {
    fn get_file(&self) -> Option<File>;
}

impl<O: IsA<FileIcon>> FileIconExt for O {
    fn get_file(&self) -> Option<File> {
        unsafe {
            from_glib_none(ffi::g_file_icon_get_file(self.to_glib_none().0))
        }
    }
}

impl fmt::Display for FileIcon {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "FileIcon")
    }
}
