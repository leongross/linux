//! This implementaion is primarily focussed on providing Sha256 since I need it the most
//!
//! C headers: [`include/crypto/hash.h>`](../../../../include/crypto/hash.h)


use crate::{bindings, Result};
use core::*;
use alloc::boxed::Box;


// https://elixir.bootlin.com/linux/latest/source/include/crypto/hash.h#L239
struct crypto_shash {
    pub(crate) ptr: *mut bindings::crypto_shash
}

// https://elixir.bootlin.com/linux/latest/source/include/crypto/hash.h#L150
struct shash_desc ( *mut bindings::shash_desc );
// https://github.com/torvalds/linux/blob/master/include/crypto/hash.h#L42
struct hash_alg_common {
    pub(crate) ptr: *mut bindings::hash_alg_common,
}

impl crypto_shash {
    // https://elixir.bootlin.com/linux/latest/source/include/crypto/hash.h#L718
    pub unsafe fn crypto_alloc_shash(alg_name: &str, cipher_type: u32, cipher_mask: u32) -> Result<Self> {
        let ptr = unsafe {
            bindings::crypto_alloc_shash(
                alg_name as *mut c_types::c_char,
                cipher_type,
                cipher_mask
            )
        };
        if ptr.is_null() {
            return Err(Error::EBADF);
        }
        Ok( Self{ prt } )
    }

    /// Calc hash of input and save it to the output
    /// ```
    /// use kernel::crypto;
    ///
    /// let mut input   = [0u8; 32];
    /// let mut output  = [0u8; 32];
    /// let c = crypto::crypto_alloc_shash("sha256", &input, &output).unwrap();
    /// c.calc_hash(&mut input, &mut output);
    /// ```
    pub unsafe fn calc_hash(&mut self, input: &mut [u8], digest: &mut [u8]) -> err::Error {
        let s: shash_desc = sdesc::init().unwrap()?;

        // https://elixir.bootlin.com/linux/latest/source/include/crypto/hash.h#L867
        let ret = unsafe {
            bindings::crypto_shash_digest(
                s.0,
                input as c_types::uchar,
                input.len(),
                digest as *mut c_types::uchar
            )
        };
    }
}
// fixating size to 32, just for testing
pub struct sdesc {
    pub shash: shash_desc,
    pub ctx: [u8; 32]
}

// set attributes: https://github.com/Rust-for-Linux/linux/blob/rust/rust/kernel/file.rs#L42
impl sdesc {
    pub unsafe fn init(alg: *mut crypto_shash) -> Box::<sdesc> {
        let b = Box::try_new(sdesc);
        *b.tfm = alg;
        b
    }
}
