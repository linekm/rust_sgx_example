use super::*;
use proptest::collection::SizeRange;
use proptest::prelude::*;

fn arb_two_sorted_vecs(
    size_range: impl Into<SizeRange>,
) -> impl Strategy<Value = (Vec<u64>, Vec<u64>)> {
    proptest::collection::vec(any::<u64>(), size_range).prop_flat_map(|mut a_vec| {
        a_vec.sort_unstable();
        let size = a_vec.len();
        let b_vec = proptest::collection::vec(any::<u64>(), size).prop_map(|mut b_vec| {
            b_vec.sort_unstable();
            b_vec
        });
        (Just(a_vec), b_vec)
    })
}

proptest! {
    #[test]
    fn test_sort(mut input in prop::collection::vec(any::<u64>(), 0..1000)) {
        bitonic_sort(&mut input);
        prop_assert!(array_is_sorted_by(&input, |a, b| a.cmp(b)));
    }

    #[test]
    fn test_merge((a, b) in arb_two_sorted_vecs(0..1000)) {
        let res = bitonic_merge_sorted_slices(&a, &b);
        prop_assert!(array_is_sorted_by(&res, |a, b| a.cmp(b)));
    }

    #[test]
    fn test_merge_iters((a, b) in arb_two_sorted_vecs(0..1000)) {
        let res = bitonic_merge_sorted_iters(a.into_iter().rev(), b.into_iter(), None);
        prop_assert!(array_is_sorted_by(&res, |a, b| a.cmp(b)));
    }
}
