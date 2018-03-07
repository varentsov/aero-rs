extern crate aero_rs;

use aero_rs::*;

use std::io::prelude::*;
use std::net::TcpStream;

#[test]
fn has_header_type() {
    let h = Header{
        version: 2,
        message_type: MessageType::Info,
        size: 0,
    };
}

#[test]
fn connect_to_aerospike() {
    let mut stream = TcpStream::connect("127.0.0.1:3000").unwrap();

    let _ = stream.write(&[1]);
    let _ = stream.read(&mut [0; 128]);
}
