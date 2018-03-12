extern crate byteorder;

use self::byteorder::{BigEndian, WriteBytesExt, ReadBytesExt};
use std::io::{Write, Cursor};

#[derive(Clone, Debug)]
pub struct Header {
    pub version: u8,
    pub message_type: MessageType,
    // size in network byte order
    pub size: [u8; 6],
}

#[repr(u8)]
#[derive(Copy, Clone, Debug)]
pub enum MessageType {
    Info = 1,
    Message = 3,
}

#[derive(Clone, Debug)]
pub struct InfoMessage {
    build: String,
    edition: String,
    node: String,
    replicas_read: String,
    replicas_write: String,
    service: String,
    services: String,
    statistics: String,
    version: String,
}

impl Header {
    pub fn new_blank(m_type: MessageType) -> Header {
        Header{
            version: 2,
            message_type: m_type,
            size: msg_len_to_bytes(0),
        }
    }

    pub fn deserialize(buf: [u8; 8]) -> Header {
        let version = unsafe { *buf.get_unchecked(0) };
        let msg_type = unsafe { *buf.get_unchecked(1) };
        let message_type = match msg_type {
            1 => MessageType::Info,
            3 => MessageType::Message,
            _ => panic!("Header#deserialize: Unknown message type")
        };
        let mut size: [u8; 6] = [0; 6];
        size.copy_from_slice(&buf[2..]);
        Header {
            version,
            message_type,
            size,
        }
    }

    pub fn serialize(&self) -> [u8; 8] {
        let mut buf: [u8; 8] = [0; 8];
        {
            let mut cursor = Cursor::new(&mut buf[..]);
            cursor.write_u8(self.version).unwrap();
            cursor.write_u8(self.message_type as u8).unwrap();
            for b in &self.size {
                cursor.write_u8(*b).unwrap();
            }
        }
        buf
    }

    pub fn datalen(&self) -> u64 {
        msg_len_from_bytes(&self.size)
    }
}

fn msg_len_to_bytes(data_len: u64) -> [u8; 6] {
    let mut buf: [u8; 8] = [0; 8];
    {
        let mut cursor = Cursor::new(&mut buf[..]);
        cursor.write_u64::<BigEndian>(data_len).unwrap();
    }
    let mut ary: [u8; 6] = Default::default();
    ary.copy_from_slice(&buf[2..]);
    ary
}

fn msg_len_from_bytes(src: &[u8; 6]) -> u64 {
    let mut buf: [u8; 8] = [0; 8];
    for (inxex, b) in src.iter().enumerate() {
        unsafe {
            *buf.get_unchecked_mut(inxex + 2) = *b;
        }
    }
    let mut cursor = Cursor::new(&buf[..]);
    cursor.read_u64::<BigEndian>().unwrap()
}