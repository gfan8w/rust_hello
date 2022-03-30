extern crate actix;

use actix::dev::{MessageResponse, ResponseChannel};
use actix::prelude::*;
use futures::{FutureExt, TryFutureExt};

// Arbiter 即 Actor 异步执行器，运行在 tokio （rt-core单线程）之上。




// 测试 Actor 对象
struct SumActor {
}


impl Actor for SumActor {
    // 每个 Actor 都有一个执行上下文
    type Context = Context<Self>;

    // 更改默认actor的mailbox 大小
    fn started(&mut self, ctx: &mut Self::Context) {
        ctx.set_mailbox_capacity(100);
    }
}

// Actor 接收的参数
struct Value(usize, usize);

// Actor 接收的参数必须实现 Message 对象
impl Message for Value {
    // 定义 该参数的 返回值类型
    type Result = usize;
}

// Actor 的具体处理函数
impl Handler<Value> for SumActor {
    type Result = usize;

    fn handle(&mut self, msg: Value, ctx: &mut Self::Context) -> Self::Result {
        msg.0+msg.1
    }
}


//定义处理返回值的actor
struct DisplayActor {}

impl Actor for DisplayActor {
    type Context = Context<Self>;
}

#[derive(Debug)]
struct Display(usize);


impl Message for Display {
    type Result = ();
}


impl Handler<Display> for DisplayActor {
    type Result = ();

    fn handle(&mut self, msg: Display, ctx: &mut Self::Context) -> Self::Result {
        println!("got:{}",msg.0)
    }
}




fn main(){

    let system =System::new("single-arbiter-demo");

    //创建Addr
    let sum_addr =SumActor{}.start();
    let display_addr = DisplayActor{}.start();

    // 定义一个执行流的Future
    // 起初发送 `Value(6, 7)` 给 `SumActor`
    // `Addr::send` 返回 `Request` 类型，该类型实现了 `Future`
    // Future::Output = Result<usize, MailboxError>
    let execution = sum_addr.send(Value(2,4))
        // `.map_err` 转换 `Future<usize, MailboxError>` 为 `Future<usize, ()>`
        //   如果有错误将打印错误信息
        // 实现来自于 use futures::TryFutureExt;
        .map_err(|e| eprintln!("Encountered mailbox error: {:?}",e))
        // 假设发送成功，并成功返回，and_then将得到执行，其中参数为上一个Future的Result<T, E> 的 T
        // 实现来自于 use futures::TryFutureExt;
        .and_then(move |res|{
            // `res` 是 `SumActor` 参数为 `Value(2, 4)` 的返回值，类型为 `usize`

            // res 发送给 DisplayActor 展示
            display_addr.send(Display(res)).map_err(|_|())
        })
        .map(|_|{
            // 当 DisplayActor 返回后停止，将关闭所有 Actor
            System::current().stop()
        });

    // 提交 Future 到 Arbiter/event 循环中
    Arbiter::spawn(execution);


    // stop system and exit
    system.run().unwrap()
}