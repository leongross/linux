// SPDX-License-Identifier: GPL-2.0

//! Rust minimal sample.

use kernel::device_mapper::*;
use kernel::prelude::*;
use kernel::bindings;

module! {
    type: DevMapper,
    name: b"rust_minimal",
    author: b"Rust for Linux Contributors",
    description: b"Rust minimal sample",
    license: b"GPL v2",
}

struct DevMapper {}

impl KernelModule for DevMapper {
    fn init(_name: &'static CStr, module: &'static ThisModule) -> Result<Self> {
        pub unsafe extern "C" fn rust_ctr(
            target: *mut kernel::bindings::dm_target,
            argc: c_types::c_uint,
            argv: *mut *mut c_types::c_char,
        ) -> u32 {
        }

        pub unsafe extern "C" fn rust_dtr(target: *mut kernel::bindings::dm_target) {}
        pub unsafe extern "C" fn rust_map_fn(target: *mut kernel::bindings::dm_target, bio: *mut bio) {}

        let t: TargetType = TargetType::new(
            c_str!("dev_mapper"),
            [1u32, 2u32, 3u32],
            module,
            Some(rust_ctr),
            Some(rust_dtr),
            Some(rust_map_fn),
        );
        Ok(DevMapper {})
    }
}

impl Drop for DevMapper {
    fn drop(&mut self) {}
}
