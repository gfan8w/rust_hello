
use std::{future, thread};
use std::time::{Duration, Instant};
use futures::channel::mpsc;
use futures::executor::ThreadPool;
use futures::{executor, FutureExt, pin_mut, select, StreamExt, TryFutureExt};

async fn get_book() {
    thread::sleep(Duration::from_millis(1000));
    println!("get_book")
}

async fn get_music() {
    thread::sleep(Duration::from_millis(800));
    println!("get_music")
}


async fn get_music2() ->Result<String,String> {
    thread::sleep(Duration::from_millis(800));
    println!("get_music");
    Ok("musice".to_string())
}

async fn get_book2() -> Result<String,String>{
    thread::sleep(Duration::from_millis(1000));
    println!("get_book");
    Ok("book".to_string())
}



async fn get_music3() ->Result<String,String> {
    thread::sleep(Duration::from_millis(800));
    println!("get_music");
    Ok("musice".to_string())
}

async fn get_book3() -> Result<String,i32>{
    thread::sleep(Duration::from_millis(1000));
    println!("get_book");
    Ok("book".to_string())
}


///select 也支持 default 和 complete 分支。
/// default 会在被 select 的future都没有完成时执行，因此，带有 default 分支的 select 总是马上返回，
/// 因为 default 会在没有其它future准备好的时候返回。
/// complete 分支则用来处理所有被 select 的 future 都完成并且不需进一步处理的情况。这在循环 select 时很好用：
async fn get_count() {
    let mut a_fut =futures::future::ready(4);
    let mut b_fut =futures::future::ready(6);
    let mut total =0;

    loop {
        select! {
            a = a_fut => total+=a,
            b = b_fut => total +=b,
            complete => break,
            default => unreachable!(),
        }
    }
    println!("total count:{}",total);
    assert_eq!(total, 10);

}




async fn run1() {

    let book =get_book(); //这里不会实际执行内容。要调用await后才会拿到直接内容，future是延迟执行
    let music =get_music();

    book.await;
    music.await;


}


async fn run2() {

    let book =get_book(); //这里不会实际执行内容。要调用await后才会拿到直接内容，future是延迟执行
    let music =get_music();
    let mut start =Instant::now();
    // join 只会在所有子 future 都完成后才会完成，它甚至会在子 future 返回 Err 之后继续处理其他的子future。
    futures::join!(book,music);
    println!("span：{}",(Instant::now()-start).as_secs_f32())

}

///演示 try_join
async fn run3() {

    let book =get_book2(); //这里不会实际执行内容。要调用await后才会拿到直接内容，future是延迟执行
    let music =get_music2();
    //注意，传进 try_join! 的 future 必须要用相同的错误类型。
    // 与 join! 不同，try_join! 会在其中的子future返回错误后立即完成，所以错误类型 只有1个，
    // try_join和join宏类似，唯一的区别就是，当执行发送错误时就马上返回。
    let r = futures::try_join!(book,music);

}

///演示 try_join，使用 futures::future::TryFutureExt 库的 .map_err(|e| ...) 或 err_into() 函数来统一错误类型
async fn run4() {

    //使用 futures::future::TryFutureExt 库的 .map_err(|e| ...) 或 err_into() 函数来统一错误类型
    let book =get_book3().map_err(|e| format!("err code:{}",e));

    let music =get_music3();
    //注意，传进 try_join! 的 future 必须要用相同的错误类型。
    // 与 join! 不同，try_join! 会在其中的子future返回错误后立即完成，所以错误类型 只有1个
    let r = futures::try_join!(book,music);

}


async fn run_select(){

    let music =get_music().fuse();
    let book = get_book().fuse();

    pin_mut!(music,book);

    select! {
        () = book => println!("book done"),
        () = music => println!("music done"),
    };

    get_count().await;

}



pub fn run_join (){
    let mut start =Instant::now();
    futures::executor::block_on(run1()); //必须用一个执行器 来执行它，run1()返回的是future
    let span= Instant::now()-start;
    println!("cost:{}",span.as_millis()); // 实际耗时 在 1.8m左右，可见 get_book, get_music 没有并发执行

    start =Instant::now();
    futures::executor::block_on(run2());
    let span= Instant::now()-start;
    println!("cost:{}",span.as_millis()); // 实际耗时 在 1.8m左右，没有并发执行?? 为什么？？？


    futures::executor::block_on(run3());

    futures::executor::block_on(run4());

}

pub fn main (){

    //run_join();

    futures::executor::block_on(run_select());

}