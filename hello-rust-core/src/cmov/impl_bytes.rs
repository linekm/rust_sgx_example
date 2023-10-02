use super::CMov;
use crate::aligned::{Aligned, AlignedBox, A16, A32, A64, A8};
use core::arch::asm;
use core::mem;

/// CMov bytes array which is 8-bytes aligned.
///
/// # Safety
///
/// count should be non-zero. src and dst are properly aligned.
#[inline(always)]
pub unsafe fn cmov_bytes_a8(cnd: bool, src: *const u8, dst: *mut u8, count: usize) {
    debug_assert!(count > 0);
    debug_assert_eq!(count % 8, 0);
    debug_assert_eq!(src.align_offset(8), 0);
    debug_assert_eq!(dst.align_offset(8), 0);
    let cnd = cnd as u64;
    let count = count / 8;
    asm!(
        "neg {0}",
        "2:",
            "mov {0}, qword ptr [{3} + 8*{1} - 8]",
            "cmovc {0}, qword ptr [{2} + 8*{1} - 8]",
            "mov qword ptr [{3} + 8*{1} - 8], {0}",
            "dec {1}",
            "jnz 2b",
        inout(reg) cnd => _,
        inout(reg) count => _,
        in(reg) src,
        in(reg) dst,
        options(nostack),
    );
}

/// CMov bytes array which is 32-bytes aligned.
///
/// # Safety
///
/// count should be non-zero. src and dst are properly aligned.
#[inline(always)]
#[cfg(target_feature = "avx2")]
pub unsafe fn cmov_bytes_a32(cnd: bool, src: *const u8, dst: *mut u8, count: usize) {
    debug_assert!(count > 0);
    debug_assert_eq!(count % 32, 0);
    debug_assert_eq!(src.align_offset(32), 0);
    debug_assert_eq!(dst.align_offset(32), 0);
    let cnd = cnd as u64;
    asm!(
        "neg {0}",
        "vmovq xmm2, {0}",
        "vbroadcastsd ymm1, xmm2",
        "mov {0}, {3}",
        "2:",
            "vmovdqa ymm2, ymmword ptr [{1} + {0} - 32]",
            "vpmaskmovq ymmword ptr [{2} + {0} - 32], ymm1, ymm2",
            "sub {0}, 32",
            "jnz 2b",
        inout(reg) cnd => _,
        in(reg) src,
        in(reg) dst,
        in(reg) count,
        options(nostack),
    );
}

/// CMov bytes array which is 32-bytes aligned.
///
/// # Safety
///
/// count should be non-zero. src and dst are properly aligned.
#[inline(always)]
#[cfg(not(target_feature = "avx2"))]
pub unsafe fn cmov_bytes_a32(cnd: bool, src: *const u8, dst: *mut u8, count: usize) {
    cmov_bytes_a8(cnd, src, dst, count)
}

/// CMov bytes array which is 64-bytes aligned.
///
/// # Safety
///
/// count should be non-zero. src and dst are properly aligned.
#[inline(always)]
#[cfg(target_feature = "avx2")]
pub unsafe fn cmov_bytes_a64(cnd: bool, src: *const u8, dst: *mut u8, count: usize) {
    debug_assert!(count > 0);
    debug_assert_eq!(count % 64, 0);
    debug_assert_eq!(src.align_offset(64), 0);
    debug_assert_eq!(dst.align_offset(64), 0);
    let cnd = cnd as u64;
    asm!(
        "neg {0}",
        "vmovq xmm2, {0}",
        "vbroadcastsd ymm1, xmm2",
        "mov {0}, {3}",
        "2:",
            "vmovdqa ymm2, ymmword ptr [{1} + {0} - 64]",
            "vpmaskmovq ymmword ptr [{2} + {0} - 64], ymm1, ymm2",
            "vmovdqa ymm3, ymmword ptr [{1} + {0} - 32]",
            "vpmaskmovq ymmword ptr [{2} + {0} - 32], ymm1, ymm3",
            "sub {0}, 64",
            "jnz 2b",
        inout(reg) cnd => _,
        in(reg) src,
        in(reg) dst,
        in(reg) count,
        options(nostack),
    );
}

/// CMov bytes array which is 64-bytes aligned.
///
/// # Safety
///
/// count should be non-zero. src and dst are properly aligned.
#[inline(always)]
#[cfg(not(target_feature = "avx2"))]
pub unsafe fn cmov_bytes_a64(cnd: bool, src: *const u8, dst: *mut u8, count: usize) {
    cmov_bytes_a8(cnd, src, dst, count)
}

impl<T: Copy> CMov for Aligned<A8, T> {
    #[inline]
    fn cnd_select(a: &Self, b: &Self, choice: bool) -> Self {
        let mut out = *a;
        out.cnd_assign(b, choice);
        out
    }

    #[inline]
    fn cnd_assign(&mut self, other: &Self, choice: bool) {
        let count = mem::size_of::<Self>();
        if count != 0 {
            let src = other as *const Self as *const u8;
            let dst = self as *mut Self as *mut u8;
            unsafe {
                cmov_bytes_a8(choice, src, dst, count);
            }
        }
    }
}

impl<T: Copy> CMov for Aligned<A16, T> {
    #[inline]
    fn cnd_select(a: &Self, b: &Self, choice: bool) -> Self {
        let mut out = *a;
        out.cnd_assign(b, choice);
        out
    }

    #[inline]
    fn cnd_assign(&mut self, other: &Self, choice: bool) {
        let count = mem::size_of::<Self>();
        if count != 0 {
            let src = other as *const Self as *const u8;
            let dst = self as *mut Self as *mut u8;
            unsafe {
                cmov_bytes_a8(choice, src, dst, count);
            }
        }
    }
}

impl<T: Copy> CMov for Aligned<A32, T> {
    #[inline]
    fn cnd_select(a: &Self, b: &Self, choice: bool) -> Self {
        let mut out = *a;
        out.cnd_assign(b, choice);
        out
    }

    #[inline]
    fn cnd_assign(&mut self, other: &Self, choice: bool) {
        let count = mem::size_of::<Self>();
        if count != 0 {
            let src = other as *const Self as *const u8;
            let dst = self as *mut Self as *mut u8;
            unsafe {
                cmov_bytes_a32(choice, src, dst, count);
            }
        }
    }
}

impl<T: Copy> CMov for Aligned<A64, T> {
    #[inline]
    fn cnd_select(a: &Self, b: &Self, choice: bool) -> Self {
        let mut out = *a;
        out.cnd_assign(b, choice);
        out
    }

    #[inline]
    fn cnd_assign(&mut self, other: &Self, choice: bool) {
        let count = mem::size_of::<Self>();
        if count != 0 {
            let src = other as *const Self as *const u8;
            let dst = self as *mut Self as *mut u8;
            unsafe {
                cmov_bytes_a64(choice, src, dst, count);
            }
        }
    }
}

impl<T: Copy> CMov for AlignedBox<A8, [T]> {
    #[inline]
    fn cnd_select(a: &Self, b: &Self, choice: bool) -> Self {
        let mut out = a.clone();
        out.cnd_assign(b, choice);
        out
    }

    #[inline]
    fn cnd_assign(&mut self, other: &Self, choice: bool) {
        assert_eq!(self.len(), other.len());
        let count = self.cmov_byte_size();
        if count != 0 {
            let src = other.as_ptr() as *const T as *const u8;
            let dst = self.as_mut_ptr() as *mut T as *mut u8;
            unsafe {
                cmov_bytes_a8(choice, src, dst, count);
            }
        }
    }
}

impl<T: Copy> CMov for AlignedBox<A16, [T]> {
    #[inline]
    fn cnd_select(a: &Self, b: &Self, choice: bool) -> Self {
        let mut out = a.clone();
        out.cnd_assign(b, choice);
        out
    }

    #[inline]
    fn cnd_assign(&mut self, other: &Self, choice: bool) {
        assert_eq!(self.len(), other.len());
        let count = self.cmov_byte_size();
        if count != 0 {
            let src = other.as_ptr() as *const T as *const u8;
            let dst = self.as_mut_ptr() as *mut T as *mut u8;
            unsafe {
                cmov_bytes_a8(choice, src, dst, count);
            }
        }
    }
}

impl<T: Copy> CMov for AlignedBox<A32, [T]> {
    #[inline]
    fn cnd_select(a: &Self, b: &Self, choice: bool) -> Self {
        let mut out = a.clone();
        out.cnd_assign(b, choice);
        out
    }

    #[inline]
    fn cnd_assign(&mut self, other: &Self, choice: bool) {
        assert_eq!(self.len(), other.len());
        let count = self.cmov_byte_size();
        if count != 0 {
            let src = other.as_ptr() as *const T as *const u8;
            let dst = self.as_mut_ptr() as *mut T as *mut u8;
            unsafe {
                cmov_bytes_a32(choice, src, dst, count);
            }
        }
    }
}

impl<T: Copy> CMov for AlignedBox<A64, [T]> {
    #[inline]
    fn cnd_select(a: &Self, b: &Self, choice: bool) -> Self {
        let mut out = a.clone();
        out.cnd_assign(b, choice);
        out
    }

    #[inline]
    fn cnd_assign(&mut self, other: &Self, choice: bool) {
        assert_eq!(self.len(), other.len());
        let count = self.cmov_byte_size();
        if count != 0 {
            let src = other.as_ptr() as *const T as *const u8;
            let dst = self.as_mut_ptr() as *mut T as *mut u8;
            unsafe {
                cmov_bytes_a64(choice, src, dst, count);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::collection::SizeRange;
    use proptest::prelude::*;
    use proptest_derive::Arbitrary;

    #[derive(Debug, Clone, PartialEq, Eq, Arbitrary)]
    struct AlignedItem(Aligned<A64, [u8; 8]>);

    fn arb_two_vecs(
        size_range: impl Into<SizeRange>,
    ) -> impl Strategy<Value = (usize, Vec<AlignedItem>, Vec<AlignedItem>)> {
        proptest::collection::vec(any::<AlignedItem>(), size_range).prop_flat_map(|a_vec| {
            let size = a_vec.len();
            let b_vec = proptest::collection::vec(any::<AlignedItem>(), size);
            (
                Just(size * mem::size_of::<AlignedItem>()),
                Just(a_vec),
                b_vec,
            )
        })
    }

    macro_rules! test_cmov_bytes {
        ($name: ident, $func: ident) => {
            proptest! {
                #![proptest_config(ProptestConfig { fork: true, ..Default::default() })]

                #[test]
                fn $name((size, a, b) in arb_two_vecs(1..10)) {
                    let mut res = a.clone();
                    unsafe {
                        $func(false, a.as_ptr() as *const u8, res.as_mut_ptr() as *mut u8, size);
                    }
                    prop_assert_eq!(res, a.clone());

                    let mut res = a.clone();
                    unsafe {
                        $func(true, b.as_ptr() as *const u8, res.as_mut_ptr() as *mut u8, size);
                    }
                    prop_assert_eq!(res, b.clone());
                }
            }
        };
    }

    test_cmov_bytes!(test_a8, cmov_bytes_a8);
    test_cmov_bytes!(test_a32, cmov_bytes_a32);
    test_cmov_bytes!(test_a64, cmov_bytes_a64);
}
