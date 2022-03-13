// SPDX-License-Identifier: GPL-2.0

use kernel::bindings;
use kernel::device_mapper;
use kernel::prelude::*;
use kernel::{bio, c_str, c_types};

module! {
    type: DevMapper,
    name: b"rust_minimal",
    author: b"Rust for Linux Contributors",
    description: b"Rust minimal sample",
    license: b"GPL v2",
}

pub struct DevMapper {
    t: device_mapper::TargetType,
}

pub unsafe extern "C" fn rust_ctr(
    target: *mut kernel::bindings::dm_target,
    argc: c_types::c_uint,
    argv: *mut *mut c_types::c_char,
) -> u32 {
    1
}

pub unsafe extern "C" fn rust_dtr(target: *mut kernel::bindings::dm_target) {}
pub unsafe extern "C" fn rust_map_fn(target: *mut kernel::bindings::dm_target, bio: *mut bio::bio) {
}

pub trait Functions {
    unsafe extern "C" fn ctr(
        ti: *mut bindings::dm_target,
        argc: u32,
        argv: *mut *mut c_types::c_char,
    ) -> i32 {
        1
    }
    unsafe extern "C" fn dtr(ti: *mut bindings::dm_target) {}
    unsafe extern "C" fn map(ti: *mut bindings::dm_target, bio: *mut bindings::bio) -> i32 {
        1
    }
}

type dmx = device_mapper::TargetType;
impl Functions for dmx {}

impl KernelModule for DevMapper {
    fn init(_name: &'static CStr, module: &'static ThisModule) -> Result<Self> {
        let mut t: device_mapper::TargetType = unsafe { device_mapper::TargetType::new(
            c_str!("dev_mapper"),
            [1u32, 2u32, 3u32],
            module,
            //Some(rust_ctr),
            //Some(rust_dtr),
            //Some(rust_map_fn),
        )};

        dbg!(&t);
        device_mapper::dm_register_target(&mut t);
        Ok(DevMapper { t })
    }
}

impl Drop for DevMapper {
    fn drop(&mut self) {
        device_mapper::dm_unregister_target(&mut self.t);
    }
}
