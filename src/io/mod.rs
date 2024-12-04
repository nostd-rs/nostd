// Copyright (c) Brendan Molloy <brendan@bbqsrc.net>
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

mod buffered;
mod cursor;
mod error;
mod impls;
mod traits;
mod util;

#[cfg(not(feature = "std"))]
pub use buffered::{BufReader, BufWriter, LineWriter};
#[cfg(not(feature = "std"))]
pub use cursor::Cursor;
#[cfg(not(feature = "std"))]
pub use error::{Error, ErrorKind, Result};
#[cfg(not(feature = "std"))]
pub use traits::{BufRead, Bytes, Chain, Read, Seek, SeekFrom, Take, Write};
#[cfg(not(feature = "std"))]
pub use util::copy;

#[cfg(feature = "std")]
pub use std::io::{
    BufRead, Bytes, Chain, Cursor, Error, ErrorKind, Read, Result, Seek, SeekFrom, Take, Write,
};
