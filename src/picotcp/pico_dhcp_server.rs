#![allow(non_camel_case_types)]

extern crate libc;

#[allow(unused_imports)] 
use libc::{c_int, c_void, size_t, uint8_t, uint16_t, uint32_t, uint64_t, int32_t};

use pico_stack::*;
use pico_ipv4::*;

#[repr(C)]
pub struct pico_dhcp_opt {
    pub code: uint8_t,
    pub len: uint8_t,
    pub ext: uint32_t, /* TODO: what to do with this ?? */
}

#[repr(C)]
pub struct pico_dhcp_server_setting {
    pub pool_start: uint32_t,
    pub pool_next: uint32_t,
    pub pool_end: uint32_t,
    pub lease_time: uint32_t,
    pub dev: *mut pico_device,
    pub s: *mut pico_socket,
    pub server_ip: pico_ip4,
    pub netmask: pico_ip4,
    pub flags: uint8_t,
}

/* 
 * FOREIGN FUNCTION INTERFACE
 */

#[link(name = "picotcp")]
extern "C" {
    /*
    pub fn dhcp_get_next_option(begin: *mut uint8_t,
                                data: *mut uint8_t,
                                len: *mut c_int,
                                nextopt: *mut *mut uint8_t) -> uint8_t;
    pub fn pico_dhcp_next_option(ptr: *mut *mut pico_dhcp_opt) -> *mut pico_dhcp_opt;
    pub fn pico_dhcp_are_options_valid(ptr: *mut c_void, len: int32_t) -> uint8_t;
    pub fn pico_dhcp_opt_netmask(ptr: *mut c_void, ip: *mut pico_ip4) -> uint8_t;
    pub fn pico_dhcp_opt_router(ptr: *mut c_void, ip: *mut pico_ip4) -> uint8_t;
    pub fn pico_dhcp_opt_dns(ptr: *mut c_void, ip: *mut pico_ip4) -> uint8_t;
    pub fn pico_dhcp_opt_broadcast(ptr: *mut c_void, ip: *mut pico_ip4) -> uint8_t;
    pub fn pico_dhcp_opt_reqip(ptr: *mut c_void, ip: *mut pico_ip4) -> uint8_t;
    pub fn pico_dhcp_opt_leasetime(ptr: *mut c_void, time: uint32_t) -> uint8_t;
    pub fn pico_dhcp_opt_msgtype(ptr: *mut c_void, _type: uint8_t) -> uint8_t;
    pub fn pico_dhcp_opt_serverid(ptr: *mut c_void, ip: *mut pico_ip4) -> uint8_t;
    pub fn pico_dhcp_opt_paramlist(ptr: *mut c_void) -> uint8_t;
    pub fn pico_dhcp_opt_maxmsgsize(ptr: *mut c_void, size: uint16_t) -> uint8_t;
    pub fn pico_dhcp_opt_end(ptr: *mut c_void) -> uint8_t;
    */
    pub fn pico_dhcp_server_initiate(dhcps: *mut pico_dhcp_server_setting) -> c_int;
    pub fn pico_dhcp_server_destroy(dev: *mut pico_device) ->  c_int;
}


/* 
 * RUST FUNCTION INTERFACE
 */

pub fn dhcp_server_initiate(dhcps: *mut pico_dhcp_server_setting) -> c_int
{
    unsafe { pico_dhcp_server_initiate(dhcps) as c_int }
}

pub fn dhcp_server_destroy(dev: *mut pico_device) ->  c_int
{
    unsafe { pico_dhcp_server_destroy(dev) as c_int }
}


