#![allow(non_camel_case_types)]

extern crate libc;

use libc::{c_char, c_void, c_int};

/* 
 * FOREIGN FUNCTION INTERFACE
 */

#[link(name = "picotcp")]
extern "C" {
    pub fn pico_mdns_init(hostname: *mut c_char, cb_initialised: Option<extern "C" fn (arg1: *mut c_char, arg2: *mut c_void)>, arg: *mut c_void) -> c_int;
    pub fn pico_mdns_getaddr(url: *const c_char, callback: Option<extern "C" fn (arg1: *mut c_char, arg2: *mut c_void)>, arg: *mut c_void) -> c_int;
    pub fn pico_mdns_getname(ip: *const c_char, callback: Option<extern "C" fn (arg1: *mut c_char, arg2: *mut c_void)>, arg: *mut c_void) -> c_int;
}

/* 
 * RUST FUNCTION INTERFACE
 */

