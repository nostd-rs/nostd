// Copyright (C) Brendan Molloy <brendan@bbqsrc.net>
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Traits, helpers, and type definitions for core I/O functionality.

mod buffered;
mod cursor;
mod error;
mod impls;
mod traits;
mod util;

pub use buffered::{BufReader, BufWriter, LineWriter};
pub use cursor::Cursor;
pub use error::{Error, ErrorKind, Result};
pub use traits::{BufRead, Bytes, Chain, Read, Seek, SeekFrom, Take, Write};
pub use util::copy;

/// The I/O Prelude.
///
/// The purpose of this module is to alleviate imports of many common I/O traits
/// by adding a glob import to the top of I/O heavy modules:
///
/// ```
/// # #![allow(unused_imports)]
/// use nostd::io::prelude::*;
/// ```
pub mod prelude {
    pub use super::{BufRead, Read, Seek, Write};
}
