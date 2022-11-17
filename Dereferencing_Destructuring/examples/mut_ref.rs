
// 参考： https://www.reddit.com/r/rust/comments/thvc2e/comment/i1a8zpw/?utm_source=share&utm_medium=web2x&context=3

fn modify(arg: &String) {
    let mut arg: String = *arg;
    //┗━ 这句话在这里，尝试 通过 *arg 解引用，把 &String转变为String，然后移出所有权，将值移动到一个新建的同名变量arg。
    // 这里有2点原因 不允许： 1） 只能移动你拥有的数据，拥有所有权的数据才能被你移动，你不能移动一个借用的数据。
    //                    2)  arg只是一个不可变引用&String，你无法将它移动到一个要改变这个数据的变量上。无法从 arg 移动到 mut arg。
    //                    3)  假设数据能执行到下面一行，则你改变的也仅仅是局部变量arg，而不是你传入的那个arg。改变的是内部新建的那个arg。
    arg.push_str("smth");

    /*
    猜想，你想实现的是这样一个版本：入参是 &mut，内部是 arg直接push
    fn modify(arg: &mut String) {
        arg.push_str("smth");
    }

    移动所有权是 = 做的事情， *arg 只是做解引用。 = 在其他编程语言里的赋值都是复制(Copy)操作。从右边的内存处 复制数据到 左边
    但Rust 有点特殊，在没有实现Copy trait的情况下，它做的是移动操作。 从右边的内存处移出数据，放入左边处
    这个 . 操作符（the . operator） ，在 Rust 中会自动对引用类型进行 解引用。
    */
}

/*
p: &T          p是一个引用，第一次给p赋值后，不能改变p指向其他数据，也不能改变当前p指向的数据的内容。
              means that p cannot refer to other data, and you cannot change the data that p refers to

mut p: &T      p是一个可变引用， 第一次给p赋值后，还可以修改p指向其他数据，但不能改变p指向的数据的内容。
               means that you can change p to refer to other data, but not change the data that p refers to

p: &mut T      p是一个引用，指向的内容可变，第一次给p赋值后，不能改变p指向其他数据， 但可以需改p指向的数据的内容
               means that you cannot p to refer to other data, but you can change the data which p refers to.

mut p: &mut T  p是一个可变引用， 指向的内容可变，第一次给p赋值后，还可以修改p指向其他数据，能改变p指向的数据的内容。
               means that you can both change which data p refers to, as well and changing the data which p is currently referring to.

*/

#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn test_p_nono_mut(){
        let a = 1;
        let b = 2;
        let mut p = &a;  // p可变，后面还能给p赋值 p指向a
        dbg!(a); // a = 1
        dbg!(b); // b = 2
        dbg!(p); // p = 1
        p = &b;             // p指向b
        dbg!(a); // a = 1
        dbg!(b); // b = 2
        dbg!(p); // p = 2
    }

    #[test]
    fn test_p_mut(){
        let mut a = 1;
        dbg!(a);
        let p = &mut a;
        //dbg!(p);  //dbg! 会拿走所有权

        *p= 2;              // p 是 &mut ，能修改它指向的数据的内容， 这里相当于说 *p 是可变的，

        dbg!(p); // p = 2
        dbg!(a); // a = 2
    }

    #[test]
    fn test_destructuring_un_sized(){

    }

}





fn main(){}