use std::sync::{Arc};
use std::sync::atomic::{AtomicU32, Ordering};
use std::time::Duration;
use tokio::sync::RwLock;

#[tokio::main]
async fn main(){
    // Arc结合RwLock
    let lock = Arc::new(RwLock::new("hello".to_string()));
    let r_lock = lock.clone();
    let w_lock = lock.clone();

    let n = lock.read().await;

    assert_eq!(*n, "hello".to_string());
    let r_idex =AtomicU32::new(1);
    let r = tokio::spawn(async move {
        let mut t = tokio::time::interval(Duration::from_millis(10));
        loop {
            t.tick().await;
            //新开个代码块，使得i自动drop掉
            {
                let i = r_idex.fetch_add(1, Ordering::Acquire);
                // main 现成持有一个read锁，我们这里依然能活的到
                let r = r_lock.read().await;
                println!("{}. read:{}", i, r);
            }
        }
    });

    let w_idex =AtomicU32::new(1);
    let w = tokio::spawn(async move {
        let mut t = tokio::time::interval(Duration::from_millis(200));
        loop {
            let i =w_idex.fetch_add(1,Ordering::Acquire);
            t.tick().await;
            println!("{}. START WRITE", i);
            {
                let mut r = w_lock.write().await;
                *r = "world".to_string();
                println!("{}. WRITE:{}", i, r);
            }
        }
    });

    drop(n); //丢掉 main 主线程里的读锁，write 才能获取到锁

    //r.await.expect("read thread is crashed");
    w.await.expect("write thread is crashed");
    //tokio::join!(r,w);


}