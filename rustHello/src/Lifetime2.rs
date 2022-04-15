

///The Result object to be cached
#[derive(Debug)]
pub struct Result{
    data: u32,
}

///这里 self.value = Some(&Result { data });  保存一个引用是不可以的，引用是对一个存在所有权的数据的引用，必须有其他某个东西拥有该变量，
/// 这里的临时数据 data，不存在其他所有者，所以引用无法成立。
/// get_value 返回的是一个引用，这个返回值，不可能比Cacher这个结构体的数据活的更长久
mod A {

    use super::*;

    pub struct Cacher<'a, T>
        where T: Fn(u32) -> u32 {
        calc: T,
        value: Option<&'a Result>,

    }

    impl<'a, T> Cacher<'a, T>
        where T: Fn(u32) -> u32 {
        pub fn new(calc: T) -> Self {
            Cacher { calc: calc, value: None }
        }

        pub fn get_value(&mut self, arg: u32) -> &Result {
            match self.value {
                Some(v) => v,
                None => {
                    let data = (self.calc)(arg);
                    // self.value = Some(&Result { data });  // temporary value dropped while borrowed
                    self.value.unwrap()
                }
            }
        }
    }
}

mod B {
    use super::*;

    pub struct Cacher<T>
        where T: Fn(u32) -> u32 {
        calc: T,
        value: Option<Result>,

    }

    impl<T> Cacher<T>
        where T: Fn(u32) -> u32 {
        pub fn new(calc: T) -> Self {
            Cacher { calc: calc, value: None }
        }

        pub fn get_value(&mut self, arg: u32) -> &Result {
            match self.value {
                Some(ref v) => v,
                None => {
                    let data = (self.calc)(arg);
                    self.value = Some(Result { data });  // 修改
                    //self.value.as_ref().unwrap()   // 这样可以, 分解为以下几个步骤
                    let aa = &self.value;  //只获取引用 &Option<Result>，避免value移动？ 还是避免self移动？
                    let ref dd = self.value; // 等同aa, &Option<Result>
                    let bb =dd.as_ref();   //获取内部值的引用 Option<&Result>
                    let cc = bb.unwrap();         // 拿到resut内部的内容，内容是引用。&Result
                    cc
                }
            }
        }
    }
}


mod C {
    //链表，实现一个打印功能，把所有元素都打印出来
    use std::cell::RefCell;
    use std::fmt::{Debug, Display, Formatter};
    use std::rc::Rc;


    type Link<T> = Option<Rc<RefCell<Node<T>>>>;

    pub struct Node<T>{
        pub(crate) val:T,
        pub(crate) next: Link<T>,
    }

    pub struct List<T>{
        pub(crate) head: Link<T>,
        pub(crate) tail: Link<T>,
        pub(crate) len: usize,
    }

    impl<T: Debug> Display for List<T> {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {

            // temporary value is freed at the end of this statement
            // let mut temp = &self.head;
            // while let Some(r) = temp {
            //     write!(f, "{:?} -> ", r.borrow().val);
            //     temp = &r.borrow().next; // ERROR
            // }

            let mut node = self.head.clone();
            while let Some(t) = node {
                write!(f,"{:?}->",t.borrow().val);
                node = t.borrow().next.clone();
            }

            Ok(())
        }
    }
}







#[cfg(test)]
mod test1 {
    use super::A::*;

    #[test]
    fn test_temporary_value_is_freed_at_the_end_of_this_statement () {
            let mut cache = Cacher::new(|x|x+5);
            let a= cache.get_value(4);
        println!("a:{:?}",a)
    }
}

#[cfg(test)]
mod test2 {
    use super::B::*;

    #[test]
    fn test_temporary_value_is_freed_at_the_end_of_this_statement () {
        let mut cache = Cacher::new(|x|x+5);
        let a= cache.get_value(4);
        println!("a:{:?}",a)
    }
}



#[cfg(test)]
mod testC {
    use std::cell::RefCell;
    use std::rc::Rc;
    use super::C::*;

    #[test]
    fn test_temporary_value_is_freed_at_the_end_of_this_statement () {
        let node_1 =Node{ val: 1, next: None };
        let node_2 =Node{ val: 2, next: Some(Rc::new(RefCell::new(node_1))) };
        let node_3 =Node{ val: 3, next: Some(Rc::new(RefCell::new(node_2))) };
        let node_4 =Node{ val: 4, next: Some(node_3.next.as_ref().unwrap().clone()) };

        let list =List{
            head: Some(Rc::new(RefCell::new(node_4))),
            tail: None,
            len: 0
        };

        println!("a:{}",list)
    }
}





