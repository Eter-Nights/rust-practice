use crate::http_request::HttpRequest;
use crate::router::Router;
use std::io::prelude::*;
use std::net::TcpListener;
use std::str;

pub struct Server<'a> {
    socket_addr: &'a str,
}

impl<'a> Server<'a> {
    pub fn new(socket_addr: &'a str) -> Self {
        Server { socket_addr }
    }
    pub fn run(&self) {
        let connection_listener = TcpListener::bind(self.socket_addr).unwrap();
        println!("Listening on {}", self.socket_addr);

        for stream in connection_listener.incoming() {
            let mut stream = stream.unwrap();
            println!("Connection established");

            let mut buffer = [0; 1024];
            stream.read(&mut buffer).unwrap();

            let request: HttpRequest = String::from_utf8(buffer.to_vec()).unwrap().into();
            Router::route(request, &mut stream);
        }
    }
}
