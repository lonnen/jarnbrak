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


/* HTTP Request
Method Request-URI HTTP-Version CRLF
headers CRLF
message-body
*/

/* HTTP response
HTTP-Version Status-Code Reason-Phrase CRLF
headers CRLF
message-body
*/


fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];

    stream.read(&mut buffer).unwrap();

    let response = "HTTP/1.1 200 OK\r\n\r\n";

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
