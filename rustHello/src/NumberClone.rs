


struct Number {
    odd:bool,
    value:i32
}

pub fn runNumber(){
    let n=Number{odd:true,value:51};
    print_number(n); //`n` is moved
    //print_number(n); // error: use of moved value: `n`

    let n1=Number{odd:true,value:53};
    print_number_reference(&n1);
    print_number_reference(&n1); //引用是可以的

    //invert(&mut n1); //这样是不行的，n1 不可变

    let mut n2=Number{odd:true,value:53};
    print_number_reference(&n2);
    invert(&mut n2);        //这样是可以的，可变的引用，只能作用在可变的变量上
    print_number_reference(&n2); //引用是可以的


    //
    let  n3=Number{odd:true,value:103};
    print_number_reference(&n3); //引用，是borrow，不是move
    let mut n4=n3.clone();
    n4.value+=3;
    print_number(n3);
    print_number(n4);
    //print_number(n4); //n4在之前已经move了，这里不能再访问



    let  n5=Number{odd:true,value:113};
    let mut n6 =std::clone::Clone::clone(&n5); //等同于 let n6 = n5.clone();
    print_number(n5);

    n6.value+=5;
    print_number(n6);



    //
    let  n7=Number{odd:true,value:127};
    print_number(n7);
    let mut n8=n7.clone();
    n8.value+=3;
    print_number(n7); //这里发生了隐士的copy
    print_number(n8);


    let  n9=Number{odd:true,value:139};

    //在Number实现了 copy这个 Marker 特性后，可以工作
    let mut n10=n9; // `n10` is a copy of `n9`
    let n11 = n9;   //`n9` is neither moved nor borrowed.
    print_number( n11)

}

fn print_number(n:Number){
    println!("{} number {}", if n.odd {"odd"} else {"even"}, n.value);
}

fn print_number_reference(n:&Number){
    println!("{} number {}", if n.odd {"odd"} else {"even"}, n.value);
}

fn invert(n: &mut Number){
    n.value=-n.value;
}


impl std::clone::Clone for Number {
    fn clone(&self) -> Self {
        Self{..*self} //剩余字段自动匹配完成
    }
}

// Copy trait 和 Drop trait 是互斥的，两者不能共存，当你尝试为同一种数据类型实现 Copy 时，也实现 Drop，
// 编译器就会报错。这其实很好理解：Copy 是按位做浅拷贝，那么它会默认拷贝的数据没有需要释放的资源；
// 而 Drop 恰恰是为了释放额外的资源而生的。
//Marker 特性没有方法，只是告诉运行时，某个类型具有某种特质
impl std::marker::Copy for Number {
    
}

//使用derive 自动实现 Clone，Copy 特性
#[derive(Clone, Copy)]
struct Number2 {
    odd: bool,
    value: i32,
}



