#![no_std]
#![cfg_attr(all(not(feature = "std"), feature = "unstable"),
            feature(core_intrinsics, core_panic, once_cell, unicode_internals, generic_assert_internals, error_in_core, ip_in_core))]
#![cfg_attr(all(not(feature = "std"), feature = "alloc", feature = "unstable"),
            feature(raw_vec_internals, async_iterator))]

#[cfg(all(not(feature = "std"), feature = "alloc"))]
extern crate alloc as __alloc;

#[cfg(not(feature = "std"))]
extern crate core as __core;

#[cfg(not(feature = "std"))]
mod generated;

#[cfg(not(feature = "std"))]
pub use self::generated::*;

#[cfg(all(not(feature = "std"), feature = "compat_macros"))]
#[macro_export]
macro_rules! print {
    () => {{}};
    ($($arg:tt)+) => {{
        // Avoid unused arguments complaint. This surely must get
        // optimized away? TODO: Verify that
        let _ = format_args!($($arg)+);
    }};
}
#[cfg(all(not(feature = "std"), feature = "compat_macros"))]
#[macro_export]
macro_rules! println {
    ($($arg:tt)*) => { print!($($arg)*) }
}
#[cfg(all(not(feature = "std"), feature = "compat_macros"))]
#[macro_export]
macro_rules! eprint {
    ($($arg:tt)*) => { print!($($arg)*) }
}
#[cfg(all(not(feature = "std"), feature = "compat_macros"))]
#[macro_export]
macro_rules! eprintln {
    ($($arg:tt)*) => { print!($($arg)*) }
}

#[cfg(all(not(feature = "std"), feature = "compat_macros"))]
#[macro_export]
macro_rules! dbg {
    () => {};
    ($($val:expr),+) => { ($($val),+) }
}