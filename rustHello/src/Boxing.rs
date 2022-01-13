use std::ops::Deref;
use crate::Boxing::List::{Cons, Nil};
use std::rc::Rc;
use std::cell::RefCell;
use std::borrow::BorrowMut;

/// 参考： https://doc.rust-lang.org/stable/book/ch15-02-deref.html


enum List {
    Cons(i32,Box<List>),
    Nil
}

enum List2 {
    Cons(i32,Rc<List2>),
    Nil
}

///最终版本，Cons的value部分能被共享和修改，Cons的list部分可以被共享，不能修改，是只读的，Rc只读，RefCell可修改
/// 参考： https://doc.rust-lang.org/stable/book/ch15-05-interior-mutability.html#having-multiple-owners-of-mutable-data-by-combining-rct-and-refcellt
#[derive(Debug)]
enum List3 {
    Cons(Rc<RefCell<i32>>, Rc<List3>),
    Nil,
}


struct MyBox<T>(T);

impl<T> MyBox<T> {
    pub fn new(t: T) ->MyBox<T> {
        MyBox(t)
    }
}


struct CustomerSmartPointer {
    data: String
}

impl Drop for CustomerSmartPointer {
    fn drop(&mut self) { // 这个代码无法手工调用，只能由编译器插入
        println!("Dropping CustomerSmartPointer with data:{}",self.data)
    }
}



impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}


pub fn run(){

    println!("Smart Pointer Demo");

    let b= Box::new(5);
    println!("{}",b);
    assert_eq!(5,*b);

    let list = List::Cons(1,
                          Box::new(List::Cons(2,
                                              Box::new(List::Cons(3,
                                                                  Box::new(List::Cons(4,
                                                                                      Box::new(List::Nil))))))));

    let mx = 5;
    let mb =MyBox::<i32>::new(mx);
    assert_eq!(5,*mb); // 会自动调用deref

    let mystr =MyBox::new(String::from("hello"));
    hello(&mystr); // 会自动调用deref

    let m1 =CustomerSmartPointer {data:"memory stuff".to_string()};

    let c1 =CustomerSmartPointer {data:"my stuff".to_string()};
    let d1 =CustomerSmartPointer {data:"other stuff".to_string()};

    drop(m1); // 在prelude中已经定义， drop是手动释放


    //Rc<T>

    let a = Cons(5,Box::new(Cons(10,Box::new(Nil))));
    let b= Cons(3,Box::new(a));
    //let c =Cons(4,Box::new(a)); // 这里会报错：a 已经移动，无法使用，修改方法是替换 Box 为Rc

    let a1=Rc::new(List2::Cons(5,Rc::new(List2::Cons(10,Rc::new(List2::Nil)))));
    println!("Count after creating a = {}", Rc::strong_count(&a1));

    let b1 =List2::Cons(3,Rc::clone(&a1));
    println!("Count after creating b = {}", Rc::strong_count(&a1));
    {
        let c1 = List2::Cons(4, Rc::clone(&a1)); // 这里不用 a1.clone()，是为了避免深拷贝
        println!("Count after creating c = {}", Rc::strong_count(&a1));
    }
    println!("Count after c goes out of scope = {}", Rc::strong_count(&a1));



    //使用了RefCell和Rc
    let value =Rc::new(RefCell::new(5));
    let a3 =Rc::new(List3::Cons(Rc::clone(&value),
                        Rc::new(List3::Cons(Rc::new(RefCell::new(10)), Rc::new(List3::Nil)))));
    let b3=List3::Cons(Rc::new(RefCell::new(3)),Rc::clone(&a3));
    let c3=List3::Cons(Rc::new(RefCell::new(8)),Rc::clone(&a3));

    *(*value).borrow_mut()+=10; //改变值，一次到位

    println!("a after = {:?}", a3);
    println!("b after = {:?}", b3);
    println!("c after = {:?}", c3);


}

fn hello(hello: &str) {
    println!("test deref:{}",hello)
}




