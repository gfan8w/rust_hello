use std::ops::{Deref, DerefMut};


/*

DerefMut “继承”了 Deref，只是它额外提供了一个 deref_mut 方法，用来获取可变的解引用。
所以这里重点学习 Deref。
对于普通的引用，解引用很直观，因为它只有一个指向值的地址，从这个地址可以获取到所需要的值，比如下面的例子：
let mut x = 4;
let y = &mut x;
*y += 1;            // 解引用，内部调用 DerefMut（其实现就是 *self）


但对智能指针来说，拿什么域来解引用就不那么直观了，我们来看之前学过的 Rc 是怎么实现 Deref 的：

impl Deref for Rc
{
    type Target = T;
    fn deref(&self) -> &T
    {
        &self.inner().value
    }
}

参见： deref_rc.png 查看 Rc的解引用
从图中还可以看到，Deref 和 DerefMut 是自动调用的，*b 会被展开为 *(b.deref())。

*/


///演示Buf这个自己定义的结构，如何Deref
/// 在这个例子里，数据结构 Buffer 包裹住了 Vec，
/// 但这样一来，原本 Vec 实现了的很多方法，现在使用起来就很不方便，需要用 buf.0 来访问。
/// 怎么办？可以实现 Deref 和 DerefMut，
/// 这样在解引用的时候，直接访问到 buf.0，
/// 省去了代码的啰嗦和数据结构内部字段的隐藏。
///
/// 在这段代码里，还有一个值得注意的地方：写 buf.sort() 的时候，
/// 并没有做解引用的操作，为什么会相当于访问了 buf.0.sort() 呢？
/// 这是因为 sort() 方法第一个参数是 &mut self，
/// 此时 Rust 编译器会强制做 Deref/DerefMut 的解引用，所以这相当于 (*(&mut buf)).sort()。
#[derive(Debug)]
struct Buf<T>(Vec<T>);

impl<T> Buf<T> {
    pub fn new(v: impl Into<Vec<T>>) -> Self {
        Self(v.into())
    }
}

impl<T> Deref for Buf<T> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for Buf<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}


pub fn run() {

    let vec = vec![4,2,7,6];
    let mut buf =Buf::new(vec);

    //buf.sort();
    // 因为实现了 Deref 和 DerefMut，这里 buf 可以直接访问 Vec 的方法
    // 上面面这句相当于：(&mut buf).deref_mut().sort()，也就是 (&mut buf.0).sort()
    //let mut v =buf.0;  // 所有权转移了。下面只能打印v， 可以 &mut buf.0，借用，下面还是能用buf 打印的
    //v.sort();
    //println!("sorted buf:{:?}",v);

    // (&mut buf.0).sort(); // 这里是手工deref，没有调用到 上面定义的 deref_mut

    let bm =(&mut buf).deref_mut();
    bm.sort();

    println!("sorted buf:{:?}",buf);

}



#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1() {
        run();
    }
}










































