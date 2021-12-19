use std::f64::consts::PI;

pub trait CalculateArea{
    fn calc(&self) ->f64;
}

///非pub，外部不可见
struct Triangle{
    bottom: u32,
    height: u32
}

// A public struct with a public field of generic type `T`
pub struct  OpenBox<T> {
    pub content: T,
}

// A public struct with a private field of generic type `T`
#[allow(dead_code)]
pub struct CloseBox<T> {
    content: T,
}

impl<T> CloseBox<T> {
    // A public constructor method
    pub fn new(content: T) -> CloseBox<T> {
        CloseBox{ content }
    }
}

//元组结构体
pub struct Car(pub String);

struct EmptyStruct();  //同样支持空的元组结构体





