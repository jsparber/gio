// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use Action;
use ffi;
use glib;
#[cfg(any(feature = "v2_46", feature = "dox"))]
use glib::StaticType;
#[cfg(any(feature = "v2_46", feature = "dox"))]
use glib::Value;
#[cfg(any(feature = "v2_38", feature = "dox"))]
use glib::object::Downcast;
use glib::object::IsA;
#[cfg(any(feature = "v2_38", feature = "dox"))]
use glib::signal::SignalHandlerId;
#[cfg(any(feature = "v2_38", feature = "dox"))]
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
#[cfg(any(feature = "v2_38", feature = "dox"))]
use std::boxed::Box as Box_;
use std::fmt;
use std::mem;
#[cfg(any(feature = "v2_38", feature = "dox"))]
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct PropertyAction(Object<ffi::GPropertyAction>): Action;

    match fn {
        get_type => || ffi::g_property_action_get_type(),
    }
}

impl PropertyAction {
    #[cfg(any(feature = "v2_38", feature = "dox"))]
    pub fn new<P: IsA<glib::Object>>(name: &str, object: &P, property_name: &str) -> PropertyAction {
        unsafe {
            from_glib_full(ffi::g_property_action_new(name.to_glib_none().0, object.to_glib_none().0, property_name.to_glib_none().0))
        }
    }
}

pub trait PropertyActionExt {
    #[cfg(any(feature = "v2_46", feature = "dox"))]
    fn get_property_invert_boolean(&self) -> bool;

    #[cfg(any(feature = "v2_38", feature = "dox"))]
    fn connect_property_enabled_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v2_38", feature = "dox"))]
    fn connect_property_parameter_type_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v2_38", feature = "dox"))]
    fn connect_property_state_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v2_38", feature = "dox"))]
    fn connect_property_state_type_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<PropertyAction> + IsA<glib::object::Object>> PropertyActionExt for O {
    #[cfg(any(feature = "v2_46", feature = "dox"))]
    fn get_property_invert_boolean(&self) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "invert-boolean".to_glib_none().0, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    #[cfg(any(feature = "v2_38", feature = "dox"))]
    fn connect_property_enabled_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::enabled",
                transmute(notify_enabled_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    #[cfg(any(feature = "v2_38", feature = "dox"))]
    fn connect_property_parameter_type_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::parameter-type",
                transmute(notify_parameter_type_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    #[cfg(any(feature = "v2_38", feature = "dox"))]
    fn connect_property_state_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::state",
                transmute(notify_state_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    #[cfg(any(feature = "v2_38", feature = "dox"))]
    fn connect_property_state_type_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::state-type",
                transmute(notify_state_type_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

#[cfg(any(feature = "v2_38", feature = "dox"))]
unsafe extern "C" fn notify_enabled_trampoline<P>(this: *mut ffi::GPropertyAction, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<PropertyAction> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&PropertyAction::from_glib_borrow(this).downcast_unchecked())
}

#[cfg(any(feature = "v2_38", feature = "dox"))]
unsafe extern "C" fn notify_parameter_type_trampoline<P>(this: *mut ffi::GPropertyAction, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<PropertyAction> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&PropertyAction::from_glib_borrow(this).downcast_unchecked())
}

#[cfg(any(feature = "v2_38", feature = "dox"))]
unsafe extern "C" fn notify_state_trampoline<P>(this: *mut ffi::GPropertyAction, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<PropertyAction> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&PropertyAction::from_glib_borrow(this).downcast_unchecked())
}

#[cfg(any(feature = "v2_38", feature = "dox"))]
unsafe extern "C" fn notify_state_type_trampoline<P>(this: *mut ffi::GPropertyAction, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<PropertyAction> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&PropertyAction::from_glib_borrow(this).downcast_unchecked())
}

impl fmt::Display for PropertyAction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "PropertyAction")
    }
}
