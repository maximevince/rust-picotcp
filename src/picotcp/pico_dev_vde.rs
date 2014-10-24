#![allow(non_camel_case_types)]

extern crate libc;

use libc::{c_char, uint8_t, uint32_t};
use pico_stack::*;

/* 
 * FOREIGN FUNCTION INTERFACE
 */

#[link(name = "picotcp")]
extern "C" {
    pub fn pico_vde_destroy(vde: *mut pico_device);
    pub fn pico_vde_create(sock: *mut c_char, name: *mut c_char, mac: *mut uint8_t) -> *mut pico_device;
    pub fn pico_vde_set_packetloss(dev: *mut pico_device, in_pct: uint32_t, out_pct: uint32_t);
}


/* 
 * RUST FUNCTION INTERFACE
 */


