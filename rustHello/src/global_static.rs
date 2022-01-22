use std::collections::HashMap;
use std::sync::{Arc, Mutex};

//用 lazy_static 实现全局的static，要自己加锁
lazy_static::lazy_static! {
    pub static ref HASH_MAP: Arc<Mutex<HashMap<i32, &'static str>>> = {
        let mut h =HashMap::new();
        h.insert(0,"Bob");
        h.insert(1,"Alice");
        h.insert(2,"Charilie");
        Arc::new(Mutex::new(h))
    };
}


static mut COUNTER: i32 =1;


pub fn run(){

    //除非使用 unsafe，static 无法作为 mut 使用，因为这意味着它可能在多个线程下被修改，所以不安全，编译不通过
    //COUNTER++;

    unsafe{
        COUNTER+1;
    }



    let mut hash_map =HASH_MAP.lock().unwrap();
    hash_map.insert(4,"Dave");

    let mut a =0;
    unsafe {
        a =COUNTER;
    }

    println!("static data - counter:{}, map:{:?}", a, hash_map);



}









