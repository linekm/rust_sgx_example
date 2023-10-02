use super::{Aligned, Alignment};
use alloc::{
    alloc::{alloc, alloc_zeroed, dealloc, Layout},
    boxed::Box,
    vec::Vec,
};
use core::{
    cmp::{Eq, Ord, Ordering, PartialEq, PartialOrd},
    convert::{AsMut, AsRef, From},
    fmt,
    hash::{Hash, Hasher},
    mem::{self, ManuallyDrop, MaybeUninit},
    ops::{Deref, DerefMut},
    ptr::{self, NonNull},
    slice,
};
use serde::{de::Deserializer, ser::Serializer, Deserialize, Serialize};

pub struct AlignedBox<A, T>
where
    A: Alignment,
    T: ?Sized,
{
    inner: Box<Aligned<A, T>>,
}

impl<A, T> AlignedBox<A, T>
where
    A: Alignment,
{
    #[inline]
    pub fn new(value: T) -> Self {
        Self {
            inner: Box::new(Aligned::new(value)),
        }
    }

    #[inline]
    pub fn into_inner(self) -> T {
        self.inner.into_inner()
    }

    /// Create `AlignedBox<A, [MaybeUninit<T>]>`.
    pub fn new_uninit_slice(len: usize) -> AlignedBox<A, [MaybeUninit<T>]> {
        let align = A::SIZE;
        let size = len * mem::size_of::<T>();

        if size > usize::MAX - (align - 1) {
            panic!("size overflow");
        }

        // SAFETY: align is definitely valid. We checked size is not overflow.
        let inner = unsafe {
            let layout = Layout::from_size_align_unchecked(size, align).pad_to_align();
            let ptr = if size == 0 {
                align as *mut u8
            } else {
                alloc(layout)
            };
            let slice = slice::from_raw_parts_mut(ptr as *mut MaybeUninit<T>, len);
            debug_assert!(slice.as_ptr() as usize % A::SIZE == 0);
            let ptr = slice as *mut [MaybeUninit<T>];
            let slice = &mut *(ptr as *mut Aligned<A, [MaybeUninit<T>]>);
            Box::from_raw(slice)
        };
        AlignedBox { inner }
    }

    /// Create zeroed `AlignedBox<A, [MaybeUninit<T>]>`.
    pub fn new_zeroed_slice(len: usize) -> AlignedBox<A, [MaybeUninit<T>]> {
        let align = A::SIZE;
        let size = len * mem::size_of::<T>();

        if size > usize::MAX - (align - 1) {
            panic!("size overflow");
        }

        // SAFETY: align is definitely valid. We checked size is not overflow.
        let inner = unsafe {
            let layout = Layout::from_size_align_unchecked(size, align).pad_to_align();
            let ptr = if size == 0 {
                align as *mut u8
            } else {
                alloc_zeroed(layout)
            };
            let slice = slice::from_raw_parts_mut(ptr as *mut MaybeUninit<T>, len);
            debug_assert!(slice.as_ptr() as usize % A::SIZE == 0);
            let ptr = slice as *mut [MaybeUninit<T>];
            let slice = &mut *(ptr as *mut Aligned<A, [MaybeUninit<T>]>);
            Box::from_raw(slice)
        };
        AlignedBox { inner }
    }
}

impl<A, T> AlignedBox<A, [MaybeUninit<T>]>
where
    A: Alignment,
{
    /// Convert `AlignedBox<A, [MaybeUninit<T>]>` to `AlignedBox<A, [T]>`.
    ///
    /// # Safety
    ///
    /// Value should be initialized.
    #[inline]
    pub unsafe fn assume_init(self) -> AlignedBox<A, [T]> {
        let raw = Box::into_raw(self.inner);
        AlignedBox {
            inner: Box::from_raw(raw as *mut Aligned<A, [T]>),
        }
    }
}

impl<A, T> AlignedBox<A, [T]>
where
    A: Alignment,
{
    /// Size to be used by `cmov_bytes_a*`.
    #[inline]
    pub fn cmov_byte_size(&self) -> usize {
        let align = A::SIZE;
        let len = mem::size_of::<T>() * self.len();
        len.wrapping_add(align).wrapping_sub(1) & !align.wrapping_sub(1)
    }

    fn current_memory(&mut self) -> Option<(NonNull<u8>, Layout)> {
        let len = self.len();
        if mem::size_of::<T>() == 0 || len == 0 {
            None
        } else {
            let align = A::SIZE;
            let size = len * mem::size_of::<T>();
            unsafe {
                let ptr = NonNull::new_unchecked(self.as_mut());
                let layout = Layout::from_size_align_unchecked(size, align).pad_to_align();
                Some((ptr.cast(), layout))
            }
        }
    }
}

impl<A, T> Deref for AlignedBox<A, T>
where
    A: Alignment,
    T: ?Sized,
{
    type Target = T;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.inner.deref()
    }
}

impl<A, T> DerefMut for AlignedBox<A, T>
where
    A: Alignment,
    T: ?Sized,
{
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.inner.deref_mut()
    }
}

impl<A, T> AsRef<T> for AlignedBox<A, T>
where
    A: Alignment,
    T: ?Sized,
{
    #[inline]
    fn as_ref(&self) -> &T {
        self.inner.as_ref()
    }
}

impl<A, T> AsMut<T> for AlignedBox<A, T>
where
    A: Alignment,
    T: ?Sized,
{
    #[inline]
    fn as_mut(&mut self) -> &mut T {
        self.inner.as_mut()
    }
}

impl<A, T> From<T> for AlignedBox<A, T>
where
    A: Alignment,
{
    #[inline]
    fn from(input: T) -> Self {
        Self::new(input)
    }
}

impl<A, T> From<Vec<T>> for AlignedBox<A, [T]>
where
    A: Alignment,
{
    #[inline]
    fn from(input: Vec<T>) -> Self {
        let len = input.len();
        let mut output = AlignedBox::new_uninit_slice(len);
        let mut input = ManuallyDrop::new(input);
        unsafe {
            if mem::size_of::<T>() != 0 {
                ptr::copy_nonoverlapping(input.as_ptr(), output.as_mut_ptr() as *mut T, len);
            }
            input.set_len(0);
            let _ = ManuallyDrop::into_inner(input);
            output.assume_init()
        }
    }
}

impl<A, T> From<AlignedBox<A, [T]>> for Vec<T>
where
    A: Alignment,
{
    #[inline]
    fn from(input: AlignedBox<A, [T]>) -> Self {
        let len = input.len();
        let mut output = Vec::with_capacity(len);
        let mut input = ManuallyDrop::new(input);
        unsafe {
            if mem::size_of::<T>() != 0 {
                ptr::copy_nonoverlapping(input.as_ptr(), output.as_mut_ptr() as *mut T, len);
            }
            output.set_len(len);
            if let Some((ptr, layout)) = input.current_memory() {
                dealloc(ptr.as_ptr(), layout);
            }
        }
        output
    }
}

impl<A, T, const N: usize> From<[T; N]> for AlignedBox<A, [T]>
where
    A: Alignment,
{
    #[inline]
    fn from(input: [T; N]) -> Self {
        let len = input.len();
        let mut output = AlignedBox::new_uninit_slice(len);
        let input = ManuallyDrop::new(input);
        unsafe {
            if mem::size_of::<T>() != 0 {
                ptr::copy_nonoverlapping(input.as_ptr(), output.as_mut_ptr() as *mut T, len);
            }
            output.assume_init()
        }
    }
}

impl<'a, A, T> From<&'a [T]> for AlignedBox<A, [T]>
where
    A: Alignment,
    T: Copy,
{
    #[inline]
    fn from(input: &'a [T]) -> Self {
        let len = input.len();
        let mut output = AlignedBox::new_uninit_slice(len);
        unsafe {
            if mem::size_of::<T>() != 0 {
                ptr::copy_nonoverlapping(input.as_ptr(), output.as_mut_ptr() as *mut T, len);
            }
            output.assume_init()
        }
    }
}

impl<A, T> Clone for AlignedBox<A, T>
where
    A: Alignment,
    T: Clone,
{
    #[inline]
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }

    #[inline]
    fn clone_from(&mut self, other: &Self) {
        self.inner.clone_from(&other.inner)
    }
}

impl<A, T> Clone for AlignedBox<A, [T]>
where
    A: Alignment,
    T: Clone,
{
    #[inline]
    fn clone(&self) -> Self {
        let slice = AlignedBox::new_uninit_slice(self.len());
        let mut slice = unsafe { slice.assume_init() };
        slice.clone_from_slice(self);
        slice
    }

    #[inline]
    fn clone_from(&mut self, other: &Self) {
        if self.len() == other.len() {
            self.clone_from_slice(other);
        } else {
            *self = other.clone();
        }
    }
}

impl<A, T> Default for AlignedBox<A, T>
where
    A: Alignment,
    T: Default,
{
    #[inline]
    fn default() -> Self {
        Self::new(Default::default())
    }
}

impl<A, T> fmt::Debug for AlignedBox<A, T>
where
    A: Alignment,
    T: ?Sized + fmt::Debug,
{
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("AlignedBox").field(&self.inner).finish()
    }
}

impl<A, T> fmt::Display for AlignedBox<A, T>
where
    A: Alignment,
    T: ?Sized + fmt::Display,
{
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.inner.fmt(f)
    }
}

impl<A, T> PartialEq for AlignedBox<A, T>
where
    A: Alignment,
    T: ?Sized + PartialEq,
{
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.inner == other.inner
    }
}

impl<A, T> Eq for AlignedBox<A, T>
where
    A: Alignment,
    T: ?Sized + Eq,
{
}

impl<A, T> PartialOrd for AlignedBox<A, T>
where
    A: Alignment,
    T: ?Sized + PartialOrd,
{
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.inner.partial_cmp(&other.inner)
    }
}

impl<A, T> Ord for AlignedBox<A, T>
where
    A: Alignment,
    T: ?Sized + Ord,
{
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        self.inner.cmp(&other.inner)
    }
}

impl<A, T> Hash for AlignedBox<A, T>
where
    A: Alignment,
    T: ?Sized + Hash,
{
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.inner.hash(state);
    }
}

impl<A, T> Serialize for AlignedBox<A, T>
where
    A: Alignment,
    T: ?Sized + Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.inner.serialize(serializer)
    }
}

impl<'de, A, T> Deserialize<'de> for AlignedBox<A, T>
where
    A: Alignment,
    T: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Deserialize::deserialize(deserializer).map(AlignedBox::new)
    }
}

impl<'de, A, T> Deserialize<'de> for AlignedBox<A, [T]>
where
    A: Alignment,
    T: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Deserialize::deserialize(deserializer).map(|value: Vec<T>| value.into())
    }
}

#[cfg(test)]
mod tests {
    use super::super::A64;
    use super::*;

    #[test]
    fn test_uninit() {
        let len = 20;
        let mut slice: AlignedBox<A64, [MaybeUninit<usize>]> = AlignedBox::new_uninit_slice(len);
        let slice = unsafe {
            for i in 0..len {
                slice[i].as_mut_ptr().write(i);
            }
            slice.assume_init()
        };

        assert!(slice.as_ptr() as usize % 64 == 0);
        assert_eq!(slice.len(), len);
        for i in 0..len {
            assert_eq!(slice[i], i);
        }
    }

    #[test]
    fn test_zeroed() {
        let len = 20;
        let slice: AlignedBox<A64, [MaybeUninit<usize>]> = AlignedBox::new_zeroed_slice(len);
        let slice = unsafe { slice.assume_init() };

        assert!(slice.as_ptr() as usize % 64 == 0);
        assert_eq!(slice.len(), len);
        for i in 0..len {
            assert_eq!(slice[i], 0);
        }
    }

    #[test]
    fn test_zero_sized() {
        let len = 20;
        let slice: AlignedBox<A64, [MaybeUninit<()>]> = AlignedBox::new_zeroed_slice(len);
        let slice = unsafe { slice.assume_init() };

        assert!(slice.as_ptr() as usize % 64 == 0);
        assert_eq!(slice.len(), len);
        for i in 0..len {
            assert_eq!(slice[i], ());
        }
    }

    #[test]
    fn test_zero_length() {
        let slice: AlignedBox<A64, [MaybeUninit<usize>]> = AlignedBox::new_zeroed_slice(0);
        let slice = unsafe { slice.assume_init() };

        assert!(slice.as_ptr() as usize % 64 == 0);
        assert_eq!(slice.len(), 0);
    }

    #[test]
    fn test_from_into_vec() {
        use alloc::rc::Rc;
        use core::cell::RefCell;

        #[derive(Clone)]
        struct A(Rc<RefCell<usize>>);

        impl Drop for A {
            fn drop(&mut self) {
                *self.0.borrow_mut() += 1;
            }
        }

        let counter = Rc::new(RefCell::new(0));
        let x: AlignedBox<A64, [A]> = vec![A(counter.clone()); 5].into();
        let _: Vec<A> = x.into();
        assert_eq!(5, *counter.borrow());
    }

    #[test]
    fn test_serde() {
        let x: AlignedBox<A64, u64> = AlignedBox::new(42);
        let x_json = serde_json::to_string(&x).unwrap();
        assert_eq!(x, serde_json::from_str(&x_json).unwrap());

        let slice: AlignedBox<A64, [MaybeUninit<usize>]> = AlignedBox::new_zeroed_slice(16);
        let slice = unsafe { slice.assume_init() };
        let slice_json = serde_json::to_string(&slice).unwrap();
        assert_eq!(slice, serde_json::from_str(&slice_json).unwrap());
    }
}
