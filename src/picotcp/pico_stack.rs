#![allow(non_camel_case_types)]

extern crate libc;

use libc::{c_int, c_uchar, c_void, uint8_t, int8_t, uint16_t, uint32_t, uint64_t};
use std::io::timer;
use std::time::Duration;

/* 
 * TYPE and STRUCT declarations
 */

pub type pico_time = uint64_t;

/* TODO: to be refined!!! */
pub type pico_device = uint32_t;
pub type pico_socket = uint32_t;

/* FROM: pico_frame.h */
#[repr(C)]
pub struct pico_frame {

    /* Connector for queues */
    pub next: *mut pico_frame,

    /* Start of the whole buffer, total frame length. */
    pub buffer: *mut c_uchar,
    pub buffer_len: uint32_t,

    /* For outgoing packets: this is the meaningful buffer. */
    pub start: *mut c_uchar,
    pub len: uint32_t,

    /* Pointer to usage counter */
    pub usage_count: *mut uint32_t,

    /* Pointer to protocol headers */
    pub datalink_hdr: *mut uint8_t,

    pub net_hdr: *mut uint8_t,
    pub net_len: uint16_t,
    pub transport_hdr: *mut uint8_t,
    pub transport_len: uint16_t,
    pub app_hdr: *mut uint8_t,
    pub app_len: uint16_t,

    /* Pointer to the phisical device this packet belongs to.
     * Should be valid in both routing directions
     */
    pub dev: *mut pico_device,

    pub timestamp: pico_time,

    /* Failures due to bad datalink addressing. */
    pub failure_count: uint16_t,

    /* Protocol over IP */
    pub proto: uint8_t,

    /* PICO_FRAME_FLAG_* */
    pub flags: uint8_t,

    /* Pointer to payload */
    pub payload: *mut c_uchar,
    pub payload_len: uint16_t,

    #[cfg(cargo_feature = "PICO_SUPPORT_IPFRAG")]
        /* Payload fragmentation info (big endian)*/
        pub frag: uint16_t,

    /* Pointer to socket */
    pub sock: *mut pico_socket,

    /* Pointer to transport info, used to store remote UDP endpoint (IP + port) */
    pub info: *mut c_void,

    /*Priority. "best-effort" priority, the default value is 0. Priority can be in between -10 and +10*/
    pub priority: int8_t,
    pub transport_flags_saved: uint8_t,
}



/* 
 * FOREIGN FUNCTION INTERFACE
 */

//#[link(name = "picotcp", kind="static")]
#[link(name = "picotcp")]
extern "C" {
    /* ----- Initialization ----- */
    pub fn pico_stack_init() -> c_int;
    /* ----- Loop Function. ----- */
    pub fn pico_stack_tick(); 
    pub fn pico_stack_loop(); 

    /* ---- Notifications for stack errors */
    pub fn pico_notify_socket_unreachable(f: *mut pico_frame) -> c_int;
    pub fn pico_notify_proto_unreachable(f: *mut pico_frame) -> c_int;
    pub fn pico_notify_dest_unreachable(f: *mut pico_frame) -> c_int;
    pub fn pico_notify_ttl_expired(f: *mut pico_frame) -> c_int;

}

pub fn stack_init() -> int {
    unsafe { pico_stack_init() as int }
}

fn stack_tick() {
    unsafe { pico_stack_tick(); }
}

pub fn stack_loop() {
    loop {
        stack_tick();
        timer::sleep(Duration::milliseconds(1));
    }
}

/*
pub fn notify_socket_unreachable(f: *mut pico_frame) -> c_int {
    unsafe { pico_notify_socket_unreachable(f) as c_int }
}

pub fn notify_proto_unreachable(f: *mut pico_frame) -> c_int {
    unsafe { pico_notify_proto_unreachable(f) as c_int }
}

pub fn notify_dest_unreachable(f: *mut pico_frame) -> c_int {
    unsafe { pico_notify_dest_unreachable(f) as c_int }
}

pub fn notify_ttl_expired(f: *mut pico_frame) -> c_int {
    unsafe { pico_notify_ttl_expired(f) as c_int }
}
*/


