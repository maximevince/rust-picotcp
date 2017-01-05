#![allow(non_camel_case_types)]

use libc::uint8_t;

use pico_ipv4::pico_ip4;
use pico_ipv6::pico_ip6;

#[repr(C)]
pub struct pico_address {
    addr: [uint8_t; 16],
}

impl pico_address {
    pub fn new_ip4(ip4: &pico_ip4) -> pico_address {
        let addr_be = ip4.addr.to_be();
        let addr: [uint8_t; 16] = [
            ((addr_be >> 24) % 8) as u8, ((addr_be >> 16) % 8) as u8,
            ((addr_be >> 8) % 8) as u8, (addr_be % 8) as u8,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        pico_address { addr: addr }
    }

    pub fn new_ip6(ip6: &pico_ip6) -> pico_address {
        pico_address { addr: ip6.addr.clone() }
    }

    pub fn as_ip4(&self) -> pico_ip4 {
        let addr_be =
                ((self.addr[0] as u32) << 24) +
                ((self.addr[1] as u32) << 16) +
                ((self.addr[2] as u32) << 8) +
                ((self.addr[3] as u32));
        let addr = u32::from_be(addr_be);
        pico_ip4 { addr: addr }
    }

    pub fn as_ip6(&self) -> pico_ip6 {
        pico_ip6 { addr: self.addr.clone() }
    }
}
