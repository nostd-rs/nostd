// Copyright (c) Jeeyong Um <conr2d@proton.me>, Jungyong Um <ian.jungyong.um@gmail.com>
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "alloc")]
extern crate alloc as alloc_;

#[cfg(feature = "alloc")]
pub use alloc_::*;

pub use core::*;

#[cfg(feature = "alloc")]
pub use alloc_::{alloc, borrow, ffi, fmt, slice, str, sync, task};

pub mod prelude {
    #[cfg(feature = "alloc")]
    pub use alloc_::{
        borrow::ToOwned,
        boxed::Box,
        format,
        string::{String, ToString},
        vec,
        vec::Vec,
    };
}

#[cfg(all(feature = "io", not(feature = "std")))]
pub mod io;
#[cfg(feature = "std")]
pub use std::io;
