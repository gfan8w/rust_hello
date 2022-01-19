use std::mem::{size_of_val, size_of};


pub fn run(){
    a_array();
    unsafe {
        size();
    }

    slice_ok_bad();
}



fn a_array(){
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

/// &[T] 是一个胖指针，包含了一个指针和一个len，表示 指向内容的指针和长度，它是借用的切片（borrowed slice）
/// Box<[T]> 是一个胖指针，包含了一个指针和一个len，但它是一个有所有权的切片（owned slice）
/// [T; N] 是一个在编译期就知道长度的数组，静态存储的
/// Box<[T; N] 是一个在编译期就知道长度的指针，它无需额外存储其他信息，它不是胖指针
/// [T] 和 [T; N] 都是表示一块连续的内存，切片（slice）和数组（array）的区别在于 在编译期是否知道长度
fn size_show() {
    println!("{}", size_of::<Box<u8>>()); // Pointer: 8
    println!("{}", size_of::<&[u8]>()); // Fat Pointer: 16
    println!("{}", size_of::<Box<[u8]>>()); // Fat Pointer: 16
    println!("{}", size_of::<Box<[u8; 32]>>()); // Pointer: 8，因为编译时已经知道了大小
    println!("{}", size_of::<[u8; 32]>()); // Not a pointer: 32
}

///在rust中，如果一个变量知道类型，但不知道长度，是无意义的。
fn slice_ok_bad(){
    let v: Vec<u8> = vec![1,2,3,4,5]; //Vec是一个结构体，包含3个：容量、长度、指向内容的指针
    //let s = v[1..3]; //编译错误。这里 的s 是一个有所有权的变量，但它不知道长度。s的内容是指向它所代表的内容的第一个位置，但不知道什么时候截止。
    let s = &v[1..3]; //加&后 变为引用，s 是一个胖指针，包含指向所代表内容的第一个元素，还有一个长度
}









