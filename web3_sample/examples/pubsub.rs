use std::str::FromStr;
use std::time;
use futures::StreamExt;
use hex_literal::hex;
use secp256k1::SecretKey;
use web3::contract::{Contract, Options};
use web3::types::{Address, U256};
use web3_sample::WeiEth;

#[tokio::main]
async fn main() -> web3::Result<()>{

    std::env::set_var("RUST_LOG","DEBUG");
    std::env::set_var("RUST_BACKTRACE","1");

    let _ = env_logger::try_init();

    //获取连接点，创建链接
    // 1) ws
    let transport = web3::transports::WebSocket::new("ws://localhost:8545").await?;
    let web3s = web3::Web3::new(transport);

    let mut sub =web3s.eth_subscribe().subscribe_new_heads().await?;
    println!("got subscript id: {:?}", sub.id());

    (&mut sub).take(15).for_each(|n|{
        println!("got header:{:?}",n);
        futures::future::ready(())
    }).await;

    //sub 在前面 take的时候已经移动走了， 为什么 把 sub 变为 （&mut sub) 就可以了？
    sub.unsubscribe().await?;


    Ok(())

}