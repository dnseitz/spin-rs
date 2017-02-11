#![crate_type = "lib"]
#![warn(missing_docs)]

//! Synchronization primitives based on spinning

#![cfg_attr(feature = "asm", feature(asm))]
#![cfg_attr(feature = "core_intrinsics", feature(core_intrinsics))]
#![cfg_attr(feature = "const_fn", feature(const_fn))]
#![feature(cfg_target_has_atomic)]

#![no_std]

#[cfg(test)]
#[macro_use]
extern crate std;

#[cfg(not(target_has_atomic="ptr"))]
extern crate cm0_atomic as atomic;

#[cfg(target_has_atomic="ptr")]
use core::sync::atomic as atomic;

pub use mutex::*;
pub use rw_lock::*;

#[cfg(feature = "once")]
pub use once::*;

mod mutex;
mod rw_lock;

#[cfg(feature = "once")]
mod once;

mod util;
