/// Configures bounded command queues; both limits default to 128 messages.
#[derive(Debug)]
pub struct Options {
    /// Capacity of the outgoing request queue.
    ///
    /// Must be nonzero. Asynchronous sends wait for capacity when the queue is full;
    /// filling this queue does not automatically disconnect the client.
    pub send_queue_limit: usize,
    /// Capacity of the incoming event and disconnect notification queue.
    ///
    /// Must be nonzero. Producers wait when this queue is full, so the application
    /// must keep draining it during normal operation and shutdown.
    pub recv_queue_limit: usize,
}

impl Default for Options {
    fn default() -> Self {
        Self {
            send_queue_limit: 128,
            recv_queue_limit: 128,
        }
    }
}
