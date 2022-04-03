use std::fmt::{Display, Formatter};
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::time;

#[derive(Debug)]
struct Thing;

impl Display for Thing {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "invalid first item to double")
    }
}

impl std::error::Error for Thing {}


fn handle_client(mut stream: TcpStream) -> Result<(), Box<dyn std::error::Error>>{
    let mut buf = [0; 500]; //创建一个缓冲区，用来保存数据

        while match stream.read(&mut buf) { //读取数据，如果读取数据异常直接返回
            Ok(n) if n==0 => {
                //"abc".parse::<u32>().map_err(|e|Thing);
                //std::error::Error::from("provide a file");
                Err::<(),_>("I am an error");
                //如果读取到的长度是0，表示关闭连接，函数返回
                eprintln!("received 0, end");

                false
            },
            Ok(n) => {
                //打印接收到的字符
                println!("received:{}", String::from_utf8_lossy(&buf[0..n]));
                //将读取到的内容返写到客户端去
                stream.write(&buf[..n])?;
                true
            },
            Err(e) => {
                eprintln!("socket read error:{:?}",e);
                false
            },
        }{}



    // loop { //循环读取
    //     let bytes_read = stream.read(&mut buf)?; //读取数据，如果读取数据异常直接返回
    //     if bytes_read == 0 {
    //         //如果读取到的长度是0，表示关闭连接，函数返回
    //         eprintln!("received 0, end");
    //         return Ok(());
    //     }
    //
    //     //打印接收到的字符
    //     println!("received:{}", String::from_utf8_lossy(&buf[0..bytes_read]));
    //
    //     stream.write(&buf[..bytes_read])?;  //将读取到的内容返写到客户端去
    //     //thread::sleep(time::Duration::from_secs(1 as u64));
    // }

    Ok(()) //没有异常，返回 void
}

pub fn tcp_server() -> std::io::Result<()> {
    println!("start tcp server");

    //在本地8080端口创建TCP连接
    let listener = TcpListener::bind("127.0.0.1:8080")?;

    //不断的接收链接进来，等同于 listener.accept()
    for stream in listener.incoming() {
        //模式匹配，incoming得到的是一个Result,有可能是Ok，也有可能是Err。
        match stream {
            Ok(stream) =>{
                let h =handle_client(stream);

                    h.unwrap_or_else(|error| eprintln!("Server error got:{:?}", error));
                //如果遇到错误，会打印错误，错误由 stream的read，write 发出
            },
            Err(e) => eprintln!("connection failed!") //链接错误
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


