use std::f64::consts::PI;

///trait 和实现 trait 的数据类型，至少有一个是在当前 crate 中定义的，
/// 也就是说，你不能为第三方的类型实现第三方的 trait，当你尝试这么做时，Rust 编译器会报错
pub trait CalculateArea{
    fn calc(&self) ->f64;
}

///非pub，外部不可见
struct Triangle{
    bottom: u32,
    height: u32
}

struct Square {
    length: u32
}

struct Cycle{
    radius: u32
}


impl CalculateArea for Triangle {
    // 函数参数 是self，还是 &self的区别：
    // 1）如果参数是self，说明该函数会消耗self对象，self在该函数调用后，就移动不存在了
    // 2) 如果是 &self，则不会发生
    fn calc(&self) -> f64 {
        return (self.bottom * self.height) as f64;
    }
}

impl CalculateArea for Square{
    fn calc(&self) -> f64 {
        return (self.length * self.length) as f64;
    }
}

impl CalculateArea for Cycle {
    fn calc(&self) -> f64 {
        return PI* ((self.radius*self.radius) as f64);
    }
}


pub fn main() {

    let tri= Triangle{height:34,bottom:21};
    let tri_area=tri.calc();
    println!("triangle area is {}", tri_area);

    let sq=Square{ length:20};
    let sq_are=sq.calc();
    println!("square area is {}", sq_are);

    let cy=Cycle{ radius:3};
    let cy_area=cy.calc();
    println!("cycle area is {}", cy_area);

}





