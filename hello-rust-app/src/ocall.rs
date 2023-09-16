use std::{collections::HashMap, slice, ptr::copy_nonoverlapping};

use hello_rust_core::util::{Point, L2Dist};
use serde::de::DeserializeOwned;

pub static mut POINT_PAIR_MAP: Option<HashMap<usize, (Point, Point)>> = Option::None;
pub static mut POINT_PAIR_BYTES_BUFFER: Option<HashMap<usize, Vec<u8>>> = Option::None;

pub static mut RESULTS_BUFFER: Option<Vec<L2Dist>> = Option::None;

unsafe fn from_bytes<T: DeserializeOwned>(bytes: *const u8, bytes_len: usize) -> T {
    let buf = slice::from_raw_parts(bytes, bytes_len);
    let data: T = postcard::from_bytes(buf).unwrap();
    data
}

#[no_mangle]
pub unsafe extern "C" fn ocall_return_result(result_bytes: *const u8, bytes_len: usize) -> i32 {
    let data: L2Dist = from_bytes(result_bytes, bytes_len);
    RESULTS_BUFFER.as_mut().unwrap().push(data);
    0
}

#[no_mangle]
pub unsafe extern "C" fn ocall_get_outside_data_len(
    query_key: usize,
    data_len: *mut usize,
) -> i32 {
    let query_data = POINT_PAIR_MAP
        .as_mut()
        .unwrap()
        .get(&query_key)
        .unwrap();
    let data_bytes = postcard::to_allocvec(&vec![query_data.0.clone(), query_data.1.clone()]).unwrap();

    *data_len = data_bytes.len();

    POINT_PAIR_BYTES_BUFFER.as_mut().unwrap().insert(query_key, data_bytes);
    0
}

#[no_mangle]
pub unsafe extern "C" fn ocall_outside_data(
    query_key: usize,
    data: *mut u8,
    data_len: usize,
) -> i32 {
    let bytes = POINT_PAIR_BYTES_BUFFER.as_ref().unwrap().get(&query_key).unwrap();
    copy_nonoverlapping(
        bytes.as_ptr(),
        data,
        data_len,
    );
    0
}