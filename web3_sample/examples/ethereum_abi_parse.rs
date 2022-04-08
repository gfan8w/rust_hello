use std::io::BufReader;
use ethereum_abi::Abi;

pub fn main(){

    let abi_file = include_str!("../build/storage.abi");
    let reader = BufReader::new(abi_file.as_bytes());
    let abi_storage = Abi::(reader).unwrap();





}

