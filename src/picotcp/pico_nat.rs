#![allow(non_camel_case_types)]

extern crate libc;

use libc::{c_int, uint8_t, uint16_t};
use pico_stack::*;
use pico_ipv4::*;


/* 
 * FOREIGN FUNCTION INTERFACE
 */

#[link(name = "picotcp")]
extern "C" {
    pub fn pico_ipv4_nat_print_table();
    pub fn pico_ipv4_nat_find(nat_port: uint16_t, src_addr: *mut pico_ip4, src_port: uint16_t, proto: uint8_t) -> c_int;
    pub fn pico_ipv4_port_forward(nat_addr: pico_ip4, nat_port: uint16_t, src_addr: pico_ip4, src_port: uint16_t, proto: uint8_t, flag: uint8_t) -> c_int;
    pub fn pico_ipv4_nat_inbound(f: *mut pico_frame, link_addr: *mut pico_ip4) -> c_int;
    pub fn pico_ipv4_nat_outbound(f: *mut pico_frame, link_addr: *mut pico_ip4) -> c_int;
    pub fn pico_ipv4_nat_enable(link: *mut pico_ipv4_link) -> c_int;
    pub fn pico_ipv4_nat_disable() -> c_int;
    pub fn pico_ipv4_nat_is_enabled(link_addr: *mut pico_ip4) -> c_int;
}


/* 
 * RUST FUNCTION INTERFACE
 */

