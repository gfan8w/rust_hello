use std::ops::Mul;
use std::process::Output;



///柯里化
fn run_curry(){

    let x =curry(2);

    let z =x(5);

    println!("z is {}",z)


}





fn curry<T>(x: T) -> impl Fn(T) -> T
where T: Mul<Output= T> + Copy
{
     move |y| y*x    // 这里要加 move ？ 担心 x 活的时间不够长？还要加 Copy？
}














#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_run_fn_once() {
        run_curry();
    }

}
















