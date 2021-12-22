
// 需要静态编译
//$ rustc executable.rs --extern rary=library.rlib --edition=2018 && ./executable

//extern crate rary;

pub fn run(){

    println!("demo how to call external rust lib");

    //rary::public_function();

    // Error! `private_function` is private
    //rary::private_function();

    //rary::indirect_access();



}


