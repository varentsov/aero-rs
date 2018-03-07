pub struct Header {
    pub version: u8,
    pub message_type: MessageType,
    pub size: u8,
}

pub enum MessageType {
    Info,
    Message,
}

pub struct InfoMessage {
    build
    edition
    node
    replicas-read
    replicas-write
    service
    services
    statistics
    version
}