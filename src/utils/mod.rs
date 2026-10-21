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
