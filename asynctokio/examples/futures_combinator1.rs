use std::thread;
use std::time::Duration;
use futures::future::{err, ok};
use futures::stream::empty;
use futures::{FutureExt, StreamExt, TryFutureExt};

async fn run_leaf(){
    // leaf future ，也就是 FutureResult，它只要调用 poll 就能够及时的返回值
    let f = ok::<u32,u32>(1);
    let v =f.await.unwrap();
    println!("f:{}",v);
    assert_eq!(v,1);

    let f =err::<u32,u32>(4);
    //let v = f.await.unwrap(); // 这样写，程序返回的是Err，会崩溃，改为如下：
    let v =f.await.unwrap_err();// 把错误转换为 Option,然后去取出值
    println!("f:{}",v);
    assert_eq!(v,4);


    let f = ok::<u32,u32>(2);
    let f2 =f.map(|x|x.unwrap()+4);
    let v = f2.await;
    println!("f2:{}",v);
    assert_eq!(v,6);

    //把err 映射为另外一个err，这里把 u32的Err 转换为 String 的Err
    let f = err::<u32,u32>(1);
    let f2 = f.map_err(|x| format!("error:{}",x));
    let v = f2.await.unwrap_err();
    println!("f2:{}",v);
    assert_eq!(v,"error:1".to_string());

    //使用then 把 future 串起来，then返回的必须是一个 future，用async包裹起来是一个future
    let f =ok::<u32,u32>(1);
    let f2 = f.then(|x| async move {x.map(|y|y+1)});
    let v = f2.await.unwrap();
    println!("f2:{}",v);

    //使用then 把 future 串起来，then返回的必须是一个 future，用async包裹起来是一个future
    let f =ok::<u32,u32>(1);
    let f2 = f.and_then(|x| async move {Ok::<String,u32>((x+1).to_string())}); // f返回 Ok，则执行and_then里的代码，f的Err类型要与and_then的一致
    let v = f2.await.unwrap();
    println!("f2 and_then:{}",v); // 这里的v 是String类型

    let f = async {Err::<i32,i32>(4)};
    let f2 = f.and_then(|x| async move {println!("x:{}",x); Err::<i32,i32>(x+1) });
    let v = f2.await.unwrap_err(); // 最后还是返回4，因为f返回的是Err，导致 and_then 不会执行。改为 Ok 可以输出5
    println!("f2 and_then:{}",v);


    let f = async {Err::<i32,i32>(1)};
    let f2 =f.or_else(|x| async move { Err::<i32,i32>(x+8)}); // f 返回err 才继续，f的OK类型需要一致
    let v = f2.await.unwrap_err();
    println!("f2 or_slse:{}",v);


    //let f = async {  Ok(2)}; // 直接这样写会报错，目前，无法给 f一个类型或在async 块内指定返回类型。只能通过下面的turbofish 方式指定类型
    //参考：https://rust-lang.github.io/async-book/07_workarounds/02_err_in_async_blocks.html#-in-async-blocks
    let f = async {  Ok::<i32,i32>(2)};
    let f2  = f.then(|x|async move {x.unwrap()+1});

    let f3 = async {1};
    let f4 = async { f3};
    let v = f4.flatten().await; // 也可以使用 flatten 来得到最后一个 Future 的值
    println!("f4:{}",v);


    let f1 =ok::<i32,i32>(3);
    let f2 = f1.fuse();



}


pub fn main (){

    futures::executor::block_on(run_leaf());


}