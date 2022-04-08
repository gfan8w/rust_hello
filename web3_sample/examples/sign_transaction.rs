use std::str::FromStr;
use std::time;
use hex_literal::hex;
use web3::contract::{Contract, Options};
use web3::types::{Address, TransactionParameters, TransactionRequest, U256};
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
    // 2) http
    //let transport = web3::transports::Http::new("http://localhost:8545").unwrap();
    //let web3s = web3::Web3::new(transport);


    // Insert the 32-byte private key in hex format (do NOT prefix with 0x)
    let prvk = secp256k1::SecretKey::from_str("7f2dba38c010f6aad93c48bd77e72c1ea6720a40f45e46e96cc81e4e65a33866").unwrap();
    let from_account: Address =hex!("9ea356d25c658A648f408ABE2322F2f01F12A0F0").into();

    let to_account:Address = hex!("ff93B45308FD417dF303D6515aB04D9e89a750Ca").into();

    let from_account_balance = web3s.eth().balance(from_account, None).await.unwrap();
    let to_account_balance = web3s.eth().balance(to_account, None).await.unwrap();
    println!("from_account balance:{:?}, to_account balance:{:?}",from_account_balance.wei_to_eth(),to_account_balance.wei_to_eth());

    let gas_price =web3s.eth().gas_price().await.unwrap();
    // Build the tx object
    let tx_obj =TransactionParameters {
        to: Some(to_account),
        value: U256::exp10(19), // 10eth
        //gas: U256::from(21000),
        //gas_price: Some(gas_price), //Some(U256::from(1_000_000)),
        //chain_id: Some(1990_u64),
        ..Default::default()
    };

    // Sign the tx (can be done offline)
    let signed = web3s.accounts().sign_transaction(tx_obj, &prvk).await?;

    // Send the tx to ethnet
    let tx = web3s.eth().send_raw_transaction(signed.raw_transaction).await?;
    println!("Tx succeeded with hash: {}", tx);

    // Build the tx object, 如果服务端节点里有private key保存，from账号是内置账号，则直接发送交易
    let tx_object = TransactionRequest {
        from:from_account,
        to: Some(to_account),
        value: Some(U256::exp10(20)), //100 eth
        ..Default::default()
    };

    // Send the tx to a node embed `from` account
    let _result = web3s.eth().send_transaction(tx_object).await?;


    std::thread::sleep(std::time::Duration::from_secs(10));

    //再次查看值
    let from_account_balance = web3s.eth().balance(from_account, None).await.unwrap();
    let to_account_balance = web3s.eth().balance(to_account, None).await.unwrap();
    println!("from_account balance:{:?}, to_account balance:{:?}",from_account_balance.wei_to_eth(),to_account_balance.wei_to_eth());







    Ok(())

}