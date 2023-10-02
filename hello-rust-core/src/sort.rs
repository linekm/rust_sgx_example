//! Bitonic Sort.
//! Ref: <https://www.inf.hs-flensburg.de/lang/algorithmen/sortieren/bitonic/oddn.htm>

use crate::cmov::CMov;
use alloc::vec::Vec;
use core::cmp::Ordering;

#[inline(always)]
unsafe fn compare_and_swap<T, F>(array: &mut [T], i: usize, j: usize, cmp: &mut F, ascending: bool)
where
    T: CMov,
    F: FnMut(&T, &T) -> Ordering,
{
    let ptr = array.as_mut_ptr();
    let a = &mut *ptr.add(i);
    let b = &mut *ptr.add(j);
    let choice = (cmp(a, b) == Ordering::Less) != ascending;
    <_ as CMov>::cnd_swap(a, b, choice);
}

unsafe fn bitonic_merge_inner<T, F>(
    array: &mut [T],
    start: usize,
    len: usize,
    cmp: &mut F,
    ascending: bool,
) where
    T: CMov,
    F: FnMut(&T, &T) -> Ordering,
{
    if len > 1 {
        let first_half = len.next_power_of_two() / 2;
        let second_half = len - first_half;
        for i in (start..).take(second_half) {
            compare_and_swap(array, i, i + first_half, cmp, ascending);
        }
        bitonic_merge_inner(array, start, first_half, cmp, ascending);
        bitonic_merge_inner(array, start + first_half, second_half, cmp, ascending);
    }
}

unsafe fn bitonic_sort_inner<T, F>(
    array: &mut [T],
    start: usize,
    len: usize,
    cmp: &mut F,
    ascending: bool,
) where
    T: CMov,
    F: FnMut(&T, &T) -> Ordering,
{
    if len > 1 {
        let half = len / 2;
        bitonic_sort_inner(array, start, half, cmp, !ascending);
        bitonic_sort_inner(array, start + half, len - half, cmp, ascending);
        bitonic_merge_inner(array, start, len, cmp, ascending);
    }
}

/// Bitonic sort by custom cmp function.
#[inline]
pub fn bitonic_sort_by<T, F>(array: &mut [T], mut cmp: F)
where
    T: CMov,
    F: FnMut(&T, &T) -> Ordering,
{
    unsafe { bitonic_sort_inner(array, 0, array.len(), &mut cmp, true) }
}

/// Bitonic sort by key.
#[inline]
pub fn bitonic_sort_by_key<T, F, K>(array: &mut [T], mut f: F)
where
    T: CMov,
    F: FnMut(&T) -> K,
    K: Ord,
{
    bitonic_sort_by(array, |a, b| f(a).cmp(&f(b)))
}

/// Bitonic sort.
#[inline]
pub fn bitonic_sort<T>(array: &mut [T])
where
    T: CMov + Ord,
{
    bitonic_sort_by(array, |a, b| a.cmp(b))
}

#[allow(dead_code)]
fn array_is_sorted_by<T, F>(array: &[T], mut cmp: F) -> bool
where
    F: FnMut(&T, &T) -> Ordering,
{
    let len = array.len();

    if len == 0 {
        return true;
    }

    for i in 0..len - 1 {
        if cmp(&array[i], &array[i + 1]) == Ordering::Greater {
            return false;
        }
    }

    true
}

/// Merge two sorted iterators with the same length into one sorted array using bitonic merge.
/// The left iterator should be in descending order. And the right iterator should be in ascending order.
#[inline]
pub fn bitonic_merge_sorted_iters<T: CMov + Ord>(
    left: impl Iterator<Item = T>,
    right: impl Iterator<Item = T>,
    len_hint: Option<usize>,
) -> Vec<T> {
    let total_len_hint = len_hint.unwrap_or_default() * 2;
    let mut array = Vec::with_capacity(total_len_hint);
    array.extend(left);
    let left_len = array.len();
    array.extend(right);
    let total_len = array.len();
    assert_eq!(
        left_len * 2,
        total_len,
        "input iterators should have the same length"
    );
    debug_assert!(
        array_is_sorted_by(&array[..left_len], |a, b| a.cmp(b).reverse()),
        "left is not sorted"
    );
    debug_assert!(
        array_is_sorted_by(&array[left_len..], |a, b| a.cmp(b)),
        "right is not sorted"
    );

    let mut cmp = |a: &T, b: &T| a.cmp(b);
    unsafe { bitonic_merge_inner(&mut array, 0, total_len, &mut cmp, true) }

    array
}

/// Merge two sorted slices with the same length into one sorted array using bitonic merge.
#[inline]
pub fn bitonic_merge_sorted_slices<T: CMov + Ord>(left: &[T], right: &[T]) -> Vec<T> {
    assert_eq!(
        left.len(),
        right.len(),
        "input slices should have the same length"
    );
    debug_assert!(
        array_is_sorted_by(left, |a, b| a.cmp(b)),
        "left is not sorted"
    );
    debug_assert!(
        array_is_sorted_by(right, |a, b| a.cmp(b)),
        "right is not sorted"
    );

    if left.is_empty() {
        return Vec::new();
    }

    let len = left.len() * 2;
    let mut array = Vec::with_capacity(len);
    array.extend(left.iter().rev().cloned());
    array.extend_from_slice(right);

    let mut cmp = |a: &T, b: &T| a.cmp(b);
    unsafe { bitonic_merge_inner(&mut array, 0, len, &mut cmp, true) }

    array
}

#[cfg(test)]
mod tests;
