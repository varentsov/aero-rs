mod header;

use header::{Header, MessageType};

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn has_header_type() {
        let h = Header{
            version: 2,
            message_type: MessageType::Info,
            size: 0,
        };
    }
}
