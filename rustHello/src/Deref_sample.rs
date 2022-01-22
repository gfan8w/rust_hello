use std::ops::Deref;

struct MyBox<T>(T);

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0    //这里返回的是引用，我们不要owner权限。
    }
}

///演示 Deref
pub fn run(){

    println!("demo auto Deref");


    let hello =MyBox(String::from("hello"));

    show_deref(&hello); // 会自动调用deref，1）hello 是 Mybox<String>类型的，  deref操作 把 &MyBox<String> 变为 &String，标准库的String实现了deref，把&String变为 &str
    show_deref( &hello.0[..]);  //如果没有实现 `Deref` 这个trait，需要这样传参，直接获取到内部String，再转换为&str
    show_deref( &(*hello)[..])  // 这里的*hello 其实也是调了Deref，从 &MyBox<String> 变为 &String
}


fn show_deref(str: &str){
    println!("show deref str:{}",str)
}

/*
Rust 的deref的约束规则：
From &T to &U when T: Deref<Target=U>
From &mut T to &mut U when T: DerefMut<Target=U>
From &mut T to &U when T: Deref<Target=U>

从 &T     到 &U     当 T 实现 Deref<Target=U>
从 &mut T 到 &mut U 当 T 实现 DerefMut<Target=U>
从 &mut T 到 &U     当 T 实现 Deref<Target=U>

1）case 1： 当你有一个 &T 数据，如果 `T` 类型实现了`Deref` ，可转换为 `U` 类型，那你可以透明（transparently）的 获取到 &U 数据，
2）case 2： 区别于1）只是 这里是 mut 可变类型
3）case 3： 当你有一个 可变的 &mut T，如果`T`类型实现了`Deref`，可以转换为`U`类型，那你可以转变为一个 不可变的 &U。
           无法从一个不可变的 &T 转换为一个 &mut U。 必须遵循borrowing 规则。（其实如果只有1个不可变的 &T 引用，是可以转换为 可变的 &mut U的，但编译器无法保证 ）


*/
























