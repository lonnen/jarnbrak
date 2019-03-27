# Jarnbrak is an HTTP tarpit written in Rust

It implements an HTTP server that holds open HTTP connections by slowly sending random HTTP headers forever.

This was written as a learning exercise in Rust and should not be used in a production setting.

To try it yourself, build and run using Cargo. The server will attempt to bind `127.0.0.1:8080` and allocate a threadpool of 4 threads.

### Reference Materials
* [Wellon's Endlessh writeup](https://nullprogram.com/blog/2019/03/22/)
* [Rust Book's Multithreaded Web Server Example](https://doc.rust-lang.org/1.30.0/book/second-edition/ch20-00-final-project-a-web-server.html)
* [Tomayko's Unicorn is Unix](https://tomayko.com/blog/2009/unicorn-is-unix)
* [@polachok's Fahrenheit Server](https://rust-lang-nursery.github.io/futures-rs/blog/2018/08/17/toykio.html)
