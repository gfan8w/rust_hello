
///如何实现通过元组调用一个多参数的函数，
/// actix-web 的提取器（Extractor） 实现了从1元组到10元组的FromRequest，
/// 这里演示如何实现动态一套代码，实现多个元组元素参数入参
trait CallFnWithTuple<A,R> {
    fn call_with_tuple(&self, param: A) -> R;
}


impl<Func,A,R> CallFnWithTuple<(A,), R> for Func where Func: Fn(A,) -> R {
    fn call_with_tuple(&self, param: (A, )) -> R {
        (self)(param.0)
    }
}

impl<Func,A,B,R> CallFnWithTuple<(A,B),R> for Func where Func: Fn(A,B) ->R {
    fn call_with_tuple(&self, param: (A, B)) -> R {
        (self)(param.0, param.1)
    }
}

fn proxy<T, R> (f: impl CallFnWithTuple<T,R>, p: T) -> R {
    f.call_with_tuple(p)
}

fn test_1(i:i32) -> i32{
    i +1
}

fn test_2(i:i32, j: i32) -> i32{
    i+j
}

fn main() {
    println!("{}", proxy(test_1, (3,)));
    println!("{}", proxy(test_2, (2,3)));
    println!("{}", test_2.call_with_tuple((3,4)))
}