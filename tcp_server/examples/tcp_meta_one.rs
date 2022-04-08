use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

fn main() {
    // 创建TcpListener， 并监听127.0.0.1:8080
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    // 获取stream，并作处理
    for stream in listener.incoming() {
        match stream {
            // 有正确结果处理方式
            Ok(st) => handle_connection(st),
            // 错误继续等待
            Err(_) => continue,
        }
    }
}

fn handle_connection(mut stream: TcpStream) {
    // 定义buffer
    let mut buffer = [0; 1024];
    // 读取请求内容
    stream.read(&mut buffer).unwrap();
    // 定义get 请求头
    let get = b"GET / HTTP/1.1\r\n";
    // 判断是否为get请求
    let status_line = if buffer.starts_with(get) {
        "HTTP/1.1 200 OK"
    } else {
        "HTTP/1.1 404 NOT FOUND"
    };
    // 将buffer转 String
    let contents = String::from_utf8_lossy(&buffer[..]);

    // 创建返回值
    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );
    // 返回数据
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}