use std::time::Duration;
use rand::{Rng, thread_rng};

const ENVS: &'static [&'static str] = &["testing", "production"];
async fn data_collector() {
    // 模拟执行一个定时循环的任务。
    let mut collect_interval = tokio::time::interval(Duration::from_millis(10));
    loop {
        collect_interval.tick().await;
        let mut rng = thread_rng();
        let response_time: f64 = rng.gen_range(0.001..10.0); // 返回一个随机值的范围
        let response_code: usize = rng.gen_range(100..=599); // = 表示包含
        let env_index: usize = rng.gen_range(0..2);

        track_status_code(response_code, ENVS.get(env_index).expect("exists"));
    }
}

fn track_status_code(status_code: usize, env: &str) {
    match status_code {
        500..=599 => println!("http code:{}", 500),  // 范围匹配
        400..=499 => println!("http code:{}", 400),
        300..=399 => println!("http code:{}", 300),
        200..=299 => println!("http code:{}", 200),
        100..=199 => println!("http code:{}", 100),
        _ => (),
    };
}

#[tokio::main]
pub async fn main () {
    let jh = tokio::task::spawn(data_collector());
    jh.await.unwrap();  // block 等待data_collector运行结束才返回，但这里data_collector是循环，不会返回
}






