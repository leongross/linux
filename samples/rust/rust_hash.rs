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

fn test_sha256_0() {
    let h = unsafe { crypto::crypto_shash::new(c_str!("sha256"), 0, 0) }.unwrap();
    let s: crypto::sdesc = unsafe { crypto::sdesc::new(&h) };

    // Sha256(0000000000000000000000000000000000000000000000000000000000000000)
    // = 66687aadf862bd776c8fc18b8e9f8e20089714856ee233b3902a591d0d5f2925
    let mut input = [0u8; 32];
    let mut output = [0u8; 32];

    unsafe { h.calc_hash(&mut input, &mut output) };
    pr_info!("Sha256 of 32 null bytes:");
    pr_info!("{:#02X?}", output);
    pr_info!("last byte: {:x?}", output[31])
}

fn test_sha256_1() {
    let h = unsafe { crypto::crypto_shash::new(c_str!("sha256"), 0, 0) }.unwrap();
    let s: crypto::sdesc = unsafe { crypto::sdesc::new(&h) };

    // SHa256(0101010101010101010101010101010101010101010101010101010101010101)
    // = 72cd6e8422c407fb6d098690f1130b7ded7ec2f7f5e1d30bd9d521f015363793
    let mut input = [1u8; 32];
    let mut output = [0u8; 32];

    unsafe { h.calc_hash(&mut input, &mut output) };
    pr_info!("Sha256 of 32 one bytes:");
    pr_info!("{:#02X?}", output);
    pr_info!("last byte: {:x?}", output[31])
}


fn test_sha512() {
    let h = unsafe { crypto::crypto_shash::new(c_str!("sha512"), 0, 0) }.unwrap();
    let s: crypto::sdesc = unsafe { crypto::sdesc::new(&h) };

    // Sha256(0000000000000000000000000000000000000000000000000000000000000000)
    // = 5046adc1dba838867b2bbbfdd0c3423e58b57970b5267a90f57960924a87f1960a6a85eaa642dac835424b5d7c8d637c00408c7a73da672b7f498521420b6dd3
    let mut input = [0u8; 32];
    let mut output = [0u8; 64];

    unsafe { h.calc_hash(&mut input, &mut output) };
    pr_info!("Sha512 of 32 null bytes:");
    pr_info!("{:#02X?}", output);
    pr_info!("last byte: {:x?}", output[63])
}

impl KernelModule for Hashing {
    fn init(_name: &'static CStr, _module: &'static ThisModule) -> Result<Self> {
        pr_info!("Hashing module!");

        test_sha256_0();
        test_sha256_1();
        test_sha512();

        Ok(Hashing {})
    }
}

impl Drop for Hashing {
    fn drop(&mut self) {}
}
