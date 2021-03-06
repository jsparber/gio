// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use InetAddress;
use InetSocketAddress;
use SocketAddress;
use SocketConnectable;
use ffi;
use glib::object::Downcast;
use glib::object::IsA;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::fmt;
use std::mem;
use std::ptr;

glib_wrapper! {
    pub struct ProxyAddress(Object<ffi::GProxyAddress, ffi::GProxyAddressClass>): InetSocketAddress, SocketAddress, SocketConnectable;

    match fn {
        get_type => || ffi::g_proxy_address_get_type(),
    }
}

impl ProxyAddress {
    pub fn new<'a, 'b, P: Into<Option<&'a str>>, Q: Into<Option<&'b str>>>(inetaddr: &InetAddress, port: u16, protocol: &str, dest_hostname: &str, dest_port: u16, username: P, password: Q) -> ProxyAddress {
        let username = username.into();
        let username = username.to_glib_none();
        let password = password.into();
        let password = password.to_glib_none();
        unsafe {
            SocketAddress::from_glib_full(ffi::g_proxy_address_new(inetaddr.to_glib_none().0, port, protocol.to_glib_none().0, dest_hostname.to_glib_none().0, dest_port, username.0, password.0)).downcast_unchecked()
        }
    }
}

unsafe impl Send for ProxyAddress {}
unsafe impl Sync for ProxyAddress {}

pub trait ProxyAddressExt {
    fn get_destination_hostname(&self) -> String;

    fn get_destination_port(&self) -> u16;

    #[cfg(any(feature = "v2_34", feature = "dox"))]
    fn get_destination_protocol(&self) -> Option<String>;

    fn get_password(&self) -> Option<String>;

    fn get_protocol(&self) -> String;

    #[cfg(any(feature = "v2_34", feature = "dox"))]
    fn get_uri(&self) -> Option<String>;

    fn get_username(&self) -> Option<String>;
}

impl<O: IsA<ProxyAddress>> ProxyAddressExt for O {
    fn get_destination_hostname(&self) -> String {
        unsafe {
            from_glib_none(ffi::g_proxy_address_get_destination_hostname(self.to_glib_none().0))
        }
    }

    fn get_destination_port(&self) -> u16 {
        unsafe {
            ffi::g_proxy_address_get_destination_port(self.to_glib_none().0)
        }
    }

    #[cfg(any(feature = "v2_34", feature = "dox"))]
    fn get_destination_protocol(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::g_proxy_address_get_destination_protocol(self.to_glib_none().0))
        }
    }

    fn get_password(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::g_proxy_address_get_password(self.to_glib_none().0))
        }
    }

    fn get_protocol(&self) -> String {
        unsafe {
            from_glib_none(ffi::g_proxy_address_get_protocol(self.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v2_34", feature = "dox"))]
    fn get_uri(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::g_proxy_address_get_uri(self.to_glib_none().0))
        }
    }

    fn get_username(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::g_proxy_address_get_username(self.to_glib_none().0))
        }
    }
}

impl fmt::Display for ProxyAddress {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ProxyAddress")
    }
}
