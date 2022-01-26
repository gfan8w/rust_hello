use std::ops::Add;

//我们来定义一个复数类型，尝试使用下这个 add trait
#[derive(Debug)]
struct Complex {
    real: f64,
    imagine: f64,
}


impl Complex {
    fn new(real: f64, imagine: f64) -> Self {
        Self {
            real,
            imagine
        }
    }
}


impl Add for Complex {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut a =Complex{ real: 0.0, imagine: 0.0 };
        a.real =self.real+rhs.real;
        a.imagine =self.imagine+rhs.imagine;
        return a;
    }
}

/// 如果不想移动所有权，可以为 &Complex 实现 add，这样可以做 &c1 + &c2
impl Add for &Complex {
    // 注意返回值不应该是 Self 了，因为此时 Self 是 &Complex
    type Output = Complex;

    fn add(self, rhs: Self) -> Self::Output {
        let mut a =Complex{ real: 0.0, imagine: 0.0 };
        a.real =self.real+rhs.real;
        a.imagine =self.imagine+rhs.imagine;
        return a;
    }
}

///之前的例子，Add trait的泛型参数都是默认的，这里改为f64，实现复数的 实部相加，虚部 不变。
impl Add<f64> for &Complex {
    type Output = Complex;

    fn add(self, rhs: f64) -> Self::Output {
        let real =self.real+rhs;
        Complex::new(real, self.imagine)
    }
}





/// 特设多态是同一种行为的不同实现。所以其实，通过定义 trait 以及为不同的类型实现这个 trait，我们就已经实现了特设多态。
/// Add trait 就是一个典型的特设多态，同样是加法操作，根据操作数据的不同进行不同的处理。
fn main() {

    let a =Complex{ real: 1.0, imagine: 2.0 };
    let b =Complex{ real: 3.0, imagine: 5.0 };
    let c = a+b;
    println!("complex:{:?}",c);
    // println!("{:?}",a); // 这里无法使用a，a 在做 +的时候移动走了。 给 `&Complex` 实现add

    // 给 `&Complex` 实现add
    let a1 =&Complex{ real: 1.0, imagine: 2.0 };
    let b1 =&Complex{ real: 3.0, imagine: 5.0 };
    let c = a1 + b1;
    println!("complex:{:?}",c);
    println!("{:?}",a1);

    // 给 `&Complex` 实现add
    let a1 =Complex{ real: 1.0, imagine: 2.0 };
    let b1 =Complex{ real: 3.0, imagine: 5.0 };
    let c = &a1 + &b1;
    println!("complex:{:?}",c);
    println!("{:?}",a1);

    //实现一个复数 跟普通数 相加
    let d =&a1+64.0;
    println!("complex d:{:?}",d)

}