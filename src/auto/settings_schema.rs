// This file was generated by gir (746446b) from gir-files (469db10)
// DO NOT EDIT

#[cfg(any(feature = "v2_40", feature = "dox"))]
use SettingsSchemaKey;
use ffi;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::mem;
use std::ptr;

glib_wrapper! {
    pub struct SettingsSchema(Shared<ffi::GSettingsSchema>);

    match fn {
        ref => |ptr| ffi::g_settings_schema_ref(ptr),
        unref => |ptr| ffi::g_settings_schema_unref(ptr),
        get_type => || ffi::g_settings_schema_get_type(),
    }
}

impl SettingsSchema {
    pub fn get_id(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::g_settings_schema_get_id(self.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v2_40", feature = "dox"))]
    pub fn get_key(&self, name: &str) -> Option<SettingsSchemaKey> {
        unsafe {
            from_glib_full(ffi::g_settings_schema_get_key(self.to_glib_none().0, name.to_glib_none().0))
        }
    }

    pub fn get_path(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::g_settings_schema_get_path(self.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v2_40", feature = "dox"))]
    pub fn has_key(&self, name: &str) -> bool {
        unsafe {
            from_glib(ffi::g_settings_schema_has_key(self.to_glib_none().0, name.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v2_44", feature = "dox"))]
    pub fn list_children(&self) -> Vec<String> {
        unsafe {
            FromGlibPtrContainer::from_glib_full(ffi::g_settings_schema_list_children(self.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v2_46", feature = "dox"))]
    pub fn list_keys(&self) -> Vec<String> {
        unsafe {
            FromGlibPtrContainer::from_glib_full(ffi::g_settings_schema_list_keys(self.to_glib_none().0))
        }
    }
}
