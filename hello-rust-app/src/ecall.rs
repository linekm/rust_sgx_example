use crate::error::Result;
use crate::utils::SharedSgxEnclave;
use anyhow::Ok;

use sgx_types::*;

mod ffi {
    #![allow(clippy::all)]
    #![allow(dead_code)]
    use sgx_types::*;

    include!(concat!(env!("OUT_DIR"), "/enclave_ffi.rs"));
}

pub fn enclave_compute_l2_distance(
    enclave: &SharedSgxEnclave,
    query_key: usize
) -> Result<()> {

    let mut retval = 0;
    let sgx_ret = unsafe {
        ffi::ecall_sgx_l2_dist(
            enclave.geteid(),
            &mut retval as *mut _,
            query_key,
        )
    };
    match sgx_ret {
        sgx_status_t::SGX_SUCCESS => {
            if retval != 0 {
                Err(anyhow::anyhow!("ecall_sgx_l2_dist failed: {}", retval))
            } else {
                Ok(())
            }
        }
        _ => Err(anyhow::anyhow!("ecall_sgx_l2_dist failed: {}", sgx_ret)),
    }
}

pub fn enclave_add(
    enclave: &SharedSgxEnclave,
    a: f64,
    b: f64,
) -> Result<f64> {
    let mut retval = 0.0;
    let sgx_ret = unsafe {
        ffi::ecall_sgx_add(
            enclave.geteid(),
            &mut retval as *mut _,
            a,
            b,
        )
    };
    match sgx_ret {
        sgx_status_t::SGX_SUCCESS => {
                Ok(retval)
        }
        _ => Err(anyhow::anyhow!("ecall_add failed: {}", sgx_ret)),
    }
}