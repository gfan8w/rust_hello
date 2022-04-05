

// & vs. ref 在  Rust 模式匹配中的区别
//http://xion.io/post/code/rust-patterns-ref.html

#[warn(non_snake_case)]
#[warn(unused_variables)]
fn deRef(){
    let query_params: Vec<(String, String)> =vec![
        ("1".to_string(),"2".to_string()),
        ("3".to_string(),"4".to_string()),
        ("5".to_string(),"6".to_string()),
    ];

    for (name,value) in &query_params {  // 这里用了 &query_params 则，所有权没有移动，迭代的引用，如果用 query_params， 则所有权移动到for循环里。后续无法使用 query_params
        println!("name:{}, value:{}", name, value);
    }

    println!("{:?},",query_params);

}


#[warn(unused_variables)]
fn ref_deRef(){
    let query_params: Vec<(String, String)> =vec![
        ("1".to_string(),"2".to_string()),
        ("3".to_string(),"4".to_string()),
        ("5".to_string(),"6".to_string()),
    ];

    // for前面 加 个 & 后，name，value 的类型变为 String，因为&query_params是引用，所以 for 内变量都是引用，
    // name，value无法变为是有所有权类型的String，在前面加上 ref变为引用才可以。
    // & 是模式匹配的一部分，而 ref 不是，ref 只是说 类型是引用
    for &(ref name, ref value) in &query_params {
        println!("name:{}, value:{}", name, value);
    }

    println!("{:?},",query_params);

}

#[derive(Debug)]
struct Foo(String);

#[derive(Debug)]
struct Goo(String);


fn match_foo(){
    let fo: Result<Foo,String> = Ok(Foo("hello".into()));
    let mut foo = fo;

    let mut s =String::from("hell");
    s=s+"oo";
    let goo =&Foo(s);
    // match foo {
    //     &Foo(ref s) => println!("Matched with string: {}", s),
    //     x => println!("Matched!"),
    // }

    if let &mut Ok(Foo( ref mut c)) =&mut foo {
        println!("str is :{}",c);
        *c+=" world";
    }

    println!("foo:{:?}", foo);
}










pub fn main(){
    deRef();
    ref_deRef();
    match_foo();
}