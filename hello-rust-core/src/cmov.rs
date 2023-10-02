//! This is a fork of [aligned-cmov](https://github.com/mobilecoinofficial/mc-oblivious/tree/master/aligned-cmov).
//!
//! Origin License: GPL-3.0

/// Trait for CMOV operations.
pub trait CMov: Clone {
    /// Return a if choice is 0 (false), or b if choice is 1 (true).
    fn cnd_select(a: &Self, b: &Self, choice: bool) -> Self;

    /// Assign self as other if choice is 1 (true).
    #[inline]
    fn cnd_assign(&mut self, other: &Self, choice: bool) {
        *self = Self::cnd_select(self, other, choice);
    }

    /// Swap inputs if choice is 1 (true).
    #[inline]
    fn cnd_swap(a: &mut Self, b: &mut Self, choice: bool) {
        let t: Self = a.clone();
        a.cnd_assign(b, choice);
        b.cnd_assign(&t, choice);
    }
}

mod impl_bool;
mod impl_bytes;
mod impl_tuples;
mod impl_u32_u64_usize;
mod impl_u8_u16;

pub use impl_bytes::{cmov_bytes_a32, cmov_bytes_a64, cmov_bytes_a8};

mod cnd_option;
pub use cnd_option::*;

pub use hello_rust_cmov_derive::*;

#[cfg(test)]
mod tests;
