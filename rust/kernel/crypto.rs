//! This implementaion is primarily focussed on providing Sha256 since I need it the most
//!
//! C headers: [`include/crypto/hash.h>`](../../../../include/crypto/hash.h)


use crate::{bindings, Result};
use core::*;

pub struct crypto_hash {
    pub(crate) ptr: *mut bindings::crypto_hash,
}

struct shash_desc {
    pub(crate) ptr: *mut bindings::shash_desc,
}

// https://elixir.bootlin.com/linux/latest/source/include/crypto/hash.h#L150
impl shash_desc {
    pub unsafe fn new()
}

// https://github.com/torvalds/linux/blob/master/include/crypto/hash.h#L42
struct hash_alg_common {
    pub(crate) ptr: *mut bindings::hash_alg_common,
}

// https://elixir.bootlin.com/linux/latest/source/include/crypto/hash.h#L239
struct crypto_shash {
    pub(crate) ptr: *mut bindings::crypto_shash
}

// fixating size to 32, just for testing
pub struct sdesc {
    pub shash: shash_desc,
    pub ctx: [u8; 32]
}

// set attributes: https://github.com/Rust-for-Linux/linux/blob/rust/rust/kernel/file.rs#L42
impl sdesc {
    pub unsafe fn init(alg: *mut bindings::crypto_shash ) -> Self {
        let s: shash_desc =
        Self {
            shash.tfm = alg,
        }
    }
}


pub trait Hash {
    pub unsafe fn digest() -> &'static [u8] {}
}

pub struct Sha256 {
    state: [u8; 32]
}

impl Hash for Sha256 {
    pub unsafe fn digest() -> &'static [u8] {

    }
}
