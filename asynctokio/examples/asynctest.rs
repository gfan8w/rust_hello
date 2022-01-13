use std::future::Future;
use std::time::{Duration, Instant};
use tokio::sync::oneshot;
use tokio::task::JoinError;
use tokio::time::sleep;

///在这个例子中，hello 和 world 函数都使用了 async 关键字，
/// 表示该函数要以异步的方式执行。两个函数的返回值本来为 String，
/// 但是加了 async 关键字后，这两个函数的最终的签名会在内部表示为 fn hello() -> impl Future<Output=String>。
/// 即返回值是一个 Future 类型，这个 Future 执行后，会返回 String 类型的结果。
async fn world() -> String {
    "world".to_string()
}

///加了 async 关键字后，这个函数的最终的签名会在内部表示为 fn hello() -> impl Future<Output=String>。
async fn hello() -> String {
    // 使用 .await 关键字调用 world() 函数
    let w = world().await;
    format!("hello {} async from rust",w);

     async {
         println!("direct invoke async by append .await")
     }.await; //async 代码块，直接用await调用

    w

}

///在 hello 函数中，使用了 world().await 来调用 world 函数，并等待该函数返回，
/// 其结果不是 Future 类型，而是 Future 关联的 Output 类型，在这里即 String。
/// 除了直接使用 await 关键字，我们还使用了 tokio::runtime::Runtime::new() 创建了 tokio 运行时，
/// 并在其中运行我们的 Future ，即 rt.block_on(hello()) 和 rt.block_on(async {}) 这两处。
fn run_hello(){
    // 创建运行时
    let rt =tokio::runtime::Runtime::new().unwrap();
    // 使用 block_on 调用 async 函数 ,会阻塞
    let msg = rt.block_on(hello());
    println!("message:{}",msg);

    // 使用 block_on 调用 async block
    rt.block_on(async {
        println!("hello async block")
    });
}

//只需要在函数上面加上 #[tokio::main] ，前面加上 async 关键字，即可在其内部直接执行 await 方法，而不必使用 block_on 或者 spawn 方法。




///tokio 提供了一个非常方便的注解（或称属性），方便我们在 main 函数中执行 Future 任务。
#[tokio::main]
async fn run_hello_async(){

    hello().await;

    async {
        println!("async block invoke!")
    }.await

}

// async 关键字可以创建一个 Future ，与之相对，.await 则会销毁（解构）这个 Future.
// 因此，我们也可以说这两个关键字互相消解，async { foo.await } 就相当于 foo。



#[tokio::main]
async fn run_1() {
    //spawn 会启动一个异步任务，并返回 JoinHandle 类型的结果。这个任务虽然启动，但是 spawn 不保证（等待）它会正常执行完成。
    // hard work finished 还没输出 程序就结束了。
    tokio::spawn(async {
        sleep(Duration::from_millis(1000)).await;
        println!("hard work finished");
    });

    println!("mission start")

}

///获取一个 JoinHandler,然后等待完成
#[tokio::main]
async fn run_2() {
    //spawn 会启动一个异步任务，并返回 JoinHandle 类型的结果。这个任务虽然启动，但是 spawn 不保证（等待）它会正常执行完成。
    // hard work finished 还没输出 程序就结束了。
    let jh=tokio::spawn(async {
        sleep(Duration::from_millis(1000)).await;
        println!("hard work finished");
    });

    println!("mission start");
    //这里我们只需要拿到 spawn 的 JoinHandle ，并使用 await，即可以等待该任务结束，从而确保在所有工作完成后，再退出 main 函数。
    jh.await.unwrap();
}

#[tokio::main]
async fn run_3() {
    let jh_value= tokio::spawn(async {
        sleep(Duration::from_secs(1)).await;
        let o: Result<i32,()> = Ok(1+4);
        o
    });

    let val= jh_value.await.unwrap().unwrap();
    println!("return from tokio::spawn:{:?}",val);
}

///比较 run_3 和 run_4,他们的差异：
/// 1）返回值， run_4有返回值，是因为要处理 ?，run_3没有，是因为unwrap了 （本来2个函数就不打算有返回值， run_4 必须有返回值）
/// 2）unwrap.().unwrap() 和 ??，?? 是先解Future的Err，解开后是一个Result,需要再解Result的Err
/// 3) run_3 里的 OK(1+4) 的o变量必须定义类型，E 是 ()，run_4的 spawn不需要
#[tokio::main]
async fn run_4() ->Result<(),JoinError>{
    let jh_value= tokio::spawn(async {
        sleep(Duration::from_secs(1)).await;
        Ok(1+4)
    });

    let val= jh_value.await??;
    println!("return from tokio::spawn:{:?}",val);
    Ok(())
}

#[tokio::main]
async fn run_ch() {
    let (sender,receiver) =oneshot::channel();
    tokio::spawn(async {
        sleep(Duration::from_millis(800)).await;
        println!("hard work done, send signel");
        sender.send("ping".to_string()).unwrap();
    });

    let result = receiver.await.unwrap();
    println!("get data from channel:{}",result);
}

///等待 所有任务 全部完成
#[tokio::main]
async fn run_wait_all(){

    println!("start @ {:?}",chrono::Local::now());

    let f = async {
        sleep(Duration::from_millis(900)).await;
        println!("f finished @ {:?}", chrono::Local::now());
        "hello"
    };

    let f1 = tokio::spawn(async {
        sleep(Duration::from_millis(100)).await;
        println!("f1 finished @:{:?}", chrono::Local::now());
        5
    });

    let (str, i) =tokio::join!(f,f1);
    println!("wait all return: f ->{}, f1->{}",str,i.unwrap())
}

#[tokio::main]
async fn test_select(t1:u32, t2: u64, timeout: u64) {

    let f1 = async {
        sleep(Duration::from_millis(t1 as u64)).await;
        1
    };

    let f2 = async {
        sleep(Duration::from_millis(t2 )).await;
        // TryInto::<String>::try_into("hello").unwrap();
        "hello".to_string() //TryInto 即可
    };

    let timeout =sleep(Duration::from_millis(timeout));

    tokio::select! {
        _ = timeout => {
            println!("timeout")
        },
        v1 = f1 => {
            println!("f1 return: {}",v1)
        },
        v2 = f2 => {
            println!("f2 return: {}",v2)
        }
    }
}


fn run_tokio_select () {
    println!("wait any return(three result):");
    test_select(100,500,600); // 输出 1
    test_select(500,200,800); //输出 hello
    test_select(900,700,200); //输出 timeout
}








pub fn main(){
    run_hello();
    run_hello_async();
    run_2();
    run_3();
    run_4();
    run_ch();
    run_wait_all();
    run_tokio_select();
}