use thiserror::Error;

/// Reports invalid options, a missing Tokio context, or stream preparation failures.
#[derive(Error, Debug)]
pub enum CreationError {
    /// The outgoing request queue capacity was zero.
    #[error("Invalid send queue limit.")]
    InvalidSendQueueLimit,

    /// The incoming notification queue capacity was zero.
    #[error("Invalid recv queue limit.")]
    InvalidRecvQueueLimit,

    /// No current Tokio runtime handle was available.
    #[error("No current Tokio runtime is available.")]
    InvalidAsyncContext,

    /// Preparing the connected Unix stream for asynchronous I/O failed.
    #[error("Failed to prepare the client stream for asynchronous I/O: {0}")]
    ConfigureStream(#[from] std::io::Error),
}

/// Reports failure to await completion of the client task.
#[derive(Error, Debug)]
pub enum ClosingError {
    /// The client task panicked or was aborted before close completed.
    #[error("The client task failed to complete close.")]
    Failed,
}
