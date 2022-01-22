use std::mem::{align_of, size_of};

#[derive(Debug)]
struct Site{
    name:String,
    domain: String,
    nation:String,
    found:u32
}

#[derive(Debug)]
struct Person {
    name: String,
    age: u32
}


impl PartialEq for Person {
    /*
        这里我们自己实现eq方法，只需要age相等即为相等。
        只需要实现eq，rust会自动帮我们实现ne。
    */
    fn eq(&self, other: &Self) -> bool {
        self.name==other.name
    }
}



pub fn run(){

    let runoob = Site{
        name:String::from("Lin"),
        domain: String::from("Tai Wan"),
        nation: String::from("China"),
        found:23
    };
    println!("{:#?}", runoob);

    let bob = Person {
        name:String::from("Bob"),
        age: 12
    };

    let alice = Person{
        name: String::from("Bob"),
        age: 12
    };

    // assert_eq 必须实现 PartialEq
    assert_eq!(bob,alice);

    println!("bob==alice:{}",bob==alice);

    //查看结构体的对其
    size_of_struct();


}


struct S1 {
    a: u8,
    b: u16,
    c: u8,
}

struct S2 {
    a: u8,
    b: u8,
    c: u16,
}


fn size_of_struct(){
    println!("size_of_struct S1:{}, S2:{}", size_of::<S1>(),size_of::<S2>());
    println!("align_of_struct S1:{}, S2:{}", align_of::<S1>(),align_of::<S2>());
}

