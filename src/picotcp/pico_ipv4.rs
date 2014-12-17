#![allow(non_camel_case_types)]

extern crate libc;

use libc::{c_int};
use std::fmt;
use std::num::Int;

use pico_stack::*;

#[packed]
#[repr(C)]
pub struct pico_ip4 {
    pub addr: u32,
}

#[repr(C)]
pub struct pico_ipv4_link {
    dev: *const pico_device,
    address: pico_ip4,
    netmask: pico_ip4,
    /*
     * TODO: what to do with these conditionally compiled fields?
     *
    #ifdef PICO_SUPPORT_MCAST
    struct pico_tree *MCASTGroups;
    uint8_t mcast_compatibility;
    uint8_t mcast_last_query_interval;
    #endif
    */
}


pub static INADDR_ANY: pico_ip4 = pico_ip4{ addr: 0 };

impl fmt::Show for pico_ip4 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let addr = self.addr.to_be();
        let a = (addr >> 24) & 0xFF;
        let b = (addr >> 16) & 0xFF;
        let c = (addr >> 8) & 0xFF;
        let d = (addr) & 0xFF;
        write!(f, " {}.{}.{}.{}", a,b,c,d)
    }
}

impl pico_ip4 { 
    pub fn new(address: &str) -> pico_ip4 {
        let mut addr_u:u32  = 0;
        let v: Vec<&str> = address.as_slice().split('.').collect();
        let mut i:uint = 0;
        for &byte in v.iter() {
            let u:Option<u32> = from_str(byte);
            let sum = u.unwrap() << i;
            addr_u += sum;
            i+=8;
        }
        pico_ip4 { addr: addr_u}
    }
}

impl Clone for pico_ip4 {
    fn clone(&self) -> pico_ip4 {
        pico_ip4 { addr: self.addr }
    }
}

//#[link(name = "picotcp", kind="static")]
#[link(name = "picotcp")]
extern "C" {
    pub fn pico_ipv4_link_add(dev: *mut pico_device,
                              address: pico_ip4,
                              netmask: pico_ip4) -> c_int;
    pub fn pico_ipv4_link_del(dev: *mut pico_device,
                              address: pico_ip4) -> c_int;
}

pub fn ipv4_link_add(dev: *mut pico_device,
                          address: pico_ip4,
                          netmask: pico_ip4) -> c_int
{
    unsafe { pico_ipv4_link_add(dev, address, netmask) as c_int }
}

pub fn ipv4_link_del(dev: *mut pico_device,
                          address: pico_ip4) -> c_int
{
    unsafe { pico_ipv4_link_del(dev, address) as c_int }
}


