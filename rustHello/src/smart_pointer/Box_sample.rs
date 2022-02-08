use std::ops::Deref;
use crate::smart_pointer::Box_sample::List::{Cons, Nil};
use std::rc::Rc;
use std::cell::RefCell;
use std::borrow::{Borrow, BorrowMut};

/// 参考： https://doc.rust-lang.org/stable/book/ch15-02-deref.html
/// Box解决的 是在堆上分配数据，只能有一个owner，数据是可以修改的，定义的时候，可以有 `引用` 和 `可变引用`  Box<T> allows immutable or mutable borrows checked at compile time
/// Rc 解决的是 在Box的基础上，有多个owner。数据是只读的。   Rc<T> allows only immutable borrows checked at compile time
/// RefCell 解决的是 对某个只读数据进行可变借用, 在Rc的基础上，可以修改数据，RefCell是单一owner的。需要配合Rc使用才是多owner   RefCell<T> allows immutable or mutable borrows checked at runtime.
/// 如果你有一个 不可变变量，你不能把它借用为一个可变变量，用RefCell解决，参看：RefCell_sample.rs::recall_mut_variable_not_work
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


///给Mybox实现 Deref 解引用
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

    let mut bm =Box::new(6);

    *bm+=1;

    assert_eq!(7,*bm);


    let list = List::Cons(1,
                          Box::new(List::Cons(2,
                                              Box::new(List::Cons(3,
                                                                  Box::new(List::Cons(4,
                                                                                      Box::new(List::Nil))))))));

    let mx = 5;
    let mb =MyBox::<i32>::new(mx); //使用trubofish
    assert_eq!(5,*mb); // 会自动调用deref 等效： *(mb.deref())

    let mystr =MyBox::new(String::from("hello"));
    hello(&mystr); // 会自动调用deref，1）mystr 是 Mybox<String>类型的 hello，  deref操作 把 &MyBox<String> 变为 &String，标准库的String实现了deref，把&String变为 &str
    hello(&(*mystr)[..]); //手工转换的方式， *mystr 是一个String，然后 [..] 得到一个切片， 加上 & 返回引用


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

    /*let aa = &*value;
    let mut bb = aa.borrow_mut();
    *bb+=17;*/

    println!("a after = {:?}", a3);
    println!("b after = {:?}", b3);
    println!("c after = {:?}", c3);


}

fn hello(hello: &str) {
    println!("test deref:{}",hello)
}



#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_run_serde_cow() {
        run()
    }

}
