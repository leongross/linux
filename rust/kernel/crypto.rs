//! This implementaion is primarily focussed on providing Sha256 since I need it the most
//!
//! C headers: [`include/crypto/hash.h>`](../../../../include/crypto/hash.h)

#[allow(unused_imports)]
use crate::{bindings, c_types, error, Error, Result};
use alloc::boxed::Box;
// use core::*;
use core::convert::TryInto;

// https://elixir.bootlin.com/linux/latest/source/include/crypto/hash.h#L239
#[allow(non_camel_case_types)]
pub struct crypto_shash {
    pub(crate) ptr: *mut bindings::crypto_shash,
}

// https://elixir.bootlin.com/linux/latest/source/include/crypto/hash.h#L150
#[allow(non_camel_case_types)]

// pub struct shash_desc {
//     pub tfm: *mut crypto_shash,
//     pub __ctx: __IncompleteArrayField<*mut c_types::c_void>,

pub struct shash_desc(*mut bindings::shash_desc);

impl shash_desc {
    pub fn from(alg: &crypto_shash) -> Self {
        Self (
            &mut bindings::shash_desc {
                tfm: alg.ptr,
                __ctx: bindings::__IncompleteArrayField::new(),
            }
        )
    }
}

// https://github.com/torvalds/linux/blob/master/include/crypto/hash.h#L42
#[allow(non_camel_case_types)]
pub struct hash_alg_common {
    pub(crate) ptr: *mut bindings::hash_alg_common,
}

impl crypto_shash {
    // https://elixir.bootlin.com/linux/latest/source/include/crypto/hash.h#L718
    pub unsafe fn crypto_alloc_shash(
        alg_name: &mut str,
        cipher_type: u32,
        cipher_mask: u32,
    ) -> Result<Self> {
        let ptr = unsafe {
            bindings::crypto_alloc_shash(
                alg_name.as_mut_ptr() as *mut c_types::c_char,
                cipher_type,
                cipher_mask,
            )
        };
        if ptr.is_null() {
            return Err(Error::EBADF);
        }
        Ok(Self { ptr })
    }

    /// Calculate hash of input and save it to the output
    /// ```
    /// use kernel::crypto;
    ///
    /// let mut input   = [0u8; 32];
    /// let mut output  = [0u8; 32];
    /// let c = crypto::crypto_alloc_shash("sha256", &input, &output).unwrap();
    /// c.calc_hash(&mut input, &mut output);
    /// ```
    pub(crate) unsafe fn calc_hash(&mut self, input: &mut [u8], digest: &mut [u8]) -> c_types::c_int {
        let s = unsafe { sdesc::new(&self) };

        // https://elixir.bootlin.com/linux/latest/source/include/crypto/hash.h#L867
        unsafe {
            bindings::crypto_shash_digest(
                s.shash.0,
                input.as_mut_ptr() as *mut c_types::c_uchar,
                input.len().try_into().unwrap(),
                digest.as_mut_ptr() as *mut c_types::c_uchar,
            )
        }
    }
}
// fixating size to 32, just for testing
// the ctx will hold the digest, so it has to have a variable length depending on the selected cipher
#[allow(non_camel_case_types)]
pub struct sdesc {
    pub shash: shash_desc,
    pub ctx: [u8; 32],
}

// set attributes: https://github.com/Rust-for-Linux/linux/blob/rust/rust/kernel/file.rs#L42
// TODO: Make this a box
impl sdesc {
    pub unsafe fn new(alg: &crypto_shash) -> sdesc {
        // algo size: https://elixir.bootlin.com/linux/latest/source/include/crypto/hash.h#L826
        //let algo_size: c_types::c_uint = unsafe { bindings::crypto_shash_descsize(alg.0) };
        //if algo_size == 0 {
        //    panic!("Hash algo size cannot be 0!")
        //}
        Self {
            shash: shash_desc::from(alg),
            ctx: [0u8; 32],
        }
    }
}
