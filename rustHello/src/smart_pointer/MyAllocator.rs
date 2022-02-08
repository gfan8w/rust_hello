
//实现一个自己的内存分配器

use std::alloc::{GlobalAlloc, Layout, System};
use std::mem::size_of;


struct MyAllocator;

// 下面我们来实现一个自己的内存分配器。别担心，这里就是想 debug 一下，
// 看看内存如何分配和释放，并不会实际实现某个分配算法。首先看内存的分配。
// 这里 MyAllocator 就用 System allocator，
// 然后加 eprintln!()，和我们常用的 println!() 不同的是，eprintln!() 将数据打印到 stderr
unsafe impl GlobalAlloc for MyAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let data =System.alloc(layout);

        // 注意这里不能使用 println!() 。因为 stdout 会打印到一个由 Mutex 互斥锁保护的共享全局 buffer 中，
        // 这个过程中会涉及内存的分配，分配的内存又会触发 println!()，
        // 最终造成程序崩溃。而 eprintln! 直接打印到 stderr，不会 buffer。
        eprintln!("ALLOC: {:p}, size:{}", data,layout.size());
        data
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        System.dealloc(ptr,layout);
        eprintln!("FREE:{:p}, size:{}",ptr, layout.size())
    }
}

///替换默认的内存分配器，可以使用 #[global_allocator] 标记宏，定义你自己的全局分配器。
/// 注释掉，免的影响全局
/*#[global_allocator]
static GLOBAL: MyAllocator = MyAllocator;*/


#[allow(dead_code)]
struct Matrix{
    // 使用不规则的数字如 505 可以让 dbg! 的打印很容易分辨出来
    data: [u8; 505]
}

impl Default for Matrix {
    fn default() -> Self {
        Self {
            data:[0;505]
        }
    }
}


fn run(){
    // 在这句执行之前已经有好多内存分配
    let data = Box::new(Matrix::default());

    // 输出中有一个 1024 大小的内存分配，是 println! 导致的
    println!("!!!Allocate mem:{:p}, len:{}", &*data, size_of::<Matrix>())


    // data 在这里 drop，可以在打印中看到 FREE
    // 之后还有很多其它内存被释放

}


fn run_box_huge(){
    // 在堆上分配 16M 内存，但它会现在栈上出现，再移动到堆上 , debug 模式下回报stackoverflow
    // “cargo run —release” 编译成 release 代码运行，会正常执行，release 是做了 inline 处理。
    // 如果不 inline，整个 16M 的大数组会通过栈内存传递给 Box::new，导致栈溢出
    let boxed = Box::new([0u8; 1 << 24]);
    println!("len: {}", boxed.len());
}








#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_run_custom_allocator() {
        run()
        // 运行这段代码，你可以看到类似如下输出，其中 505 大小的内存是我们 Box::new() 出来的：
        // FREE:0x7f8558d01180, size:224
        // ALLOC: 0x7f8558c01e50, size:505
        // !!!Allocate mem:0x7f8558c01e50, len:505
        // FREE:0x7f8558c01e50, size:505
        // FREE:0x7f8558d005b0, size:8
        // ALLOC: 0x7f8558d016a0, size:384
        // FREE:0x7f8558c01f50, size:130
        // FREE:0x7f8558d012b0, size:704
        // FREE:0x7f8558d01840, size:180
    }

    #[test]
    fn test_run_box_huge() {
        run_box_huge()
    }


}











