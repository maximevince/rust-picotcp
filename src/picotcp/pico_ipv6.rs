#![allow(non_camel_case_types)]

extern crate libc;

use libc::{c_int};
use std::fmt;

use pico_stack::*;

#[packed]
#[repr(C)]
pub struct pico_ip6 {
    pub addr: [u8, ..16]
}

impl fmt::Show for pico_ip6 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let a = self.addr;
        write!(f, " {:02x}{:02x}:{:02x}{:02x}:{:02x}{:02x}:{:02x}{:02x}:{:02x}{:02x}:{:02x}{:02x}:{:02x}{:02x}:{:02x}{:02x}", 
            a[0], a[1], a[2], a[3], a[4], a[5], a[6], a[7], a[8], a[9], a[10], a[11], a[12], a[13], a[14], a[15])
    }
}

impl pico_ip6 { 
    pub fn new(address: &str) -> pico_ip6 {
        let v: Vec<&str> = address.as_slice().split(':').collect();
        let mut a: [u8, ..16] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0 ];
        for (i,&byte) in v.iter().enumerate() {
            let b:Option<u16> = from_str(byte);
            match b {
                Some(x) => {
                    a[ 2 * i ] = ((x >> 8) & 0xFF) as u8;
                    a[ 2 * i + 1] = (x & 0xFF) as u8;
                }, 
                None => {
                    a[ 2 * i ] = 0; 
                    a[ 2 * i + 1] = 0; 
                }
            }
        }
        pico_ip6 { addr: a}
    }
}

#[link(name = "picotcp")]
extern "C" {
    pub fn pico_ipv6_link_add(dev: *mut pico_device,
                              address: pico_ip6,
                              netmask: pico_ip6) -> c_int;
    pub fn pico_ipv6_link_del(dev: *mut pico_device,
                              address: pico_ip6) -> c_int;

}

pub fn ipv6_link_add(dev: *mut pico_device,
                          address: pico_ip6,
                          netmask: pico_ip6) -> c_int
{
    unsafe { pico_ipv6_link_add(dev, address, netmask) as c_int }
}

pub fn ipv6_link_del(dev: *mut pico_device,
                          address: pico_ip6) -> c_int
{
    unsafe { pico_ipv6_link_del(dev, address) as c_int }
}


