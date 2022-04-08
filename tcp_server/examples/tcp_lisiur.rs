use std::{net::{TcpStream, TcpListener}, io::{Read, Write}};
use std::thread;

// 处理 client 的 tcp stream
fn handle_client(stream: &mut TcpStream) -> std::io::Result<()> {
    // 声明一个 128 字节的缓冲区
    let mut buf = [0 as u8; 128];
    // 持续监听 client stream 的消息，并将其读取到缓冲区中
    loop {
        match stream.read(&mut buf) {
            Ok(size) => {
                // 打印读取到的消息
                println!("received: {}", String::from_utf8_lossy(&buf[0..size]).to_string());
                // 将读取到的消息原封不到的写回
                stream.write(&buf[0..size])?;
            }
            Err(error) => {
                eprintln!("{:?}", error);
            }
        }
    }
}

fn main() -> std::io::Result<()> {
    // 创建一个 TCP 服务监听 8080 端口
    let listener = TcpListener::bind("127.0.0.1:8080")?;

    // 处理客户端的 TCP 连接
    for stream in listener.incoming() {
        let mut stream = stream.expect("failed!");
        // 对每一个客户端的连接启动一个线程处理
        thread::spawn(move || {
            // 处理 client 的 tcp stream
            handle_client(&mut stream).unwrap_or_else(|error| eprintln!("{:?}", error));
        });
    }
    Ok(())
}