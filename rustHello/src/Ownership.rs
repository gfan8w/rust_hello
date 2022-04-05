
/// 来源：https://doc.rust-lang.org/stable/book/ch04-01-what-is-ownership.html
///      https://doc.rust-lang.org/stable/book/ch10-03-lifetime-syntax.html
/// 在一个作用域内，仅允许一个活跃的可变引用。所谓活跃，就是真正被使用来修改数据的可变引用，
/// 如果只是定义了，却没有使用或者当作只读引用使用，不算活跃。
///
/// 在一个作用域内，活跃的可变引用（写）和只读引用（读）是互斥的，不能同时存在。
///
/// Borrow 语义：
/// 允许一个值的所有权，在不发生转移的情况下，被其它上下文使用。
/// Borrow 语义通过引用语法（& 或者 &mut）来实现
/// 对值的borrow有约束，这个约束是：借用不能超过（outlive）值的生存期。
///
/// 在其他语言中，引用是一种别名，你可以简单理解成鲁迅之于周树人，
/// 多个引用拥有对值的无差别的访问权限，本质上是共享了所有权；
///
/// 而在 Rust 下，所有的引用都只是借用了“临时使用权”，它并不破坏值的单一所有权约束。
/// 但 Rust 没有传引用的概念，Rust 所有的参数传递都是传值，不管是 Copy 还是 Move。
/// 所以在 Rust 中，你必须显式地把某个数据的引用，传给另一个函数。
///
/// Rust 的引用实现了 Copy trait，所以按照 Copy 语义，这个引用会被复制一份交给要调用的函数。
/// 对这个函数来说，它并不拥有数据本身，数据只是临时借给它使用，所有权还在原来的拥有者那里。（可以参见 pointer_stack.rs 的示例）
pub fn run () {
    println!("OwnerShip demo:");


    let hello ="hello".to_string();
    let (greeting,size) = takes_and_gives_back(hello);  // hello的所有权移动到函数内，函数又返回该所有权，并返回一个i32的所有权
    println!("greeting:{}, size:{}",greeting,size);

    let world=give_ownership();

    let size1=calculate_length(&world);
    let size2 =calculate_length2(&world);

    println!("{},{}",size1, size2);

    let dangling_str=dangling();

    // 多个引用，后不能跟可变引用
    multi_reference();
    // 多个可变引用 也是 不可以的
    multi_mut_reference();
    // 查找第一个单词，练习slice，切片
    find_first_words();

    modify_in_loop();

    modify_vec_twice_not_allow();

    //演示 i32的vec 和 String的vec的区别
    demo_modify_vec();

    //Some里的值的所有权会移动
    some_inner_value_moved();
}

// 该函数获取了一个string和它的所有权，最后又返回一个string和它的所有权，并返回一个i32的所有权给调用者
fn takes_and_gives_back(str: String) ->(String,usize) {
    let len=str.len();
    (str,len)
}

fn give_ownership() -> String {                 // gives_ownership will move its
    let s=String::from("world");      // return value into the function
                                                // that calls it

     s                                          // s is returned and
                                                // moves out to the calling
                                                // function
}

fn calculate_length(s: &String) -> usize {      //s is a reference to a String
    s.len()
}// Here, s goes out of scope. But because it does not have ownership of what
// it refers to, nothing happens.

fn calculate_length2(s: &str) -> usize {
    s.len()
}

fn dangling() -> &'static str {   //这个能正常运行
    let s= "hello";
    s
}

/*
fn dangling_2() -> &String {          //这个不会运行，悬空指针，返回一个局部变量的引用
                                    // dangle returns a reference to a String
    let s= String::from("hello");  // s is a new String
    &s                              // we return a reference to the String, s
}// Here, s goes out of scope, and is dropped. Its memory goes away.
// Danger!
// The solution here is to return the String directly:
fn no_dangle() -> String {
    let s = String::from("hello");

    s
}
*/


fn multi_reference() {
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2);
    // variables r1 and r2 will not be used after this point

    let r3 = &mut s; // no problem
    println!("{}", r3); //ok

    /* 有问题的代码： 会引起数据竞争（Data races ）的都不允许
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    let r3 = &mut s; // BIG PROBLEM

    println!("{}, {}, and {}", r1, r2, r3);
    //这句话会报错，多个引用，还有一个可变引用是不行的。r3如果改变数据，r1、r2 的只读不可知，

    */
}

fn multi_mut_reference(){
    let mut s =String::from("hello");

    {   //增加一层作用域隔离，是可以 有多个 可变引用的，更多可以参见： RefCell_sapmle.rs 里的代码
        let r1 =&mut s;
    }// r1 goes out of scope here, so we can make a new reference with no problems.

    let r2 =&mut s;


    /* 有问题的代码：
    let mut s = String::from("hello");

    let r1 = &mut s;
    let r2 = &mut s;    //这里有问题，会引起数据竞争(Data races)的都有问题

    println!("{}, {}", r1, r2);
    */
}

fn find_first_words(){
    let mut s= String::from("hello world Beijing I love you");
    let first =first_words(&s); //这里是一个不可变引用，不可变引用被first hold住了，
    //s.clear();                        // 这里是一个可变引用。
    println!("first word is:{}",first); //这里继续使用那个不可变引用，但是前面有个可变引用导致数据竞争后不可控，故报错
    //要正确运行 ^ ,注释掉 s.clear(); 或 println!这两行里的任意一行，为什么呢？因为 不可变引用与可变引用不能同时存在
    // 我这里临时注释掉 s.clear();
}


///查找一段字符串中的第一个单词，以空格为标记，如果没有空格，就返回整个字符串作为一个单词
fn first_words(s: &String) ->&str {  // fn first_words(s: &str) ->&str  -> 这里参数使用 &str会更通用
     let bytes = s.chars().into_iter().enumerate();
    for (pos,ch) in bytes {
        if ch==' ' {
           return &s[..pos]
        }
    }

    &s[..]

}

/// 它破坏了循环的不变性（loop invariant）
fn modify_in_loop(){
    let mut data =vec![1,2,3,4];
    for item in data.iter_mut() {  //data 这里已经borrow了，它是一个borrowed的可变引用
        //data.push(*item+1)         //这里会有问题，因为又一次使用了可变引用。不允许2次使用可变引用
    }
}


///这段代码是有问题的。在data push的时候，如果要扩张容量，data会重新分配，
/// data1指向的是data的第一个元素，如果data重新分配内存会在一个新内存，data1的指向会失效。
/// 但rust编译预先有约定，可变引用之后不能再有引用
/// 具体例子还可以参见：vec_reallocate_immutable_address_change.rs
fn modify_vec_twice_not_allow(){
    let mut data =vec![1,2,3,4];
    let data1 =vec![&data[0]]; //不可变引用（borrow）, borrow跟引用是一个意思。
    println!("data[0]: {:p}",&data[0]);

    for i in 1..10{
        data.push(i);  // 可变引用（borrow）
    }
                                                    // data1 如果不使用，已经等于死亡，不可变引用无效了。

    println!("data[0]: {:p}", &data[0]);
    //println!("boxed: {:p}", &data1);   // 使用了不可变引用，这里会报错是不允许的。编译器 不允许 可变引用和 不可变引用同时存在
}


fn demo_modify_vec(){
    modify_vec_i32_raw();
    modify_vec_i32_ok();
    modify_vec_String();
}

///把最后一行注释取消掉，运行，看是否会报错。这个代码因为存在最后一个 last是活跃的不可变引用，导致编译失败，修改的方法可参见 modify_vec_i32_ok
fn modify_vec_i32_raw(){
    let mut arr = vec![1,2,3,4];
    let last =arr.last(); // 活跃的不可变引用，arr is borrowed
    arr.push(5); // 可变引用

    //println!("last is {:?}",last); // 在这里又使用了不可变引用，所以才说它是活跃的
}

/// 接 modify_vec_i32_raw，要使得编译通过，
/// 1）可以把最后一行的println 移动到 push之前，在可变引用之前，让不可变引用都使用完毕后，再使用可变引用
/// 2）其他方法：拿到last后，clone一个last_c,后面都使用last_c.
/// 3) 使用 数组下标取值，实质是发生了move，但i32实现了copy trait，就执行clone语义，拿到一个 l ，后续使用l，l是个clone 出来的值。
/// 4）使用最后一个值，解引用，实质是发生了move，其实也是使用了i32的clone，得到一个lll,后续使用lll
fn modify_vec_i32_ok(){
    let mut arr = vec![1,2,3,4];
    let last =arr.last();
    let last_c =last.cloned();
    let l=arr[arr.len()-1];
    let lll=*arr.last().unwrap();
    arr.push(5);

    println!("last is {:?}",last_c);
    println!("last is {:?}",l);
    println!("last is {:?}",lll);
}

// 接上面的 modify_vec_i32_ok，因为 i32实现了clone，我们把 vec元素类型换位String，即换为一个默认没有实现Copy语音的类型看看
fn modify_vec_String(){
    let mut arr = vec!["1".to_string(),"2".to_string(),"3".to_string(),"4".to_string()];
    let last =arr.last();
    let last_c =last.cloned();
    let l=arr[arr.len()-1].to_string();   //这里的to_String() 相当于重新生成一个String对象。如果把to_String() 去掉，看看报错，本质上跟i32的clone一样的。必须有一个新对象生成
    let lll=arr.last().unwrap().to_string(); //同上，如果不想要有过多的内存分配，只有把 push放到println 后面。即让活跃的不可变引用都失去活跃，最后只剩一个可变引用
    arr.push("5".to_string());

    println!("last is {:?}",last_c);
    println!("last is {:?}",l);
    println!("last is {:?}",lll);
}



/*
borrow、move、copy 3个语义要清晰
1)一个值在同一时刻只有一个所有者。当所有者离开作用域，其拥有的值会被丢弃。
  赋值或者传参会导致值 Move，所有权被转移，一旦所有权转移，之前的变量就不能访问。
2) 如果值实现了 Copy trait，那么赋值或传参会使用 Copy 语义，相应的值会被按位拷贝，产生新的值。
3) 一个值 可以 以传递地址的方式传参给其他函数，这个叫引用 或 borrow。
4） 一个值可以有多个只读引用。
5) 一个值可以有唯一一个活跃的可变引用。可变引用（写）和只读引用（读）是互斥的关系，就像并发下数据的读写互斥那样。可变与不可变借用不能相互交缠
6) 引用的生命周期不能超出值的生命周期。
*/

///Some做match时，里面的值是移动走的，如果不移动，要加`ref`，
/// 如果修改Some里面值，所有权还在Some里面？
fn some_inner_value_moved() {
    let x = Some(String::from("hello"));
    match x {
        Some(ref a) => {println!("match a: {}",a)}, //永远匹配到这个arm， 要加条件： if a.len()>10，才会匹配到 b； a和b这里是变量定义
        Some(ref b) =>{println!("match b: {}",b)},
        None => {println!("match none")},
    }


    let mut x = Some(String::from("world"));
    if let Some(_) = x {  // 如果把 `_` 替换为 _s  就会发生所有权的移动，x后续无法使用，但如果使用 `_` 就不会发生所有权移动
        println!("find a string")
    }

    if let Some(_) =  x {  // 如果把 `_` 替换为 _s  就会发生所有权的移动，x后续无法使用，但如果使用 `_` 就不会发生所有权移动
        println!("find a string")
    }

    if let Some(c) =  &mut x {   //修改Some内部的值，原来的变量还存在
        *c= String::from("good");
        println!("find a string")
    }

    println!("modified x in some is :{:?}",x);
}





