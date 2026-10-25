use crate::protocol;

/// Reports incoming compositor events and the end of the client connection.
#[derive(Debug)]
pub enum CommandRecv {
    /// The compositor sent a decoded core-protocol event.
    Event {
        /// Event containing its receiving object ID and wire arguments.
        command: protocol::CommandEvent,
    },
    /// The connection was closed and socket I/O has ended.
    Disconnect {
        /// Endpoint or failure that caused the connection to close.
        reason: protocol::DisconnectReason,
    },
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
