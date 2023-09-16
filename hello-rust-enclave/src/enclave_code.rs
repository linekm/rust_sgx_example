use alloc::{vec::{Vec}, slice}; 
use serde::{Serialize, Deserialize};
use sgx_types::*;
use hello_rust_core::util::{Point, L2Dist, compute_l2_distance};

extern "C" {
    fn ocall_get_outside_data_len(
        retval: *mut i32,
        query_key: usize,
        data_len: *mut usize
    ) -> sgx_status_t;

    fn ocall_outside_data(
        retval: *mut i32,
        query_key: usize,
        data: *mut u8,
        data_len: usize,
    ) -> sgx_status_t;

    fn ocall_return_result(
        retval: *mut i32,
        result_bytes: *const u8,
        bytes_len: usize,
    ) -> sgx_status_t;
}

#[no_mangle]
pub unsafe extern "C" fn ecall_sgx_add(a: f64, b:f64) -> f64 {
    std::eprintln!("ecall_sgx_add: a={}, b={}", a, b);
    return a+b;
}

#[no_mangle]
pub unsafe extern "C" fn ecall_sgx_l2_dist(key: usize) -> i32 {
    let mut retval = 0;
    let mut data_len = 0;

    let sgx_ret = ocall_get_outside_data_len(
        &mut retval as *mut _,
        key, 
        &mut data_len as *mut _
    );
    if sgx_ret != sgx_status_t::SGX_SUCCESS || retval != 0 {
        std::eprintln!("Failed to get outside data length, status={}.", sgx_ret);
        return 1;
    }

    let mut vec_bytes: Vec<u8> = Vec::with_capacity(data_len);
    let sgx_ret = ocall_outside_data(
        &mut retval as *mut _,
        key,
        vec_bytes.as_mut_ptr() as *mut _,
        data_len,
    );

    if sgx_ret != sgx_status_t::SGX_SUCCESS || retval != 0 {
        std::eprintln!("Failed to get outside data, status={}.", sgx_ret);
        return 1;
    }
    vec_bytes.set_len(data_len);

    let points: Vec<Point> = postcard::from_bytes(&vec_bytes).unwrap();

    let l2 = compute_l2_distance(&points[0], &points[1]);
    
    let ret_bytes = postcard::to_allocvec(&l2).unwrap();
    let bytes_len = ret_bytes.len();

    let mut retval: i32 = 0;
    let sgx_ret = ocall_return_result(&mut retval as *mut _, ret_bytes.as_ptr(), bytes_len);

    if sgx_ret != sgx_status_t::SGX_SUCCESS || retval != 0 {
        std::eprintln!("[Enclave Error] Failed to return l2 distance result.");
        std::eprintln!(" DETAIL: sgx_ret={}, retval={}.", sgx_ret, retval);
        return 1;
    }
    return 0;
}