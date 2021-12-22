

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




     let ref_x = {
         let x =42;
         //&x                 // x的生命周期不够长，代码块外部 引用不到
         1
     };
     println!("{}", ref_x);

    let n=Number{value:2};
    let v=number_value(&n);
    // `v` borrows `n` (immutably), thus: `v` cannot outlive `n`.
    // While `v` exists, `n` cannot be mutably borrowed, mutated, moved, etc.

    println!("{}",v);


    let i2=32 as u32;
    let nf=  NumRef {  // nf 无法活的比i2时间长
        val: &i2
    };
    println!("{:?}", nf);

    let nf1=as_num_ref(&i2);
    println!("{:?}", nf1);

    let n1=nf1.as_u32_ref();
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

    //如何在person上存储一个非 static的 string，参看Person1的，1）指定一个'a 的生命周期，2）参看 Person2，获取String的所有权
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
/// 之间生命周期的相互关系
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















