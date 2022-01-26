
struct Person<T>
    where T: Fn(i32) -> i32 // 从其环境获取不可变的借用值
{
    name:String,
    age:T,
}


struct Person1<T>
    where T: Fn()  // 从其环境获取不可变的借用值
                   // 由于是不可变借用，所以在闭包外部也可以使用被借用的变量
{
    age: T
}



struct Person2<T>
    where T: FnOnce() // 在定义闭包时会夺走环境中的变量所有权，闭包只能被调用一次，调用后引用被释放
                      // 由于所有闭包都可以被调用至少一次，所以所有闭包都实现了 FnOnce
{
    name:String,
    age:T,
}


struct Person3<T>
    where T: FnMut()  // 获取可变的借用值所以可以改变其环境

{
    name:String,
    age:T,
}


pub fn run(){
    let num =200;
    let name  =String::from("bob");
    let p = Person{
        name,
        age: |n|  n+num,
    };

    println!("ClourseFun:");
    println!("{}", (p.age)(1));
    println!("{}", (p.age)(2));

    let mut alice  =String::from("alice");

    // 这里定义了闭包，alice 被借走了，而且是可变引用
    let p2 = Person2{
        name: String::from("alice"),
        age: ||  {
            alice.push_str(" hello ");
            println!("{}",alice);
        },
    };

    // 这里已经不能再使用 alice
    //alice.push_str(" good");

    // 调用一次之后，可变引用被释放
    (p2.age)();
    //(p2.age)();   //第二次会报错
    println!("{}", alice);


    let mut stack  =String::from("jack ");

    //这里定义了闭包，stack 被借走了，而且是可变引用
    let mut p3 = Person3{
        name: String::from("Jack"),
        age: ||  {
            stack.push_str(" hello ");
            println!("{}",stack);
        },
    };

    //这里不能再使用stack
    //stack.push_str(" good ");

    (p3.age)();
    (p3.age)();

    //到这里，编译器认为闭包不再调用，所以使用不可变引用 stack
    println!("{}", stack);

    //如果这里还要使用闭包，那么上面这句会报错，因为可变引用和不可变引用不可同时存在
    //(p3.age)();


    let  ss =String::from("lily");
    let p1 =Person1 {
        age:|| {
            //ss.push_str(" hello ");
            println!("{}",ss) // 这里定义了闭包，ss 被借走了，而且是可变引用
        }
    };

    (p1.age)();
    (p1.age)();
    println!("{}",ss);

    let  aa =String::from("Jim");
    let p11 =Person1 {
        age: move || {
            println!("{}",aa);
        }
    };

    (p11.age)();
    (p11.age)();

    //println!("{}",aa); // 这里我们不能再访问了,已转移


    let v= vec![1,2,3,4];
    let equal= move |x| x==v ;
    let y =vec![1,2,3,4];
    assert!(equal(y));
    //println!("{:?}",v); // v 已经移动走了，这里不能再使用

    let aj=0;


}



#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_Closure() {

        run();

    }
}








