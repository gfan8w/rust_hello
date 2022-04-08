use std::collections::BTreeMap;
use std::fs::File;
use std::io::BufReader;
use std::str::FromStr;
use std::time;
use chrono::prelude::*;
use web3::{types::{H160, U256}, contract::{Contract, Options}};
use web3::types::{BlockId, BlockNumber};


// 参考： https://tms-dev-blog.com/rust-web3-token-transactions-from-blocks/

use web3_sample::WeiEth;



/// 1)安装 solidity 编译器，这里安装的是js版本的编译器
/// 2） solcjs -o build --bin --abi contracts/erc20.sol 生成bin 和 abi 文件
#[tokio::main]
async fn main() -> web3::Result<()> {
    std::env::set_var("RUST_LOG","info");
    std::env::set_var("RUST_BACKTRACE","1");

    dotenv::from_filename(".block.web3.env").unwrap();

    // 预先计算好的4字节方法名字和方法签名， 以BTreeMap的方式存放
    let sig_cont = include_str!("signatures.json");  // let f =File::open("src/signatures.json").unwrap();
    let reader = BufReader::new(sig_cont.as_bytes());
    let code_sig_lookup: BTreeMap<String, Vec<String>> = serde_json::from_reader(reader).unwrap();




    //获取连接点，创建链接
    let transport = web3::transports::Http::new("http://localhost:8545")?;
    let web3s = web3::Web3::new(transport);

    //获取账号
    let mut accounts = web3s.eth().accounts().await.unwrap();
    accounts.push(H160::from_str(&std::env::var("ACCOUNT_ADDRESS").unwrap()).unwrap());
    println!("accounts:{:?}",accounts);

    //遍历每个账号的余额
    for acct in &accounts {
        let balance = web3s.eth().balance(*acct, None).await.unwrap();
        println!("Eth balance of {:?}: {}", acct, balance.wei_to_eth().unwrap())

    }


    //获取最新块的信息
    let latest_block = web3s.eth().block(BlockId::Number(BlockNumber::Latest))
        .await
        .unwrap().unwrap();

    let timestamp = latest_block.timestamp.as_u64() as i64;
    let naive =NaiveDateTime::from_timestamp(timestamp,0);
    let local_dt = Local.from_utc_datetime(&naive);
    println!(
        "[{}] block num {}, parent {:?}, transactions: {}, gas used {}, gas limit {}, base fee {:?}, difficulty {}, total difficulty {}",
        local_dt.format("%Y-%m-%d %H:%M:%S"),
        latest_block.number.unwrap(),
        latest_block.parent_hash,
        latest_block.transactions.len(),
        latest_block.gas_used,
        latest_block.gas_limit,
        latest_block.base_fee_per_gas,
        latest_block.difficulty,
        latest_block.total_difficulty.unwrap()
    );
    
    
    // 获取交易信息
    for tx in latest_block.transactions {
        let tx = match web3s
            .eth()
            .transaction(web3::types::TransactionId::Hash(tx))
            .await {
                Ok(Some(tx)) => tx,
                Ok(None) => { println!("not found tx:{}",tx); continue;},
                Err(e) => {println!("error:{:?}",e); continue;},
            };

        let from_addr = tx.from.unwrap_or(H160::zero());
        let to_addr = tx.to.unwrap_or(H160::zero());
        println!(
            "[{}] from {}, to {}, value {:?}, gas {}, gas price {}",
            tx.transaction_index.unwrap_or(web3::types::U64::zero()),
            web3::helpers::to_string(&from_addr),
            web3::helpers::to_string(&to_addr),
            tx.value.wei_to_eth(),
            tx.gas,
            tx.gas_price
        );

        let smart_contract_addr =match tx.to {
            Some(addr) => {
                match web3s.eth().code(addr, None).await {
                    Ok(code) => {
                        if code == web3::types::Bytes::from([]) {
                            println!("Empty code, skipping");
                            None
                        } else {
                            println!("Not empty code, return address.");
                            Some(addr)
                        }
                    },
                    _ => {
                        println!("Unable to retrieve code, skipping.");
                        None
                    }
                }
            },
            _ => {
                println!("To address is not a valid address, skipping.");
                None
            }
        };

        if smart_contract_addr.is_some() {
            let smart_contract = match Contract::from_json(
                web3s.eth(),
                smart_contract_addr.unwrap(),
                include_bytes!("../build/erc20.abi")) {
                Ok(contract) => Some(contract),
                _ => {println!("Failed to init contract, skipping."); None}

            };

            if smart_contract.is_some() {
                let token_name:String =smart_contract.unwrap().query("name",(),None,Options::default(),None).await.unwrap();
                //println!("smart_contract name:{}",token_name);

                //查找方法名
                let mut func_signature ="".to_string();
                let input_str: String = web3::helpers::to_string(&tx.input);
                if input_str.len()>=12{
                    let fn_code = input_str[3..11].to_string();
                    func_signature = match code_sig_lookup.get(&fn_code) {
                        Some(fn_name) => format!("{:?}",fn_name),
                        None =>{println!("Function not found."); fn_code.to_string()}
                    };
                }

                println!(
                    "[{}] ({} -> {}) from {}, to {}, value {:?}, gas {}, gas price {}",
                    tx.transaction_index.unwrap_or(web3::types::U64::from(0)),
                    &token_name,
                    &func_signature,
                    web3::helpers::to_string(&from_addr),
                    web3::helpers::to_string(&to_addr),
                    tx.value.wei_to_eth(),
                    tx.gas,
                    tx.gas_price,
                );

            }
        }
    }







    //部署合约
    let storage_bin = include_str!("../build/erc20.bin");
    let contract = Contract::deploy(web3s.eth(),
                                    include_bytes!("../build/erc20.abi")).unwrap()
        .confirmations(1)
        .poll_interval(time::Duration::from_secs(10))
        .options(Options::with(|opt|opt.gas=Some(3_000_000.into())))
        .execute(storage_bin,
                 (U256::from(1_000_000), "MyToken".to_owned(), "MT".to_owned()), // 没有参数填 ()
                 accounts[0] )
        .await.unwrap();

    //获取合约地址
    println!("contract of Storage deployed at:{:?}", contract.address());

    //获取合约名字和总发行量
    let name:String =contract.query("name",(),None,Options::default(),None).await.unwrap();
    let total_supply:U256 =contract.query("totalSupply",(),None,Options::default(),None).await.unwrap();
    println!("Token name: {}, total supply: {:?}", name, total_supply.wei_to_eth());



    //futures::pin_mut!()




    Ok(())

}
