use std::str;

/// String  str vec<u8> vec<char> 相互转换
pub fn run() {
    // -- FROM: vec of chars --
    let src1: Vec<char> = vec!['j', '{', '"', 'i', 'm', 'm', 'y', '"', '}'];
    // to String
    let string1: String = src1.iter().collect::<String>();
    // to str
    let str1: &str = &src1.iter().collect::<String>();
    // to vec of byte
    let byte1: Vec<u8> = src1.iter().map(|c| *c as u8).collect::<Vec<_>>();
    println!("Vec<char>:{:?} | String:{:?}, str:{:?}, Vec<u8>:{:?}", src1, string1, str1, byte1);

    // -- FROM: vec of bytes --
    // in rust, this is a slice
    // b - byte, r - raw string, br - byte of raw string
    let src2: Vec<u8> = br#"e{"ddie"}"#.to_vec();
    // to String
    // from_utf8 consume the vector of bytes
    let string2: String = String::from_utf8(src2.clone()).unwrap();
    // to str
    let str2: &str = str::from_utf8(&src2).unwrap();
    // to vec of chars
    let char2: Vec<char> = src2.iter().map(|b| *b as char).collect::<Vec<_>>();
    println!("Vec<u8>:{:?} | String:{:?}, str:{:?}, Vec<char>:{:?}", src2, string2, str2, char2);

    // -- FROM: String --
    let src3: String = String::from(r#"o{"livia"}"#);
    let str3: &str = &src3;
    let char3: Vec<char> = src3.chars().collect::<Vec<_>>();
    let byte3: Vec<u8> = src3.as_bytes().to_vec();
    println!("String:{:?} | str:{:?}, Vec<char>:{:?}, Vec<u8>:{:?}", src3, str3, char3, byte3);

    // -- FROM: str --
    let src4: &str = r#"g{'race'}"#;
    let string4 = String::from(src4);
    // str::to_string, str::to_owned, String::from, str::into 均可以转换为一个String，
    // to_string()是比较通用的一个，任何实现了Display 的复杂结构体如果要想展示一个string字面量信息，都可以调用to_string，最后得到一个String
    let string44 =src4.to_string();
    let char4: Vec<char> = src4.chars().collect();
    let byte4: Vec<u8> = src4.as_bytes().to_vec();
    println!("str:{:?} | String:{:?}, Vec<char>:{:?}, Vec<u8>:{:?}", src4, string4, char4, byte4);

    // 从 &[u8]变为string，因为有可能有非法字符，所以安全的做法是match一个Err
    let buf = &[0x41u8, 0x41u8, 0x42u8];
    let s = match str::from_utf8(buf) {
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };
    println!("result: {}", s);
}