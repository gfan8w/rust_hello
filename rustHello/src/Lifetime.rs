use std::collections::HashMap;



fn run_temporary_lifetime(){
    /* 这里没有定义一个临时变量来存放 "hello".to_string()，导致 hello的所有权被立刻释放，导致编译错误
        这个相当于是
        let y = {
            x =String::from("hello");
            &x
        }
        返回 &x是不允许的，x活的比y短。
    */
    //let r1:&String = "hello".to_string().borrow();
    //println!("r1: {:p}", &r1);

    let a =&String::from("hello");  //这个可以
    println!("a:{}",a);


    // 这个会报错：- temporary value is freed at the end of this statement
    // creates a temporary which is freed while still in use
    // note: consider using a `let` binding to create a longer lived value
    //
    // `hello`的所有权已经消失

    //let b =String::from("hello").as_str();
    //println!("b:{}",b);

}




fn life_time1(){
    let a =local_ref();

}

fn local_ref<'a>() -> &'a i32 {
    let v =1;
    //&v  //返回一个局部变量 ，这个是不允许的。因为 v 的生命周期只在该函数中，到外部函数life_time1中的变量a 无法引用到v。
    &1      //这个又是可以的，因为 1是常量，常量生命周期是'static
}


fn life_time2(){
    let mut data: Vec<&u32> = Vec::new(); // //堆上
    let a =1;       //栈上，

    data.push(&a); // 堆上，引用栈上的元素

    println!("v:{:?}",data)

    //这里看得出来，堆上的数据能引用 栈上的数据，这个在其他语言里是不可能的。这里 a 和data 在life_time2里的生命周期是一样的
    // 堆变量的生命周期不具备任意长短的灵活性，因为堆上内存的生死存亡，跟栈上的所有者牢牢绑定。
    // 而栈上内存的生命周期，又跟栈的生命周期相关，所以我们核心只需要关心调用栈的生命周期。堆上的数据跟着栈而变
}


fn life_time3(){
    let mut data: Vec<&u32> = Vec::new(); // //堆上
    local_vec_push(&mut data);

    println!("v:{:?}",data)

}

fn local_vec_push(data: &mut Vec<&u32>) {
    //let v = 1u32;
    //data.push(&v);

    data.push(&4)  //改为常量 编译通过，常量生命周期更久

}// v在函数结束后，就丢弃了。无法活的比 data更长，这是不允许的。





struct Number {
    value: u32,
}

fn number_value<'a>(num: &'a Number) -> &'a u32 {
    &num.value
}

#[derive(Debug)]
struct NumRef<'a> {
    val: &'a u32
}

fn as_num_ref(num: &u32) ->NumRef<'_> {  //省略 生命周期
    NumRef {val: &num}
}

impl<'a> NumRef<'a> {
    fn as_u32_ref(&'a self) -> &'a u32 {
        self.val
    }
}

//'static静态生命周期
#[derive(Debug)]
struct Person {
    name: &'static str
}

#[derive(Debug)]
struct Person1<'a> {
    name: &'a str
}


#[derive(Debug)]
struct Person2{
    name:String
}


pub fn run () {

    println!("LifeTime demo:");

    //通过这里的3个函数演示 生命周期，v必须活的比data更长久才可以。
    //这三段代码看似错综复杂，但如果抓住了一个核心要素“在一个作用域下，同一时刻，一个值只能有一个所有者”，你会发现，其实很简单。
    //情况 1 和情况 3 的代码无法编译通过了，因为它们引用了生命周期更短的值，而情况 2 的代码虽然在堆内存里引用栈内存，但生命周期是相同的，所以没有问题。
    life_time1();
    life_time2();
    life_time3();

    find_first_word();

    test_strtok();


    put_hashmap();


     let ref_x = {
         let x =42;
         //&x                 // x的生命周期不够长，代码块外部 引用不到
         1
     };
     println!("{}", ref_x);

    let n=Number{value:2};
    let v=number_value(&n); // number_value<'a>(num: &'a Number) -> &'a u32
    // `v` borrows `n` (immutably), thus: `v` cannot outlive `n`.
    // While `v` exists, `n` cannot be mutably borrowed, mutated, moved, etc.

    println!("{}",v);


    let i2=32 as u32;
    let nf=  NumRef {  // nf 无法活的比i2时间长  NumRef<'a> { val: &'a u32 }
        val: &i2
    };
    println!("{:?}", nf);

    let nf1=as_num_ref(&i2);  // as_num_ref(num: &u32) ->NumRef<'_>
    println!("{:?}", nf1);

    let n1=nf1.as_u32_ref();  // impl<'a> NumRef<'a> {  fn as_u32_ref(&'a self) -> &'a u32 { self.val } }
    println!("{}",n1);
    //n1 无法活的比i2长


    let p =Person{
        name:"fasterthanlime"  // 这个是常量，具有'static 的生命中周期
    };
    println!("{:?}",p);

    let pname = format!("fasterthan{}","lime");
    let p1 = Person{
        //name:&pname     //这里会报错， pname活的不够长久
        name: "a"
    };
    println!("{:?}",p1);

    //如何在 person 上存储一个非 static的 string，参看Person1的，1）指定一个'a 的生命周期，2）参看 Person2，获取String的所有权
    let p2 =Person1 {
        name:&pname
    }; //p2不会比pname活的更长
    println!("{:?}",p2);

    let p3 =Person2 {
        name:pname
    };
    // `name` was moved into `p3`, their lifetimes are no longer tied.
    println!("{:?}",p3);

    /// rust里的引用类型
    /// Strings: String is owned, &str is a reference
    /// Paths: PathBuf is owned, &Path is a reference
    /// Collections: Vec<T> is owned, &[T] is a reference



    let v = vec![1,2,3,4,5,6];
    let t = tail(&v);
    println!("{:?}",t);


    let yarr ={
        let x =&[1,2,3,4,5,6,7,8,9];  // vec![1, 2, 3, 4, 5] 会报错，因为 a vector is heap-allocated, and it has a non-'static lifetime
        &x[1..=3]   // 这是合法的，x必须是'static 数组
    };
    println!("{:?}",yarr);



    let v1 : Vec<i32>=(1..=10).collect();
    println!("{:?}",v1);
    assert_eq!(2,v1[1usize]);   // 通过下标来访问数组，越界会报错
    assert_eq!(Some(&3),v1.get(2)); // get 是安全的方式，不会panic，越界的时候，返回 None
    assert_eq!(None, v1.get(20));

    let file_name = "Read Me or don't.txt";
    //let file_ext=file_ext(&file_name);
    if let Some(file_ext) = file_ext(&file_name) {
        println!("{:?}", file_ext);
    }else {
        println!("no file ext");
    }

    let f_ext ={
        let f_name = String::from("Read Me or Don't.mp4");
        let f_et= file_ext(&f_name).unwrap_or("");
        f_et;
    };
    println!("{:?}",f_ext);

    print_longest();

    print_different_life_longest();

    compose_importExcerpt();

}



fn tail<'a>(arr: &'a[u32]) -> &'a[u32]{
    &arr[2..]
}


fn file_ext(file_name: &str) -> Option<&str> {
   let a= file_name.split(".").last();
    println!("file ext:{:?}",a);
    a
    // this does not create a new string - it returns
    // a slice of the argument.
}


fn print_longest(){
    let hello =String::from("hello");
    let jet ="jet";
    let long=longest(hello.as_str(),jet);
    println!("the longest str is :{}",long)



}

fn print_different_life_longest(){
    let long: &str;
    let string1 =String::from("hello");
    {
        let string2 =String::from("xyz");
        long=longest(&string1,&string2);

        println!("longest:{}",long); // 可以
    }

    //println!("longest:{}",long);
    // ^ 这句话 不会工作，因为rust知道 longest的参数生命周期一样，现在string2明显小于其他2个，要使得longest能工作，
    // 生命周期必须满足 三个参数 string1，string2，long里最小的那个。即满足string2 那个
}

/// rust 有个功能叫：borrow checker，如下的函数，编译器无法确定 返回的引用是 s1 还是 s2
/// 所以需要手工指定一个泛型的生命周期参数，生命周期参数不会改变参数的实际生命周期，它只是指示各个参数
/// 之间生命周期的相互关系,
/// ! 生命周期参数，描述的是参数和参数之间、参数和返回值之间的关系，并不改变原有的生命周期。
/// 生命周期参数 'a 放置的位置：
/// &i32
/// &'a i32
/// &'a mut i32
/// 单个参数上有一个生命周期参数是无意义的，生命周期是多个参数之间相互关联的，这时的生命周期才有意义
/// longest函数上的 引用类型的s1 和 引用类型的s2 和返回引用值 都有'a的生命周期，说明 这3个参数活的一样的长，都与泛型参数'a 活的一样长，
/// 活的多久是由最短命的那个生命决定的，必须跟最短命的一样长，
/// 三个生命周期必须满足三个参数中最小生命周期的那个。
/// 函数签名：有某个生命周期'a，s1的input lifetime生命周期是'a，s2的input lifetime生命周期是'a，返回值的output lifetime生命周期也是'a。
/// 当实际参数传入的时候，'a代表实际参数x,或y的生命周期的较小的一个
fn longest<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len()>s2.len() {
        return s1
    }
    s2
}


fn find_first_word(){
    let a_str ="hello world";
    let first = first_word(a_str);
    println!("first word:{}",first);
}

///虽然我们没有做任何生命周期的标注，但编译器会通过一些简单的规则为函数自动添加标注：
/// 1) 所有引用类型的参数都有独立的生命周期 'a 、'b 等。
/// 2) 如果只有一个引用型输入，它的生命周期会赋给所有输出。
/// 3) 如果有多个引用类型的参数，其中一个是 self，那么它的生命周期会赋给所有输出。
fn first_word(str: &str) -> &str {
    let trimmed = str.trim();
    let idx =trimmed.find(" ");
    let ret= match idx {
        Some(v) => &str[..v],
        _ => &"",
    };

    ret

}


fn test_strtok(){
    let  hello="hello world".to_string();
    let mut hello_str = hello.as_str();
    let str_rst = strtok(&mut hello_str,'r'); //返回一个新值， 改变原值，
    println!("sub string is:{}, left string is :{}, raw string is {}",str_rst,hello_str,hello);


}


/// 1） 函数签名： strtok(str: &mut &str, delimter: char) ->&str
///     会遇到编译错误。是因为按照编译器的规则， &mut &str 添加生命周期后变成 &'b mut &'a str，这将导致返回的 '&str 无法选择一个合适的生命周期
/// 2）函数签名改为： strtok<'b, 'a>(s: &'b mut &'a str, delimiter: char) -> &'a str
///    返回值和谁的生命周期有关？是指向字符串引用的可变引用 &mut ，还是字符串引用 &str 本身？
/// 3) 因为返回值的生命周期跟字符串引用有关，我们只为这部分的约束添加标注就可以了，剩下的标注交给编译器自动添加，
///    所以代码也可以简化成如下这样: strtok<'b>(str: &mut &'b str, delimter: char) ->&'b str
/// 4）如果把生命周期换位置，函数签名变为：
///    strtok<'b>(str: &'b mut &str, delimter: char) ->&'b str
///    因为现在我们把对字符串的可变引用的生命周期和返回值的生命周期关联起来了，
///    str_rst 得到了 strtok 的返回值，所以从编译器看来，&mut hello_str 的生命周期并没有结束，所以发生了 &mut 和 & 同时存在的局面
///    在print的地方 &hello_str 是 immutable borrow 发生了，这个是不允许的
///
/// 生命周期标注的目的是，在参数和返回值之间建立联系或者约束。调用函数时，传入的参数的生命周期需要大于等于（outlive）标注的生命周期。
/// 当每个函数都添加好生命周期标注后，编译器，就可以从函数调用的上下文中分析出，在传参时，引用的生命周期，是否和函数签名中要求的生命周期匹配。
/// 如果不匹配，就违背了“引用的生命周期不能超出值的生命周期”，编译器就会报错。
fn strtok<'b>(str: &mut &'b str, delimter: char) ->&'b str {
    let len=delimter.len_utf8();
    let trimmed = str.trim();
    match trimmed.find(delimter) {
        Some(idx) => {
            let llen =idx + len;
            let prefix =&(*str)[..idx];  // str 是 &mut &str 类型，那个 (*str) 解一次引用后是 &str
            // 由于 delimiter 可以是 utf8，所以我们需要获得其 utf8 长度，
            // 直接使用 len 返回的是字节长度，会有问题
            *str = &(*str)[llen..];
            prefix
        },
        _ => {
            // 如果没找到，返回整个字符串，把原字符串指针 s 指向空串
            let prefix =&(*str)[..];
            *str =&"";
            prefix
        }
    }
    //&""


}


fn put_hashmap(){

    let mut hm = HashMap::new();
    hm.insert("key","value");
    let key ="key3";

    // 按照之前的说法，这段代码无法编译通过，因为同一个 scope 下不能有两个可变引用
    // 但因为 RFC2094 non-lexical lifetimes，Rust 编译器可以处理这个场景，
    // 因为当 None 时，map.get_mut() 的引用实际已经结束
    match hm.get_mut(key) { /* <----- 可变引用的生命周期一直持续到 match 结果 */
        Some(v) => {
            println!("hash value:{}",v)
        },
        _ => {
            hm.insert(key,"value3");   // <--- 这里又获得了一个可变引用
        }
    }

}


/// 数据结构的生命周期标注也是类似。比如下面的例子，Employee 的 name 和 title 是两个字符串引用，
/// let e= Employee{..} , e的生命周期不能大于 外部拥有owner的name变量的生命周期，e也不能大于外部拥有woner的title变量的生命周期，否则会访问失效的内存
struct Employee<'a,'b>{
    name: &'a str,
    title: &'b str,
    age: u32
}


///结构体成员可以包含引用，但需要注明生命周期，这里的意思是 ImportExcerpt的实体不可能活的比name对应的那个字符串更长。
struct ImportExcerpt<'a> {
    name: &'a str,
}

///这里实现一个方法（类、结构体的成员函数叫方法），在impl后必须加上生命周期参数，在结构体名字后面也必须加上生命周期参数
/// 方法上的生命周，根据rust的生命周期推断，无需添加
impl<'a> ImportExcerpt<'a> {
    fn level(&self) ->i32{
        3
    }
    fn announcement_and_apart(&self, announcement: &str) ->&str {
        println!("Attention please: {}", announcement);
        self.name
    }
}

fn compose_importExcerpt(){
    let novel = String::from("Call me Ishmael. Some years ago...");
    let sp=novel.split(".").next().expect("not found a .");
    let ie=ImportExcerpt{
        name: sp
    };

    println!("{} level:{}",ie.name,ie.level())

}





#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_run_temporary_lifetime() {
        run_temporary_lifetime();
    }
}










