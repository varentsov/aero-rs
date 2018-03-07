pub struct Header {
    pub version: u8,
    pub message_type: MessageType,
    pub size: [u8; 6],
}

#[repr(u8)]
pub enum MessageType {
    Info,
    Message,
}

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