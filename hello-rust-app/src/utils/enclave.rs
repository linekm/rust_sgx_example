use super::*;
use std::{mem, path::Path, sync::Arc};

use anyhow::Error;
use sgx_types::{sgx_launch_token_t, sgx_misc_attribute_t};
use sgx_urts::SgxEnclave;

pub type SharedSgxEnclave = Arc<SgxEnclave>;

pub struct TEEJoinWorkerFactory {
    pub enclave: SharedSgxEnclave,
}

impl TEEJoinWorkerFactory {
    pub fn new(enclave_path: &Path) -> Result<Self> {
        println!("Init SGX enclave from {}.", enclave_path.display());
        let debug = 1;
        let mut launch_token: sgx_launch_token_t = unsafe { mem::zeroed() };
        let mut launch_token_updated: i32 = 0;
        let mut misc_attr: sgx_misc_attribute_t = unsafe { mem::zeroed() };
        let enclave = SgxEnclave::create(
            enclave_path,
            debug,
            &mut launch_token,
            &mut launch_token_updated,
            &mut misc_attr,
        )
        .map_err(Error::msg)?;
        let enclave = Arc::new(enclave);
        Ok(Self { enclave })
    }

    pub fn use_enclave_in_the_same_dir() -> Result<Self> {
        let dir = binary_directory()?;
        Self::new(&dir.join(env!("ENCLAVE_FILE_NAME")))
    }
}
