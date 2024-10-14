use futures_util::stream::{SplitSink, SplitStream};
use futures_util::{SinkExt, StreamExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::mpsc;
use tokio_util::codec::{Framed, LinesCodec};

type LineFramedStream = SplitStream<Framed<TcpStream, LinesCodec>>;
type LineFramedSink = SplitSink<Framed<TcpStream, LinesCodec>, String>;

#[tokio::main]
async fn main() {
    let server = TcpListener::bind("127.0.0.1:8088").await.unwrap();
    while let Ok((client_stream, _client_addr)) = server.accept().await {
        tokio::spawn(async move {
            process_client(client_stream).await;
        });
    }
}

async fn process_client(client_stream: TcpStream) {
    // 将TcpStream转换为Framed
    let framed = Framed::new(client_stream, LinesCodec::new());
    // 将Framed分离，可得到独立的读写端
    let (frame_writer, frame_reader) = framed.split::<String>();
    // 当Reader从客户端读取到数据后，发送到通道中，
    // 另一个异步任务读取该通道，从通道中读取到数据后，将内容按行写给客户端
    let (msg_tx, msg_rx) = mpsc::channel::<String>(100);

    // 负责读客户端的异步子任务
    let mut read_task = tokio::spawn(async move {
        read_from_client(frame_reader, msg_tx).await;
    });

    // 负责向客户端写行数据的异步子任务
    let mut write_task = tokio::spawn(async move {
        write_to_client(frame_writer, msg_rx).await;
    });

    // 无论是读任务还是写任务的终止，另一个任务都将没有继续存在的意义，因此都将另一个任务也终止
    if tokio::try_join!(&mut read_task, &mut write_task).is_err() {
        eprintln!("read_task/write_task terminated");
        read_task.abort();
        write_task.abort();
    };
}

async fn read_from_client(mut reader: LineFramedStream, msg_tx: mpsc::Sender<String>) {
    loop {
        match reader.next().await {
            None => {
                println!("client closed");
                break;
            }
            Some(Err(e)) => {
                eprintln!("read from client error: {}", e);
                break;
            }
            Some(Ok(str)) => {
                println!("read from client. content: {}", str);
                // 将内容发送给writer，让writer响应给客户端，
                // 如果无法发送给writer，继续从客户端读取内容将没有意义，因此break退出
                if msg_tx.send(str).await.is_err() {
                    eprintln!("receiver closed");
                }
            }
        }
    }
}

async fn write_to_client(mut writer: LineFramedSink, mut msg_rx: mpsc::Receiver<String>) {
    while let Some(str) = msg_rx.recv().await {
        if writer.send(str).await.is_err() {
            eprintln!("write to client failed");
            break;
        }
    }
}
