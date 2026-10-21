/// Configures bounded command queues; both limits default to 128 messages.
pub struct Options {
    /// Capacity of the server command queue and each client's outgoing event queue.
    ///
    /// Must be nonzero. A failed attempt to enqueue an event into a full client
    /// queue requests that client's disconnection.
    pub send_queue_limit: usize,
    /// Capacity of the shared notification queue delivered to the application.
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
