
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

    println!("bob==alice:{}",bob==alice)
}









