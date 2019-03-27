#![feature(futures_api, async_await, await_macro)]
extern crate fahrenheit;
extern crate futures;
extern crate rand;

use rand::random;

use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::thread;
use std::time;

use fahrenheit::AsyncTcpListener;
use fahrenheit::AsyncTcpStream;
use futures::io::AsyncReadExt;
use futures::stream::StreamExt;
use std::net::SocketAddr;

async fn listen(addr: &str) {
    let addr: SocketAddr = addr.parse().unwrap();
    let listener = AsyncTcpListener::bind(addr).unwrap();
    let mut incoming = listener.incoming();

    while let Some(stream) = await!(incoming.next()) {
        fahrenheit::spawn(process(stream));
    }
}

async fn process(mut stream: AsyncTcpStream) {
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

fn main() {
    fahrenheit::run(listen("127.0.0.1:8080"));
}
