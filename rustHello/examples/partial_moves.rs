//所有权部分移动  https://doc.rust-lang.org/rust-by-example/scope/move/partial_move.html

#[derive(Debug)]
struct Person {
    name: String,
    age: Box<u8>,
}



pub fn show(){

    let p =Person{name:"lint".into(), age: Box::new(12)};

    // person.name 的所有权因为对象解构而移动到变量name中，age因为是ref所以只是引用
    let Person{ name,  ref age} = p;
    println!("The person's name is:{}",name);
    println!("The person's age is {}", age);

    //无法打印p,因为 name已经移动走了。
    //println!("The person struct is {:?}", p);

    // person.age 还是可以访问的
    println!("The person's age from person struct is {}", p.age);

    //在定义Person的时候，我们把age分配在堆上，如果age 直接定义为u8，让age在栈上分配
    //因为 u8具有 Copy属性，在解构的时候，不加 ref 也是可以的。加上Box，不会发生Copy，而是发生移动

}



















pub fn main(){
    show();
}