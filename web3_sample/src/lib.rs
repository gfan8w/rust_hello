use web3::{types::{ U256} };

///Convert the wei to eth
pub trait WeiEth {
    fn wei_to_eth(&self) -> Option<U256>;
}

pub fn wei_to_eth(wei: U256) -> f64 {
    let res = wei.as_u128() as f64;
    res / 1_000_000_000_000_000_000.0
}

///Convert wei to eth
impl WeiEth for U256 {
    fn wei_to_eth(&self) -> Option<U256> {
        self.checked_div(U256::exp10(18))
    }
}