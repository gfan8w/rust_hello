// 一个简单的async starter 模板

use std::future::Future;
use log::{error, info};
use std::io::Write;
use chrono::Duration;
use std::fmt::Debug;

///定义一个Result，在async语境中，需要一些额外的约束
type Result<T> =std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

async fn app() ->Result<()> {
    // 把这里当做一个main函数

    let r = our_async_program().await;
    println!("a fn returned future:{:?}",r);

    let r = our_async_programm().await;
    println!("a async fn returned:{:?}",r);

    serial_get().await;

    paral_get().await;

    Ok(())
}


pub fn main(){
    //env_logger 依赖 RUST_LOG 来确定日志级别
    std::env::set_var("RUST_LOG", "debug");

    let start =std::time::Instant::now();
    env_logger::Builder::from_default_env().format(move |buf, rec|{
        //自定义log格式
        let t = start.elapsed().as_secs_f32();
        writeln!(buf, "{:.03} [{}] -{}",t,rec.level(), rec.args())
    }).init();

    let mut rt = tokio::runtime::Runtime::new().unwrap();
    match rt.block_on(app()){
        Ok(_) => info!("Done"),
        Err(e) => error!("An error occurred:{}",e)
    };
}


///一个普通函数，返回Future, 也是async 函数，等同于下面的 our_async_programm
fn our_async_program() -> impl Future<Output=Result<String>> {
    futures::future::ok("ready".to_string())
}

async fn our_async_programm() -> Result<String> {
    Ok("ready".to_string())
}

/// slowwly.robertomurray.co.uk 会延迟返回
fn slowwly(delay_ms: i32) -> reqwest::Url {
    let url = format!(
        "http://slowwly.robertomurray.co.uk/delay/{}/url/http://www.google.co.uk",
        delay_ms
    );
    reqwest::Url::parse(&url).unwrap()
}

 async fn serial_get() -> Result<()> {

     // 一个 一个的await 是串行操作，这个函数总共要3秒多

    info!("start serial request");
     //网页坏了，改为下面sleep模式来模拟一个长时间的操作。

    //let _resp1 = reqwest::get(slowwly(10000)).await;
    //info!("got response 1");
    //let _reps2 = reqwest::get(slowwly(20000)).await;
    //info!("got response 2");

     long_op(1000).await;
     long_op(2000).await;

    Ok(())
}


async fn paral_get() -> Result<()> {

    // 一个 一个的await 是串行操作

    info!("start paral request");

    let task1 = tokio::spawn(long_op(1000));
    let task2 =tokio::spawn(long_op(2000));


    let _reps1 = task1.await;
    let _resp2 = task2.await;


    Ok(())
}


async fn long_op(ms: u64) -> Result<()>{
    //std::thread::sleep(std::time::Duration::from_secs(ms)).await;

    tokio::time::sleep(std::time::Duration::from_millis(ms)).await;
    info!("loong run cost:{}, start time:{:?}",ms, chrono::Local::now());

    Ok(())
}






















