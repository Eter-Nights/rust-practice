use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8088").await?;

    loop {
        let (mut socket, addr) = listener.accept().await?;
        println!("accepted connection from {:?}", addr);

        tokio::spawn(async move {
            let mut buf = vec![0; 1024];

            loop {
                match socket.read(&mut buf).await {
                    // 返回值为 `Ok(0)` 的话说明远程关闭了
                    Ok(0) => return,
                    Ok(n) => {
                        // 拷贝数据返回到套接字中
                        if socket.write_all(&buf[..n]).await.is_err() {
                            // 意料之外的套接字错误，我们不能为此干什么，
                            // 只能把服务先关了
                            return;
                        }
                    }
                    Err(_) => {
                        // 意料之外的套接字错误，我们不能为此干什么，
                        // 只能把服务先关了
                        return;
                    }
                }
            }
        });
    }
}