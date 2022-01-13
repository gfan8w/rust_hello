



use tokio::time;
async fn promise() {
    println!("promise resolved.");
}
async fn f() {
    println!("enter");
    promise().await;
    println!("leave");
}

// 这段代码用于测试 JavaScript的输出，
// 源自：https://zhuanlan.zhihu.com/p/385965048

#[tokio::main]
pub async fn main() -> Result<(),()> {
    println!("start");
    let task:tokio::task::JoinHandle<_>= tokio::spawn(f());
    //f().await;
    println!("end");
    //task.await;
    time::sleep(time::Duration::from_secs(10)).await;

    Ok(())
}







