
/*

Dereferencing uses *
Destructuring uses &, ref, and ref mut

*/

#[derive(Debug)]
struct Car {
    id: String,
    passenger: Vec<Passenger>
}

#[derive(Debug)]
struct Passenger{
    name: String
}

fn dereferencing(){

    let reference = Car{
        id: "1".to_string(),
        passenger: vec![Passenger{
            name: "Jim".to_string()
        }]
    };

    let a = &reference;
    let b = *a;    // 解引用操作都是转移所有权的。dereference 时都是将所有权从指针所指向的地方移动到变量处
                        // &*reference 也同样没区别，会移动所有权

    let mut c = "hello";
    let d = *c;

    let myv = vec!["hel".to_string(),"llo".to_string()];
    let myv_ref = &myv;
    let myv_deref: Vec<_> = *myv_ref;

}


fn destructuring(){
    // 定义个引用变量， & 表示是一个引用， 等号右边已经是一个引用
    let reference = &4;

    match reference {
        // If `reference` is pattern matched against `&val`, it results
        // in a comparison like:
        // `&i32`
        // `&val`
        // ^ We see that if the matching `&`s are dropped, then the `i32`
        // should be assigned to `val`.
        // reference与 &val做模式匹配，例如类似 &i32这个模式，如果匹配到的话，&被丢弃，i32值放入val中。这个是所有权转移? 还是发生了clone？
        &val => println!("Got a value via destructuring: {:?}", val),
    }

    println!("{}", reference);

    // To avoid the `&`, you dereference before matching.
    // 避免使用 &，可以在做match前，先解引用
    // *reference 是会导致所有权移动的。 这里实际发生了一次 clone()
    match *reference {
        val => println!("Got a value via dereferencing: {:?}", val),
    }

    // What if you don't start with a reference? `reference` was a `&`
    // because the right side was already a reference. This is not
    // a reference because the right side is not one.
    // 如果等号右边是一个值，不是引用，该如何获取引用呢？ 有 ref 关键字可以使用， 还可以使用 ref mut，获取一个可变引用
    let _not_a_reference = 3;

    // Rust provides `ref` for exactly this purpose. It modifies the
    // assignment so that a reference is created for the element; this
    // reference is assigned.
    let ref _is_a_reference = 3;

    // Accordingly, by defining 2 values without references, references
    // can be retrieved via `ref` and `ref mut`.
    let value = 5;
    let mut mut_value = 6;

    // Use `ref` keyword to create a reference.
    match value {
        ref r => println!("Got a reference to a value: {:?}", r),
    }

    // Use `ref mut` similarly.
    match mut_value {
        ref mut m => {
            // Got a reference. Gotta dereference it before we can
            // add anything to it.
            *m += 10;
            println!("We added 10. `mut_value`: {:?}", m);
        },
    }
}




fn destructuring_car(){
    // 定义个引用变量， & 表示是一个引用， 等号右边已经是一个引用
    let reference = &Car{
        id: "1".to_string(),
        passenger: vec![Passenger{
            name: "Jim".to_string()
        }]
    };

    //这里是destructure导致的所有权移动， destruct发生在 &val
    match reference {
        // reference与 &val做模式匹配，例如类似 &i32这个模式，如果匹配到的话，&被丢弃，i32值放入val中。这个是所有权转移
        &val => println!("Got a value via destructuring: {:?}", val),
    }

    println!("{:?}", reference);

    // To avoid the `&`, you dereference before matching.
    // 如果想避免使用 &，可以在做match前，先解引用(dereference)
    // 指针的 *v 是一个所有权移动的操作。如果没有实现 Copy或 Clone，就是移动所有权
    // match表达式会用place expressions作为输入，所有权不会从place expressions中移出。 这里是移动到 val里，而不是从match的匹配那里移出
    // 所以这里不会有移动出 reference的所有权？

    // when you pattern match, you are using place expressions.
    // This means that successfully pattern matching doesn't always move out of the input!
    //
    // match expressions also take place expressions as input,
    // so even they don't have to move out of their input! This allows you to match on unsized values like slices!
    match *reference {
        val => println!("Got a value via dereferencing: {:?}", val),
    }
    let cc=reference;

    // What if you don't start with a reference? `reference` was a `&`
    // because the right side was already a reference. This is not
    // a reference because the right side is not one.
    // 如果等号右边是一个值，不是引用，该如何获取引用呢？ 有 ref 关键字可以使用， 还可以使用 ref mut，获取一个可变引用
    let _not_a_reference = Car{
        id: "2".to_string(),
        passenger: vec![Passenger{
            name: "Skye".to_string()
        }]
    };

    // 使用 Rust 提供的 ref 来 定义 引用
    let ref _is_a_reference = Car{
        id: "4".to_string(),
        passenger: vec![Passenger{
            name: "Skye".to_string()
        }]
    };

    // Accordingly, by defining 2 values without references, references
    // can be retrieved via `ref` and `ref mut`.
    let value = _not_a_reference;

    let mut mut_value = Car{
        id: "4".to_string(),
        passenger: vec![Passenger{
            name: "Skye".to_string()
        }]
    };

    // Use `ref` 在 match 内部创建了一个引用变量
    match value {
        ref r => println!("Got a reference to a value: {:?}", r),
    }

    // Use `ref mut` 类似的 在 match内部创建一个可变引用，它要求原变量是 可变的。需加 mut.
    match mut_value {
        ref mut m => {
            // Got a reference. Gotta dereference it before we can
            // add anything to it.
            // 改变值
            (*m).id = "10".to_string();
            println!("We change id to 10. `mut_value`: {:?}", m);
        },
    }
    println!("mut value: {:?}", mut_value);

}

// match表达式会用place expressions作为输入，所有权不会从place expressions中移出。
// 所以这里不会有移动出 reference的所有权？

// when you pattern match, you are using place expressions.
// This means that successfully pattern matching doesn't always move out of the input!
//
// match expressions also take place expressions as input,
// so even they don't have to move out of their input! This allows you to match on unsized values like slices!
fn destructuring_un_sized() {
    let x = [0, 1, 2];
    let x = &x as &[i32];

    // note *x: [i32], which is !Sized place expressions
    match *x {
        [] => println!("empty"),
        [_, ..] => println!("one"),
        _ => println!("many"),
    }
}


/*
位置表达式
位置表达式(Place Expression)一般叫做左值，是表示内存位置的表达式，有以下几类：

本地变量
静态变量
解引用 (* express)
数组索引 (expr[expr])
字段引用 (expr.field)
位置表达式组合
通过位置表达式可以对某个数据单元的内存进行读写。位置表达式可以用于赋值。

值表达式
值表达式(Value Expression)一般叫做右值，值表达式引用了某个存储单元地址中的数据。它相当于数据，只能进行读操作。

从语义角度来说，位置表达式代表了持久性数据，值表达式代表了临时数据。位置表达式一般有持久的状态，值表达式要不是字面量，要不就是表达式求值过程中创建的临时值。
*/


#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn test_destructuring(){
        destructuring();
    }

    #[test]
    fn test_destructuring_car(){
        destructuring_car();
    }

    #[test]
    fn test_destructuring_un_sized(){
        destructuring_un_sized();
    }

}




fn main(){

}