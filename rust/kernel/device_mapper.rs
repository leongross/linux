// SPDX-License-Identifier: GPL-2.0

//! Device mapper
//!
//! C header: [`/include/linux/device-mapper.h`](../../../../include/linux/device-mapper.h)

#![allow(unused_imports)]
use crate::{bindings, bio, c_types, error, str::CStr, Result, ThisModule};
use core::convert::TryInto;

#[repr(C)]
pub struct dm_target {
    pub(crate) ptr: *mut bindings::dm_target,
}

pub struct TargetType {
    pub ptr: bindings::target_type,
}

/// why is there sometimes ti and sometimes target for struct dm_target?
/// https://rust-lang.github.io/unsafe-code-guidelines/layout/function-pointers.html
///
/// Example:
/// ```
/// use kernel::device_mapper;
///
/// pub unsafe extern "C" fn rust_ctr(
///     target: *mut dm_target,
///     argc: c_types::c_uint,
///     argv: *mut *mut c_types::c_char) {
/// }
///
/// pub unsafe extern "C" fn rust_dtr( target: *mut dm_target) {}
///
/// let t: TargetType = TargetType::new()
/// ```

static TODO: i32 = 1337;

pub trait Functions {
    unsafe extern "C" fn ctr(
        ti: *mut bindings::dm_target,
        argc: u32,
        argv: *mut *mut c_types::c_char,
    ) -> i32 {
        TODO
    }
    unsafe extern "C" fn dtr(ti: *mut bindings::dm_target) {}
    unsafe extern "C" fn map(ti: *mut bindings::dm_target, bio: *mut bindings::bio) -> i32 {
        TODO
    }
}

impl Functions for TargetType{}
unsafe impl Sync for TargetType{}

impl TargetType {
    pub unsafe fn new(
        name: &'static CStr,
        version: [u32; 3],
        module: &'static ThisModule,
        //ctr: unsafe extern "C" fn(ti: *mut bindings::dm_target, argc: u32, argv: *mut *mut c_types::c_char) -> i32,
        //dtr: unsafe extern "C" fn(ti: *mut bindings::dm_target),
        //map: unsafe extern "C" fn(ti: *mut bindings::dm_target, bio: *mut bindings::bio) -> i32,

        //ctr: bindings::dm_ctr_fn,
        //dtr: bindings::dm_dtr_fn,
        //map: bindings::dm_map_fn,
    ) -> Self {
        let mut t = bindings::target_type::default();
        t.name = name.as_char_ptr();
        t.module = module.0;
        t.version = version;

        t.ctr = Some(TargetType::ctr);
        t.dtr = Some(TargetType::dtr);
        t.map = Some(TargetType::map);

        Self { ptr: t }
    }
}

/// Call this in the Init of the module
pub fn dm_register_target(t: &mut TargetType) {
    let r = unsafe { bindings::dm_register_target(&mut t.ptr) };
    if r != 0 {
        panic!("could not register target: {}", r);
    }
}

/// Call this in the Drop of the module
pub fn dm_unregister_target(t: &mut TargetType) {
    unsafe { bindings::dm_unregister_target(&mut t.ptr) }
}
