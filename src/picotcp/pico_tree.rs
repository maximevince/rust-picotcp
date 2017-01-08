#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use libc::{c_int, c_void, uint8_t};

#[repr(C)]
pub struct pico_tree_node
{
    pub keyValue: *mut c_void, /* generic key */

    pub parent: *mut pico_tree_node,
    pub leftChild: *mut pico_tree_node,
    pub rightChild: *mut pico_tree_node,
    pub color: uint8_t
}

#[repr(C)]
pub struct pico_tree
{
    pub root: *mut pico_tree_node,
    pub compare: extern fn (keyA: *mut c_void, keyB: *mut c_void) -> c_int,
}

extern "C" {
    pub static LEAF: pico_tree_node;
}

extern "C" {
    pub fn pico_tree_insert_implementation(tree: *mut pico_tree, key: *mut c_void, allocator: uint8_t) -> *mut c_void; // TODO: #ifdef PICO_SUPPORT_MM
    pub fn pico_tree_delete_implementation(tree: *mut pico_tree, key: *mut c_void, allocator: uint8_t) -> *mut c_void; // TODO: #ifdef PICO_SUPPORT_MM
}


/*
 * Manipulation functions
 */
extern "C" {
    pub fn pico_tree_insert(tree: *mut pico_tree, key: *mut c_void) -> *mut c_void;
    pub fn pico_tree_delete(tree: *mut pico_tree, key: *mut c_void) -> *mut c_void;
    pub fn pico_tree_findKey(tree: *mut pico_tree, key: *mut c_void) -> *mut c_void;
    pub fn pico_tree_drop(tree: *mut pico_tree);
    pub fn pico_tree_empty(tree: *mut pico_tree) -> c_int;
    pub fn pico_tree_findNode(tree: *mut pico_tree, key: *mut c_void) -> *mut pico_tree_node;

    pub fn pico_tree_first(tree: *mut pico_tree) -> *mut c_void;
    pub fn pico_tree_last(tree: *mut pico_tree) -> *mut c_void;
}

/*
 * Traverse functions
 */
extern "C" {
    pub fn pico_tree_lastNode(node: *mut pico_tree) -> *mut pico_tree_node;
    pub fn pico_tree_firstNode(node: *mut pico_tree) -> *mut pico_tree_node;
    pub fn pico_tree_next(node: *mut pico_tree) -> *mut pico_tree_node;
    pub fn pico_tree_prev(node: *mut pico_tree) -> *mut pico_tree_node;
}


