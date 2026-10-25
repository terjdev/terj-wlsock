use crate::protocol;

/// Queues requests for transmission to the Wayland compositor.
///
/// Successful channel submission only queues a request for encoding and writing;
/// it does not acknowledge delivery or processing by the compositor.
#[derive(Debug)]
pub enum CommandSend {
    /// Sends a core-protocol request on this connection.
    Request {
        /// Request containing its receiving object ID and wire arguments.
        command: protocol::CommandRequest,
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
