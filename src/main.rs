use std::io::prelude::*;
use std::net::TcpStream;
use std::net::TcpListener;

fn main() {
    // note: we are doing nothing to handle potential failures here
    let listener = TcpListener::bind("127.0.0.1:2222").unwrap();

    for stream in listener.incoming() {
        handle_connection(stream.unwrap());
    }
}

fn handle_connection(mut stream: TcpStream) {
    stream.write(b"HTTP/1.1 200 OK\r\n").unwrap();

    loop {
        let (header, value) = ('1', '2');
        let response = format!("X-{}: {}\r\n", header, value);
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
        println!("response: {}", response);
    }
}
