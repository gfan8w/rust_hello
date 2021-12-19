
///花式玩法：关联类型、泛型参数、默认参数、Self 一起使用
pub trait Convert<T=Self> {   //泛型参数是可以指定默认类型的，在 trait 的定义中也不例外
    type Output;
    fn convert(&self) -> (Self::Output,T);
}

#[derive(Debug, Copy, Clone)]
struct MyInt(i32);

impl Convert for MyInt {    //因为使用了默认参数，这里不需要显示指定
    type Output = Self;

    fn convert(&self) -> (Self::Output, Self) {
        (*self, *self)
    }
}

impl Convert<f32> for MyInt {
    type Output = Self;

    fn convert(&self) -> (Self::Output, f32) {
        (*self,52.4)
    }
}


//主入口函数
pub fn main() {

    let my_int = MyInt(42);

    let output: (MyInt,MyInt)= my_int.convert();  //这里必须指定 i32,否则模棱两可
    println!("{:?}",output);

    let output1: (MyInt,f32)= my_int.convert();
    println!("{:?}",output1);

    //可以看到，其实它们之间没有必然的关系，本身维度是分开的。
}





