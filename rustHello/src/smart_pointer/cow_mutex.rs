use std::borrow::{Borrow, BorrowMut, Cow};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use serde::de::MapAccess;

fn run_mutex_cow(){
    // 用 Arc 来提供并发环境下的共享所有权（使用引用计数）
    let metrics : Arc<Mutex<HashMap<Cow<'static,str>,u32>>> = Arc::new(Mutex::new(HashMap::new()));

    for _ in 0..32 {
        let m_t = metrics.clone();
        thread::spawn(move ||{
            let mut met = m_t.lock().unwrap();
            
            // 此时只有拿到 MutexGuard 的线程可以访问 HashMap
            let  hs  = met.borrow_mut();
            let data = &mut *hs;

            // Cow 实现了很多数据结构的 From trait，
            // 所以我们可以用 "hello".into() 生成 Cow
            let  entry =hs.entry("hello".into()).or_insert(0);
            *entry+=1;
            // MutexGuard 被 Drop，锁被释放
        });
    }
    thread::sleep(Duration::from_millis(1000));

    let v =metrics.lock().unwrap();
    let vv =(*v).get("hello".into());
    println!("{:?}", vv)



}




#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_run_serde_cow() {
        run_mutex_cow()
    }

}










