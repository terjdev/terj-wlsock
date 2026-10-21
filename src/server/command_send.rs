use crate::protocol::{ClientId, CommandEvent};

/// Sends an event or requests disconnection of a client.
///
/// Successful channel submission only queues the command for server processing;
/// it does not acknowledge delivery to the client.
#[derive(Debug)]
pub enum CommandSend {
    /// Queues a core-protocol event for the specified client.
    Event {
        /// Connection that receives the event.
        client_id: ClientId,
        /// Core-protocol message to encode.
        command: CommandEvent,
    },
    /// Requests disconnection, reported through [`super::CommandRecv::Disconnect`].
    Disconnect {
        /// Client to disconnect.
        client_id: ClientId,
    },
}

#[cfg(test)]
mod tests {
    use super::CommandSend;

    #[test]
    fn command_send_size_stays_within_budget() {
        let size = std::mem::size_of::<CommandSend>();
        assert!(
            size <= 128,
            "CommandSend occupies {size} bytes, exceeding the 128-byte queue budget; review large variants"
        );
    }
}
