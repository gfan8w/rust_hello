use futures::channel::mpsc;
use futures::executor::ThreadPool;
use futures::{executor, StreamExt};

pub fn run(){

    let pool = ThreadPool::new().expect("unable to create pool");

    let (tx, mut rx) =mpsc::unbounded::<i32>();

    // 使用async块创建一个future,返回一个Future的实现
    // 此时尚未提供executor给这个future,因此它不会运行
    let fut_value = async {

        // 创建另外一个async块,同样会生成Future的实现,
        // 它在父async块里面,因此会在父async块执行后,提供executor给子async块执行
        // executor连接是由Future::poll的第二个参数std::task::Context完成,
        // 它代表了我们的executor,子async块生成的Future实现它能被polled(使用父async块的executor)   **** 这个翻译有问题啊

        let fut_tx_result = async move {
            (0..=100).for_each(|v| {
                tx.unbounded_send(v).expect("failed to send");
            })
        };

        //另外起一个线程去执行发行逻辑
        //执行器执行fut_tx_result这个future，发出 channel的sender传输
        pool.spawn_ok(fut_tx_result);

        let mut pending =vec![];

        while let Some(v) = rx.next().await {
            pending.push(v*2)
        }

        pending

    };


    let val = executor::block_on(fut_value);
    println!("get pending:{:?}", val)

}



pub fn main (){

    run();


}