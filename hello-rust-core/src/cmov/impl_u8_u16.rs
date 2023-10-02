use super::CMov;

// From https://github.com/dalek-cryptography/subtle/blob/b4b070c3faf87cb8f324bd0ed0a5e5ec32d3a5b0/src/lib.rs#L442-L512
// if choice = 0, mask = (-0) = 0000...0000
// if choice = 1, mask = (-1) = 1111...1111
macro_rules! impl_cmov_for_primitives {
    ($x:ty, $signed_x:ty) => {
        impl_cmov_for_primitives!(@inner, $x, $signed_x);
        impl_cmov_for_primitives!(@inner, $signed_x, $signed_x);
    };
    (@inner, $x:ty, $signed_x:ty) => {
        impl CMov for $x {
            #[inline]
            fn cnd_select(a: &Self, b: &Self, choice: bool) -> Self {
                let mask = -(choice as $signed_x) as $x;
                a ^ (mask & (a ^ b))
            }

            #[inline]
            fn cnd_assign(&mut self, other: &Self, choice: bool) {
                let mask = -(choice as $signed_x) as $x;
                *self ^= mask & (*self ^ *other);
            }

            #[inline]
            fn cnd_swap(a: &mut Self, b: &mut Self, choice: bool) {
                let mask = -(choice as $signed_x) as $x;
                let t = mask & (*a ^ *b);
                *a ^= t;
                *b ^= t;
            }
        }
    };
}

impl_cmov_for_primitives!(u8, i8);
impl_cmov_for_primitives!(u16, i16);
