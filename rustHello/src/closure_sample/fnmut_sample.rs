
/*

pub trait FnMut: FnOnce
{
    extern "rust-call" fn call_mut( &mut self, args: Args ) -> Self::Output;
}

FnMut “继承”了 FnOnce，或者说 FnOnce 是 FnMut 的 super trait。
所以 FnMut 也拥有 Output 这个关联类型和 call_once 这个方法。
此外，它还有一个 call_mut() 方法。

注意 call_mut() 传入 &mut self，它不移动 self，所以 FnMut 可以被多次调用。

FnOnce 是 FnMut 的 super trait，所以，一个 FnMut 闭包，可以被传给一个需要 FnOnce 的上下文，
此时调用闭包相当于调用了 call_once()




*/



pub fn run_fn_mut(){

    let mut tom = "tom".to_string();
    let mut jim ="Jim".to_string();

    //这里捕获 &mut tom，tom被借用后，不能移动到其他闭包里去，比如不能移动到 c2闭包
    let mut c =||{
        tom.push_str(" hello");
        println!("{}",tom); //这里不能返回 tom，返回tom 就把tom的所有权转移走了，变为 FnOnce了
    };

    // 这里捕获的是 mut jim，注意 jim 需要声明成 mut
    let mut c2 =move || {
        jim.push_str(" hello");
        println!("{}",jim);
    };

    c(); // 这里 c 是 FnMut，要把定义改为 mut c。

    c2();

    call_mut(&mut c);
    call_mut(&mut c2);

    // c 和 c1 这两个符合 FnMut 的闭包，能作为 FnOnce 来调用
    call_once(c); // 后面不能再使用c，c已经移动了。
    call_once(c2);

    // 在作为参数时，FnMut 也要显式地使用 mut，或者 &mut
    fn call_mut(cal: &mut impl FnMut()) {
        cal();
    }

    // 想想看，为啥 call_once 不需要 mut？ 因为它的super trait是 FnOnce， FnOnce 不需要 &mut self，只需要 self
    fn call_once(cal: impl FnOnce()) {
        cal();
    }

    //闭包都已经使用完毕，继续可以使用 tom。
    tom.push_str(" aa");
    println!("tom:{}",tom)
}




















#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_run_fn_once() {
        run_fn_mut();
    }

}






