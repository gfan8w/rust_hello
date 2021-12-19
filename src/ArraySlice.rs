use std::mem::{size_of_val, size_of};

pub fn run(){
    let arr = [1,2,3,4,5,6];
    println!("size of array：{}",size_of_val(&arr));

    let mut v=vec![1,2,3,4,5,6];
    println!("v capacity is :{}, first element:{:p}", v.capacity(),&v[0]);
    v.push(7);
    v.push(8);
    println!("v capacity is :{}, first element:{:p}", v.capacity(),&v[0]);

    let mut s =&mut v[2..4];
    v.push(8);
    //println!("{:?}",s);  // 这里还要使用s，是不允许的，使用了2次mutable 引用。这是不允许的

}


pub unsafe fn size(){
    println!("the size of reference to i32 is :{}",size_of::<&i32>());
    println!("the size of reference to &[i32] is :{}",size_of::<&[i32]>());
}














