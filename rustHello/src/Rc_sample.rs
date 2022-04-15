use std::rc::Rc;

pub fn run(){

    simple_starter();
    run_node();
}






///一个对象有多个owner，如何处理? 使用Rc， Reference counter
/// 多线程并发时，一个对象有多个owner，如何处理？使用 Arc，Atomic reference counter
/// 对一个 Rc 结构进行 clone()，不会将其内部的数据复制，只会增加引用计数。
/// 而当一个 Rc 结构离开作用域被 drop() 时，也只会减少其引用计数，直到引用计数为零，才会真正清除对应的内存
fn simple_starter(){
    // 创建了三个 Rc，分别是 a、b 和 c。它们共同指向堆上相同的数据 1，
    // 也就是说，堆上的数据有了三个共享的所有者。
    // 在这段代码结束时，c 先 drop，引用计数变成 2，
    // 然后 b drop、a drop，引用计数归零，堆上内存被释放
    let a=Rc::new(1);
    let b = Rc::clone(&a); // 这里约定使用 Rc::clone(&a)，而不是 a.clone()，因为避免 a 可能的deep-copy， Rc::clone 只是简单的增加引用计数
    let c = a.clone();

}


//实现一个 有向无环图DAG


///directed acyclic graph  节点，包含一个id 和它的邻近节点
#[derive(Debug)]
struct Node {
    id:usize,
    next:Option<Rc<Node>>
}

impl Node {
    pub fn new(id: usize) -> Self {
        Self {
            id,
            next:None,
        }
    }

    pub fn update_next(&mut self, next:Rc<Node>){
        self.next=Some(next)
    }

    pub fn get_next(&self) -> Option<Rc<Node>> {
        //self.next.as_ref().map(|n| n.clone())
        let a =self.next.as_ref();
        let b =a.unwrap();
        Some(b.clone())
        //map的参数要求是self，会移动self，我们这里不能移动next，编译报错：cannot move out of `self.next` which is behind a shared reference
        //self.next.map(|n| Rc::clone(&n))
    }
}

fn run_node(){
    //生成节点。node1，node2，node3，node4
    // node1 -> node3 -> node4
    // node2 -> node3 -> node4
    let mut node1 = Node::new(1);
    let mut node2 =Node::new(2);
    let mut node3 =Node::new(3);
    let node4 =Node::new(4);

    node3.update_next(Rc::new(node4));

    node1.update_next(Rc::new(node3)); // node3 在这里move了

    node2.update_next(node1.get_next().unwrap()); //这里node1返回的node3是一个Rc的clone

    let node5 = Node::new(5);

    //无法再修改node3
    //node3.update_next(Rc::new(node5));            // node3 在上面已经move走了，无法再用node3，必须间接通过node1.get_next() 来获得node3
    //let mut node33 = node1.get_next().unwrap(); //这里很辛苦的拿到了node3，可是Rc没有试下DerefMut 无法拿到可变引用，就无法执行下面的update_next
    //node33.update_next(Rc::new(node5));

    println!("node1: {:?}, node2: {:?}", node1, node2);

}


#[cfg(test)]
mod test1 {
    use super::*;

    #[test]
    fn test_Node_Rc () {
        run_node();
    }
}





















