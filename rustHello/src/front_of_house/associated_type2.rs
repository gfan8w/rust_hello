
///trait 中的泛型参数
/// 其实使用泛型也可以做到类似的效果。如果不使用关联类型，
/// trait 可以这样定义（示意）：
pub trait Convert<T=i32> {   //泛型参数是可以指定默认类型的，在 trait 的定义中也不例外
    fn convert(&self) -> T;
}


struct MyInt;

impl Convert for MyInt {        //因为使用了默认参数，这里不需要显示指定
    fn convert(&self) -> i32 {
        42
    }
}

impl Convert<f32> for MyInt {
    fn convert(&self) -> f32 {
        432.9
    }
}


//主入口函数
pub fn main() {

    let my_int = MyInt;

    // Error: could not use turbofish syntax here
    // let output = my_int.convert::<i32>();
    let output: i32= my_int.convert();  //这里必须指定 i32,否则模棱两可
    println!("{}",output);

    let output1: f32= my_int.convert();
    println!("{}",output1);

}





