use std::cell::{Ref, RefCell};
use std::rc::Rc;
use std::sync::mpsc::Receiver;

pub fn run(){
    recall_mut_variable();
    simple_starter();
    simple_starter_one_scope();
    ordinary_simple_object_mut();
    mut_object_move_to_scope_drop();
    run_modified_node();
}

// 借用（borrowing）规则的一个后果是，如果你有一个 不可变变量，你不能把它借用为一个可变变量，recall_mut_variable_not_work无法编译， RefCell通过内部可变性，去除该约束
/*fn recall_mut_variable_not_work(){
    let x =5;
    let y = &mut x;

    *y+=1;
    println!("recall_mut_variable:{}",x)
}*/

///回忆一下 外部可变性的常规使用： `recall_mut_variable_not_work` 这个函数，注释掉，看编译错误
fn recall_mut_variable(){
    let mut x =5;
    let mut y = &mut x;

    *y+=1;
    println!("recall_mut_variable:{}",x)
}


///RefCell 的内部可变性， 用let mut a =1; 这个 带mut申明的 我们称之为 外部可变性
fn simple_starter(){
    //并未将 data 声明为可变变量
    let data = RefCell::new(3);

    // 用花括号分装到另外一个作用域下，如果没有这个独立的作用域，
    // 根据所有权规则，在同一个作用域下，我们不能同时有活跃的可变借用和不可变借用。
    // 通过这对花括号，我们明确地缩小了可变借用的生命周期，不至于和后续的不可变借用冲突。
    // 可以把这对 花括号去掉，看看，编译时没有错误，但运行时会报：already mutably borrowed: BorrowError
    // 可以看到，所有权的借用规则在此依旧有效，只不过它在运行时检测。
    {
        //通过使用 RefCell 的 borrow_mut() 方法，来获得一个可变的内部引用
        let mut v = data.borrow_mut();
        *v+=3;
    }

    // 通过 RefCell 的 borrow() 方法，获得一个不可变的内部引用
    let vo= data.borrow();
    println!("vo:{}",vo);
}



fn simple_starter_one_scope(){
    //并未将 data 声明为可变变量
    let data = RefCell::new(3);

        //通过使用 RefCell 的 borrow_mut() 方法，来获得一个可变的内部引用
        let mut v = data.borrow_mut();
        *v+=3;

    drop(v);  //如果想去掉 {...} 这个特有的作用域，可以改变原来编译器在函数末尾的drop(v)，我们这了提前drop，后面不用这个 可变引用。只留下不可变引用。
                  // data.borrow_mut() 产生的 v 会一直活跃到作用域结束。这是运行期检查和编译期检查的区别

    // 通过 RefCell 的 borrow() 方法，获得一个不可变的内部引用
    let vo= data.borrow();

    println!("vo:{}",vo);

    // 隐含的 drop
    // 原本 drop(v) 发生在这里
}

// 可变引用只能有1个，这里有2个，但这个可以编译通过运行，因为编译器有 优化，v在println之前 就 不活跃了，
// 在println之前 v 就已经失效了。只留下一个活跃的可变引用 data。
// 可以模拟v还是依然有效，在println 后面再加一个 `println!("v:{}",v)` 使得v活跃，则会编译报错
fn ordinary_simple_object_mut() {
    let mut data = 1;
    let mut v = &mut data;
    *v += 1;
    println!("data: {:?}", &data);
    // println!("v:{}",v)
}


fn mut_object_move_to_scope_drop(){

    let mut data =vec![1,2,3];

    {
        // data;  // borrow of moved value: `data` , data 的所有权被转移给了 {} 作用域内部，并且由于没有人使用这个花括号的返回值，所以 data 被 drop 了。之后再用 data 就会报错
        data.push(4);  // 这个解释有点牵强啊？？？？？
    }

    data.push(5);

    println!("data:{:?}",data);

}


#[derive(Debug)]
struct Node {
    id: i32,
    next: Option<Rc<RefCell<Node>>>,
}


impl Node {
    fn new(id: i32) ->Self {
        Self {
            id,
            next:None,
        }
    }

    fn update_next(&mut self, next: Rc<RefCell<Node>>) {
        self.next=Some(next);
    }

    fn get_next(&self) -> Option<Rc<RefCell<Node>>> {
        self.next.as_ref().map(|n| n.clone())
    }
}


fn run_modified_node(){
    let mut node1 =Node::new(1);
    let mut node2 =Node::new(2);
    let mut node3 =Node::new(3);

    let node4 =Node::new(4);

    node3.update_next(Rc::new(RefCell::new(node4)));
    node1.update_next(Rc::new(RefCell::new(node3)));
    node2.update_next(node1.get_next().unwrap());

    println!("node1：{:?} , node2:{:?}",node1,node2);

    //现在来更改node 3，让node 3 指向node 5，而不是上面的指向 node4
    println!("modify refcell node3:");
    let node5 =Node::new(5);
    let node3 = node1.get_next().unwrap();
    // 获得可变引用，来修改 downstream, 这里 为什么 不可以有个临时变量？ let mut aa= node3.borrow_mut();
    node3.borrow_mut().update_next(Rc::new(RefCell::new(node5)));
    println!("change to node5 - node1：{:?} , node2:{:?}",node1,node2);
}


/*
&mut x 编译时编译器可以检查是否违背所有权规则，borrow_mut() 是个函数，只有运行到这一刻，才知道是否违背所有权规则。
*/

#[cfg(test)]
mod test1 {
    use super::*;

    #[test]
    fn test_Node_Rc_RefCell () {
        run_modified_node();
    }

    #[test]
    fn test_simple_starter () {
        simple_starter();
    }


    #[test]
    fn test_mut_object_move_to_scope_drop () {
        mut_object_move_to_scope_drop();
    }
}











