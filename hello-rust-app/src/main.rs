pub use anyhow as error;
use ecall::enclave_add;
use utils::TEEJoinWorkerFactory;


pub mod ecall;
pub mod ocall;
pub mod utils;

fn main() {
    let enclave = TEEJoinWorkerFactory::use_enclave_in_the_same_dir().unwrap();
    let add = enclave_add(&enclave.enclave, 1.0, 2.0).unwrap();
    println!("add: {}", add);
}