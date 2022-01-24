use std::fmt::{Debug, Formatter};
use std::slice;


/// 注意这里，我们实现了 Copy，这是因为 *mut u8/usize 都支持 Copy
#[derive(Copy,Clone)]
struct RawBuffer {
    ptr: *mut u8,
    len:usize
}

impl From<Vec<u8>> for RawBuffer {
    fn from(vec: Vec<u8>) -> Self {
        let len =vec.len();
        let slice = vec.into_boxed_slice();
        Self {
            // into_raw 之后，Box 就不管这块内存的释放了，RawBuffer 需要处理释放
            ptr: Box::into_raw(slice) as *mut u8,
            len,
        }
    }
}

impl Debug for RawBuffer {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f,"ptr({:p}):{:?}",self.ptr, self.as_ref())
    }
}

impl AsRef<[u8]> for RawBuffer {
    fn as_ref(&self) -> &[u8] {
        unsafe {slice::from_raw_parts(self.ptr,self.len)}
    }
}

// 如果 RawBuffer 实现了 Drop trait，就可以在所有者退出时释放堆内存
// 然后，Drop trait 会跟 Copy trait 冲突，要么不实现 Copy，要么不实现 Drop
// 如果不实现 Drop，那么就会导致内存泄漏，但它不会对正确性有任何破坏
// 比如不会出现 use after free 这样的问题。
// 你可以试着把下面注释去掉，看看会出什么问题
/*impl Drop for RawBuffer {
    fn drop(&mut self) {
        let mut slcs = unsafe {slice::from_raw_parts_mut(self.ptr,self.len)};
        let data =unsafe {Box::from_raw(slcs)};
        drop(data)
    }
}*/




fn use_buffer(buf: RawBuffer){
    println!("buf to die:{:?}", buf);

    // 这里不用特意 drop，写出来只是为了说明 Copy 出来的 buf 被 Drop 了
    drop(buf);
}


///Copy trait 和 Drop trait 是互斥的，两者不能共存，当你尝试为同一种数据类型实现 Copy 时，
/// 也实现 Drop，编译器就会报错。这其实很好理解：Copy 是按位做浅拷贝，那么它会默认拷贝的数据没有需要释放的资源；
/// 而 Drop 恰恰是为了释放额外的资源而生的。
/// 我们还是写一段代码来辅助理解，
/// 在代码中，强行用 Box::into_raw 获得堆内存的指针，放入 RawBuffer 结构中，这样就接管了这块堆内存的释放。
/// 虽然 RawBuffer 可以实现 Copy trait，但这样一来就无法实现 Drop trait。
/// 如果程序非要这么写，会导致内存泄漏，因为该释放的堆内存没有释放。
/// 但是这个操作不会破坏 Rust 的正确性保证：即便你 Copy 了 N 份 RawBuffer，
/// 由于无法实现 Drop trait，RawBuffer 指向的那同一块堆内存不会释放，
/// 所以不会出现 use after free 的内存安全问题。
pub fn run(){

    let data =vec![1,2,3,4];
    let raw:RawBuffer =data.into();

    // 因为 buf 允许 Copy，所以这里 Copy 了一份
    use_buffer(raw);

    // buf 还能用
    println!("buf:{:?}",raw);

}



#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1() {
        run();
    }
}







