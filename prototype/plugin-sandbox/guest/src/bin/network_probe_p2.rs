//! WASI 0.2 guest: attempts a real TCP connect through the WASI sockets API.
//! Stdout markers: P2_CONNECT_OK / P2_CONNECT_DENIED::<error>.

use std::net::TcpStream;

fn main() {
    let port: u16 = std::env::args()
        .nth(1)
        .and_then(|value| value.parse().ok())
        .unwrap_or(9);
    match TcpStream::connect(("127.0.0.1", port)) {
        Ok(_) => println!("P2_CONNECT_OK"),
        Err(error) => println!("P2_CONNECT_DENIED::{error}"),
    }
}
