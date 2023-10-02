#![cfg_attr(not(test), no_std)]
#![feature(asm)]
#![feature(int_log)]
#![feature(async_fn_in_trait)]
#![feature(array_windows)]
#![allow(clippy::too_many_arguments)]
#[macro_use]
extern crate alloc;

pub use anyhow as error;

pub mod sort;
pub mod aligned;
pub mod cmov;
pub mod util;

mod example;