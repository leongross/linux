//! This implementation is primarily focussed on providing Sha256 since I need it the most
//!
//! C headers: [`include/crypto/hash.h>`](../../../../include/crypto/hash.h)

// https://doc.rust-lang.org/rustdoc/lints.html
#![allow(rustdoc::broken_intra_doc_links)] // allows the lint, no diagnostics will be reported

#[allow(unused_imports)]
use crate::{bindings, c_types, error, pr_info, str::CStr, Error, Result};
use core::convert::TryInto;
// https://elixir.bootlin.com/linux/latest/source/include/crypto/hash.h#L150
#[allow(non_camel_case_types)]
#[derive(Debug)]
pub struct shash_desc(*mut bindings::shash_desc);

impl shash_desc {
    pub fn from(alg: &crypto_shash) -> Self {
        let count = 32;

        // https://doc.rust-lang.org/beta/nomicon/phantom-data.html
        let mut s = bindings::shash_desc {
            tfm: alg.ptr,
            __ctx: bindings::__IncompleteArrayField::new(),
        };

        Self(&mut s)
    }
}

// https://github.com/torvalds/linux/blob/master/include/crypto/hash.h#L42
#[allow(non_camel_case_types)]
#[derive(Debug)]
pub struct hash_alg_common {
    pub(crate) ptr: *mut bindings::hash_alg_common,
}

// https://elixir.bootlin.com/linux/latest/source/include/crypto/hash.h#L239
#[allow(non_camel_case_types)]
#[derive(Debug)]
pub struct crypto_shash {
    pub(crate) ptr: *mut bindings::crypto_shash,
}

impl crypto_shash {
    // https://elixir.bootlin.com/linux/latest/source/include/crypto/hash.h#L718
    // use &'static CStr instead of &str, see amba.rs as reference
    pub unsafe fn new(name: &'static CStr, cipher_type: u32, cipher_mask: u32) -> Result<Self> {
        let ptr =
            unsafe { bindings::crypto_alloc_shash(name.as_char_ptr(), cipher_type, cipher_mask) };
        if ptr.is_null() {
            return Err(Error::EBADF);
        }
        Ok(Self { ptr })
    }

    pub unsafe fn calc_hash(&self, data: &mut [u8], out: &mut [u8]) -> c_types::c_int {
        let s = unsafe { sdesc::new(&self) };

        // https://elixir.bootlin.com/linux/latest/source/include/crypto/hash.h#L867
        unsafe {
            // pub fn crypto_shash_digest( desc: *mut shash_desc, data: *const u8_, len: c_types::c_uint, out: *mut u8_, ) -> c_types::c_int;
            bindings::crypto_shash_digest(
                s.shash.0,
                data.as_mut_ptr() as *const c_types::c_uchar,
                data.len().try_into().unwrap(),
                out.as_mut_ptr() as *mut c_types::c_uchar,
            )
        }
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug)]
pub struct sdesc {
    pub shash: shash_desc,
    pub __ctx: [u8; 512],
}

// set attributes: https://github.com/Rust-for-Linux/linux/blob/rust/rust/kernel/file.rs#L42
// TODO: Make this a box
impl sdesc {
    pub unsafe fn new(alg: &crypto_shash) -> sdesc {
        // algo size: https://elixir.bootlin.com/linux/latest/source/include/crypto/hash.h#L826
        Self {
            shash: shash_desc::from(alg),
            __ctx: [0u8; 512],
        }
    }
}
