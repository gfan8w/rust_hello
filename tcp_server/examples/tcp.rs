
use std::fs::File;
use std::io::{Result, Read, Write};
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::thread;
use std::time;


fn handle_client(mut stream: TcpStream) -> Result<()>{
    let mut buf = [0; 500]; //创建一个缓冲区，用来保存数据
        while match stream.read(&mut buf) { //读取数据，如果读取数据异常直接返回
            Ok(n) if n==0 => {
                false                 //如果读取到的长度是0，表示关闭连接，函数返回
            },
            Ok(n) => {
                println!("received:{}", String::from_utf8_lossy(&buf[..n]));
                stream.write(&buf[..n])?; //将读取到的内容返写到客户端去
                true
            },
            Err(e) => {
                false  //遇到错误，返回
            },
        }{}
    Ok(()) //没有异常，返回 void
}

pub fn tcp_server() -> std::io::Result<()> {
    //在本地8080端口创建TCP连接
    let addr = "127.0.0.1:8080".parse::<SocketAddr>().unwrap();
    let listener = TcpListener::bind(addr)?;

    //不断的接收连接进来，等同于循环的 listener.accept()
    for stream in listener.incoming() {
        //模式匹配，incoming得到的是一个Result,有可能是Ok，也有可能是Err。
        match stream {
            Ok(stream) =>{
                handle_client(stream)
                    .unwrap_or_else(|error| eprintln!("Server error got: {:?}", error));
                //如果遇到错误，会打印错误，错误由 stream的read，write 发出
            },
            Err(e) => eprintln!("connection failed!") //连接错误
        }
    }
    Ok(())
}

pub fn tcp_server_multi_thread() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080")?; //在本地8080端口创建TCP连接
    let mut thread_vec: Vec<thread::JoinHandle<()>> = Vec::new();

    for stream in listener.incoming() {
        let stream = stream.expect("failed!");
        //启动一个新线程来处理客户端的数据流
        let handle = thread::spawn(move || {        //move 使stream可以穿越移动
            handle_client(stream)
                .unwrap_or_else(|error| eprintln!("{:?}", error)); //如果遇到错误，会打印错误，错误由 stream的read，write 发出
        });

        thread_vec.push(handle);//把每个线程都保存起来
    }

    //等待，不要让服务端运行结束
    for handle in thread_vec {
        handle.join().unwrap();
    }

    Ok(())
}



/// client端 telnet localhost 8080，
/// client端如何优雅的退出：
///  ctrl+]，会返回到 telnet>
/// 然后输入 close 退出
pub fn main() -> std::io::Result<()> {
    tcp_server()
}


