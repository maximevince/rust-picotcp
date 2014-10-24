#![allow(non_camel_case_types)]

extern crate libc;

use libc::{c_uint, c_int, uint8_t, uint16_t, uint32_t, int8_t};

use pico_stack::*;
use pico_ipv4::*;


/* enum filter_action */
pub const FILTER_PRIORITY: c_uint = 0;
pub const FILTER_REJECT: c_uint = 1;
pub const FILTER_DROP: c_uint = 2;
pub const FILTER_COUNT: c_uint = 3;

/* 
 * FOREIGN FUNCTION INTERFACE
 */

#[link(name = "picotcp")]
extern "C" {
    pub fn pico_ipv4_filter_add(dev: *mut pico_device, proto: uint8_t,
                                out_addr: *mut pico_ip4,
                                out_addr_netmask: *mut pico_ip4,
                                in_addr: *mut pico_ip4,
                                in_addr_netmask: *mut pico_ip4,
                                out_port: uint16_t, in_port: uint16_t,
                                priority: int8_t, tos: uint8_t,
                                action: c_uint) -> uint32_t;
    pub fn pico_ipv4_filter_del(filter_id: uint32_t) -> c_int;
    pub fn ipfilter(f: *mut pico_frame) -> c_int;
}


/* 
 * RUST FUNCTION INTERFACE
 */

