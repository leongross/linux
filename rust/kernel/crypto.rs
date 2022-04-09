//! This implementation is primarily focussed on providing Sha256 since I need it the most
//!
//! C headers: [`include/crypto/hash.h`](../../../../include/crypto/hash.h)

// https://doc.rust-lang.org/rustdoc/lints.html
#![allow(rustdoc::broken_intra_doc_links)] // allows the lint, no diagnostics will be reported

use crate::c_types::{c_char, c_int, c_uchar, c_uint};
#[allow(unused_imports)]
use crate::{
    bindings, c_str, c_types, error, pr_crit, pr_err, pr_info, str::BStr, str::CStr, Error, Result,
};
use core::{convert::TryInto, panic, str::FromStr};

/// Supported Hashes for matching
pub enum Hashes {
    Sha1,
    Sha224,
    Sha256,
    Sha384,
    Sha512,
}

// impl FromStr for Hashes {
//     type Err = ();
//     fn from_str(input: &str) -> Result<Hashes, Self::Err> {
//         match input {
//             "sha1" => Ok(Hashes::Sha1),
//             "sha224" => Ok(Hashes::Sha224),
//             "sha256" => Ok(Hashes::Sha256),
//             "sha384" => Ok(Hashes::Sha384),
//             "sha512" => Ok(Hashes::Sha512),
//             _ => Err(()),
//         }
//     }
// }

// https://elixir.bootlin.com/linux/latest/source/include/crypto/hash.h#L150
#[allow(non_camel_case_types)]
#[derive(Debug)]
pub struct shash_desc(*mut bindings::shash_desc);

impl shash_desc {
    #[no_mangle]
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

    // This is working!
    pub unsafe fn calc_hash_raw(
        &self,
        data: *mut c_uchar,
        out: *mut c_uchar,
        len: c_uint,
    ) -> c_types::c_int {
        let s = unsafe { sdesc::new(&self) };

        // https://elixir.bootlin.com/linux/latest/source/include/crypto/hash.h#L867
        unsafe {
            // https://manpages.debian.org/testing/linux-manual-4.8/crypto_shash_digest.9.en.html
            // Since there is only one update, digest can easily be used instead of calling
            // * crypto_shash_init
            // * crypto_shash_update
            // * crypto_shash_final
            bindings::crypto_shash_digest(s.shash.0, data, len, out)
        }
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug)]
#[no_mangle]
pub struct sdesc {
    pub shash: shash_desc,
    pub __ctx: [u8; 512],
}

// set attributes: https://github.com/Rust-for-Linux/linux/blob/rust/rust/kernel/file.rs#L42
impl sdesc {
    pub unsafe fn new(alg: &crypto_shash) -> sdesc {
        // algo size: https://elixir.bootlin.com/linux/latest/source/include/crypto/hash.h#L826
        Self {
            shash: shash_desc::from(alg),
            __ctx: [0u8; 512],
        }
    }
}

#[allow(non_camel_case_types)]
#[no_mangle]
pub fn rust_hash_buffer(
    input: &mut [u8],
    output: &mut [u8],
    hash: *const c_char,
) -> c_types::c_int {
    let hash = unsafe { CStr::from_char_ptr(hash) };
    if hash.is_empty() {
        pr_info!("hash cannot be empty!");
        return 0;
    }

    pr_info!("Calling hasher with hash {}", hash.to_str().unwrap());
    let h = unsafe { crypto_shash::new(&hash, 0, 0) }.unwrap();
    let s: sdesc = unsafe { sdesc::new(&h) };

    let ret: c_types::c_int = unsafe { h.calc_hash(input, output) };
    //pr_info!("{:#02X?}", output);
    return ret;
}

#[allow(non_camel_case_types)]
#[no_mangle]
pub fn rust_hash_buffer_sha256(input: &mut [u8], output: &mut [u8]) -> c_types::c_int {
    let hash = c_str!("sha256");
    if hash.is_empty() {
        pr_info!("hash cannot be empty!");
        return 0;
    }

    pr_info!("Calling hasher with hash {}", hash);
    let h = unsafe { crypto_shash::new(&hash, 0, 0) }.unwrap();
    let s: sdesc = unsafe { sdesc::new(&h) };

    let ret: c_types::c_int = unsafe { h.calc_hash(input, output) };
    //pr_info!("{:#02X?}", output);
    return ret;
}

#[allow(non_camel_case_types)]
#[no_mangle]
pub fn rust_hash_buffer_sha256_raw(
    input: *mut c_uchar,
    output: *mut c_uchar,
    len: c_uint,
) -> c_int {
    let hash = c_str!("sha256");
    if input.is_null() {
        panic!("Invalid input");
    }

    if output.is_null() {
        panic!("Invalid output")
    }

    pr_info!("Calling hasher with hash {}", hash);
    let h = unsafe { crypto_shash::new(&hash, 0, 0) }.unwrap();
    let s: sdesc = unsafe { sdesc::new(&h) };

    let ret: c_types::c_int = unsafe { h.calc_hash_raw(input, output, len) };
    return ret;
}

// let bytes = unsafe { core::slice::from_raw_parts(ptr as _, len as _) };
#[allow(non_camel_case_types)]
#[no_mangle]
pub fn rust_hash_buffer_raw_hack(
    input: *mut c_uchar,
    output: *mut c_uchar,
    len: c_uint,
    hash: *const c_char,
) -> c_int {
    if input.is_null() {
        panic!("Invalid input");
    }

    if output.is_null() {
        panic!("Invalid output")
    }

    let hash = unsafe { CStr::from_char_ptr(hash) };
    pr_info!("Calling hasher with hash {}", hash);

    let h = unsafe { crypto_shash::new(&hash, 0, 0) }.unwrap();
    let s: sdesc = unsafe { sdesc::new(&h) };

    let ret: c_types::c_int = unsafe { h.calc_hash_raw(input, output, len) };
    return ret;
}
