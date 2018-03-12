extern crate aero_rs;

use aero_rs::*;

use std::io::prelude::*;
use std::net::TcpStream;
use std::time::Duration;

#[test]
fn connect_to_aerospike() {
    let mut stream = TcpStream::connect("127.0.0.1:3000").unwrap();
    stream.set_read_timeout(Some(Duration::new(3, 0))).unwrap();
    stream.set_write_timeout(Some(Duration::new(3, 0))).unwrap();

    let info_header = ProtocolHeader::new_blank(MessageType::Info);
    let bytes = info_header.serialize();

    let mut resp: [u8; 8] = [0; 8];
    stream.write(&bytes).unwrap();
    stream.read_exact(&mut resp).unwrap();
    println!("response: {:?}", resp);

    let recv_header = ProtocolHeader::deserialize(resp);
    println!("response header: {:?}", recv_header);
    println!("response datalen: {:?}", recv_header.datalen());

    let mut body: Vec<u8> = vec![0; recv_header.datalen() as usize];
    let readed = stream.read(&mut body).unwrap();
    let info_response = InfoResponse::from_bytes(&body);
    println!("body: {:?}", info_response);

}
