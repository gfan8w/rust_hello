pub mod ClosuresFun;
mod fn_closure;
mod fnonce_sample;
mod fnmut_sample;
mod curry_sample;
mod closure_mem_layout;
mod executor_fn_sample;


/*
闭包表述用
 |args| {code}      code里捕获的变量是borrowed
 move |args| {code} code里的捕获变量是owned

闭包是一种匿名类型，一旦声明，就会产生一个新的类型，但这个类型无法被其它地方使用。
这个类型就像一个结构体，会包含所有捕获的变量。

闭包产生的匿名数据类型，格式和 struct 是一样的。
闭包是存储在栈上，并且除了捕获的数据外，
闭包本身不包含任何额外函数指针指向闭包的代码。

如果不使用 move 转移所有权，闭包会引用上下文中的变量，这个引用受借用规则的约束，
所以只要编译通过，那么闭包对变量的引用就不会超过变量的生命周期，没有内存安全问题。

如果使用 move 转移所有权，上下文中的变量在转移后就无法访问，闭包完全接管这些变量，
它们的生命周期和闭包一致，所以也不会有内存安全问题。


*/

pub fn run() {
    ClosuresFun::run();
}

//当闭包不使用move时，是推断着判断如何去捕获变量的，
// 先尝试不可变引用，然后尝试可变引用，最后尝试Move/Copy，
// 一旦尝试成功，将不再尝试。
// 当使用move时，是强制Move/Copy，而不是一步一步地去推断尝试
fn run_closure(){

    let mut name ="hello".to_string();

    // 1.不可变引用，&name被存储在闭包c1里
    let c = ||&name;
    // 可使用所有者变量name，且可多次调用闭包
    println!("{},{},{}",name,c(),c());

    // 2.可变引用，&mut name被存储在闭包c2里，调用c2的时候要修改这个字段，
    // 因此c2也要设置为mut c2
    let mut c2 = ||{
        name.push_str(" world");
    };

    // 可多次调用c2闭包
    // 但不能调用c2之前再使用name或引用name，因为&mut name已经存入c2里了
    //println!("{}",&name); // 取消注释将报错
    //println!("{}",name); // 取消注释将报错
    c2();
    c2();
    println!("{}",name); //c2之后可以使用name，应为已经不存在活跃的可变引用

    //3.隐式的 Move/Copy，将name移入到闭包c3中
    let c3 = || {
        let x =name;
        //let y =name;  // 取消注释见报错，use of moved value
    };

    c3();
    //c3(); //无法再调用c3，其实是 c3 移动走了，无法再适用c3，c3是FnOnce类型，那是什么导致c3是 FnOnce呢？因为它把name移入了。
    // println!("{}",name); // 不可再使用name，已经被move走了

}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_run() {
        run_closure();
    }

}

