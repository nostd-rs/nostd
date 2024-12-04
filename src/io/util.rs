// Copyright (c) Brendan Molloy <brendan@bbqsrc.net>
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use core::mem::MaybeUninit;

use crate::io::{ErrorKind, Read, Write};

pub fn copy<R, W, const S: usize>(reader: &mut R, writer: &mut W) -> crate::io::Result<u64>
where
    R: ?Sized + Read,
    W: ?Sized + Write,
{
    let mut buf = MaybeUninit::<[u8; S]>::uninit();
    // FIXME: #42788
    //
    //   - This creates a (mut) reference to a slice of _uninitialized_ integers, which is
    //     **undefined behavior**
    //
    //   - Only the standard library gets to soundly "ignore" this, based on its privileged
    //     knowledge of unstable rustc internals;
    unsafe {
        reader.initializer().initialize(buf.assume_init_mut());
    }

    let mut written = 0;
    loop {
        let len = match reader.read(unsafe { buf.assume_init_mut() }) {
            Ok(0) => return Ok(written),
            Ok(len) => len,
            Err(ref e) if e.kind() == ErrorKind::Interrupted => continue,
            Err(e) => return Err(e),
        };
        writer.write_all(unsafe { &buf.assume_init_ref()[..len] })?;
        written += len as u64;
    }
}
