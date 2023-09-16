use std::collections::HashMap;

pub use anyhow as error;
use ecall::enclave_add;
use hello_rust_core::util::Point;
use utils::TEEJoinWorkerFactory;

use crate::{ocall::{POINT_PAIR_BYTES_BUFFER, POINT_PAIR_MAP, RESULTS_BUFFER}, ecall::enclave_compute_l2_distance};


pub mod ecall;
pub mod ocall;
pub mod utils;

fn main() {
    let enclave = TEEJoinWorkerFactory::use_enclave_in_the_same_dir().unwrap();
    let add = enclave_add(&enclave.enclave, 1.0, 2.0).unwrap();
    println!("add: {}", add);

    unsafe {
        POINT_PAIR_MAP = Some(HashMap::new());
        POINT_PAIR_BYTES_BUFFER = Some(HashMap::new());
        RESULTS_BUFFER = Some(Vec::new());

        let mut point_pair_map = POINT_PAIR_MAP.as_mut().unwrap();
        point_pair_map.insert(
            1, 
            (
                Point{point_vec: vec![1.0, 0.0]},
                Point{point_vec: vec![0.0, 1.0]},
            )
        );
        point_pair_map.insert(
            2, 
            (
                Point{point_vec: vec![0.0, 0.0]},
                Point{point_vec: vec![3.0, 4.0]},
            )
        );
    }
    enclave_compute_l2_distance(&enclave.enclave, 1).unwrap();
    enclave_compute_l2_distance(&enclave.enclave, 2).unwrap();

    let result_list = unsafe {
        RESULTS_BUFFER.as_ref().unwrap().clone()
    };
    for res in result_list {
        println!("{:?}", res)
    }
}