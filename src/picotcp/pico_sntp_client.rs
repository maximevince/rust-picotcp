#![allow(non_camel_case_types)]

extern crate libc;

use libc::{c_int, c_char};
use pico_stack::*;


#[repr(C)]
pub struct pico_timeval {
    pub tv_sec: pico_time,
    pub tv_msec: pico_time,
}


/* 
 * FOREIGN FUNCTION INTERFACE
 */

#[link(name = "picotcp")]
extern "C" {
    pub fn pico_sntp_sync(sntp_server: *const c_char, cb_synced: Option<extern "C" fn (arg1: pico_err_t)>) -> c_int;
    pub fn pico_sntp_gettimeofday(tv: *mut pico_timeval) -> c_int;
}


/* 
 * RUST FUNCTION INTERFACE
 */

