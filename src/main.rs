extern crate rand;
use rand::random;

extern crate structopt;
use structopt::StructOpt;

extern crate jarnbrak;
use jarnbrak::ThreadPool;

use std::io::prelude::*;
use std::net::IpAddr;
use std::net::Ipv4Addr;
use std::net::SocketAddr;
use std::net::TcpListener;
use std::net::TcpStream;
use std::thread;
use std::time;

#[derive(StructOpt)]
struct Opt {
    /// Port 0 (default) will request that the OS assigns a port.
    #[structopt(short = "p", long = "port", default_value = "0")]
    port: u16,
}


fn main() {
    let opt = Opt::from_args();

    let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), opt.port);

    let listener = TcpListener::bind(addr).unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming().take(2) {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }

    println!("Shut it down.");
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
