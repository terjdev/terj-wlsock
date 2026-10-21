use thiserror::Error;

use crate::server::CommandRecv;

/// Reports invalid options or a missing Tokio context when creating a server.
#[derive(Error, Debug)]
pub enum CreationError {
    /// The outgoing queue capacity was zero.
    #[error("Invalid send queue limit.")]
    InvalidSendQueueLimit,

    /// The notification queue capacity was zero.
    #[error("Invalid recv queue limit.")]
    InvalidRecvQueueLimit,

    /// No current Tokio runtime handle was available.
    #[error("Invalid async context.")]
    InvalidAsyncContext,
}

/// Reports failure to await normal completion of the server task.
#[derive(Error, Debug)]
pub enum ShutdownError {
    /// The server task panicked or was aborted before shutdown completed.
    #[error("The server task failed to complete shutdown.")]
    Failed,
}

#[derive(Error, Debug)]
pub(crate) enum AcceptError {
    #[error("Failed to acquire client id.")]
    IndexOverflow,

    #[error("{0}")]
    ServerRequestQueueClosed(#[from] tokio::sync::mpsc::error::SendError<CommandRecv>),
}

#[derive(Error, Debug)]
pub(crate) enum SendCommandError {
    #[error("{0}")]
    ServerRequestQueueClosed(#[from] tokio::sync::mpsc::error::SendError<CommandRecv>),
}
