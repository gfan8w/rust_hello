extern crate actix;

use actix::dev::{MessageResponse, ResponseChannel};
use actix::prelude::*;

//Actix 是一个 基于 Actor 模型 的 Rust 库
// 该模型允许将应用程序编写为一组通过消息进行通信的独立执行但相互协作的 Actor 。
// Actor 是封装状态和行为并在actix库提供的Actor系统中运行的对象。
//
// Actor在特定的执行上下文 Context<A> 中运行。上下文对象仅在执行期间可用。
// 每个参与者都有一个单独的执行上下文。执行上下文还控制 Actor 的生命周期。
//
// Actor 仅通过交换消息进行通信。发送方可以选择等待响应。Actor 不是直接引用的，而是通过地址引用的。
//
// 任何类型的都可以是一个 Actor，它只需要实现 Actor 特质即可。
//
// 为了能够处理特定的消息，Actor 必须为此消息提供 Handler<M> 实现。所有消息都是静态类型的。
// 消息可以异步方式处理。Actor可以产生其他actor或将Future或stream添加到执行上下文。
// Actor特征提供了几种方法，可以控制Actor的生命周期。




// 测试 Actor 对象
struct MyActor {
    count: usize,
}

// 所有的 Actor 必须实现 Actor 特质
// Actor 维护一个内部执行上下文或状态。这样，参与者可以确定自己的地址，更改邮箱限制或停止执行。
//
// 设置邮箱容量 ctx.set_mailbox_capacity(1);
// 获取自身地址 ctx.address()
// 停止 Actor ctx.stop()
impl Actor for MyActor {
    // 每个 Actor 都有一个执行上下文
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        ctx.set_mailbox_capacity(100);
    }
}

// Actor 接收的参数
struct Ping(usize);

// Actor 接收的参数必须实现 Message 对象
impl Message for Ping {
    // 定义 该参数的 返回值类型
    type Result = Pong;
}

// Actor 的具体处理函数
impl Handler<Ping> for MyActor {
    type Result = Pong;

    fn handle(&mut self, msg: Ping, ctx: &mut Self::Context) -> Self::Result {
        self.count+=msg.0;
        Pong(true)
    }
}


//定义返回值
#[derive(Debug)]
struct Pong(bool);

// 返回值必须实现 MessageResponse 特质
impl<A,M> MessageResponse<A, M> for Pong
where A: Actor,
      M: Message<Result=Pong>
{
    // 将 返回值 发送出去的逻辑
    fn handle<R: ResponseChannel<M>>(self, ctx: &mut A::Context, tx: Option<R>) {
            if let Some(t) =tx {
                t.send(Pong(true));
            }
    }
}



//再定义一个消息：
struct WhoAmI;

//为该消息实现Message
impl Message for WhoAmI {
    type Result = Result<actix::Addr<MyActor>,()>;
}

impl Handler<WhoAmI> for MyActor {
    type Result = Result<actix::Addr<MyActor>, ()>;

    fn handle(&mut self, msg: WhoAmI, ctx: &mut Self::Context) -> Self::Result {
        Ok(ctx.address())
    }
}






#[actix_rt::main]
async fn main(){
    // 启动一个Actor，返回一个 Addr<MyActor>
    // Actor 仅通过交换消息进行通信。发送方可以选择等待响应。无法仅通过其地址直接引用Actor。
    // 有几种获取 Actor 地址的方法。Actor 特征提供了两种启动 Actor 的辅助方法。两者都返回 started 角色的地址。
    // MyActor.start();
    // let addr = ctx.address();
    let addr =MyActor { count: 1}.start();

    // 要将消息发送给参与者，需要使用Addr对象。Addr提供了几种发送消息的方法。
    // Addr::do_send(M) -> () 此方法将忽略邮件发送中的任何错误。如果邮箱已满，则邮件仍在排队，
    // 绕过限制。如果参与者的邮箱已关闭，则该消息将被静默丢弃。
    // 此方法不会返回结果，因此，如果邮箱已关闭并且发生故障，则不会有任何指示。

    // Addr::try_send(M) -> Result<(), SendError> 此方法尝试立即发送消息。
    // 如果邮箱已满或关闭（角色已死），则此方法返回 SendError。
    // Addr::send(M) -> Future<Result<M::Result, MailboxError>> 该消息返回一个将来的对象，
    // 该对象解析为消息处理过程的结果。如果丢弃返回的Future对象，则该消息将被取消
    let res = addr.send(Ping(2)).await;

    // handle() returns tokio handle
    // 返回一个结果
    println!("RESULT: {:?}", res.unwrap());


    let who_addr = addr.send(WhoAmI{}).await;



    // stop system and exit
    System::current().stop();
}