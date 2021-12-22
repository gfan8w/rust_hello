
///关联类型与泛型参数一起使用, 这里还带了默认参数一起使用
pub trait Convert<T=i32> {   //泛型参数是可以指定默认类型的，在 trait 的定义中也不例外
    type Output;
    fn convert(&self) -> (Self::Output,T);
}


struct MyInt;

impl Convert for MyInt {    //因为使用了默认参数，这里不需要显示指定
    type Output = i32;

    fn convert(&self) -> (Self::Output, i32) {
        (42,42)
    }
}

impl Convert<f32> for MyInt {
    type Output = i32;

    fn convert(&self) -> (Self::Output, f32) {
        (52,52.4)
    }
}


//主入口函数
pub fn main() {

    let my_int = MyInt;

    let output: (i32,i32)= my_int.convert();  //这里必须指定 i32,否则模棱两可
    println!("{:?}",output);

    let output1: (i32,f32)= my_int.convert();
    println!("{:?}",output1);

    //可以看到，其实它们之间没有必然的关系，本身维度是分开的。
}





