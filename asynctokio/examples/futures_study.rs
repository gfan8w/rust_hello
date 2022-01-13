use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::sync::mpsc::{Receiver, sync_channel, SyncSender};
use std::task::{Context, Poll, Waker};
use std::thread;
use std::thread::sleep;
use std::time::Duration;
use futures::future::BoxFuture;
use futures::FutureExt;
use futures::task::{ArcWake, waker_ref};

// 参考：https://www.cnblogs.com/s-lisheng/p/13072570.html


struct ShareStatus {
    complete: bool,
    waker: Option<Waker>,
}

/// 我们想要实现一个定时器Future,
/// 个人觉得，主要是为了用Arc 和Mutex 来包装ShareStatus
struct TimerFuture {
    share_state: Arc<Mutex<ShareStatus>>,
}


impl Future for TimerFuture{
    type Output = String;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
       let mut share_state= self.share_state.lock().unwrap();
        match share_state.complete {
            true => {
                println!("future ready. execute poll to return.");
                Poll::Ready(("ready".to_string()))
            },
            _ =>{
                println!("future not ready, tell the future task how to wakeup to executor");
                // 你要告诉future，当事件就绪后怎么唤醒任务去调度执行，而这个waker根具体的调度器有关
                // 调度器执行的时候会将上下文信息传进来，里面最重要的一项就是Waker
                share_state.waker =Some(cx.waker().clone());
                Poll::Pending
            }
        }
    }
}


impl TimerFuture {
    fn new (duration: u64) -> TimerFuture {
        let share_state = Arc::new(Mutex::new(ShareStatus{ complete: false, waker: None }));
        let share_state_clone = share_state.clone();
        thread::spawn(move||{
            sleep(Duration::from_millis(duration));
            let mut share_state =share_state_clone.lock().unwrap();
            share_state.complete=true;
            println!("detect future is ready, wakeup the future task to executor.");
            if let Some(waker) =share_state.waker.take() {
                waker.wake();
            }
        });

        TimerFuture{share_state}
    }
}





pub fn main(){

    //输出：
    //future not ready, tell the future task how to wakeup to executor
    //detect future is ready, wakeup the future task to executor.
    //future ready. execute poll to return.

    //可以看到，刚开始的时候，定时10s事件还未完成，处在Pending状态，
    // 这时要告诉这个任务后面就绪后怎么唤醒去调度执行。等10s后，定时事件完成了，
    // 通过前面的设置的Waker，唤醒这个Future任务去调度执行。这里，我们看一下Context和Waker是怎么定义的：
    //futures::executor::block_on(TimerFuture::new(900));      // 我们现在还没有实现调度器，所以要用一下futues库里的一个调度器。

    run_executor();

}


//以下自己实现一个调度器

/// 等待调度执行的Future任务，这个任务必须要实现ArcWake，表明怎么去唤醒任务去调度执行。
struct Task {
    future: Mutex<Option<BoxFuture<'static, String>>>,
    task_sender: SyncSender<Arc<Task>>,
}

impl ArcWake for Task {
    // A way of waking up a specific task.
    fn wake_by_ref(arc_self: &Arc<Self>) {
        let clone = arc_self.clone();
        arc_self.task_sender.send(clone).expect("too many task queued")
    }
}


/// 负责将一个Future封装成一个Task，分发到调度器去执行。
struct Spawner {
    task_sender: SyncSender<Arc<Task>>,
}

impl Spawner {
    // encapsul a future object to task , wakeup to executor.
    fn spawn(&self, future: impl Future<Output = String> + Send + 'static ) {
        let future = future.boxed();
        let task = Arc::new(Task {
            future: Mutex::new(Some(future)),
            task_sender: self.task_sender.clone()
        });
        println!("first dispatch the future task to executor.");
        self.task_sender.send(task).expect("too many task queued")
    }
}


struct Executor {
    // executor , received ready task to execute.
    ready_queue: Receiver<Arc<Task>>,
}


impl Executor {
    // 实际运行具体的Future任务，不断的接收Future task执行。
    fn run(&self) {
        let mut count = 0;
        while let Ok(task) = self.ready_queue.recv() {
            count = count + 1;
            println!("receive task:{}", count);
            let mut future_slot = task.future.lock().unwrap();
            if let Some(mut future) = future_slot.take() {
                let waker = waker_ref(&task);
                let context = &mut Context::from_waker(&*waker);
                if let Poll::Pending = future.as_mut().poll(context) {
                    *future_slot = Some(future);
                    println!("executor run the future task, but is not ready, create a future again.");
                } else {
                    println!("executor run the future task, is ready. the future task is done.");
                }
            }
        }
    }
}


fn new_executor_and_spawner() -> (Executor, Spawner){
    const MAX_QUEUE_SIZE: usize = 100;
    let(sender, receiver) =sync_channel(MAX_QUEUE_SIZE);

    (Executor{ready_queue:receiver},Spawner{task_sender:sender})

}


fn run_executor(){
    let (executor,spawner) =new_executor_and_spawner();
    spawner.spawn(async {
        let tf = TimerFuture::new(1000).await;
        println!("received:{}",tf);
        tf
    });

    drop(spawner);

    executor.run();
}


