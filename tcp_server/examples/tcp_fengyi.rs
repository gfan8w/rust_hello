use std::{io::{Read, Write}, net::{TcpListener, TcpStream}};
use std::thread;
use std::str;


fn main() {
// 定义一个请求地址
    let addr = "127.0.0.1:8866".to_string();
// 创建一个Tcp监听，通过字符串切片将addr 传入
    let listener = TcpListener::bind(&addr).unwrap();
// 调用 incoming() 方法接收客户端的链接信息，如果有新的信息进来就会返回一个Result枚举，OK(T:TcpStream)
    for stream in listener.incoming() {
        println!("DEBUG::有新的链接进入，这行字就会打印 -------");
        // 模式匹配
        match stream {
            // 当Result 枚举类型匹配Ok时
            Ok(stream) => {
                // 如果链接成功，开启一个新的线程，之所以用多线程的原因是TCP客户请求可能有多个。
                thread::spawn(move || {
                    // 将客户端处理信息解耦到 handle_client 函数中，并移交 stream 变量所有权
                    handle_client(stream);
                });
            }
            // 当Result 枚举匹配错误时
            Err(e) => { panic!("妈蛋！遇到错误 {:?}", e) }
        }
    }

    drop(listener);
}

fn handle_client(mut stream: TcpStream) {
    println!("DEBUG::客户链接处理中。");
    // 定义一个存储用的数组
    let mut buf = [0; 512];
    // 建立一个循环，来反复读取客户的输入信息
    loop {
        // 通过read方法
        let bytes_read = stream.read(&mut buf).expect("读取出现错误，这里直接中断程序运行");
        // 输出调试信息
        println!("DEBUG::byte size: {}", bytes_read);
        // 如果输入流的字符长度是直接退出循环。
        if bytes_read == 0 {
            // 退出loop
            break;
        }
        let s = match str::from_utf8(&buf[..bytes_read]) {
            // 如果转换成功返回字符串值。
            Ok(v) => v,
            // 遇到转换错误输出错误信息，终止程序运行。
            Err(e) => { // 输出调试信息。
                stream.write(b"Need utf-8 sequence.").unwrap();
                // 继续监听
                continue;
            },
        };
        // 为了防止越界所以需要先判断 s.len() >= 3
        if s.len() >= 3 && s[0..3] == "bye".to_string() {
            // 输出终止前的消息。
            stream.write(b"Bye bye and see you soon.\n").unwrap();
            // 直接跳出 loop 循环
            break;
        } // 如果程序没有终止，返回输入的消息，也就是输入什么返回什么，unwrap() 表示不处理错误，遇到错误直接出错退出程序。
        stream.write(&buf[..bytes_read]).unwrap();
    }
}