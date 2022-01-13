

// 参考：https://zhuanlan.zhihu.com/p/348408532
// 1个简单的Future的例子


use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::{Duration, Instant};


struct Delay {
    when: Instant,
}

impl Future for Delay {
    type Output = &'static str;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if Instant::now()> self.when {
            println!("hello");
            Poll::Ready("Im ready")
        }else {
            println!("not ready");
            cx.waker().wake_by_ref();
            Poll::Pending
        }
    }
}

#[tokio::main]
async fn run_delay(){
    let when = Instant::now()+Duration::from_secs(3);
    let future =Delay{when};

    let out = future.await;

    println!("{}",out)

}





pub fn main(){
    run_delay();
}



