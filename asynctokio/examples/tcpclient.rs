
/*
brew install socat

创建一个 tcp socket 服务端 监听在 6142 :

socat TCP-LISTEN:6142,fork stdout

连接到 6142  :
telnet localhost 6142，

然后随意输入内容


*/

use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;

#[tokio::main] // 该宏创建了一个tokio运行时环境
async fn main() ->std::io::Result<()> {
    // === tcp 客户端 ===
    // 创建一个Tcp连接
    let mut stream = TcpStream::connect("localhost:6142").await?;
    println!("connect to tcp server");

    let result = stream.write(b"hello world\n").await?;
    println!("write to server; success:{:?}", result);

    stream.shutdown().await?;

    Ok(())

}








