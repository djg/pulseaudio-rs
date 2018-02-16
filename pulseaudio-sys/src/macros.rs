// Copyright © 2017-2018 The Cubeb Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

macro_rules! pulse_enum {
    (pub enum $name:ident { $($variants:tt)* }) => {
        pub type $name = c_uint;
        pulse_enum!(__GEN__ $name, 0, $($variants)*);
    };
    (pub enum $name:ident: $t:ty { $($variants:tt)* }) => {
        pub type $name = $t;
        pulse_enum!(__GEN__ $name, 0, $($variants)*);
    };
    (__GEN__ $name:ident, $val:expr, $variant:ident, $($rest:tt)*) => {
        pub const $variant: $name = $val;
        pulse_enum!(__GEN__ $name, $val+1, $($rest)*);
    };
    (__GEN__ $name:ident, $val:expr, $variant:ident = $e:expr, $($rest:tt)*) => {
        pub const $variant: $name = $e;
        pulse_enum!(__GEN__ $name, $e+1, $($rest)*);
    };
    (__GEN__ $name:ident, $val:expr, ) => {}
}

macro_rules! pulse_flags {
    (pub flags $name:ident { $($variants:tt)* }) => {
        pub type $name = c_uint;
        pulse_flags!(__GEN__ $name, 1, $($variants)*);
    };
    (pub flags $name:ident: $t:ty { $($variants:tt)* }) => {
        pub type $name = $t;
        pulse_flags!(__GEN__ $name, 1, $($variants)*);
    };
    (__GEN__ $name:ident, $val:expr, $variant:ident, $($rest:tt)*) => {
        pub const $variant: $name = $val;
        pulse_flags!(__GEN__ $name, $val << 1, $($rest)*);
    };
    (__GEN__ $name:ident, $val:expr, $variant:ident = 0, $($rest:tt)*) => {
        pub const $variant: $name = 0;
        pulse_flags!(__GEN__ $name, 1, $($rest)*);
    };
    (__GEN__ $name:ident, $val:expr, $variant:ident = $e:expr, $($rest:tt)*) => {
        pub const $variant: $name = $e;
        pulse_flags!(__GEN__ $name, $e<<1, $($rest)*);
    };
    (__GEN__ $name:ident, $val:expr, ) => {}
}
