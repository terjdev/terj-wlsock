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

    let fd_read = tokio::io::unix::AsyncFd::new(read_stream)?;
    let fd_write = tokio::io::unix::AsyncFd::new(write_stream)?;

    Ok((fd_read, fd_write))
}
