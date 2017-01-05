#![allow(non_camel_case_types)]

use libc::uint16_t;

use pico_protocol::pico_protocol;
use pico_addressing::pico_address;

#[repr(C)]
pub struct pico_queue {
    proto: *mut pico_protocol,
    net: *mut pico_protocol,

    local_addr: pico_address,
    remote_addr: pico_address,

    local_port: uint16_t,
    remote_port: uint16_t,
}
