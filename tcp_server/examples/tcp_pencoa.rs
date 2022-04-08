use std::net::{TcpListener, TcpStream, SocketAddr};
use std::io::{Read, Write};
use std::str;


fn main() {
    // 声明地址
    let addrs = SocketAddr::from(([127, 0, 0, 1], 7878));

    // 绑定TcpListener的监听地址，做标准的错误处理
    let listener = match TcpListener::bind(addrs) {
        Ok(l) => l, // 成功就返回
        Err(e) => panic!("Failed to bind, {}", e), // 错误就panic
    };

    // 返回监听地址连接的TcpStream迭代器
    for stream in listener.incoming() {
        // 对TcpSteam做错误处理
        match stream {
            Ok(stream) => handle_connection(stream), // 成功就调用handle_connection函数
            Err(e) => {
                println!("Failed to connect, {}", e); // 错误就打印，继续监听
                continue
            },
        }
    }
}

// handle_connection函数，处理连接。传入的stream是字节流
fn handle_connection(mut stream: TcpStream) {
    // 声明1024字节的buffer，用于存储传入的stream字节流
    let mut buffer = [0; 1024];
    // 将stream字节流存入buffer中
    stream.read(&mut buffer).unwrap();
    // 该变量用于记录传入字节流的数据边界
    let mut buffer_length = 1024;
    // 逆向迭代，找到buffer中不是初始值0的位置
    for (i, num) in buffer.iter().rev().enumerate() {
        // 条件比较
        if *num != 0 {
            // 计算
            buffer_length = 1024 - i;
            // 中止
            break
        }
    }
    // 将传入的字节流转为字符串打印
    println!("{:?}", str::from_utf8(&buffer[..buffer_length]).unwrap());
    // 将字节流写入输出回stream
    stream.write(&buffer).unwrap();
}