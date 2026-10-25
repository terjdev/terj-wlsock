//! Writes native-endian Wayland values and file descriptors to Unix sockets.

use std::{
    io::{self, IoSlice},
    mem::MaybeUninit,
    os::{fd::AsFd, fd::OwnedFd, unix::net::UnixStream},
};

use rustix::net::{SendAncillaryBuffer, SendAncillaryMessage, SendFlags, sendmsg};
use thiserror::Error;
use tokio::io::unix::AsyncFd;

// Fixed storage batches small messages without allocating during writes.
const BYTES_CAPACITY: usize = 8192;
// Linux SCM_RIGHTS permits at most 253 descriptors in one ancillary message.
const FDS_CAPACITY: usize = 253;
const ANCILLARY_CAPACITY: usize = rustix::cmsg_space!(ScmRights(FDS_CAPACITY));

/// Reports socket, encoding, descriptor, and payload-quota failures while writing.
#[derive(Error, Debug)]
pub enum FdWriterError {
    /// An underlying Unix-socket operation failed.
    #[error("Failed to write to the Unix socket: {0}")]
    Io(#[source] io::Error),

    /// The writer has already detected that its connection is closed.
    #[error("The Unix socket connection is closed.")]
    ConnectionClosed,

    /// A socket send returned zero despite having ordinary bytes to transmit.
    #[error("The Unix socket accepted zero bytes from a nonempty write.")]
    WriteZero,

    /// A value could not be represented as a signed Wayland 24.8 fixed-point number.
    #[error("The fixed-point value must be finite and within the signed 24.8 range.")]
    InvalidFixed,

    /// An operation requested more bytes than its remaining payload quota.
    #[error("Payload quota exceeded: requested {requested} bytes, but only {remaining} remain.")]
    QuotaExceeded {
        /// Number of bytes requested by the operation.
        requested: usize,
        /// Number of bytes available in the payload quota.
        remaining: usize,
    },

    /// The control-message buffer could not hold the queued descriptors.
    #[error("The ancillary buffer is too small for the queued file descriptors.")]
    AncillaryBufferTooSmall,

    /// Descriptors could not be sent because no ordinary bytes were buffered.
    #[error("Cannot send file descriptors without at least one ordinary byte.")]
    FdsWithoutBytes,

    /// Writing cannot continue after a failure, cancelled send, or incomplete payload.
    #[error("The writer cannot continue after an error, cancelled write, or incomplete payload.")]
    WriterFailed,

    /// An outgoing value, message layout, or object metadata was invalid.
    #[error("Invalid protocol.")]
    InvalidProtocol,
}

/// Describes the lifecycle of buffered socket output.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FdWriterState {
    /// The writer accepts new values and flush requests.
    Open,
    /// An operation is in progress; cancellation leaves this state unrecoverable.
    Writing,
    /// A send detected that the connection is closed.
    Closed,
    /// An unrecoverable error prevents further writing.
    Failed,
}

/// Inline buffered bytes and progress through the current send.
struct BytesStorage {
    raw: [u8; BYTES_CAPACITY],
    len: usize,
    sent: usize,
}

impl BytesStorage {
    fn new() -> Self {
        Self {
            raw: [0; BYTES_CAPACITY],
            len: 0,
            sent: 0,
        }
    }

    /// Appends a complete frame without I/O when it fits in the remaining space.
    /// The caller checked the writer state, wire length, padding and any byte quota.
    /// `total` is four prefix bytes plus the body and initialized suffix bytes.
    /// Publishing the frame is synchronous, so cancellation cannot split it.
    fn try_buffer_frame(&mut self, value: &[u8], wire_len: u32, total: usize) -> bool {
        if total > BYTES_CAPACITY - self.len {
            return false;
        }
        let start = self.len;
        self.raw[start..start + 4].copy_from_slice(&wire_len.to_ne_bytes());
        self.raw[start + 4..start + 4 + value.len()].copy_from_slice(value);
        self.raw[start + 4 + value.len()..start + total].fill(0);
        self.len = start + total;
        true
    }
}

/// Inline control-message storage and descriptors owned until a successful send.
struct AncillaryStorage {
    raw: [MaybeUninit<u8>; ANCILLARY_CAPACITY],
    buf: [Option<OwnedFd>; FDS_CAPACITY],
    len: usize,
}

impl AncillaryStorage {
    fn new() -> Self {
        Self {
            raw: [MaybeUninit::uninit(); ANCILLARY_CAPACITY],
            buf: [const { None }; FDS_CAPACITY],
            len: 0,
        }
    }

    /// Sends bytes and queued descriptors without waiting for socket readiness.
    /// The caller records a successful send before clearing descriptor ownership.
    fn send(
        &mut self,
        socket: &UnixStream,
        iov: &[IoSlice<'_>],
    ) -> io::Result<Result<usize, FdWriterError>> {
        // Avoid constructing the borrowed-FD array for ordinary writes.
        if self.len != 0 {
            let mut fds = [socket.as_fd(); FDS_CAPACITY];
            for (borrowed, owned) in fds[..self.len].iter_mut().zip(&self.buf[..self.len]) {
                let Some(owned) = owned.as_ref() else {
                    return Ok(Err(FdWriterError::WriterFailed));
                };
                *borrowed = owned.as_fd();
            }
            let mut control = SendAncillaryBuffer::new(&mut self.raw);
            if !control.push(SendAncillaryMessage::ScmRights(&fds[..self.len])) {
                return Ok(Err(FdWriterError::AncillaryBufferTooSmall));
            }
            return sendmsg(socket, iov, &mut control, SendFlags::NOSIGNAL)
                .map(Ok)
                .map_err(io::Error::from);
        }

        let mut control = SendAncillaryBuffer::new(&mut self.raw);
        sendmsg(socket, iov, &mut control, SendFlags::NOSIGNAL)
            .map(Ok)
            .map_err(io::Error::from)
    }

    fn clear(&mut self) {
        for slot in &mut self.buf[..self.len] {
            *slot = None;
        }
        self.len = 0;
    }
}

/// Buffered native-endian writes to a nonblocking Unix stream.
///
/// The socket must already be nonblocking and its Tokio runtime must remain
/// running. All buffers are fixed arrays; construction and buffering allocate
/// no heap memory. Headers, message sizes, strings, and padding are the caller's
/// responsibility. Descriptors are owned until sent or until this writer drops.
///
/// A full buffer is sent only when more space is needed, retaining its last
/// byte so descriptors appended after the bytes can still accompany data.
/// Call [`Self::flush`] explicitly to send that final byte and any other
/// buffered data. A full descriptor queue also sends buffered data, retaining
/// the last byte when more than one byte is available. If only one byte remains,
/// it is sent and newly queued descriptors wait for subsequent byte writes.
/// Dropping the writer discards buffers and closes owned descriptors;
/// it does not send buffered bytes.
///
/// Cancelling an asynchronous operation leaves the state as `Writing`:
/// buffered or sent prefixes cannot safely be repeated. The next operation
/// rejects continuation and marks the writer as failed.
pub struct FdWriter {
    fd: AsyncFd<UnixStream>,
    bytes: BytesStorage,
    ancillary: AncillaryStorage,
    state: FdWriterState,
}

impl FdWriter {
    /// Returns the encoded array size, including its length prefix and alignment.
    /// Returns None if the length cannot fit in u32 or size arithmetic overflows.
    /// This calculation does not require a socket or perform I/O.
    pub(crate) fn encoded_array_size(len: usize) -> Option<usize> {
        u32::try_from(len).ok()?;
        len.checked_add(3)
            .map(|n| n & !3)
            .and_then(|n| n.checked_add(4))
    }

    /// Returns the encoded nullable-string size, including the terminating NUL.
    /// NULL occupies only the four-byte length prefix. Returns None on overflow.
    pub(crate) fn encoded_string_size(value: Option<&str>) -> Option<usize> {
        match value {
            None => Some(4),
            Some(value) => Self::encoded_array_size(value.len().checked_add(1)?),
        }
    }

    /// Checks a non-null string before any part of an event is written.
    /// Message size and payload quota are checked separately by the transport.
    pub(crate) fn is_valid_string(value: &str) -> bool {
        !value.as_bytes().contains(&0) && Self::encoded_string_size(Some(value)).is_some()
    }

    /// Checks signed 24.8 representability without writing or changing stream state.
    pub(crate) fn is_valid_fixed(value: f64) -> bool {
        Self::encode_fixed(value).is_some()
    }

    /// Converts a finite signed 24.8 value, rounding halfway values away from zero.
    fn encode_fixed(value: f64) -> Option<i32> {
        let scaled = value * 256.0;
        if !scaled.is_finite() || scaled < f64::from(i32::MIN) || scaled > f64::from(i32::MAX) {
            return None;
        }
        Some(scaled.round() as i32)
    }

    /// Writes a UTF-8 string with its length, terminating NUL, and zero padding.
    /// A complete frame that fits is buffered synchronously without socket I/O.
    /// Rejects embedded NUL characters and validates the length before writing.
    ///
    /// # Errors
    /// Returns an error for embedded NUL, length overflow, an oversized value,
    /// insufficient payload quota, or a failed or closed stream.
    pub async fn next_string(&mut self, value: &str) -> Result<(), FdWriterError> {
        self.next_nullable_string(Some(value)).await
    }

    /// Writes a nullable string: None has length zero, and Some("") has length one.
    ///
    /// # Errors
    /// Returns an error for embedded NUL, length overflow, an oversized value,
    /// insufficient payload quota, or a failed or closed stream.
    pub async fn next_nullable_string(&mut self, value: Option<&str>) -> Result<(), FdWriterError> {
        self.check_state()?;
        let result = async {
            let Some(value) = value else {
                return self.next_u32(0).await;
            };
            if !FdWriter::is_valid_string(value) {
                return Err(FdWriterError::InvalidProtocol);
            }
            let len = value
                .len()
                .checked_add(1)
                .ok_or(FdWriterError::InvalidProtocol)?;
            let wire_len = u32::try_from(len).map_err(|_| FdWriterError::InvalidProtocol)?;
            let padding = (4 - len % 4) % 4;
            let total =
                FdWriter::encoded_string_size(Some(value)).ok_or(FdWriterError::InvalidProtocol)?;

            // A single primitive must fit within the maximum Wayland payload size.
            if total > (usize::from(u16::MAX) & !3) - 8 {
                return Err(FdWriterError::InvalidProtocol);
            }
            if self
                .bytes
                .try_buffer_frame(value.as_bytes(), wire_len, total)
            {
                return Ok(());
            }
            self.next_u32(wire_len).await?;
            self.next_bytes(value.as_bytes()).await?;
            self.next_bytes(&[0; 4][..1 + padding]).await
        }
        .await;
        if result.is_err() && self.state != FdWriterState::Closed {
            self.state = FdWriterState::Failed;
        }
        result
    }

    /// Writes an array with its byte length and zero padding to four-byte alignment.
    /// A complete frame that fits is buffered synchronously without socket I/O.
    /// The encoded array length excludes padding.
    ///
    /// # Errors
    /// Returns an error for length overflow, an oversized value, insufficient
    /// payload quota, or a failed or closed stream.
    pub async fn next_array(&mut self, value: &[u8]) -> Result<(), FdWriterError> {
        self.check_state()?;
        let result = async {
            let len = u32::try_from(value.len()).map_err(|_| FdWriterError::InvalidProtocol)?;
            let padding = (4 - value.len() % 4) % 4;
            let total =
                FdWriter::encoded_array_size(value.len()).ok_or(FdWriterError::InvalidProtocol)?;

            // A single primitive must fit within the maximum Wayland payload size.
            if total > (usize::from(u16::MAX) & !3) - 8 {
                return Err(FdWriterError::InvalidProtocol);
            }
            if self.bytes.try_buffer_frame(value, len, total) {
                return Ok(());
            }
            self.next_u32(len).await?;
            self.next_bytes(value).await?;
            self.next_bytes(&[0; 3][..padding]).await
        }
        .await;
        if result.is_err() && self.state != FdWriterState::Closed {
            self.state = FdWriterState::Failed;
        }
        result
    }

    /// Creates a writer that owns the supplied nonblocking socket.
    pub fn new(fd: AsyncFd<UnixStream>) -> Self {
        Self {
            fd,
            bytes: BytesStorage::new(),
            ancillary: AncillaryStorage::new(),
            state: FdWriterState::Open,
        }
    }

    /// Appends a four-byte signed integer in native byte order.
    ///
    /// Four free bytes are filled synchronously; otherwise the buffered byte
    /// writer drains as needed and retains its cancellation behavior.
    ///
    /// # Errors
    /// Returns an error for socket failures, a failed or closed stream, or
    /// insufficient payload quota.
    pub async fn next_i32(&mut self, value: i32) -> Result<(), FdWriterError> {
        self.check_state()?;
        if BYTES_CAPACITY - self.bytes.len >= 4 {
            self.bytes.raw[self.bytes.len..self.bytes.len + 4]
                .copy_from_slice(&value.to_ne_bytes());
            self.bytes.len += 4;
            return Ok(());
        }
        self.next_bytes(&value.to_ne_bytes()).await
    }

    /// Appends a four-byte unsigned integer or object ID in native byte order.
    ///
    /// Four free bytes are filled synchronously; otherwise the buffered byte
    /// writer drains as needed and retains its cancellation behavior.
    ///
    /// # Errors
    /// Returns an error for socket failures, a failed or closed stream, or
    /// insufficient payload quota.
    pub async fn next_u32(&mut self, value: u32) -> Result<(), FdWriterError> {
        self.check_state()?;
        if BYTES_CAPACITY - self.bytes.len >= 4 {
            self.bytes.raw[self.bytes.len..self.bytes.len + 4]
                .copy_from_slice(&value.to_ne_bytes());
            self.bytes.len += 4;
            return Ok(());
        }
        self.next_bytes(&value.to_ne_bytes()).await
    }

    /// Encodes signed 24.8 as four bytes, rounding halfway values away from zero.
    /// Rejects values outside the representable range before rounding.
    ///
    /// # Errors
    /// Returns an error for non-finite values or values outside signed 24.8
    /// range, socket failures, a failed or closed stream, or insufficient payload quota.
    pub async fn next_fixed(&mut self, value: f64) -> Result<(), FdWriterError> {
        self.check_state()?;
        let Some(encoded) = Self::encode_fixed(value) else {
            self.state = FdWriterState::Failed;
            return Err(FdWriterError::InvalidFixed);
        };
        self.next_i32(encoded).await
    }

    /// Buffers bytes, sending prefixes asynchronously when space is needed.
    /// A failed or cancelled call may already have buffered or sent a prefix.
    ///
    /// # Errors
    /// Returns an error for socket failures, a failed or closed stream, or
    /// insufficient payload quota.
    pub async fn next_bytes(&mut self, bytes: &[u8]) -> Result<(), FdWriterError> {
        self.check_state()?;
        self.state = FdWriterState::Writing;

        let mut consumed = 0;
        while consumed < bytes.len() {
            if self.bytes.len == BYTES_CAPACITY {
                self.send_buffered(1).await?;
            }
            let count = (bytes.len() - consumed).min(BYTES_CAPACITY - self.bytes.len);
            self.bytes.raw[self.bytes.len..self.bytes.len + count]
                .copy_from_slice(&bytes[consumed..consumed + count]);
            self.bytes.len += count;
            consumed += count;
        }

        self.state = FdWriterState::Open;
        Ok(())
    }

    /// Takes ownership of a descriptor to accompany the next successful send.
    ///
    /// A full descriptor queue is sent before adding this descriptor. The last
    /// buffered byte is retained when at least two bytes are available; a sole
    /// byte is sent, and the new descriptor waits for subsequent byte writes.
    /// Sending can suspend on socket readiness or Tokio cooperative scheduling.
    /// Cancellation during a send leaves the writer unusable; the connection
    /// must be closed rather than retried.
    ///
    /// # Extension codec limitations
    /// The descriptor queue holds 253 FDs. Unix stream ancillary transfers need
    /// ordinary bytes: adding the 254th queued FD requires buffered bytes to send
    /// the previous 253. If none are available, this call fails immediately with
    /// [`FdWriterError::FdsWithoutBytes`]; writing bytes afterwards cannot recover
    /// the failed writer.
    ///
    /// A descriptor-queue send can consume the sole remaining byte. New FDs may
    /// then be queued, but more ordinary bytes must be written before another
    /// queue-overflow send or [`Self::flush`]. A flush with queued FDs and no bytes
    /// also fails with [`FdWriterError::FdsWithoutBytes`]. Codecs for extensions
    /// with many FD arguments must account for this buffering limit; it is not
    /// a Wayland protocol limit on the number of FD arguments in a message.
    /// Do not insert extra wire bytes as a workaround, since that changes framing.
    ///
    /// # Errors
    /// Returns an error for a failed or closed stream, socket failures, or a full
    /// descriptor queue without buffered bytes. Rejected descriptors are closed
    /// automatically, as is the supplied descriptor on cancellation.
    pub async fn next_fd(&mut self, fd: OwnedFd) -> Result<(), FdWriterError> {
        self.check_state()?;
        if self.ancillary.len == FDS_CAPACITY {
            // Keep the current message incomplete until its new descriptor can
            // accompany data. A sole byte must instead serve the queued FDs;
            // the caller then supplies more bytes before flushing the new FD.
            let retain = usize::from(self.bytes.len > 1);
            self.state = FdWriterState::Writing;
            self.send_buffered(retain).await?;
        }
        self.ancillary.buf[self.ancillary.len] = Some(fd);
        self.ancillary.len += 1;
        self.state = FdWriterState::Open;
        Ok(())
    }

    /// Sends all buffered bytes and descriptors to the kernel.
    /// Success does not imply that the peer has processed the message.
    /// An empty buffer is allowed only when no descriptors are queued.
    /// An empty flush completes without an explicit scheduling yield.
    ///
    /// # Errors
    /// Returns an error for socket failures, a failed or closed stream,
    /// descriptors without ordinary bytes, or insufficient ancillary storage.
    pub async fn flush(&mut self) -> Result<(), FdWriterError> {
        self.check_state()?;
        self.state = FdWriterState::Writing;
        self.send_buffered(0).await?;
        self.state = FdWriterState::Open;
        Ok(())
    }

    /// Returns the current stream state.
    pub fn state(&self) -> &FdWriterState {
        &self.state
    }

    fn check_state(&mut self) -> Result<(), FdWriterError> {
        match self.state {
            FdWriterState::Open => Ok(()),
            FdWriterState::Closed => Err(FdWriterError::ConnectionClosed),
            FdWriterState::Writing | FdWriterState::Failed => {
                self.state = FdWriterState::Failed;
                Err(FdWriterError::WriterFailed)
            }
        }
    }

    /// Drains a prefix, optionally retaining one byte without shifting the buffer.
    /// Called only while the public operation owns the `Writing` state.
    async fn send_buffered(&mut self, retain: usize) -> Result<(), FdWriterError> {
        if retain > 1 || retain > self.bytes.len {
            self.state = FdWriterState::Failed;
            return Err(FdWriterError::InvalidProtocol);
        }
        let end = self.bytes.len - retain;
        if end == 0 && self.ancillary.len != 0 {
            self.state = FdWriterState::Failed;
            return Err(FdWriterError::FdsWithoutBytes);
        }

        while self.bytes.sent < end {
            // Keep readiness guards and ancillary temporaries within one poll;
            // none are carried across a readiness wait in the outer future.
            let result = std::future::poll_fn(|cx| {
                loop {
                    let mut guard = match self.fd.poll_write_ready(cx) {
                        std::task::Poll::Pending => {
                            return std::task::Poll::Pending;
                        }
                        std::task::Poll::Ready(Ok(guard)) => guard,
                        std::task::Poll::Ready(Err(error)) => {
                            return std::task::Poll::Ready(Err(error));
                        }
                    };
                    match guard.try_io(|fd| {
                        let socket = fd.get_ref();
                        let iov = [IoSlice::new(&self.bytes.raw[self.bytes.sent..end])];
                        self.ancillary.send(socket, &iov)
                    }) {
                        // try_io clears writable readiness only for WouldBlock.
                        Err(_) => continue,
                        Ok(result) => return std::task::Poll::Ready(result),
                    }
                }
            })
            .await;

            match result {
                Ok(Ok(0)) => {
                    self.state = FdWriterState::Closed;
                    return Err(FdWriterError::WriteZero);
                }
                Ok(Ok(size)) => {
                    self.bytes.sent += size;
                    // SCM_RIGHTS is sent once, even when sendmsg writes a prefix.
                    // No await occurs between the send and recording its effects.
                    self.ancillary.clear();
                }
                Err(error) if error.kind() == io::ErrorKind::Interrupted => continue,
                Ok(Err(error)) => {
                    self.state = FdWriterState::Failed;
                    return Err(error);
                }
                Err(error) => {
                    self.state = match error.kind() {
                        io::ErrorKind::BrokenPipe
                        | io::ErrorKind::ConnectionReset
                        | io::ErrorKind::ConnectionAborted
                        | io::ErrorKind::NotConnected => FdWriterState::Closed,
                        _ => FdWriterState::Failed,
                    };
                    return Err(FdWriterError::Io(error));
                }
            }
        }

        if retain != 0 {
            self.bytes.raw[0] = self.bytes.raw[end];
        }
        self.bytes.len = retain;
        self.bytes.sent = 0;

        Ok(())
    }
}

/// Exclusive byte-quota wrapper over an [`FdWriter`].
///
/// Quota counts bytes accepted for sending, charged after each successful call;
/// descriptors do not consume quota. This wrapper does not flush. Once the
/// quota is exhausted and the wrapper drops, call [`FdWriter::flush`].
///
/// Exceeding quota or dropping an incomplete payload fails the writer without
/// I/O. Intermediate sends cannot be retracted: the application must terminate
/// the connection after a quota error instead of continuing its message stream.
/// Cancelled operations are likewise unrecoverable. No payload copies or extra
/// buffers are made by this wrapper.
pub struct FdPayloadWriter<'a> {
    writer: &'a mut FdWriter,
    quota: usize,
}

impl<'a> FdPayloadWriter<'a> {
    /// Writes a UTF-8 string with its length, terminating NUL, and zero padding.
    /// A complete frame that fits is buffered synchronously without socket I/O.
    /// Rejects embedded NUL characters and checks the length and quota before writing.
    ///
    /// # Errors
    /// Returns an error for embedded NUL, length overflow, an oversized value,
    /// insufficient payload quota, or a failed or closed stream.
    pub async fn next_string(&mut self, value: &str) -> Result<(), FdWriterError> {
        self.next_nullable_string(Some(value)).await
    }

    /// Writes a nullable string: None has length zero, and Some("") has length one.
    ///
    /// # Errors
    /// Returns an error for embedded NUL, length overflow, an oversized value,
    /// insufficient payload quota, or a failed or closed stream.
    pub async fn next_nullable_string(&mut self, value: Option<&str>) -> Result<(), FdWriterError> {
        let result = async {
            let Some(value) = value else {
                return self.next_u32(0).await;
            };
            if !FdWriter::is_valid_string(value) {
                return Err(FdWriterError::InvalidProtocol);
            }
            let len = value
                .len()
                .checked_add(1)
                .ok_or(FdWriterError::InvalidProtocol)?;
            let wire_len = u32::try_from(len).map_err(|_| FdWriterError::InvalidProtocol)?;
            let padding = (4 - len % 4) % 4;
            let total =
                FdWriter::encoded_string_size(Some(value)).ok_or(FdWriterError::InvalidProtocol)?;
            if total > self.remaining_quota() {
                return Err(FdWriterError::InvalidProtocol);
            }
            self.writer.check_state()?;
            if self
                .writer
                .bytes
                .try_buffer_frame(value.as_bytes(), wire_len, total)
            {
                self.quota -= total;
                return Ok(());
            }
            self.next_u32(wire_len).await?;
            self.next_bytes(value.as_bytes()).await?;
            self.next_bytes(&[0; 4][..1 + padding]).await
        }
        .await;
        if result.is_err() {
            self.writer.state = FdWriterState::Failed;
        }
        result
    }

    /// Writes an array with its byte length and zero padding to four-byte alignment.
    /// A complete frame that fits is buffered synchronously without socket I/O.
    /// The encoded array length excludes padding.
    ///
    /// # Errors
    /// Returns an error for length overflow, an oversized value, insufficient
    /// payload quota, or a failed or closed stream.
    pub async fn next_array(&mut self, value: &[u8]) -> Result<(), FdWriterError> {
        let result = async {
            let len = u32::try_from(value.len()).map_err(|_| FdWriterError::InvalidProtocol)?;
            let padding = (4 - value.len() % 4) % 4;
            let total =
                FdWriter::encoded_array_size(value.len()).ok_or(FdWriterError::InvalidProtocol)?;
            if total > self.remaining_quota() {
                return Err(FdWriterError::InvalidProtocol);
            }
            self.writer.check_state()?;
            if self.writer.bytes.try_buffer_frame(value, len, total) {
                self.quota -= total;
                return Ok(());
            }
            self.next_u32(len).await?;
            self.next_bytes(value).await?;
            self.next_bytes(&[0; 3][..padding]).await
        }
        .await;
        if result.is_err() {
            self.writer.state = FdWriterState::Failed;
        }
        result
    }

    /// Creates a writer that owns the supplied nonblocking socket.
    ///
    /// # Errors
    /// Returns an error if the writer is failed, closed, or has an unfinished write.
    pub fn new(writer: &'a mut FdWriter, quota: usize) -> Result<Self, FdWriterError> {
        writer.check_state()?;
        Ok(Self { writer, quota })
    }

    /// Returns the unconsumed byte quota.
    pub fn remaining_quota(&self) -> usize {
        self.quota
    }

    /// Writes a four-byte signed integer in native byte order.
    ///
    /// # Errors
    /// Returns an error for socket failures, a failed or closed stream, or
    /// insufficient payload quota.
    pub async fn next_i32(&mut self, value: i32) -> Result<(), FdWriterError> {
        const SIZE: usize = size_of::<i32>();
        self.check_quota(SIZE)?;
        let result = self.writer.next_i32(value).await;
        self.finish_write(result, SIZE)
    }

    /// Writes a four-byte unsigned integer in native byte order.
    ///
    /// # Errors
    /// Returns an error for socket failures, a failed or closed stream, or
    /// insufficient payload quota.
    pub async fn next_u32(&mut self, value: u32) -> Result<(), FdWriterError> {
        const SIZE: usize = size_of::<u32>();
        self.check_quota(SIZE)?;
        let result = self.writer.next_u32(value).await;
        self.finish_write(result, SIZE)
    }

    /// Encodes four bytes of signed 24.8, regardless of the input f64's size.
    ///
    /// # Errors
    /// Returns an error for non-finite values or values outside signed 24.8
    /// range, socket failures, a failed or closed stream, or insufficient payload quota.
    pub async fn next_fixed(&mut self, value: f64) -> Result<(), FdWriterError> {
        const SIZE: usize = size_of::<i32>();
        self.check_quota(SIZE)?;
        let result = self.writer.next_fixed(value).await;
        self.finish_write(result, SIZE)
    }

    /// Writes bytes within the remaining payload quota.
    ///
    /// # Errors
    /// Returns an error for socket failures, a failed or closed stream, or
    /// insufficient payload quota.
    pub async fn next_bytes(&mut self, bytes: &[u8]) -> Result<(), FdWriterError> {
        self.check_quota(bytes.len())?;
        let result = self.writer.next_bytes(bytes).await;
        self.finish_write(result, bytes.len())
    }

    /// Queues an owned descriptor without consuming byte quota.
    ///
    /// A full descriptor queue triggers an asynchronous send, retaining the last
    /// byte when possible. If its sole byte is sent, subsequent byte writes are
    /// required before flushing newly queued descriptors. Cancellation leaves the
    /// underlying writer unusable.
    ///
    /// # Extension codec limitations
    /// The 253-FD queue and ordinary-byte requirements of [`FdWriter::next_fd`]
    /// apply here too. In particular, an exhausted payload byte quota does not
    /// guarantee that all queued FDs can be sent. Extension codecs must arrange
    /// enough protocol-defined bytes for descriptor-queue sends and the final
    /// flush; they must not add bytes outside the declared message payload.
    ///
    /// # Errors
    /// Returns an error for a failed or closed stream, socket failures, or
    /// insufficient bytes for a descriptor-queue send. Rejected descriptors close
    /// automatically. This operation never consumes payload byte quota.
    pub async fn next_fd(&mut self, fd: OwnedFd) -> Result<(), FdWriterError> {
        let result = self.writer.next_fd(fd).await;
        self.finish_write(result, 0)
    }

    /// Returns the current stream state.
    pub fn state(&self) -> &FdWriterState {
        self.writer.state()
    }

    fn check_quota(&mut self, requested: usize) -> Result<(), FdWriterError> {
        self.writer.check_state()?;
        if requested > self.quota {
            let remaining = self.quota;
            self.quota = 0;
            self.writer.state = FdWriterState::Failed;
            return Err(FdWriterError::QuotaExceeded {
                requested,
                remaining,
            });
        }
        Ok(())
    }

    fn finish_write(
        &mut self,
        result: Result<(), FdWriterError>,
        size: usize,
    ) -> Result<(), FdWriterError> {
        if result.is_ok() {
            self.quota -= size;
        } else {
            self.writer.state = FdWriterState::Failed;
        }
        result
    }
}

impl Drop for FdPayloadWriter<'_> {
    fn drop(&mut self) {
        if self.quota != 0 || self.writer.state == FdWriterState::Writing {
            self.writer.state = FdWriterState::Failed;
        }
    }
}
