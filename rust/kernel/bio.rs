// SPDX-License-Identifier: GPL-2.0

//! Block IO abstractions.
//!
//! C header: [`include/linux/bio.h`](../../../../include/linux/bio.h)
//! C header: [`include/linux/bvec.h`]


// using tuple structs for simplicity: https://doc.rust-lang.org/std/keyword.struct.html
pub struct Bio ( *mut bindings::bio };
pub struct BioVec ( *mut bindings::bio_vec );

impl Bio {
    pub unsafe fn new(bio: *mut bindings::bio) -> Self {
        Self(bio)
    }

    // https://elixir.bootlin.com/linux/latest/source/include/linux/bio.h#L406
    pub unsafe fn init(&mut self, table: &BioVec, max_vecs: c_types::c_ushort) -> {
        unsafe {
            bindings::bio_init(&mut self.0, table.0, max_vecs);
        }
    }
    // pub unsafe fn add_page(&mut self, )
}

impl BioVec {
    pub unsafe fn new(biov: *mut bindings::bio_vec) -> Self {
        Self(bio)
    }
}

// compare with clk struct: https://elixir.bootlin.com/linux/latest/source/include/linux/clk.h#L961
impl Drop for Bio {
    fn drop(&mut self) {
        unsafe { bindings::bio_uninit(self.0)}
    }
}
