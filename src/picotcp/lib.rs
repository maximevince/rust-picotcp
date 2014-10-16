#![crate_name = "picotcp"]

#![comment = "Rust PicoTCP"]
#![license = "GPL2"]
#![crate_type = "lib"]
#![feature(globs)]

extern crate libc;

pub use pico_stack::*;

pub mod pico_stack;
//pub mod pico_ipv4;

