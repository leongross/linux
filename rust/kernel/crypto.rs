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

// https://github.com/torvalds/linux/blob/master/include/crypto/hash.h#L42
struct hash_alg_common {
    pub(crate) ptr: *mut bindings::hash_alg_common,
}

impl hash_alg_common {

}

pub 'a struct sdesc {
    pub shash: shash_desc,
    pub ctx: &'a [u8]
}

pub '_ trait Hash {
    pub '_ sdesc: crate::sdesc,

}

impl Hash {
    pub fn new() -> Self {}
    pub fn hash() -> &[u8] {}
}
