
use std::mem;

/*
v len:1, capacity:1
v1 len:0, capacity:8
v1: 0x7fd414600010, 8, 0
heap start: 0x7fd414600000
new heap start: 0x7fd414600200      <-- 前后2次的heap 地址改变了，这就是 可变引用和不可变引用不能共存的原因
v: 0x7fd414600200, 64, 33
*/
pub fn run() {

    println!("vec re-allocate demo");


    // capacity 是 1, len 是 1
    let mut v = vec![1];
    println!("v len:{}, capacity:{}", v.len(), v.capacity());

    // capacity 是 8, len 是 0 
    let v1: Vec<i32> = Vec::with_capacity(8);
    println!("v1 len:{}, capacity:{}", v1.len(), v1.capacity());
    print_vec("v1", v1);  // 因为 v1没有元素，无法获取到第一个元素，故我自己的找地址方法失效。用0x0代替

    // 我们先打印 heap 地址，然后看看添加内容是否会导致堆重分配
    println!("heap start: {:p}", &v[0] as *const i32);

    extend_vec(&mut v);

    // heap 地址改变了！这就是为什么可变引用和不可变引用不能共存的原因
    println!("new heap start: {:p}", &v[0] as *const i32);

    print_vec("v", v);


}

///不同操作系统上的重新分配的条件不一样
#[cfg(target_os = "macos")]
fn extend_vec(v: &mut Vec<i32>) {
    // Vec 堆内存里 T 的个数是指数增长的，我们让它恰好 push 33 个元素
    // capacity 会变成 64
    (2..34).into_iter().for_each(|x| v.push(x));
}

#[cfg(target_os = "linux")]
fn extend_vec(v: &mut Vec<i32>) {
    // Vec<T> 堆内存里 T 的个数是指数增长的

    // 在 linux 上 128KB (32k x 4) 内存都不会导致重分配
    // (2..32769).into_iter().for_each(|i| v.push(i));

    // 超过 128k 终于重分配
    (2..32770).into_iter().for_each(|i| v.push(i));
}


fn print_vec(name: &str, v: Vec<i32>){

    // 因为 transmute 会 move v，故在move之前把需要的值 都拿到，
    // capacity 和 len 都是 borrow v，得到的cap 和 len 已经跟 v 无关系了。不会影响 transmute
    // vp是为了取v的pointer，直接 vp=&v[0]，因为在 transmute 后还要使用vp，vp又关联到v，导致 v被move之后还要使用。
    //想办法把 vp的值clone 一个出来，那就 as 转换一下 as *const i32，这样让 vp 跟 v脱离关系。
    let cap = v.capacity();
    let len = v.len();
    let mut vp = 0 as *const i32;
    if len>0{
        vp = &v[0] as *const i32;
    }

    //下面2行代码是原来他的代码，
    let p: [usize; 3] = unsafe {mem::transmute(v)};
    // 打印 Vec 的堆地址，capacity，len
    println!("{}: 0x{:x}, {}, {}",name,p[0],p[1],p[2]);

    // 这个是我的代码，使用另外一种方法 打印出地址 vp,首地址是一样的。
    //   v: 0x7faa02500200, 64, 33
    // a v: 0x7faa02500200, 64, 33
    println!("a {}: {:p}, {}, {}",name,vp,cap,len);
}


























