use std::str::FromStr;
use std::time;
use futures::StreamExt;
use hex_literal::hex;
use secp256k1::SecretKey;
use web3::contract::{Contract, Options};
use web3::types::{Address, FilterBuilder, U256, H256};
use web3_sample::WeiEth;


#[tokio::main]
async fn main() -> web3::Result<()> {
    std::env::set_var("RUST_LOG", "DEBUG");
    std::env::set_var("RUST_BACKTRACE", "1");

    let _ = env_logger::try_init();

    //获取连接点，创建链接
    // 1) ws
    let transport = web3::transports::WebSocket::new("ws://localhost:8545").await?;
    let web3s = web3::Web3::new(transport);
    // 2) http
    //let transport = web3::transports::Http::new("http://localhost:8545").unwrap();
    //let web3s = web3::Web3::new(transport);


    // Insert the 32-byte private key in hex format (do NOT prefix with 0x)
    let prvk = SecretKey::from_str("ce61950f69e29c9e85b76ca844c728d20f6394d27b2543434e4917c12273791d").unwrap();

    let my_account: Address = hex!("ff93B45308FD417dF303D6515aB04D9e89a750Ca").into();

    //部署合约
    let storage_bin = include_str!("../build/storage.bin");
    let contract = Contract::deploy(web3s.eth(),
                                    include_bytes!("../build/storage.abi")).unwrap()
        .confirmations(1)
        .poll_interval(time::Duration::from_secs(10))
        .options(Options::with(|opt| opt.gas = Some(3_000_000.into())))
        .execute(storage_bin,
                 (U256::from(1_000_000)), // 没有参数填 ()
                 my_account)
        .await.unwrap();

    println!("contract address:{:?}", contract.address());

    // event 过滤
    let event_name = "ValueSetted(address,uint256,uint256,bytes)";
    let valuesetted_event_sig = web3::signing::keccak256(event_name.as_bytes());
    let valuesetted_event_sig_h256 = H256::from(valuesetted_event_sig);

    let hexStr = valuesetted_event_sig.iter().map(|x| format!("{:02x?}", x)).collect::<Vec<_>>().join("");
    println!("event: {} , sig: {}", event_name, hexStr);

    //event 的第0个是event的签名
    let filter = FilterBuilder::default()
        .address(vec![contract.address()])
        .topics(Some(vec![valuesetted_event_sig_h256]), None, None, None)
        .build();


    let mut sub = web3s.eth_subscribe().subscribe_logs(filter.clone()).await?;

    // 查看值
    let val: U256 = contract.query("val", (), None, Options::default(), None).await.unwrap();
    println!("query val:{:?}", val);

    //修改值
    let tx = contract.call("setValue", (U256::from(1_u32), ),
                           my_account, Options::default()).await.unwrap();
    println!("setValue tx:{}", tx);

    std::thread::sleep(std::time::Duration::from_secs(5));

    //再次查看值
    let val: U256 = contract.query("val", (), None, Options::default(), None).await.unwrap();
    println!("query val again:{:?}", val);

    //查看事件

    let aa = (&mut sub).take(10).for_each(|log| {
        println!("got event:{:?}", log);
        println!("   address:{:?}", log.unwrap().address);
        println!("   data:{:?}", log.unwrap().data);
        futures::future::ready(())
    }).await;


    sub.unsubscribe().await?;


    Ok(())
}