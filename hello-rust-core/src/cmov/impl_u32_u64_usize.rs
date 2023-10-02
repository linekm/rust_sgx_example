use super::CMov;
use core::arch::asm;

#[inline(always)]
fn cmov_u32(cnd: bool, a: u32, b: u32) -> u32 {
    let mut res = a;
    let cnd = cnd as u64;
    unsafe {
        asm!(
            "test {1}, {1}",
            "cmovnz {0:e}, {2:e}",
            inout(reg) res,
            in(reg) cnd,
            in(reg) b,
            options(nostack, nomem),
        );
    }
    res
}

#[inline(always)]
fn cmov_u64(cnd: bool, a: u64, b: u64) -> u64 {
    let mut res = a;
    let cnd = cnd as u64;
    unsafe {
        asm!(
            "test {1}, {1}",
            "cmovnz {0}, {2}",
            inout(reg) res,
            in(reg) cnd,
            in(reg) b,
            options(nostack, nomem),
        );
    }
    res
}

impl CMov for u32 {
    #[inline]
    fn cnd_select(a: &Self, b: &Self, choice: bool) -> Self {
        cmov_u32(choice, *a, *b)
    }
}

impl CMov for i32 {
    #[inline]
    fn cnd_select(a: &Self, b: &Self, choice: bool) -> Self {
        cmov_u32(choice, *a as u32, *b as u32) as i32
    }
}

impl CMov for u64 {
    #[inline]
    fn cnd_select(a: &Self, b: &Self, choice: bool) -> Self {
        cmov_u64(choice, *a, *b)
    }
}

impl CMov for i64 {
    #[inline]
    fn cnd_select(a: &Self, b: &Self, choice: bool) -> Self {
        cmov_u64(choice, *a as u64, *b as u64) as i64
    }
}

impl CMov for usize {
    #[inline]
    fn cnd_select(a: &Self, b: &Self, choice: bool) -> Self {
        cmov_u64(choice, *a as u64, *b as u64) as usize
    }
}

impl CMov for isize {
    #[inline]
    fn cnd_select(a: &Self, b: &Self, choice: bool) -> Self {
        cmov_u64(choice, *a as u64, *b as u64) as isize
    }
}
