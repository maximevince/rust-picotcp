#![allow(non_camel_case_types)]

extern crate libc;

use libc::{c_int, c_char};

use pico_stack::*;

#[packed]
pub struct pico_ip4 {
    pub addr: u32,
}

impl fmt::Show for picotcp::pico_ipv4::pico_ip4 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let addr = self.addr.to_be();
        let a = (addr >> 24) & 0xFF;
        let b = (addr >> 16) & 0xFF;
        let c = (addr >> 8) & 0xFF;
        let d = (addr) & 0xFF;
        write!(f, " {}.{}.{}.{}", a,b,c,d)
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

    pub fn pico_string_to_ipv4(ipstr: *const c_char,
                               ip: *mut pico_ip4) -> c_int;

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


pub fn string_to_ipv4(ipstr: *const c_char,
                           ip: *mut pico_ip4) -> c_int
{
    unsafe { pico_string_to_ipv4(ipstr, ip) as c_int }
}

