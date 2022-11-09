pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

// https://folyd.com/blog/rust-pin-unpin/

#[derive(Debug)]
pub struct Test<'a>{
    a: String,
    b: &'a str,
}


pub fn run(){
    let a = String::from("hello");
    //let _t = Test {a:a, b:&a}; //编译无法通过，a已经move 进 Test.a里去了，无法借用给 Test.b
}


///
/// 没办法通过Safe Rust构造一个像Test这样的自引用结构体，Rust目前对自引用结构体支持还很不完善。只能变通一下使用指针
#[derive(Debug)]
pub struct TestA{
    a: String,
    b: *const String,
}

impl TestA {
    pub fn new(str: &str) -> Self {
        TestA{
            a: str.to_owned(),
            b: std::ptr::null()
        }
    }

    pub fn init(&mut self) {
        let ref_a: *const String = &self.a;
        self.b = ref_a;
    }

    pub fn a(&self) -> &str{
        &self.a
    }

    pub  fn b(&self) -> &String{
        unsafe {&*(self.b) }
    }
}


pub fn test(){
    let mut test1 = TestA::new("test1");
    test1.init();

    let mut test2 = TestA::new("test2");
    test2.init();

    println!("test1 a: {}, test1 b: {}", test1.a(), test1.b());
    println!("test2 a: {}, test2 b: {}", test2.a(), test2.b());

    std::mem::swap(&mut test1, &mut test2);

    test1.a = "I've totally changed now!".to_string();

    println!("test1 a: {}, test1 b: {}", test1.a(), test1.b());
    println!("test2 a: {}, test2 b: {}", test2.a(), test2.b());

}













