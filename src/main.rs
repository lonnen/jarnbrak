extern crate rand;

use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

use rand::random;

fn main() {
    // note: we are doing nothing to handle potential failures here
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    for stream in listener.incoming() {
        handle_connection(stream.unwrap());
    }
}

fn handle_connection(mut stream: TcpStream) {
    stream.write(b"HTTP/1.1 200 OK\r\n").unwrap();

    loop {
        let response = format!("X-{}: {}\r\n",
            random::<u8>(),
            random::<u8>()
        );
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
        println!("response: {}", response);
    }
}
