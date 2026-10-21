use crate::protocol::{ClientId, CommandRequest, DisconnectReason, ServerError};

/// Reports connection lifecycle changes, incoming requests, and transport errors.
#[derive(Debug)]
pub enum CommandRecv {
    /// A client connection was accepted.
    Connect {
        /// Connection associated with this notification.
        client_id: ClientId,
    },
    /// A client sent a decoded core-protocol request.
    Request {
        /// Connection associated with this notification.
        client_id: ClientId,
        /// Decoded core-protocol message.
        command: CommandRequest,
    },
    /// The transport encountered a protocol or socket error.
    Error {
        /// Failure reported by the transport.
        protocol_error: ServerError,
    },
    /// A client connection was closed.
    Disconnect {
        /// Connection associated with this notification.
        client_id: ClientId,
        /// Origin or failure that caused the connection to close.
        reason: DisconnectReason,
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
