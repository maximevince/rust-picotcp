#![allow(non_camel_case_types)]

extern crate libc;

use std::ffi::CString;
use libc::{c_char};
use pico_stack::*;

/* 
 * FOREIGN FUNCTION INTERFACE
 */

#[link(name = "picotcp")]
extern "C" {
    pub fn pico_tun_create(name: *const c_char) -> *mut pico_device;
    pub fn pico_tun_destroy(tun: *mut pico_device);
}


/* 
 * RUST FUNCTION INTERFACE
 */

pub fn tun_create(name: &str) -> *mut pico_device
{
    unsafe { pico_tun_create(CString::new(name).unwrap().into_raw() as *const c_char) as *mut pico_device }
}

pub fn tun_destroy(tun: *mut pico_device)
{
    unsafe { pico_tun_destroy(tun); }
}

