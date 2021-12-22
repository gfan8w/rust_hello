
pub trait Watch<Inner = String>{
    type Item;
    fn inner(&self) -> Option<Self::Item>;
    fn info(&self) -> Inner;
}

struct A {
    data: i32,
}

impl Watch<i32> for A {
    type Item = i32;

    fn inner(&self) -> Option<Self::Item> {
        Some(self.data)
    }

    fn info(&self) -> i32 {
        self.data
    }
}

#[derive(Debug)]
struct B {
    data: String
}


impl Watch for B {
    type Item = u64;

    fn inner(&self) -> Option<Self::Item> {
        let i=self.data.parse::<u64>().ok()?;
        Some(i)
    }

    fn info(&self) -> String {
        self.data.clone()
    }
}





//主入口函数
pub fn main() {

    let a = A{data:10};

    assert_eq!(a.info(),10);
    assert_eq!(a.inner(),Some(10));

    let b= B{data:String::from("12")};

    assert_eq!(b.info(),"12");
    assert_eq!(b.inner(),Some(12));
    println!("{}",b.info());
    println!("{:?}",b.inner());

    let c= B{data:String::from("abc")};

    println!("{}",c.info());
    println!("{:?}",c.inner())
    //assert_eq!(c.info(),"abc");
    //assert_eq!(c.inner(),Some(12));

}





