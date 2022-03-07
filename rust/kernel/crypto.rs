//! This implementaion is primarily focussed on providing Sha256 since I need it the most
//!
//! C headers: [`include/crypto/hash.h>`](../../../../include/crypto/hash.h)

#[allow(unused_imports)]
use crate::{bindings, Result, c_types, Error, error};
use alloc::boxed::Box;
use core::*;


// https://elixir.bootlin.com/linux/latest/source/include/crypto/hash.h#L239
#[allow(non_camel_case_types)]
struct crypto_shash {
    pub(crate) ptr: *mut bindings::crypto_shash,
}

// https://elixir.bootlin.com/linux/latest/source/include/crypto/hash.h#L150
#[allow(non_camel_case_types)]
struct shash_desc(*mut bindings::shash_desc);

// https://github.com/torvalds/linux/blob/master/include/crypto/hash.h#L42
#[allow(non_camel_case_types)]
struct hash_alg_common {
    pub(crate) ptr: *mut bindings::hash_alg_common,
}

impl crypto_shash {
    // https://elixir.bootlin.com/linux/latest/source/include/crypto/hash.h#L718
    pub unsafe fn crypto_alloc_shash(
        alg_name: &str,
        cipher_type: u32,
        cipher_mask: u32,
    ) -> Result<Self> {
        let ptr = unsafe {
            bindings::crypto_alloc_shash(alg_name.as_mut_ptr() as *mut c_types::c_char, cipher_type, cipher_mask)
        };
        if ptr.is_null() {
            return Err(Error::EBADF);
        }
        Ok(Self { ptr })
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
    pub unsafe fn calc_hash(&mut self, input: &mut [u8], digest: &mut [u8]) -> c_types::c_int {
        let mut s = *sdesc::init(&mut self);

        // https://elixir.bootlin.com/linux/latest/source/include/crypto/hash.h#L867
        unsafe {
            bindings::crypto_shash_digest(
                &mut s.shash as *mut bindings::shash_desc,
                input as *mut c_types::c_uchar,
                input.len().try_into().unwrap(),
                digest as *mut c_types::c_uchar,
            )
        }
    }
}
// fixating size to 32, just for testing
#[allow(non_camel_case_types)]
pub struct sdesc {
    pub shash: shash_desc,
    pub ctx: [u8; 32],
}

// set attributes: https://github.com/Rust-for-Linux/linux/blob/rust/rust/kernel/file.rs#L42
impl sdesc {
    pub unsafe fn init(alg: &mut crypto_shash) -> Box<sdesc> {
        let b = Box::try_new(alg).unwrap();
        *(b).as_mut().tfm = alg;
        b
    }
}
