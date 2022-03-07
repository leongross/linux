// SPDX-License-Identifier: GPL-2.0

//! Device mapper
//!
//! C header: [`/include/linux/device-mapper.h`](../../../../include/linux/device-mapper.h)

#![allow(unused_imports)]
use crate::linked_list::Wrapper;
use crate::{bindings, bio, c_types, error, str::CStr, Result, ThisModule};
use core::convert::TryInto;

#[repr(C)]
pub struct TargetType<'a> {
    features: u64,
    name: &'a str,
    module: &ThisModule,
    version: [u8; 3],
}

pub unsafe fn dm_register_target<T>(t: &T) -> error::Result
where
    T: target_type,
{
    unsafe {
        // is this a good idea? maybe it is better to operate on the raw struct?
        bindings::dm_register_target(t.as_ref() as *mut dm_target);
    }
    Ok(())
}

#[repr(C)]
pub struct dm_target {
    pub(crate) ptr: *mut bindings::dm_target,
}

impl dm_target {}

pub trait target_type {
    unsafe fn new(features: u64, name: &str, version: [u8; 3], module: &ThisModule) -> Self;
    unsafe fn ctr(
        target: *mut dm_target,
        argc: *mut c_types::c_uint,
        argv: *mut *mut c_types::c_char,
    );
    unsafe fn dtr(target: *mut dm_target);
    unsafe fn map(target: *mut dm_target, bio: bio::Bio);
}

/// Defining the necessary targets to implement
impl target_type for TargetType<'_> {
    unsafe fn new(features: u64, name: &str, version: [u8; 3], module: &ThisModule) -> Self {
        TargetType {
            features,
            name,
            module,
            version,
        }
    }
    unsafe fn ctr(
        target: *mut dm_target,
        argc: *mut c_types::c_uint,
        argv: *mut *mut c_types::c_char,
    ) {
    }
    unsafe fn dtr(target: *mut dm_target) {}
    unsafe fn map(target: *mut dm_target, bio: bio::Bio) {}
}
