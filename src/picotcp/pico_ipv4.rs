#![allow(non_camel_case_types)]

extern crate libc;

use libc::{c_int, c_char};

use pico_stack::*;

#[repr(C)]
pub struct pico_ip4 {
    pub addr: u32,
}

//#[link(name = "picotcp", kind="static")]
#[link(name = "picotcp")]
extern "C" {
    pub fn pico_ipv4_link_add(dev: *mut pico_device,
                              address: pico_ip4,
                              netmask: pico_ip4) -> c_int;
    pub fn pico_ipv4_link_del(dev: *mut pico_device,
                              address: pico_ip4) -> c_int;

    pub fn pico_string_to_ipv4(ipstr: *const c_char,
                               ip: *mut pico_ip4) -> c_int;

}

pub fn ipv4_link_add(dev: *mut pico_device,
                          address: pico_ip4,
                          netmask: pico_ip4) -> c_int
{
    unsafe { pico_ipv4_link_add(dev, address, netmask) as c_int }
}

pub fn ipv4_link_del(dev: *mut pico_device,
                          address: pico_ip4) -> c_int
{
    unsafe { pico_ipv4_link_del(dev, address) as c_int }
}


pub fn string_to_ipv4(ipstr: *const c_char,
                           ip: *mut pico_ip4) -> c_int
{
    unsafe { pico_string_to_ipv4(ipstr, ip) as c_int }
}

