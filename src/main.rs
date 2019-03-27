use std::fs::File;
use std::io::prelude::*;
use std::net::TcpStream;
use std::net::TcpListener;

fn main() {
    // note: we are doing nothing to handle potential failures here
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";

    let (code, status_line, filename) = if buffer.starts_with(get) {
        ("200", "OK", "hello.html")
    } else {
        ("404", "NOT FOUND", "404.html")
    };

    let mut file = File::open(filename).unwrap();
    let mut contents = String::new();

    file.read_to_string(&mut contents).unwrap();

    let response = format!("HTTP/1.1 {} {}\r\n\r\n{}", code, status_line, contents);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
