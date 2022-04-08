use std::str::FromStr;
use std::sync::{Arc, atomic};
use std::time;
use std::time::Duration;
use futures::{FutureExt, TryFutureExt};
use hex_literal::hex;
use parking_lot::Mutex;
use web3::contract::{Contract, Options};
use web3::types::{Address, TransactionParameters, TransactionRequest, U256};
use web3_sample::WeiEth;

///跟踪器
struct Ticker {
    id: String,
    started: atomic::AtomicUsize,
    reqs: atomic::AtomicUsize,
    time: Mutex<time::Instant>
}

impl Ticker {
    fn new(id: &str) ->Self {
        Self{
            id: id.to_string(),
            started: Default::default(),
            reqs: Default::default(),
            time: Mutex::new(time::Instant::now()),
            
        }
    }

    pub fn start(&self) {
        self.started.fetch_add(1, atomic::Ordering::AcqRel);
    }

    pub fn tick(&self) {
        let reqs = self.reqs.fetch_add(1, atomic::Ordering::AcqRel);
        self.started.fetch_sub(1, atomic::Ordering::AcqRel);

        if reqs > 100_000 {
            self.print_result(reqs as u64);
        }
    }

    pub fn print_result(&self, reqs: u64){
        fn as_millis(dur: time::Duration) -> u64 {
            dur.as_secs()*1000 + dur.subsec_nanos() as u64 / 1_000_000
        }

        let mut time =self.time.lock();
        let elapsed = as_millis(time.elapsed());
        let result = reqs * 1000 /elapsed;

        println!("[{}] {} reqs/s ({} reqs in {} ms)", self.id, result, reqs, elapsed);

        self.reqs.store(0, atomic::Ordering::Release);

        *time = time::Instant::now();
    }


    pub fn wait(&self) {
        while self.started.load(atomic::Ordering::Relaxed)>0 {
            std::thread::sleep(Duration::from_millis(100));
        }

        self.print_result(self.reqs.load(atomic::Ordering::Acquire) as u64);
    }


}


fn bench<T: web3::Transport>(id: &str, transport:T, max: usize)
where T::Out: Send+'static
{
    let web3 = web3::Web3::new(transport);

    let ticker = Arc::new(Ticker::new(id));

    for _i in 0..max {
        let ticker =ticker.clone();
        ticker.start();
        let block =web3.eth().block_number().then(move |res|{
            if let Err(e) =res {
                eprintln!("Error:{:?}",e);
            }
            ticker.tick();
            futures::future::ready(())
        });
        tokio::spawn(block);
    }

    ticker.wait();

}










#[tokio::main]
async fn main() -> web3::Result<()>{

    std::env::set_var("RUST_LOG","INFO");
    std::env::set_var("RUST_BACKTRACE","1");

    let _ = env_logger::try_init();

    let request = 3000;

    //获取连接点，创建链接
    // 1) ws
    let ws = web3::transports::WebSocket::new("ws://localhost:8545").await?;
    bench(" ws",ws, request);

    // 2) http
    let http = web3::transports::Http::new("http://localhost:8545").unwrap();
    bench(" http",http, request);

    let ipc = web3::transports::WebSocket::new("./jsonrpc.ipc").await?;
    bench(" ipc",ipc, request);




    Ok(())

}