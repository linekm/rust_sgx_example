use alloc::vec::Vec;

use crate::{cmov::CMov, aligned::{A8, AlignedBox}};


#[derive(Debug, Clone, CMov)]
struct TestSortStruct {
    dist: u64,
    id: u64,
    payload: AlignedBox<A8, [u8]>,
}

#[cfg(test)]
mod test {
    #![allow(dead_code)]
    #![allow(unused_variables)]
    #![allow(overflowing_literals)]
    #![allow(non_snake_case)]
    #![allow(non_camel_case_types)]
    #![allow(unused_imports)]
    #![allow(unused_mut)]

    use crate::sort::bitonic_sort_by;

    use super::*;

    #[test]
    fn sort_struct_example() {
        let mut test_struct_1 = TestSortStruct {
            dist: 4,
            id: 1,
            payload: vec![0;32].into(),
        };
        let mut test_struct_2 = TestSortStruct {
            dist: 3,
            id: 2,
            payload: vec![1;32].into(),
        };
        let mut test_struct_3 = TestSortStruct {
            dist: 2,
            id: 3,
            payload: vec![2;32].into(),
        };
        let mut test_struct_4 = TestSortStruct {
            dist: 1,
            id: 4,
            payload: vec![3;32].into(),
        };
        let mut test_struct_5 = TestSortStruct {
            dist: 2,
            id: 0,
            payload: vec![4;32].into(),
        };

        let mut test_vec = vec![
            test_struct_1,
            test_struct_2,
            test_struct_3,
            test_struct_4,
            test_struct_5,
        ];
        
        bitonic_sort_by(&mut test_vec, |a, b| {
            a.dist.cmp(&b.dist)
        }.then_with(|| {
            a.id.cmp(&b.id)
        }));

        for val in test_vec {
            let payload = val.payload.as_ref();
            println!("id: {}, dist:{}, payload: {:?}", val.id, val.dist, payload);
        }
    }
}