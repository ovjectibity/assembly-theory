/*Interoperability layer to interact with Mujoco libs
 for physics. */

use libc::{c_int,size_t};

#[repr(C)]
pub struct mjData {
    narena: size_t,
    buffer: size_t,
    nplugin: c_int,
    pstack: size_t,
    pbase: size_t,
    parena: size_t,
    maxuse_stack: size_t, 
    maxuse_threadstack: [size_t;128],
    maxuse_arena: size_t,

}

#[repr(C)]
pub struct mjModel {

}