


fn test_0(){

    let mut array:  Vec<Vec<u8>> =Vec::new();

    for _i in 1..5 {
        let mut inner_arr =Vec::<u8>::new();

        for _j in 1..6 {
            inner_arr.push(rand::random::<u8>());
        }

        println!("num is : {:?},num.as_ptr(): {:?}",inner_arr,inner_arr.as_ptr());
        array.push(inner_arr);
    }

    println!("2-d-array:{:?}", array);


}


fn test_1(){

    let mut array:  Vec<* const u8> =Vec::new();

    for _i in 1..5 {
        let mut inner_arr =Vec::<u8>::new();

        for _j in 1..6 {
            inner_arr.push(rand::random());
        }

        println!("num is : {:?},num.as_ptr(): {:?}",inner_arr,inner_arr.as_ptr());
        array.push(inner_arr.as_ptr());
    }

    println!("2-d-array:{:?}", array);

    println!("hello")

    /* 这段代码的问题是 array里最后保存的是最后一次循环的内容
    num is : [21, 81, 7, 216, 149],num.as_ptr(): 0x7fa458501200
    num is : [61, 39, 149, 206, 23],num.as_ptr(): 0x7fa458501200
    num is : [126, 199, 96, 114, 96],num.as_ptr(): 0x7fa458501200
    num is : [116, 167, 147, 238, 223],num.as_ptr(): 0x7fa458501200
    2-d-array:[0x7fa458501200, 0x7fa458501200, 0x7fa458501200, 0x7fa458501200]

    */



}



/**
作者回复: 你的代码有四个问题:
1. 应该是 Vec<* const [u8]>，而不是 Vec<* const u8>，你的指针是指向一个 slice 的，而非一个 u8。
2. as_ptr() 拿到的是栈上的指针，你应该需要堆上指向实际内容的指针。所以应该用 Vec::into_boxed_slice(), 然后再用 Box::into_raw 拿到裸指针。
3. 不要乱用 unsafe。这里没有需要 unsafe 的地方。
4. 循环的变量虽然没有用到，但不应该用同一个，很容易导致误用。

至于你的问题，你可以想象一下，栈上数据是如何存取的。每次循环结束，num 的作用域就结束了，它这 24 个字节的栈内存就可以重新使用，
所以你拿到的都是同一个地址（而且作为裸指针是没有用处的地址）。

这里我看错了，我以为你用的是 as_ref()，as_ptr() 是堆上的数据。之前代码的问题是每次 num array 都被回收了


之前我回复的时候把 as_ptr() 看成了 as_ref()，所以认为是栈上的地址。as_ptr() 返回堆地址。这里的问题是 num 是个局部变量，
每次循环完都会被回收，所以一直得到相同的堆地址。我的写法用了 Box::into_raw(boxed)，避免了内存被自动回收

*/
fn test_2(){
    let mut array:  Vec<* const [u8]> =Vec::new();

    for _i in 1..5 {
        let mut inner_arr =Vec::<u8>::new();

        for _j in 1..6 {
            inner_arr.push(rand::random());
        }

        println!("num is : {:?}, num.as_ptr(): {:?}",inner_arr,inner_arr.as_ptr());
        let pt =inner_arr.as_ptr();
        //let aa = *inner_arr;
        let inr_pt =&*inner_arr;
        array.push(&*inner_arr);
    }

    println!("2-d-array:{:?}", array);

    println!("hello")
}


fn test_3(){
    let mut array:  Vec<* const [u8]> =Vec::new();

    for _i in 1..5 {
        let mut inner_arr =Vec::<u8>::new();

        for _j in 1..6 {
            inner_arr.push(rand::random());
        }

        println!("num is : {:?}, num.as_ptr(): {:?}",inner_arr,inner_arr.as_ptr());
        let pt =inner_arr.as_ptr();
        //let aa = *inner_arr;
        let inr_pt =&*inner_arr;
        let boxed = inner_arr.into_boxed_slice();
        let raw=Box::into_raw(boxed);
        array.push(raw);
    }

    println!("2-d-array:{:?}", array);

    println!("hello")
}

fn check_heap_address() {
    println!("check a vec address on heap");
    let mut data = Vec::new();
    data.push(1u8);
    println!("{:p}", &data[0]); // &*data
    println!("{:?}", data.as_ptr());
    let b = data.into_boxed_slice();
    println!("{:p}", b);
    let t = Box::into_raw(b);
    println!("{:?}", t);
}


pub fn run(){
    check_heap_address();

    test_2();


}























