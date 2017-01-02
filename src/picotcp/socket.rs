#![allow(non_camel_case_types)]

extern crate libc;
use pico_ipv4::*;
use libc::c_int;
use std::cmp;

pub enum pico_socket {} // TODO

#[link(name = "picotcp")]
extern "C" {
    pub fn pico_socket_open(net: u16, proto: u16, wakeup: extern fn(ev: u16, sock: &pico_socket))->*mut pico_socket; 
    pub fn pico_socket_bind(s: *mut pico_socket, address: *mut u8, port: *mut u16)->c_int; 
    pub fn pico_socket_listen(s: *mut pico_socket, backlog: c_int)->c_int; 
    pub fn pico_socket_accept(s: &pico_socket, address: *mut u8, port: *mut u16)->*mut pico_socket; 
    pub fn pico_socket_recv(s: &pico_socket, buf: *mut u8, len: c_int)->c_int;
    pub fn pico_socket_send(s: &pico_socket, buf: &u8, len: c_int)->c_int;
    pub fn pico_socket_close(s: &pico_socket)->c_int;
    pub fn pico_socket_shutdown(s: &pico_socket, how: c_int)->c_int;
}


/* 
 * Constants
 */

#[allow(dead_code)]
pub const PICO_PROTO_IPV4:u16 = 0;
#[allow(dead_code)]
pub const PICO_PROTO_IPV6:u16 = 41;
#[allow(dead_code)]
pub const PICO_PROTO_TCP:u16 = 6;
#[allow(dead_code)]
pub const PICO_PROTO_UDP:u16 = 17;

#[allow(dead_code)]
pub const PICO_SOCK_EV_RD:u16 = 1;
#[allow(dead_code)]
pub const PICO_SOCK_EV_WR:u16 = 2;
#[allow(dead_code)]
pub const PICO_SOCK_EV_CONN:u16 = 4;
#[allow(dead_code)]
pub const PICO_SOCK_EV_CLOSE:u16 = 8;
#[allow(dead_code)]
pub const PICO_SOCK_EV_FIN:u16 = 0x10;
#[allow(dead_code)]
pub const PICO_SOCK_EV_ERR:u16 = 0x80;

static MAXLEN:usize = 1500;

/* 
 * RUST FUNCTION INTERFACE
 */

pub fn socket(net:u16, proto:u16, wakeup: extern fn(u16, &pico_socket)) -> *mut pico_socket
{
    unsafe { pico_socket_open(net, proto, wakeup) as *mut pico_socket}
}

pub fn bind(s: *mut pico_socket, address: *mut pico_ip4, port: *mut u16) -> (i32, u16)
{
    unsafe {
        let mut be_port: u16 = *port;
        be_port = be_port.to_be();
        (pico_socket_bind(s, address as *mut u8, &mut be_port), be_port.to_be())
    }
}

pub fn listen(s: *mut pico_socket, backlog: i32) -> i32
{
    unsafe { pico_socket_listen(s, backlog) }
}

pub fn accept(s: &pico_socket, address: *mut pico_ip4, port: *mut u16) -> *mut pico_socket
{
    unsafe {
        pico_socket_accept(s, address as *mut u8, port)
    }
}

pub fn recv(s: &pico_socket)-> Vec<u8>
{
    unsafe {
        let mut buf:Vec<u8> = Vec::with_capacity(MAXLEN);
        /* Recv stream/dgram */
        let r = pico_socket_recv(s, buf.as_mut_ptr(), buf.capacity() as i32) as usize;
        buf.set_len(cmp::max(0, r));
        buf
    }

//    unsafe {
//        let mut buf:Vec<u8> = Vec::with_capacity(MAXLEN );
//        let p = buf.as_mut_ptr();
//        mem::forget(buf); /* Cast into the void to allow raw access */
//
//        /* Recv stream/dgram */
//        let mut r = pico_socket_recv(s, p, MAXLEN as i32);
//
//        if r < 0 { r = 0;}
//        Vec::from_raw_parts(r as uint, MAXLEN , p)
//    }
}

pub fn send(s: &pico_socket, buf:&[u8])->i32
{
    unsafe {
        let p = buf.as_ptr();
        pico_socket_send(s, &*p , buf.len() as c_int) as i32
    }
}

pub fn shutdown(s:&pico_socket, how: i32)->i32
{
    unsafe { pico_socket_shutdown(s, how as c_int) as i32}
}

pub fn close(s:&pico_socket)->i32
{
    unsafe { pico_socket_close(s) as i32}
}

