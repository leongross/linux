// SPDX-License-Identifier: GPL-2.0
use kernel::{crypto};
use kernel::prelude::*;

module! {
    type: Hashing,
    name: b"rust_minimal",
    author: b"Rust for Linux Contributors",
    description: b"Rust minimal sample",
    license: b"GPL v2",
}

struct Hashing {}

impl KernelModule for Hashing {
    fn init(_name: &'static CStr, _module: &'static ThisModule) -> Result<Self> {
        pr_info!("Hashing module!");
        let mut cipher = "sha256";
        let h = unsafe { crypto::crypto_shash::crypto_alloc_shash(&mut cipher, 0, 0) };
        Ok(Hashing{})
    }

}

impl Drop for Hashing {
    fn drop(&mut self) {}
}
