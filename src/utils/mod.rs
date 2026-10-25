//! Provides buffered Unix-socket codecs for Wayland values and file descriptors.
//!
//! The reader and writer operate on nonblocking sockets registered with Tokio.
//! Message framing and protocol object validation are handled by the protocol
//! and socket layers; payload wrappers enforce each message's byte quota.

pub mod reader;
pub mod writer;

pub(crate) fn stream_to_async_fd(
    stream: tokio::net::UnixStream,
) -> std::io::Result<(
    tokio::io::unix::AsyncFd<std::os::unix::net::UnixStream>,
    tokio::io::unix::AsyncFd<std::os::unix::net::UnixStream>,
)> {
    let read_stream = stream.into_std()?;
    let write_stream = read_stream.try_clone()?;

    // SAFETY: Each UnixStream owns a valid descriptor that remains unchanged
    // and open while owned by its AsyncFd.
    Ok(unsafe {
        (
            tokio::io::unix::AsyncFd::register(read_stream)?,
            tokio::io::unix::AsyncFd::register(write_stream)?,
        )
    })
}
