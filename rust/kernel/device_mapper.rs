// SPDX-License-Identifier: GPL-2.0

//! Device mapper
//!
//! C header: [`/include/linux/device-mapper.h`](../../../../include/linux/device-mapper.h)

use crate::{
    bindings,
    str::CStr,
    Result,
    c_types,
    bio
}

#[repr(C)]
pub struct TargetType {
    features: u64,
    name: &str,
    module: kernel::ThisModule,
    version: [u8; 3],
}

pub unsafe fn dm_register_target<T>(t: &T) -> Option<u8>
    where T: target_type {
    unsafe {
        // is this a good idea? maybe it is better to operate on the raw struct?
        bindings::dm_register_target(t);
    }

}

#[repr(C)]
pub struct dm_target {
    pub(crate) ptr: *bindings::dm_target,
}

impl dm_target {

}

pub trait target_type {
    pub unsafe fn new(name: &str, version: [u8;8])
    pub fn ctr(target: *mut dm_target, argc: *mut c_types::c_uint, argv: *mut *mut c_types::c_char);
    pub fn dtr(target: *mut dm_target);
    pub fn map(target: *mut dm_target, bio::bio);
}

/// Defining the necessary targets to implement
impl target_type for TargetType {
    pub unsafe fn new(features: u64, name: &str, version: [u8;8]) -> Self {
        TargetType {
            features,
            name,
            module: &'static ThisModule,
            version,
        }
    }
}
