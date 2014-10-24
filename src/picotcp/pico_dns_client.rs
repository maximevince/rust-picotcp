#![allow(non_camel_case_types)]

extern crate libc;

use libc::{c_char, c_void, c_int, c_uint, uint8_t, uint16_t, uint32_t, int8_t};

use pico_ipv4::*;


#[repr(C)]
pub struct pico_dns_header {
    pub id: uint16_t,
    pub rd: uint8_t,
    pub tc: uint8_t,
    pub aa: uint8_t,
    pub opcode: uint8_t,
    pub qr: uint8_t,
    pub rcode: uint8_t,
    pub z: uint8_t,
    pub ra: uint8_t,
    pub qdcount: uint16_t,
    pub ancount: uint16_t,
    pub nscount: uint16_t,
    pub arcount: uint16_t,
}

#[repr(C)]
pub struct pico_dns_query_suffix {
    pub qtype: uint16_t,
    pub qclass: uint16_t,
}

#[repr(C)]
pub struct pico_dns_answer_suffix {
    pub qtype: uint16_t,
    pub qclass: uint16_t,
    pub ttl: uint32_t,
    pub rdlength: uint16_t,
}

/* enum pico_dns_arpa */
pub const PICO_DNS_ARPA4: c_uint = 0;
pub const PICO_DNS_ARPA6: c_uint = 1;
pub const PICO_DNS_NO_ARPA: c_uint = 2;


/* 
 * FOREIGN FUNCTION INTERFACE
 */

#[link(name = "picotcp")]
extern "C" {
    pub fn pico_dns_client_init() -> c_int;
    pub fn pico_dns_client_nameserver(ns: *mut pico_ip4, flag: uint8_t)  -> c_int;
    pub fn pico_dns_client_getaddr(url: *const c_char,
                                   callback: Option<extern "C" fn (arg1: *mut c_char, arg2: *mut c_void)>,
                                   arg: *mut c_void) -> c_int;
    pub fn pico_dns_client_getname(ip: *const c_char,
                                   callback: Option<extern "C" fn (arg1: *mut c_char, arg2: *mut c_void)>,
                                   arg: *mut c_void) -> c_int;
    pub fn pico_dns_client_getaddr6(url: *const c_char,
                                    callback: Option<extern "C" fn (arg1: *mut c_char, arg2: *mut c_void)>,
                                    arg: *mut c_void) -> c_int;
    pub fn pico_dns_client_getname6(url: *const c_char,
                                    callback: Option<extern "C" fn (arg1: *mut c_char, arg2: *mut c_void)>,
                                    arg: *mut c_void) -> c_int;
    pub fn pico_dns_client_strlen(url: *const c_char) -> uint16_t;
    pub fn pico_dns_client_query_header(pre: *mut pico_dns_header) -> c_int;
    pub fn pico_dns_client_query_domain(ptr: *mut c_char) -> c_int;
    pub fn pico_dns_client_answer_domain(ptr: *mut c_char) -> c_int;
    pub fn pico_dns_client_query_suffix(suf: *mut pico_dns_query_suffix, _type: uint16_t, qclass: uint16_t) -> c_int;
    pub fn pico_dns_create_message(header: *mut *mut pico_dns_header, qsuffix: *mut *mut pico_dns_query_suffix, arpa: c_uint,
                                   url: *const c_char, urlen: *mut uint16_t, hdrlen: *mut uint16_t) -> c_int;
    pub fn pico_dns_client_mirror(ptr: *mut c_char) -> int8_t;
    pub fn pico_dns_ipv6_set_ptr(ip: *const c_char, dst: *mut c_char);
}


/* 
 * RUST FUNCTION INTERFACE
 */

