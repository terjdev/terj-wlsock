use crate::protocol;

/// Sends an event or requests disconnection of a client.
///
/// Successful channel submission only queues the command for server processing;
/// it does not acknowledge delivery to the client.
#[derive(Debug)]
pub enum CommandSend {
    Request { command: protocol::CommandRequest },
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
