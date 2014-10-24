#![allow(non_camel_case_types)]

extern crate libc;

use libc::{c_void, c_int, uint8_t, uint32_t};

use pico_stack::*;
use pico_ipv4::*;

/* 
 * FOREIGN FUNCTION INTERFACE
 */


#[link(name = "picotcp")]
extern "C" {
    pub fn pico_dhcp_initiate_negotiation(device: *mut pico_device,
                                          callback: Option<extern "C" fn(arg1: *mut c_void, arg2: c_int)>,
                                          xid: *mut uint32_t) -> c_int;
    pub fn pico_dhcp_process_incoming_message(data: *mut uint8_t, len: c_int);
    pub fn pico_dhcp_get_identifier(xid: uint32_t) -> *mut c_void;
    pub fn pico_dhcp_get_address(cli: *mut c_void) -> pico_ip4;
    pub fn pico_dhcp_get_gateway(cli: *mut c_void) -> pico_ip4;
    pub fn pico_dhcp_get_netmask(cli: *mut c_void) -> pico_ip4;
    pub fn pico_dhcp_get_nameserver(cli: *mut c_void) -> pico_ip4;
    pub fn pico_dhcp_client_abort(xid: uint32_t) -> c_int;
}

/* 
 * RUST FUNCTION INTERFACE
 */

