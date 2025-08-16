// Copyright (C) Jeeyong Um <conr2d@proton.me>
//               Jungyong Um <ian.jungyong.um@gmail.com>
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#[macro_export]
macro_rules! export {
    ($mod:ident) => {
        export!($mod, $mod);
    };
    ($mod:ident, $($path:ident)::*) => {
        pub mod $mod {
            import!($($path)*);
        }
    };
}

#[macro_export]
macro_rules! import {
    ($($path:ident)::*) => {
        #[cfg(feature = "alloc")]
        pub use alloc_crate::$($path::)**;
        #[allow(unused_imports)]
        pub use core::$($path::)**;
    };
}
