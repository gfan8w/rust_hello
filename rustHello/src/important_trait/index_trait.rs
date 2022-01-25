
use std::collections::LinkedList;
use std::fmt;
use std::fmt::Formatter;
use std::ops::{Deref, DerefMut, Index};

///为自定义的 List 类型实现 Index，使得所有的测试都能通过。
/// 这段代码使用了 std::collections::LinkedList，
struct List<T>(LinkedList<T>);

///实现 Deref
impl<T> Deref for List<T> {
    type Target = LinkedList<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

///实现 DerefMut
impl<T> DerefMut for List<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

///实现 Default
impl<T> Default for List<T> {
    fn default() -> Self {
        Self(Default::default())
    }
}


impl<T> fmt::Debug for List<T>
where T: fmt::Display {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut s: String = "list:".into();
        for item in self.iter() {
            let a =item;
            s.push_str(&a.to_string());
            s.push_str(",");
        }

        write!(f,"{}",s.trim_end_matches(","))
    }
}



///实现 index访问 [1], [2]
impl<T> Index<isize> for List<T>{
    type Output = T;

    // 我的实现，在实现的过程中，犯了3个错：
    // 1) iter()...unwrap() 后没加分号，没有加return，以为会return 函数，其实不会，它只是返回当前的if block，外层的 if 还不会return。
    // 2) self.iter() 已经是引用了。无需再加 &
    // 3) 不知道如何返回默认值，最后只能panic!
    // 4) take(1)  nth(ind) 得到的是 Option，要unwrap() 才是最后的值
    /*fn index(&self, index: isize) -> &Self::Output {
        let len =self.len() as isize;
        if index>=0 {
            if index < len {
                return self.iter().skip(index as usize).take(1).next().unwrap();
            }
        }else {
            let ind = (len + index) as usize;
            if ind>=0 {
                return self.iter().nth(ind).unwrap();
            }
        }

        // 我的实现，超过索引报错，不知道如何返回默认值
        panic!("index out of range")
    }*/

    // 老师的实现
    fn index(&self, index: isize) -> &Self::Output {
        let len = self.len() as isize;

        // 备注一句话：
        //标准库的checked_rem_euclid方法, 如果len=0 则返回None
        let i = (index as usize).checked_rem_euclid(self.len()).unwrap();

        // 我们需要先对负数，以及 index 超出范围的数字进行处理
        // 使其落在 0..len 之间
        let idx =(len + ( index % len) ) % len ;

        // 由于 n 一定小于 len 所以这里可以 skip n 取 next，或 用 nth 取第n个元素
        // 此时一定有值，所以可以放心 unwrap
        self.iter().nth(idx as usize).unwrap()
    }
}









struct Container {
    inner: String,
}
// 看到有人提问 self 是 borrow进来的，如何返回一个owner的data？答案：重新生成一个Owned data
impl Container {
    pub fn get_inner(&self) -> String {
        let a =&self.inner;
        a.to_string()
    }
}


fn run_Container(){
    let a = Container{inner:"hello".to_string()};
    let b =a.get_inner();
    println!("B:{}",b)
}





pub fn run() {
    let mut lst = List::default();
    for i in 0..16 {
        lst.push_back(i);
    }

    assert_eq!(lst[0], 0);
    assert_eq!(lst[-1], 15);
    assert_eq!(lst[0], 0);
    assert_eq!(lst[5], 5);
    assert_eq!(lst[15], 15);
    assert_eq!(lst[-2], 14);
    assert_eq!(lst[-15], 1);
    assert_eq!(lst[-16], 0);  // 我的实现，超过索引报错
    assert_eq!(lst[16], 0);   // 老师的实现，超过索引的的 也可以
    assert_eq!(lst[128], 0);
    assert_eq!(lst[-128], 0);

    println!("list:{:?}", lst); //自己给实现一个 Debug，看看list里的内容

    run_Container();
}




#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1() {
        run();
    }
}
