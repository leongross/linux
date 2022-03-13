// SPDX-License-Identifier: GPL-2.0
use kernel::c_str;
use kernel::crypto;
use kernel::prelude::*;

module! {
    type: Hashing,
    name: b"rust_hashing",
    author: b"leongross <leon.gross@rub.de>",
    description: b"Rust hashing sample",
    license: b"GPL v2",
}

struct Hashing {}

fn test_sha256() {
    let h = unsafe { crypto::crypto_shash::new(c_str!("sha256"), 0, 0) }.unwrap();
    let s: crypto::sdesc = unsafe { crypto::sdesc::new(&h) };
    dbg!(&s);

    let mut input = [0u8; 32];
    let mut output = [0u8; 32];

    unsafe { h.calc_hash(&mut input, &mut output) };
    pr_info!("Sha256 of 32 null bytes:");
    dbg!(&output);
}

fn test_sha512() {
    let h = unsafe { crypto::crypto_shash::new(c_str!("sha512"), 0, 0) }.unwrap();
    let s: crypto::sdesc = unsafe { crypto::sdesc::new(&h) };
    dbg!(&s);

    let mut input = [0u8; 32];
    let mut output = [0u8; 64];

    unsafe { h.calc_hash(&mut input, &mut output) };
    pr_info!("Sha512 of 32 null bytes:");
    dbg!(&output);
}

impl KernelModule for Hashing {
    fn init(_name: &'static CStr, _module: &'static ThisModule) -> Result<Self> {
        pr_info!("Hashing module!");

        test_sha256();
        test_sha512();

        Ok(Hashing {})
    }
}

impl Drop for Hashing {
    fn drop(&mut self) {}
}
