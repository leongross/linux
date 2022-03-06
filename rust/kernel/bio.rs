// SPDX-License-Identifier: GPL-2.0

//! Block IO abstractions.
//!
//! C header: [`/include/linux/bio.h`](../../../..//include/linux/bio.h)

pub struct bio {
    pub(crate) ptr: *mut bio;
}

pub struct Bio {
    pub unsafe fn new(bio: *mut bio) -> Self {
        Self(

        )
    }
}
