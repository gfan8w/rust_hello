

///检查某个类型是否实现了 Copy，利用泛型约束，很简单来检查
fn is_copy<T: Copy>() {}


fn is_Type_impl_Copy_Trait(){
    is_copy::<bool>();
    is_copy::<char>();
    is_copy::<i32>(); // all iXX and uXX, usize/isize, fXX implement Copy trait
    is_copy::<u64>();
    is_copy::<usize>();

    is_copy::<fn()>(); // function 函数类型也实现了Copy，它实际上是一个指针

    is_copy::<*const String>();
    is_copy::<*mut String>(); //raw pointer is Copy
    is_copy::<*mut i32>();   // 裸指针只有2各种：*const T, *mut T，裸指针，失去安全性。

    is_copy::<&[Vec<u8>]>(); // immutable reference is Copy
    is_copy::<&String>();

    is_copy::<[u8;4]>(); //arrary/tuple内部元素类型是 Copy的，整体它也是Copy的。
    is_copy::<(&str,&str)>();

}

/*fn types_not_impl_copy_trait(){
    // unsized or dynamic sized type is not Copy
    is_copy::<str>();
    is_copy::<[u8]>();
    is_copy::<Vec<u8>>();     // Vec<T> 在编译时可以确定大小（24 字节）, 但它不能实现 Copy trait。任何有资源需要释放的数据结构，都无法实现 Copy trait。
    is_copy::<String>();

    // mutable reference is not Copy
    is_copy::<&mut String>();

    // array / tuple 内部元素不能copy的，整个类型也是不能Copy的
    is_copy::<[Vec<u8>;4]>();
    is_copy::<(String, u32)>();

}*/

///在 Rust 下，分配在堆上的数据结构可以引用栈上的数据么?
/// 只要栈上的变量的生命周期比堆上的变量的生命周期长即可
/// 以 Vec 为例，它可以存储栈上的引用，但必须注意一点那就是该引用的生命周期不能超过栈上数据的生命周期。
fn stack_is_longer_than_heap(){

    let x=1;
    let y =2; //栈上

    let a = vec![&x,&y]; //堆上

    println!("a:{:?}",a);

    println!("x:{}",x);

}


fn practice_print_address(){
    //先复习一下 打印地址，{:p} 打印引用变量的地址。
    let x = &4;
    println!("4 address is :{:p}, x address:{:p}, value:{}", x, &x, x); // 4 address is :0x10880574c, x address:0x7ffee7603568, value:4
    let y = 5;
    println!("5 address is :{:p}, y address:{:p}, value:{:}", &y,&&y,y); // y是值类型，这里要打印地址，必须变为引用 &y表示的是地址
    // 5 address is :0x7ffee760360c, y address:0x7ffee7603698, value:5 ，5 跟 y的地址相近，但 4 的地址偏差很大，说明 4是一个常量，在常量池位置

}


fn borrow_sample(){

    let data = vec![1,2,3,4];  // vec 构造出来的是值类型？？？？
    let data1 = &data;
    // 值的地址是什么？引用的地址又是什么？
    println!("addr of value:{:p}({:p}), addr of data:{:p}, data1:{:p}", &data,data1, &&data, &data1); // 后面2个是取 data 和 data1 的地址，在他们前面再用`&`得到地址
    println!("sum of data1:{:?}",sum(data1));

    //堆上的数据是什么？
    println!("data：{:p},{:p},{:p},{:p}", &data[0], &data[1], &data[2], &data[3])

}


fn sum(data: &Vec<i32>) -> i32{
    // 值的地址会改变么？引用的地址会改变么？
    println!("addr of value: {:p}, addr of ref: {:p}", data, &data);
    data.iter().fold(0,|s,&i| s+i)
}




pub fn run() {
    //练习一下打印地址
    practice_print_address();

    println!("demo show is a type impl the Copy Trait");

    is_Type_impl_Copy_Trait();

    //把这个代码注释掉，查看编译器运行的报错
    //types_not_impl_copy_trait();

    stack_is_longer_than_heap();

    borrow_sample();

    println!("dome end");
}












