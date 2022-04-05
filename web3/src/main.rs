use std::str::FromStr;
use web3::types::{H160, U256};


fn wei_to_eth(wei: U256) -> f64 {
    let res = wei.as_u128() as f64;
    res / 1_000_000_000_000_000_000.0
}

fn wei_eth(wei: U256) -> Option<U256> {
    wei.checked_div(U256::exp10(18))
}




#[tokio::main]
async fn main() -> web3::Result<()> {

    dotenv::from_filename(".block.web3.env").unwrap();

    let end_point ="ws://localhost:8546";
    let transport = web3::transports::Http::new("http://localhost:8545")?;
    let web3s = web3::Web3::new(transport);

    let mut accounts = web3s.eth().accounts().await.unwrap();
    accounts.push(H160::from_str(&std::env::var("ACCOUNT_ADDRESS").unwrap()).unwrap());
    println!("accounts:{:?}",accounts);

    let web_conv:U256 = U256::exp10(18);
    for acct in accounts {
        let balance = web3s.eth().balance(acct, None).await.unwrap();
        println!("Eth balance of {:?}: {}", acct, wei_eth(balance).unwrap())

    }



    Ok(())

}
