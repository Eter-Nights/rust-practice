use std::io::{Read, Write};
use std::net::TcpStream;
use std::str;

fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:8088").unwrap();
    let send = String::from("hello world");
    stream.write(send.as_bytes()).unwrap();

    let mut buffer = [0; 100];
    let n = stream.read(&mut buffer).unwrap();

    println!(
        "Got response from server:{:?}",
        str::from_utf8(&buffer[..n]).unwrap()
    );
}
