
/*

pub trait FnOnce {
    type Output;
    extern "rust-call" fn call_once(self, args: Args) -> Self::Output;
}

FnOnce 有一个关联类型 Output，显然，它是闭包返回值的类型；还有一个方法 call_once，
* 要注意的是 call_once 第一个参数是 self，它会转移 self 的所有权到 call_once 函数中。

这也是为什么 FnOnce 被称作 Once ：它只能被调用一次。
再次调用，编译器就会报变量已经被 move 这样的常见所有权错误了。



*/

fn run_a_simple_fnonce(){

    //let c: FnOnce

}




///隐式的 FnOnce
/// 这个闭包 c，啥也没做，只是把捕获的参数返回。它会把name的所有权转移出去
/// 就像一个结构体里，某个字段被转移走之后，就不能再访问一样，
/// 闭包内部的数据一旦被转移，这个闭包就不完整了，也就无法再次使用，
/// 所以它是一个 FnOnce 的闭包。
fn run_fn_once(){

    let name = String::from("Tom");  // 改为： name ="Tom"  或 name =3 看看，这是c 就不说话FnOnce了

    //隐式的 FnOnce
    // 这个闭包啥也不干，只是把捕获的参数返回去，它会把name的所有权转移出去，当转移出去后，再次调用该闭包，该闭包就不完整了。所有权移入也是类似的，移入的也无法操作2次的
    let c = move |greeting: String| (greeting, name);

    let result =c("hello".to_string());
    println!("c:{:?}",result);

    // 无法再次调用, c被move
    //let result2=c("good".to_string());
    //println!("c 2:{:?}",result2);

}


/// 如果一个闭包并不转移自己的内部数据，那么它就不是 FnOnce，
/// 然而，一旦它被当做 FnOnce 调用，自己会被转移到 call_once 函数的作用域中，
/// 之后就无法再次调用了，我们看个例子
fn run_fn_is_not_fnonce(){

    ///将传入的call函数当做FnOnce调用
    fn call_once(args: String, call: impl FnOnce(String) ->(String, String)) -> (String, String) {
        call(args)
    }



    let name = String::from("Tom");  // 改为： name ="Tom"  或 name =3 看看，这是c 就不说话FnOnce了


    // 这个闭包会 clone 内部的数据返回，所以它不是 FnOnce， 闭包并不转移自己的内部数据，它是 Fn。如果把 name.clone() 的 clone() 部分去掉，它就变为 FnOnce
    let c = move |greeting: String| (greeting, name.clone());

    let result =c("hello".to_string());
    println!("c:{:?}",result);

    let result2=c("good".to_string());
    println!("c 2:{:?}",result2);



    // 然而一旦它被当成 FnOnce 被调用，就无法被再次调用
    let result3 = call_once("bonjour".to_string(),c); // 这里 c 也可以传 &c,当传 &c 引用时，它以Fn 类型传递，Fn的父类型是FnOnce，call_once 也可以工作
    println!("c 3:{:?}",result3);
    //再次调用 c失败，因为之前它已经被当做FnOnce调用了一次
    //let result4=c("你好".to_string());




    // fn 也可以被当成 FnOnce 调用，只要接口一致就可以
    let special_result = call_once("Jim".to_string(),show);
    println!("c 5:{:?}",special_result);
    fn show(str: String) ->(String,String){
        ("hello".into(), str)
    }



}


//在不同位置插入 move_name(name) 看看闭包报错
fn move_name(str: String) {

}





#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_run_fn_once() {
        run_fn_once();
    }

    #[test]
    fn test_run_fn_is_not_once() {
        run_fn_is_not_fnonce();
    }
}



























