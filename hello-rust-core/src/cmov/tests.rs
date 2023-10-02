use super::*;
use crate::aligned::{Aligned, AlignedBox, A16, A32, A64, A8};
use core::mem::MaybeUninit;
use proptest::prelude::*;

macro_rules! test_cmov {
    ($name: ident, $ty: ty) => {
        proptest! {
            #![proptest_config(ProptestConfig { fork: true, ..Default::default() })]

            #[test]
            fn $name(choice in prop::bool::ANY, a in any::<$ty>(), b in any::<$ty>()) {
                let src = a;
                let mut dst = b;
                dst.cnd_assign(&src, choice);
                prop_assert_eq!(src, a);
                if choice {
                    prop_assert_eq!(dst, a);
                    prop_assert_eq!(<$ty>::cnd_select(&a, &b, choice), b);
                } else {
                    prop_assert_eq!(dst, b);
                    prop_assert_eq!(<$ty>::cnd_select(&a, &b, choice), a);
                }
            }
        }
    };
}

test_cmov!(test_bool, bool);
test_cmov!(test_u8, u8);
test_cmov!(test_i8, i8);
test_cmov!(test_u16, u16);
test_cmov!(test_i16, i16);
test_cmov!(test_u32, u32);
test_cmov!(test_i32, i32);
test_cmov!(test_u64, u64);
test_cmov!(test_i64, i64);
test_cmov!(test_usize, usize);
test_cmov!(test_isize, isize);
test_cmov!(test_a8, Aligned<A8, [u64; 16]>);
test_cmov!(test_a16, Aligned<A16, [u64; 16]>);
test_cmov!(test_a32, Aligned<A32, [u64; 16]>);
test_cmov!(test_a64, Aligned<A64, [u64; 16]>);
test_cmov!(test_a8_zst, Aligned<A8, ()>);
test_cmov!(test_a16_zst, Aligned<A16, ()>);
test_cmov!(test_a32_zst, Aligned<A32, ()>);
test_cmov!(test_a64_zst, Aligned<A64, ()>);

#[test]
fn test_aligned_slice_box() {
    let len = 250;
    let a = {
        let mut slice: AlignedBox<A64, [MaybeUninit<usize>]> = AlignedBox::new_uninit_slice(len);
        unsafe {
            for i in 0..len {
                slice[i].as_mut_ptr().write(i);
            }
            slice.assume_init()
        }
    };
    let b = {
        let mut slice: AlignedBox<A64, [MaybeUninit<usize>]> = AlignedBox::new_uninit_slice(len);
        unsafe {
            for i in 0..len {
                slice[i].as_mut_ptr().write(2 * i);
            }
            slice.assume_init()
        }
    };

    let src = a.clone();
    let mut dst = b.clone();

    let choice = false;
    dst.cnd_assign(&src, choice);
    assert_eq!(src, a);
    assert_eq!(dst, b);
    assert_eq!(<_>::cnd_select(&a, &b, choice), a);

    let choice = true;
    dst.cnd_assign(&src, choice);
    assert_eq!(src, a);
    assert_eq!(dst, a);
    assert_eq!(<_>::cnd_select(&a, &b, choice), b);
}
