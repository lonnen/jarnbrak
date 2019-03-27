extern crate rand;

use rand::random;

use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::thread;
use std::time;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    for stream in listener.incoming() {
        handle_connection(stream.unwrap());
    }
}

fn handle_connection(mut stream: TcpStream) {
    stream.write(b"HTTP/1.1 200 OK\r\n").unwrap();

    loop {
        // pace is the trick
        thread::sleep(time::Duration::from_secs(5));

        let response = format!("X-{}: {}\r\n", random::<u8>(), random::<u8>());
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
        println!("response: {}", response);
    }
}
