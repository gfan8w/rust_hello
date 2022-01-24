use std::fmt::{Display, Debug};


//Sized trait 用于标记有具体大小的类型。在使用泛型参数时，
// Rust 编译器会自动为泛型参数加上 Sized 约束，比如下面的 Data 和处理 Data 的函数 process_data：
/*

struct Data<T: Sized> {
   inner: T,
}
fn process_data<T: Sized>(data: Data<T>)
{
   // .....
}

*/
struct Data<T> {
    inner: T,
}
fn process_data<T>(data: Data<T>)
{
    todo!();
}



fn showDisplay<T:Display>(t:T){

    println!("{}",t);
}

fn showDebug<T:Debug>(t: T) {
    println!("{:?}",t);
}

fn compare<T>(left: T, right:T) where T:Debug+PartialEq {
    println!("{:?} {} {:?}",left, if left == right { "="} else {"!="}, right);
}

pub fn run(){

    compare("tea","Coffee");
    compose_point();
}

///定义个结构体，x,y的类型分别是： T， V
#[derive(Debug)]
struct Point<T,U> {
    x:T,
    y:U
}

///泛型类型不仅可以出现在struct的类型上，在实现的方法上也可以有泛型参数，2组泛型参数是没有相互关系的。
/// 这个例子是：输出一个Point，x来自 自己Point的x，y来自另外一个Point的y。2个传入的Point的类型是没有关联的。
impl<T,U> Point<T,U> {
    fn mixup<A,B>(self, point: Point<A,B>) -> Point<T,B>{
        Point{
            x: self.x,
            y: point.y
        }
    }
}

fn compose_point(){
    let a =Point{x:1.9,y:3.8};
    let b =Point{x:"hello",y:"world"};
    let mix=b.mixup(a);
    println!("mix:{:?}",mix)
}







