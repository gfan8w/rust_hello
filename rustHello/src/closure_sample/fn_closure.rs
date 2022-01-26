

/*

pub trait Fn: FnMut
{
    extern "rust-call" fn call(&self, args: Args) -> Self::Output;
}

它“继承”了 FnMut，或者说 FnMut 是 Fn 的 super trait。
这也就意味着任何需要 FnOnce 或者 FnMut 的场合，都可以传入满足 Fn 的闭包。

*/




fn run_fn(){
    let v = vec![1,2,3];
    let v1 =vec![1,2,3,4];

    // Fn 不移动v所有权，只是借用 v
    let mut c =|x:u64| v.len() as u64 *x;
    // Fn，移动所有权
    let mut c1 = move |x:u64| v1.len() as u64 *x;

    println!("direct call:{}",c(2));
    println!("direct call:{}",c1(2));

    println!("fn call:{}",call_fn(1,&c));
    println!("fn call:{}",call_fn(2,&c1));

    println!("mut call:{}",call_mut(1,&mut c));
    println!("mut call:{}",call_mut(2,&mut c1));

    println!("once call:{}",call_once(1,c));
    println!("once call:{}",call_once(2,c1));


    fn call_fn(arg: u64, cal: & impl Fn(u64) -> u64) -> u64 {
        cal(arg)
    }


    fn call_mut(arg: u64, cal: &mut impl FnMut(u64) -> u64) ->u64 {
        cal(arg)
    }

    fn call_once(arg: u64, cal: impl FnOnce(u64) -> u64)  ->u64 {
        cal(arg)
    }


}









#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_run_fn_once() {
        run_fn();
    }

}














