// SPDX-License-Identifier: GPL-2.0
use kernel::c_str;
use kernel::crypto;
use kernel::prelude::*;

module! {
    type: Hashing,
    name: b"rust_hashing",
    author: b"Rust for Linux Contributors",
    description: b"Rust hashing sample",
    license: b"GPL v2",
}

struct Hashing {}

impl KernelModule for Hashing {
    fn init(_name: &'static CStr, _module: &'static ThisModule) -> Result<Self> {
        pr_info!("Hashing module!");
        let h = unsafe { crypto::crypto_shash::new(c_str!("sha256"), 0, 0) }.unwrap();
        dbg!(&h);

        let s: crypto::sdesc = unsafe { crypto::sdesc::new(&h) };
        dbg!(&s);

        let mut input = [0u8; 32];
        let mut output = [0u8; 32];

        unsafe { h.calc_hash(&mut input, &mut output) };
        dbg!(&input, &output);

        Ok(Hashing {})
    }
}

impl Drop for Hashing {
    fn drop(&mut self) {}
}
