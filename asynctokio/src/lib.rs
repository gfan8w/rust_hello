extern crate tokio;

use tokio::task;

#[cfg(test)]
mod tests {
    use crate::calc;

    #[test]
    fn it_works() {

        calc();

        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}


// 参考： https://blog.logrocket.com/a-practical-guide-to-async-in-rust/

///cpu紧张型工作， block
fn fib_cpu_intensive(num: u64) -> u64 {
    match num {
        0 => 0,
        1 => 1,
        n => fib_cpu_intensive(n-1) + fib_cpu_intensive(n-2),
    }
}

#[tokio::main]
async fn calc() {
    // spawn_blocking 要求传入一个 fn
    let threadpool_future = task::spawn_blocking( ||{
        fib_cpu_intensive(30) // cpu紧张型工作， block住是可以接受的
    });
    let r = threadpool_future.await.unwrap();
    println!("fib result:{}",r)
}






