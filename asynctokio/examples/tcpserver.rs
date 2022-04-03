
/*
brew install socat

创建一个 tcp socket 服务端 监听在 6142 :

socat TCP-LISTEN:6142,fork stdout

连接到 6142  :
telnet localhost 6142，

然后随意输入内容


*/

use std::net::SocketAddr;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

/// Echo服务器
#[tokio::main] // 该宏创建了一个tokio运行时环境
async fn main() ->Result<(), Box<dyn std::error::Error>> {
    // === tcp 服务端 ===
    eprintln!("tcp server started");

    //把一个string 转换为一个SocketAddr
    let addr = "127.0.0.1:6143".parse::<SocketAddr>()?;

    server_1(&addr).await?;

    Ok(())

}

///Echo 服务，参考：https://docs.rs/tokio/latest/tokio/
async fn server_1(addr: &SocketAddr) -> std::io::Result<()> {
    let listener = TcpListener::bind(addr).await?;

    //可以不断的接收链接
    loop {
        let (mut socket, _) =listener.accept().await?;

        tokio::spawn(async move {
            let mut buf =[0;10];

            // read一次可能因为buf 不够大，无法一次性读完，使用loop
            loop {
                //n 实际接收到的数据，然后写回n长度的数据
                let n = match socket.read( &mut buf).await {
                    //socket read out
                    Ok(n) if n==0 =>{
                        eprintln!("read end!");
                        return
                    } ,
                    Ok(n) => {
                        println!("received:{}",String::from_utf8_lossy(&buf[0..n]));
                        n
                    },
                    Err(e) =>{
                        eprintln!("failed to read from socket; err = {:?}", e);
                        return;
                    }
                };

                //write data back
                if let Err(e) = socket.write_all(&buf[0..n]).await {
                    eprintln!("failed to write to socket; err = {:?}", e);
                    return;
                }
            }
        });
    }

}

// 参考：https://www.rectcircle.cn/posts/rust-tokio/
async fn server_2(addr: SocketAddr) -> std::io::Result<()> {
    let mut listener = TcpListener::bind(&addr).await?;
    println!("Server running on {}", addr);

    // 以下实现通过 Stream 方式实现
    // async {
    //     // 将 TcpListener 转换为一个 Stream
    //     let incoming = listener.incoming();
    //     // 等待 stream 获取到数据（即客户端的连接）
    //     // 依赖 tokio 的 stream feature 和 futures = "0.3"
    //     while let Some(conn) = incoming.next().await {
    //         match conn {
    //             Err(e) => eprintln!("accetp failed:{:?}", e),
    //             Ok(socket) => {
    //                 // 当收到一个Tcp连接时，提交一个 Future 到 tokio 运行时
    //                 tokio::spawn(async {
    //                     // 获取到读写句柄
    //                     let (reader, writer) =socket.split();
    //
    //                     // 将接收到的数据写回，完成Echo
    //                     // 使用 tokio::io::copy 方法，同样该方法是异步的
    //                     match tokio::io::copy(reader, writer).await {
    //                         Ok(amt) => {
    //                             println!("wrote {} bytes", amt);
    //                         },
    //                         Err(e) => {
    //                             eprintln!("IO error:{:?}", e);
    //                         }
    //                     }
    //                 })
    //             }
    //         }
    //     }
    // }.await;

    Ok(())
}






