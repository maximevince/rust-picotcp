#![allow(non_camel_case_types)]

extern crate libc;

use libc::{c_uint, c_int, c_uchar, c_void, uint8_t, int8_t, uint16_t, uint32_t, uint64_t};
use std::thread;
use std::time::Duration;

use socket::pico_socket;

pub struct stack;

/* 
 * TYPE and STRUCT declarations
 */

pub type pico_time = uint64_t;

/* TODO: to be refined!!! */
pub type pico_device = uint32_t;

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

/* enum pico_err_e */ 
pub type pico_err_t = c_uint;
pub const PICO_ERR_NOERR: c_uint = 0;
pub const PICO_ERR_EPERM: c_uint = 1;
pub const PICO_ERR_ENOENT: c_uint = 2;
pub const PICO_ERR_EINTR: c_uint = 4;
pub const PICO_ERR_EIO: c_uint = 5;
pub const PICO_ERR_ENXIO: c_uint = 6;
pub const PICO_ERR_EAGAIN: c_uint = 11;
pub const PICO_ERR_ENOMEM: c_uint = 12;
pub const PICO_ERR_EACCESS: c_uint = 13;
pub const PICO_ERR_EFAULT: c_uint = 14;
pub const PICO_ERR_EBUSY: c_uint = 16;
pub const PICO_ERR_EEXIST: c_uint = 17;
pub const PICO_ERR_EINVAL: c_uint = 22;
pub const PICO_ERR_ENONET: c_uint = 64;
pub const PICO_ERR_EPROTO: c_uint = 71;
pub const PICO_ERR_ENOPROTOOPT: c_uint = 92;
pub const PICO_ERR_EPROTONOSUPPORT: c_uint = 93;
pub const PICO_ERR_EOPNOTSUPP: c_uint = 95;
pub const PICO_ERR_EADDRINUSE: c_uint = 98;
pub const PICO_ERR_EADDRNOTAVAIL: c_uint = 99;
pub const PICO_ERR_ENETDOWN: c_uint = 100;
pub const PICO_ERR_ENETUNREACH: c_uint = 101;
pub const PICO_ERR_ECONNRESET: c_uint = 104;
pub const PICO_ERR_EISCONN: c_uint = 106;
pub const PICO_ERR_ENOTCONN: c_uint = 107;
pub const PICO_ERR_ESHUTDOWN: c_uint = 108;
pub const PICO_ERR_ETIMEDOUT: c_uint = 110;
pub const PICO_ERR_ECONNREFUSED: c_uint = 111;
pub const PICO_ERR_EHOSTDOWN: c_uint = 112;
pub const PICO_ERR_EHOSTUNREACH: c_uint = 113;


/* 
 * FOREIGN FUNCTION INTERFACE
 */

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



impl stack {
    pub fn new() -> stack {
        let n;
        unsafe { n = pico_stack_init(); }
        if n < 0 {
            panic!("PicoTCP: failed to initialize stack\n");
        }
        let x = stack;
        x
    }
    fn stack_tick(&self) {
        unsafe { pico_stack_tick(); }
    }

    pub fn stack_loop(&self) {
        loop {
            self.stack_tick();
            thread::sleep(Duration::from_millis(1));
        }
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


