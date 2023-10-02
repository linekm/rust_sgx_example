//! This is a fork of [aligned-array](https://github.com/mobilecoinofficial/aligned-array).
//!
//! Origin License: MIT OR Apache-2.0

use core::{
    cmp::{Eq, Ord, Ordering, PartialEq, PartialOrd},
    convert::{AsMut, AsRef, From},
    fmt,
    hash::{Hash, Hasher},
    mem,
    ops::{Deref, DerefMut},
};
#[cfg(test)]
use proptest_derive::Arbitrary;
use serde::{Deserialize, Serialize};

/// Trait to represent the alignment.
pub trait Alignment: Copy {
    /// The size of the alignment.
    const SIZE: usize;
}

macro_rules! decl_alignment {
    ($x:ident, $size:literal) => {
        #[doc = concat!(stringify!($size), "-byte alignment")]
        #[derive(Copy, Clone, Debug, Default)]
        #[repr(C, align($size))]
        pub struct $x;

        impl Alignment for $x {
            const SIZE: usize = $size;
        }
    };
}

decl_alignment!(A2, 2);
decl_alignment!(A4, 4);
decl_alignment!(A8, 8);
decl_alignment!(A16, 16);
decl_alignment!(A32, 32);
decl_alignment!(A64, 64);

/// A newtype with alignment of at least `A` bytes
#[cfg_attr(test, derive(Arbitrary))]
#[derive(Serialize, Deserialize)]
#[serde(transparent)]
#[repr(C)]
pub struct Aligned<A, T>
where
    A: Alignment,
    T: ?Sized,
{
    #[cfg_attr(test, proptest(value = "[]"))]
    #[serde(skip)]
    _alignment: [A; 0],
    value: T,
}

impl<A, T> Aligned<A, T>
where
    A: Alignment,
{
    #[inline]
    pub fn new(value: T) -> Self {
        Self {
            _alignment: [],
            value,
        }
    }

    #[inline]
    pub fn into_inner(self) -> T {
        self.value
    }
}

impl<A, T> Deref for Aligned<A, T>
where
    A: Alignment,
    T: ?Sized,
{
    type Target = T;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<A, T> DerefMut for Aligned<A, T>
where
    A: Alignment,
    T: ?Sized,
{
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}

impl<A, T> AsRef<T> for Aligned<A, T>
where
    A: Alignment,
    T: ?Sized,
{
    #[inline]
    fn as_ref(&self) -> &T {
        &self.value
    }
}

impl<A, T> AsMut<T> for Aligned<A, T>
where
    A: Alignment,
    T: ?Sized,
{
    #[inline]
    fn as_mut(&mut self) -> &mut T {
        &mut self.value
    }
}

impl<A, T> From<T> for Aligned<A, T>
where
    A: Alignment,
{
    #[inline]
    fn from(input: T) -> Self {
        Self::new(input)
    }
}

impl<A, T> AsRef<Aligned<A, [Aligned<A, T>]>> for [Aligned<A, T>]
where
    A: Alignment,
{
    #[inline]
    fn as_ref(&self) -> &Aligned<A, [Aligned<A, T>]> {
        // SAFETY: A slice of aligned entries is in itself aligned.
        unsafe { mem::transmute(self) }
    }
}

impl<A, T> AsMut<Aligned<A, [Aligned<A, T>]>> for [Aligned<A, T>]
where
    A: Alignment,
{
    #[inline]
    fn as_mut(&mut self) -> &mut Aligned<A, [Aligned<A, T>]> {
        // SAFETY: A slice of aligned entries is in itself aligned.
        unsafe { mem::transmute(self) }
    }
}

impl<A, T> Copy for Aligned<A, T>
where
    A: Alignment,
    T: Copy,
{
}

impl<A, T> Clone for Aligned<A, T>
where
    A: Alignment,
    T: Clone,
{
    #[inline]
    fn clone(&self) -> Self {
        Self {
            _alignment: [],
            value: self.value.clone(),
        }
    }

    #[inline]
    fn clone_from(&mut self, other: &Self) {
        self.value.clone_from(&other.value);
    }
}

impl<A, T> Default for Aligned<A, T>
where
    A: Alignment,
    T: Default,
{
    #[inline]
    fn default() -> Self {
        Self {
            _alignment: [],
            value: Default::default(),
        }
    }
}

impl<A, T> fmt::Debug for Aligned<A, T>
where
    A: Alignment,
    T: ?Sized + fmt::Debug,
{
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Aligned").field(&&self.value).finish()
    }
}

impl<A, T> fmt::Display for Aligned<A, T>
where
    A: Alignment,
    T: ?Sized + fmt::Display,
{
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.value.fmt(f)
    }
}

impl<A, T> PartialEq for Aligned<A, T>
where
    A: Alignment,
    T: ?Sized + PartialEq,
{
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl<A, T> Eq for Aligned<A, T>
where
    A: Alignment,
    T: ?Sized + Eq,
{
}

impl<A, T> PartialOrd for Aligned<A, T>
where
    A: Alignment,
    T: ?Sized + PartialOrd,
{
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.value.partial_cmp(&other.value)
    }
}

impl<A, T> Ord for Aligned<A, T>
where
    A: Alignment,
    T: ?Sized + Ord,
{
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        self.value.cmp(&other.value)
    }
}

impl<A, T> Hash for Aligned<A, T>
where
    A: Alignment,
    T: ?Sized + Hash,
{
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.value.hash(state);
    }
}

/// Cast &T to &Aligned<A, T>. Panic if not aligned.
#[inline]
pub fn cast_aligned<A: Alignment, T>(t: &T) -> &Aligned<A, T> {
    assert_eq!(
        mem::size_of::<T>(),
        mem::size_of::<Aligned<A, T>>(),
        "Input cannot be aligned"
    );
    unsafe {
        let ptr: *const T = t;
        assert_eq!(ptr.align_offset(A::SIZE), 0, "Input is not aligned");
        &*(ptr as *const Aligned<A, T>)
    }
}

/// Cast &mut T to &mut Aligned<A, T>. Panic if not aligned.
#[inline]
pub fn cast_aligned_mut<A: Alignment, T>(t: &mut T) -> &mut Aligned<A, T> {
    assert_eq!(
        mem::size_of::<T>(),
        mem::size_of::<Aligned<A, T>>(),
        "Input cannot be aligned"
    );
    unsafe {
        let ptr: *mut T = t;
        assert_eq!(ptr.align_offset(A::SIZE), 0, "Input is not aligned");
        &mut *(ptr as *mut Aligned<A, T>)
    }
}

mod boxed;
pub use boxed::*;

#[cfg(test)]
mod tests {
    use super::*;
    use static_assertions::*;

    assert_eq_align!(Aligned<A2, [u8; 3]>, A2);
    assert_eq_align!(Aligned<A4, [u8; 3]>, A4);
    assert_eq_align!(Aligned<A8, [u8; 3]>, A8);
    assert_eq_align!(Aligned<A16, [u8; 3]>, A16);
    assert_eq_align!(Aligned<A32, [u8; 3]>, A32);
    assert_eq_align!(Aligned<A64, [u8; 3]>, A64);
    assert_impl_all!(Aligned<A2, [u8; 3]>: Copy, Clone, Default, Eq, Ord, Hash, Send, Sync);

    #[test]
    fn test() {
        let x: Aligned<A2, _> = Aligned::new([0u8; 3]);
        let y: Aligned<A4, _> = Aligned::new([0u8; 3]);
        let z: Aligned<A8, _> = Aligned::new([0u8; 3]);
        let w: Aligned<A16, _> = Aligned::new([0u8; 3]);

        // check alignment
        assert_eq!(mem::align_of_val(&x), 2);
        assert_eq!(mem::align_of_val(&y), 4);
        assert_eq!(mem::align_of_val(&z), 8);
        assert_eq!(mem::align_of_val(&w), 16);

        assert!(x.as_ptr() as usize % 2 == 0);
        assert!(y.as_ptr() as usize % 4 == 0);
        assert!(z.as_ptr() as usize % 8 == 0);
        assert!(w.as_ptr() as usize % 16 == 0);

        // test `deref`
        assert_eq!(x.len(), 3);
        assert_eq!(y.len(), 3);
        assert_eq!(z.len(), 3);
        assert_eq!(w.len(), 3);
    }

    #[test]
    fn test_slice_aligned() {
        type AlignedItem = Aligned<A2, u8>;
        let mut x: [AlignedItem; 4] = [Aligned::new(0); 4];
        let x_slice: &mut [AlignedItem] = &mut x;
        let y: &mut Aligned<A2, [AlignedItem]> = x_slice.as_mut();
        assert!(y.as_ptr() as usize % 2 == 0);
        *y[1] = 1;
        let x_slice: &[AlignedItem] = &x;
        let y: &Aligned<A2, [AlignedItem]> = x_slice.as_ref();
        assert!(y.as_ptr() as usize % 2 == 0);
        assert_eq!(*y[1], 1);
        assert_eq!(x[1].into_inner(), 1);
    }

    #[test]
    fn test_cast_aligned() {
        let mut x: Aligned<A2, _> = Aligned::new([0u8; 4]);
        let y: &[u8; 4] = &x;
        let z: &Aligned<A2, _> = cast_aligned(y);
        assert!(z.as_ptr() as usize % 2 == 0);
        let y: &mut [u8; 4] = &mut x;
        let z: &mut Aligned<A2, _> = cast_aligned_mut(y);
        assert!(z.as_ptr() as usize % 2 == 0);
    }

    #[test]
    #[should_panic(expected = "Input is not aligned")]
    fn test_cast_not_aligned() {
        let x: Aligned<A2, _> = Aligned::new([0u8; 4]);
        let y: &[u8; 2] = unsafe { &*(x[1..3].as_ptr() as *const [u8; 2]) };
        let _: &Aligned<A2, _> = cast_aligned(y);
    }

    #[test]
    #[should_panic(expected = "Input cannot be aligned")]
    fn test_cast_cannot_aligned() {
        let x = [0u8; 3];
        let _: &Aligned<A2, _> = cast_aligned(&x);
    }

    #[test]
    fn test_serde() {
        let x: Aligned<A2, _> = Aligned::new([0u8; 4]);
        let x_json = serde_json::to_string(&x).unwrap();
        assert_eq!(x, serde_json::from_str(&x_json).unwrap());
    }
}
