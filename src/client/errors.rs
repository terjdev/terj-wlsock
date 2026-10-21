use thiserror::Error;

/// Reports invalid options, a missing Tokio context, or stream preparation failures.
#[derive(Error, Debug)]
pub enum CreationError {
    #[error("Invalid send queue limit.")]
    InvalidSendQueueLimit,

    #[error("Invalid recv queue limit.")]
    InvalidRecvQueueLimit,

    #[error("No current Tokio runtime is available.")]
    InvalidAsyncContext,

    /// Preparing the connected Unix stream for asynchronous I/O failed.
    #[error("Failed to prepare the client stream for asynchronous I/O: {0}")]
    ConfigureStream(#[from] std::io::Error),
}

/// TODO
#[derive(Error, Debug)]
pub enum ShutdownError {}
