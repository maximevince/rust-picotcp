#![allow(non_camel_case_types)]

use libc::{c_int, c_char, uint32_t, uint16_t};

use pico_queue::pico_queue;
use pico_stack::pico_frame;

#[repr(C)]
pub enum pico_layer {
    PICO_LAYER_DATALINK = 2, /* Ethernet only. */
    PICO_LAYER_NETWORK = 3, /* IPv4, IPv6, ARP. Arp is there because it communicates with L2 */
    PICO_LAYER_TRANSPORT = 4, /* UDP, TCP, ICMP */
    PICO_LAYER_SOCKET = 5   /* Socket management */
}

#[repr(C)]
pub struct pico_protocol {
    pub name: [c_char; MAX_PROTOCOL_NAME],
    pub hash: uint32_t,
    pub layer: pico_layer,
    pub proto_number: uint32_t,
    pub q_in: pico_queue,
    pub q_out: pico_queue,
    pub alloc: extern fn (self_: *mut pico_protocol, size: uint16_t) -> pico_frame, /* Frame allocation. */
    pub push: extern fn(self_: *mut pico_protocol, p: *mut pico_frame) -> c_int,    /* Push function, for active outgoing pkts from above */
    pub process_out: extern fn(self_: *mut pico_protocol, p: *mut pico_frame) -> c_int,  /* Send loop. */
    pub process_in: extern fn(self_: *mut pico_protocol, p: *mut pico_frame) -> c_int,  /* Recv loop. */
    pub get_mtu: extern fn(self_: *mut pico_protocol) -> uint16_t,

}

#[link(name = "picotcp")]
extern "C" {
    pub fn pico_protocols_loop(loop_score: c_int) -> c_int;
    pub fn pico_protocol_init(p: *mut pico_protocol);

    pub fn pico_protocol_datalink_loop(loop_score: c_int, direction: c_int) -> c_int;
    pub fn pico_protocol_network_loop(loop_score: c_int, direction: c_int) -> c_int;
    pub fn pico_protocol_transport_loop(loop_score: c_int, direction: c_int) -> c_int;
    pub fn pico_protocol_socket_loop(loop_score: c_int, direction: c_int) -> c_int;
}

#[allow(dead_code)]
pub const MAX_PROTOCOL_NAME:usize = 16;
