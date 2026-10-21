use crate::protocol;

/// Reports connection lifecycle changes, incoming requests, and transport errors.
#[derive(Debug)]
pub enum CommandRecv {
    Event { command: protocol::CommandEvent },
    Disconnect { reason: protocol::DisconnectReason },
}

#[cfg(test)]
mod tests {
    use super::CommandRecv;

    #[test]
    fn command_recv_size_stays_within_budget() {
        let size = std::mem::size_of::<CommandRecv>();
        assert!(
            size <= 128,
            "CommandRecv occupies {size} bytes, exceeding the 128-byte queue budget; review large variants"
        );
    }
}
