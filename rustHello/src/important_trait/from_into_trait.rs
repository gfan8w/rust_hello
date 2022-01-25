use std::convert::{TryFrom, TryInto};
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
use std::str::FromStr;

pub fn run(){
    // 下面2种数据转换是等价的
    // 在实现 From 的时候会自动实现 Into,看Into的实现可知，它内部调用了 from trait
    let s =String::from("hello");
    let s1: String ="hello".into();


    let ipv4: Ipv4Addr ="2.2.2.2".parse().unwrap(); // 定义时，需要指定类型，parse 内部调用了Ipv4Addr::from_str
    let ipv4  =Ipv4Addr::from_str("2.2.2.2").unwrap();

    let ipv6: Ipv6Addr = "::1".parse().unwrap();
    let ipv6 = Ipv6Addr::from_str("::1");

    let ipv4: Ipv4Addr = [192,168,1,1].into(); // From<[u8; 4]> for Ipv4Addr
    println!("ipv4:{}",ipv4);


    let ipadd:IpAddr =[192,168,2,1].into();
    println!("ipadd:{}",ipadd);

    // IPv6Addr 实现了 From<[u16; 8]，转换 IPv6 地址
    let ipadd: IpAddr = [0xfe80, 0, 0, 0, 0xaede, 0x48ff, 0xfe00, 0x1122].into();
    println!("ipadd:{}",ipadd);

    //try_into 类似 into，只是返回的是Result<.. , ..>
    let ipadd: Ipv6Addr =[0xfe80, 0, 0, 0, 0xaede, 0x48ff, 0xfe00, 0x1122].try_into().unwrap();
    println!("ipadd:{}",ipadd);

    let ipv4add = Ipv4Addr::try_from([192,168,10,14]).unwrap();
    println!("ipv4add:{}",ipv4add);

}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1() {
        run();
    }
}




























