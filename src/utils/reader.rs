//! Reads native-endian Wayland values and file descriptors from Unix sockets.

use std::{
    collections::{TryReserveError, VecDeque},
    io::{self, IoSliceMut},
    mem::MaybeUninit,
    os::{fd::OwnedFd, unix::net::UnixStream},
};

use rustix::net::{RecvAncillaryBuffer, RecvAncillaryMessage, RecvFlags, ReturnFlags, recvmsg};
use thiserror::Error;
use tokio::io::unix::AsyncFd;

// Fixed storage keeps socket refills allocation-free while batching small messages.
const BYTES_CAPACITY: usize = 8192;
// Linux SCM_RIGHTS permits at most 253 descriptors in one ancillary message.
const FDS_PER_RECV_CAPACITY: usize = 253;
const ANCILLARY_CAPACITY: usize = rustix::cmsg_space!(ScmRights(FDS_PER_RECV_CAPACITY));
// The 16-bit wire size includes an eight-byte header and must be four-byte aligned.
const MAX_WAYLAND_PAYLOAD_BYTES: usize = (u16::MAX as usize & !3) - 8;

#[derive(Error, Debug)]
pub enum FdReaderError {
    #[error("End of stream.")]
    EndOfStream,

    #[error("Failed to read from the Unix socket: {0}")]
    Io(#[source] io::Error),

    #[error("Received ancillary data was truncated; the connection can no longer be decoded.")]
    AncillaryTruncated,

    #[error("Failed to allocate space in the file descriptor queue: {0}")]
    FdQueueAllocation(#[source] TryReserveError),

    #[error("Received an unsupported ancillary message: {kind}.")]
    UnsupportedAncillaryMessage { kind: &'static str },

    #[error("Cannot read bytes: finish the partially read four-byte number ({read} bytes read).")]
    UnfinishedNumber { read: usize },

    #[error("The reader cannot read bytes after an unrecoverable error or an incomplete payload.")]
    ReaderFailed,

    #[error("Payload quota exceeded: requested {requested} bytes, but only {remaining} remain.")]
    QuotaExceeded { requested: usize, remaining: usize },

    #[error("Invalid protocol.")]
    InvalidProtocol,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FdReaderState {
    Open,
    EndOfStream,
    Failed,
}

// Reserve one Linux SCM_RIGHTS batch up front; retain capacity as the FIFO grows.
const INLINE_FDS_CAPACITY: usize = 256;

/// Processes owned descriptors without performing further socket operations.
/// The iterator's owner must close all unvisited messages on early return.
fn collect_descriptors<'a>(
    messages: &mut impl Iterator<Item = RecvAncillaryMessage<'a>>,
    queue: &mut VecDeque<OwnedFd>,
) -> Result<(), FdReaderError> {
    for message in messages {
        match message {
            RecvAncillaryMessage::ScmRights(rights) => {
                queue
                    .try_reserve(rights.len())
                    .map_err(FdReaderError::FdQueueAllocation)?;
                for fd in rights {
                    queue.push_back(fd);
                }
            }
            _ => {
                return Err(FdReaderError::UnsupportedAncillaryMessage {
                    kind: "Unsupported ancillary message.",
                });
            }
        }
    }
    Ok(())
}

/// Inline control-message storage aligned to a native integer boundary.
/// RecvAncillaryBuffer handles the platform's cmsghdr layout and initialization;
/// alignment preserves the full reserved SCM_RIGHTS capacity on Unix targets.
#[repr(C)]
struct AncillaryStorage {
    alignment: [usize; 0],
    bytes: [MaybeUninit<u8>; ANCILLARY_CAPACITY],
}

impl AncillaryStorage {
    fn new() -> Self {
        Self {
            alignment: [],
            bytes: [MaybeUninit::uninit(); ANCILLARY_CAPACITY],
        }
    }
}

/// Buffered native-endian reads from a nonblocking Unix stream.
///
/// Small values share an inline read-ahead buffer. Uncached byte regions are
/// received directly into the initialized destination through rustix, while a
/// second iovec batches subsequent fields. A Vec suffix is zero-initialized
/// before receiving into it. Empty strings and arrays require no allocation;
/// nonempty owned values allocate once.
///
/// Construction allocates a FIFO queue with capacity for at least 256 descriptors.
/// The queue grows as needed and retains its allocation for reuse. Byte and
/// ancillary buffers remain inline. The socket must already be nonblocking,
/// and its Tokio runtime must remain running. All access requires &mut self.
///
/// Successful socket receives count both direct and read-ahead bytes toward a
/// cooperative yield threshold equal to the internal byte-buffer capacity.
/// Pending readiness resets this budget. Buffered reads do not count again;
/// a single receive can exceed the threshold when reading directly into a destination.
///
/// Descriptors remain available after EOF or a read failure. Dropping the reader
/// closes all queued descriptors.
pub struct FdReader {
    fd: AsyncFd<UnixStream>,
    bytes_raw: [u8; BYTES_CAPACITY],
    bytes_read: usize,
    bytes_len: usize,
    bytes_since_yield: usize,
    fds_raw: AncillaryStorage,
    fds_queue: VecDeque<OwnedFd>,
    number_raw: [u8; 4],
    number_len: usize,
    state: FdReaderState,
}

impl FdReader {
    /// Reads a Wayland string with a NUL-inclusive length and alignment padding.
    /// Rejects a zero length, invalid UTF-8, or an embedded NUL.
    /// After cancellation, the message must not be decoded again.
    ///
    /// # Errors
    /// Returns an error for NULL, invalid UTF-8, embedded or missing NUL, invalid
    /// lengths, allocation failures, or stream failures. Payload reads also enforce quota.
    pub async fn next_string(&mut self) -> Result<String, FdReaderError> {
        let mut remaining = MAX_WAYLAND_PAYLOAD_BYTES;
        self.next_string_limited::<true>(&mut remaining).await
    }

    /// Reads a nullable string: length zero means NULL, and length one means empty.
    ///
    /// # Errors
    /// Returns an error for invalid UTF-8, embedded or missing NUL, invalid
    /// lengths, allocation failures, or stream failures. Payload reads also enforce quota.
    pub async fn next_nullable_string(&mut self) -> Result<Option<String>, FdReaderError> {
        let mut remaining = MAX_WAYLAND_PAYLOAD_BYTES;
        self.next_nullable_string_limited::<true>(&mut remaining)
            .await
    }

    /// Reads an array length, its bytes, and padding to a four-byte boundary.
    /// Validates the length before allocation; the result excludes padding.
    /// Payload reads check the remaining quota; stream reads check the Wayland message limit.
    ///
    /// # Errors
    /// Returns an error for invalid lengths, allocation failures, or stream
    /// failures. Payload reads reject lengths exceeding their remaining quota.
    pub async fn next_array(&mut self) -> Result<Vec<u8>, FdReaderError> {
        let mut remaining = MAX_WAYLAND_PAYLOAD_BYTES;
        self.next_array_limited::<true>(&mut remaining).await
    }

    /// Non-null strings share the nullable decoder and reject NULL after framing.
    async fn next_string_limited<const CHECK_PREFIX: bool>(
        &mut self,
        remaining: &mut usize,
    ) -> Result<String, FdReaderError> {
        match self
            .next_nullable_string_limited::<CHECK_PREFIX>(remaining)
            .await?
        {
            Some(value) => Ok(value),
            None => {
                self.state = FdReaderState::Failed;
                Err(FdReaderError::InvalidProtocol)
            }
        }
    }

    /// String::from_utf8 takes ownership without copying or allocating again.
    async fn next_nullable_string_limited<const CHECK_PREFIX: bool>(
        &mut self,
        remaining: &mut usize,
    ) -> Result<Option<String>, FdReaderError> {
        let (bytes, nonnull) = self.next_framed::<true, CHECK_PREFIX>(remaining).await?;
        if !nonnull {
            return Ok(None);
        }
        match String::from_utf8(bytes) {
            Ok(value) => Ok(Some(value)),
            Err(_) => {
                self.state = FdReaderState::Failed;
                Err(FdReaderError::InvalidProtocol)
            }
        }
    }

    async fn next_array_limited<const CHECK_PREFIX: bool>(
        &mut self,
        remaining: &mut usize,
    ) -> Result<Vec<u8>, FdReaderError> {
        self.next_framed::<false, CHECK_PREFIX>(remaining)
            .await
            .map(|(bytes, _)| bytes)
    }

    /// Checks framing once, before allocating or reading a declared body.
    /// Payload callers already checked the prefix quota; raw reads check it here.
    /// Prefix, complete body, and complete padding are charged separately, including
    /// on malformed strings, and retain their previous error/cancellation behavior.
    async fn next_framed<const STRING: bool, const CHECK_PREFIX: bool>(
        &mut self,
        remaining: &mut usize,
    ) -> Result<(Vec<u8>, bool), FdReaderError> {
        let result = async {
            if CHECK_PREFIX && *remaining < 4 {
                let available = *remaining;
                *remaining = 0;
                return Err(FdReaderError::QuotaExceeded {
                    requested: 4,
                    remaining: available,
                });
            }
            let wire_len = self.next_u32().await?;
            *remaining -= 4;
            let len = usize::try_from(wire_len).map_err(|_| FdReaderError::InvalidProtocol)?;
            let padding = len.wrapping_neg() & 3;
            let total = len
                .checked_add(padding)
                .ok_or(FdReaderError::InvalidProtocol)?;
            if total > *remaining {
                return Err(FdReaderError::InvalidProtocol);
            }
            let mut bytes = Vec::new();
            if STRING && len == 1 {
                // Empty strings need only the inline terminator, not a heap buffer.
                if self.bytes_read == self.bytes_len {
                    self.refill().await?;
                }
                let terminator = self.bytes_raw[self.bytes_read];
                self.bytes_read += 1;
                *remaining -= 1;
                self.skip_bytes(padding).await?;
                *remaining -= padding;
                if terminator != 0 {
                    return Err(FdReaderError::InvalidProtocol);
                }
                return Ok((bytes, true));
            }
            if len != 0 {
                bytes
                    .try_reserve_exact(len)
                    .map_err(|_| FdReaderError::InvalidProtocol)?;
                self.fill_vec(&mut bytes, len).await?;
                *remaining -= len;
            }
            self.skip_bytes(padding).await?;
            *remaining -= padding;
            if STRING && len != 0 && (bytes.pop() != Some(0) || bytes.contains(&0)) {
                return Err(FdReaderError::InvalidProtocol);
            }
            Ok((bytes, len != 0))
        }
        .await;
        if result.is_err() && self.state == FdReaderState::Open {
            self.state = FdReaderState::Failed;
        }
        result
    }

    /// Uses already buffered bytes once, then fills the reserved final allocation.
    /// The caller supplied an empty Vec and reserved capacity for len bytes.
    async fn fill_vec(&mut self, bytes: &mut Vec<u8>, len: usize) -> Result<(), FdReaderError> {
        let buffered = len.min(self.bytes_len - self.bytes_read);
        if buffered != 0 {
            // SAFETY: The caller supplied an empty Vec with capacity for len
            // bytes. buffered <= len, and the source prefix is initialized and
            // bounded by bytes_len <= BYTES_CAPACITY. The inline source and the
            // fresh heap allocation cannot overlap. Publish only the copied bytes.
            unsafe {
                std::ptr::copy_nonoverlapping(
                    self.bytes_raw.as_ptr().add(self.bytes_read),
                    bytes.as_mut_ptr(),
                    buffered,
                );
                bytes.set_len(buffered);
            }
        }
        self.bytes_read += buffered;
        let mut initialized = bytes.len();
        if initialized < len {
            // IoSliceMut requires initialized bytes; only the uncached suffix
            // needs zeroing. The caller has already reserved the full capacity.
            bytes.resize(len, 0);
            while initialized < len {
                initialized += self.receive(&mut bytes[initialized..]).await?;
            }
        }
        Ok(())
    }

    /// Discards alignment bytes by cursor advancement, without copying them.
    /// State and framing were checked by the caller; only refills can fail here.
    async fn skip_bytes(&mut self, mut len: usize) -> Result<(), FdReaderError> {
        while len != 0 {
            if self.bytes_read == self.bytes_len {
                self.refill().await?;
            }
            let count = len.min(self.bytes_len - self.bytes_read);
            self.bytes_read += count;
            len -= count;
        }
        Ok(())
    }

    /// Creates a reader that owns the supplied nonblocking socket.
    pub fn new(fd: AsyncFd<UnixStream>) -> Self {
        Self {
            fd,
            bytes_raw: [0; BYTES_CAPACITY],
            bytes_read: 0,
            bytes_len: 0,
            bytes_since_yield: 0,
            fds_raw: AncillaryStorage::new(),
            fds_queue: VecDeque::with_capacity(INLINE_FDS_CAPACITY),
            number_raw: [0; 4],
            number_len: 0,
            state: FdReaderState::Open,
        }
    }

    /// Reads an `int` in native byte order.
    ///
    /// If this future is cancelled, any consumed prefix remains in the reader.
    /// Resume with a numeric read; `next_bytes` cannot skip that prefix.
    ///
    /// # Errors
    /// Returns an error if the stream ends or fails, socket I/O fails, or a
    /// payload read exceeds its quota.
    pub async fn next_i32(&mut self) -> Result<i32, FdReaderError> {
        self.check_state()?;
        if let Some(number) = self.buffered_number() {
            return Ok(i32::from_ne_bytes(number));
        }
        Ok(i32::from_ne_bytes(self.next_number_slow().await?))
    }

    /// Reads a `uint`, object ID, or new object ID in native byte order.
    ///
    /// Partial-read and cancellation behavior is the same as [`Self::next_i32`].
    ///
    /// # Errors
    /// Returns an error if the stream ends or fails, socket I/O fails, or a
    /// payload read exceeds its quota.
    pub async fn next_u32(&mut self) -> Result<u32, FdReaderError> {
        self.check_state()?;
        if let Some(number) = self.buffered_number() {
            return Ok(u32::from_ne_bytes(number));
        }
        Ok(u32::from_ne_bytes(self.next_number_slow().await?))
    }

    /// Reads a signed Wayland 24.8 fixed-point number, exactly represented as f64.
    ///
    /// Partial-read and cancellation behavior is the same as [`Self::next_i32`].
    ///
    /// # Errors
    /// Returns an error if the stream ends or fails, socket I/O fails, or a
    /// payload read exceeds its quota.
    pub async fn next_fixed(&mut self) -> Result<f64, FdReaderError> {
        Ok(f64::from(self.next_i32().await?) / 256.0)
    }

    /// Fills the caller's buffer, receiving uncached bytes directly into it.
    ///
    /// On error or cancellation, a prefix of `destination` may already have
    /// been filled and consumed from the stream. This operation does not retain
    /// progress for automatic resumption. It rejects unfinished numeric reads.
    ///
    /// # Errors
    /// Returns an error if the stream ends or fails, a numeric read is
    /// unfinished, socket I/O fails, or a payload read exceeds its quota.
    pub async fn next_bytes(&mut self, destination: &mut [u8]) -> Result<(), FdReaderError> {
        self.check_state()?;
        if self.number_len != 0 {
            return Err(FdReaderError::UnfinishedNumber {
                read: self.number_len,
            });
        }

        self.read_bytes(destination).await
    }

    /// The public entry point checked state and unfinished numeric reads already.
    async fn read_bytes(&mut self, mut destination: &mut [u8]) -> Result<(), FdReaderError> {
        let count = destination.len().min(self.bytes_len - self.bytes_read);
        let (buffered, rest) = destination.split_at_mut(count);
        buffered.copy_from_slice(&self.bytes_raw[self.bytes_read..self.bytes_read + count]);
        self.bytes_read += count;
        destination = rest;
        while !destination.is_empty() {
            let received = self.receive(destination).await?;
            destination = &mut destination[received..];
        }
        Ok(())
    }

    /// Takes the oldest queued descriptor without performing socket I/O.
    ///
    /// Descriptors already stored remain accessible after EOF or a read error.
    /// Dropping the reader closes all descriptors that have not been taken.
    pub fn next_fd(&mut self) -> Option<OwnedFd> {
        self.fds_queue.pop_front()
    }

    /// Returns the current stream state.
    pub fn state(&self) -> &FdReaderState {
        &self.state
    }

    fn check_state(&self) -> Result<(), FdReaderError> {
        match self.state {
            FdReaderState::Open => Ok(()),
            FdReaderState::EndOfStream => Err(FdReaderError::EndOfStream),
            FdReaderState::Failed => Err(FdReaderError::ReaderFailed),
        }
    }

    /// Synchronous fast path; the four-byte copy compiles to a native load.
    /// A partial numeric read must be completed through the slow path instead.
    #[inline]
    fn buffered_number(&mut self) -> Option<[u8; 4]> {
        if self.number_len != 0 || self.bytes_len - self.bytes_read < 4 {
            return None;
        }
        let mut number = [0; 4];
        number.copy_from_slice(&self.bytes_raw[self.bytes_read..self.bytes_read + 4]);
        self.bytes_read += 4;
        Some(number)
    }

    /// Keeps partial numeric progress in the reader across suspension/cancellation.
    /// Only the checked numeric entry points may call this operation.
    async fn next_number_slow(&mut self) -> Result<[u8; 4], FdReaderError> {
        while self.number_len < 4 {
            if self.bytes_read == self.bytes_len {
                self.refill().await?;
            }

            let count = (4 - self.number_len).min(self.bytes_len - self.bytes_read);
            self.number_raw[self.number_len..self.number_len + count]
                .copy_from_slice(&self.bytes_raw[self.bytes_read..self.bytes_read + count]);
            self.bytes_read += count;
            self.number_len += count;
        }

        self.number_len = 0;
        Ok(self.number_raw)
    }

    /// Only internal, state-checked operations may refill an exhausted buffer.
    /// Removing redundant state checks here does not bypass public validation.
    async fn refill(&mut self) -> Result<(), FdReaderError> {
        self.receive(&mut []).await.map(|_| ())
    }

    /// Receives directly into the destination and stores only read-ahead bytes.
    /// Callers have exhausted the byte buffer and retain the destination borrow.
    async fn receive(&mut self, destination: &mut [u8]) -> Result<usize, FdReaderError> {
        // Readiness guards and recvmsg buffers exist only during poll, rather than
        // becoming large locals carried through nested async_io futures.
        let result = std::future::poll_fn(|cx| {
            loop {
                let mut guard = match self.fd.poll_read_ready(cx) {
                    std::task::Poll::Pending => {
                        self.bytes_since_yield = 0;
                        return std::task::Poll::Pending;
                    }
                    std::task::Poll::Ready(Ok(guard)) => guard,
                    std::task::Poll::Ready(Err(error)) => {
                        self.state = FdReaderState::Failed;
                        return std::task::Poll::Ready(Err(FdReaderError::Io(error)));
                    }
                };
                let result = guard.try_io(|fd| {
                    let empty = destination.is_empty();
                    let mut iov = [
                        IoSliceMut::new(destination),
                        IoSliceMut::new(&mut self.bytes_raw),
                    ];
                    let iov = if empty { &mut iov[1..] } else { &mut iov[..] };
                    let mut control = RecvAncillaryBuffer::new(&mut self.fds_raw.bytes);
                    let received = recvmsg(fd, iov, &mut control, RecvFlags::CMSG_CLOEXEC)
                        .map_err(io::Error::from)?;
                    let mut messages = control.drain();
                    let result = if received.flags.contains(ReturnFlags::CTRUNC) {
                        Err(FdReaderError::AncillaryTruncated)
                    } else {
                        collect_descriptors(&mut messages, &mut self.fds_queue)
                            .map(|()| received.bytes)
                    };
                    // Continue the same iterator even after an error: dropping
                    // each remaining rights iterator closes its unclaimed FDs.
                    messages.for_each(drop);
                    Ok(result)
                });
                drop(guard);
                match result {
                    // try_io cleared only the readiness for an actual EAGAIN.
                    Err(_) => continue,
                    Ok(Err(error)) if error.kind() == io::ErrorKind::Interrupted => continue,
                    Ok(result) => {
                        let result = self.finish_receive(result).map(|size| {
                            let direct = size.min(destination.len());
                            self.bytes_read = 0;
                            self.bytes_len = size - direct;
                            self.bytes_since_yield = self.bytes_since_yield.saturating_add(size);
                            direct
                        });
                        return std::task::Poll::Ready(result);
                    }
                }
            }
        })
        .await;

        // Account for all received bytes and descriptors before yielding. A
        // readiness suspension resets the budget; resumed receives count anew.
        if result.is_ok() && self.bytes_since_yield >= BYTES_CAPACITY {
            self.bytes_since_yield = 0;
            tokio::task::yield_now().await;
        }
        result
    }

    /// Updates terminal state in one place after a socket receive completes.
    fn finish_receive(
        &mut self,
        result: io::Result<Result<usize, FdReaderError>>,
    ) -> Result<usize, FdReaderError> {
        match result {
            Ok(Ok(0)) => {
                self.state = FdReaderState::EndOfStream;
                Err(FdReaderError::EndOfStream)
            }
            Ok(Ok(size)) => Ok(size),
            Ok(Err(error)) => {
                self.state = FdReaderState::Failed;
                Err(error)
            }
            Err(error) => {
                self.state = FdReaderState::Failed;
                Err(FdReaderError::Io(error))
            }
        }
    }
}

/// Exclusive access to a byte region of an [`FdReader`].
///
/// Reads delegate directly to FdReader. Fixed-size reads charge quota after
/// successful completion. Length-prefixed reads pass the remaining quota as a
/// byte budget and charge successfully read prefixes, bodies, and padding.
/// A declared body that exceeds its budget is rejected before allocation or
/// body I/O. Exceeding quota for a fixed-size read clears it and marks the
/// underlying reader as failed without consuming bytes.
///
/// A read error marks the underlying reader as failed. If a read is cancelled,
/// the next byte read or dropping this wrapper also marks the reader as failed:
/// the delegated operation may already have consumed part of the payload.
///
/// Dropping this wrapper with unread bytes marks the underlying reader as
/// failed. It neither skips bytes nor performs I/O. Fully consuming the quota
/// leaves the reader positioned at the next byte after the region.
///
/// File descriptors are not covered by the byte quota: [`Self::next_fd`] can
/// take descriptors from the reader's shared FIFO even at zero remaining quota.
/// This wrapper does not establish which Wayland message owns each descriptor.
pub struct FdPayloadReader<'a> {
    reader: &'a mut FdReader,
    quota: usize,
    read_in_progress: bool,
}

impl<'a> FdPayloadReader<'a> {
    /// Reads a Wayland string with a NUL-inclusive length and alignment padding.
    /// Rejects a zero length, invalid UTF-8, or an embedded NUL.
    /// After cancellation, the message must not be decoded again.
    ///
    /// # Errors
    /// Returns an error for NULL, invalid UTF-8, embedded or missing NUL, invalid
    /// lengths, allocation failures, or stream failures. Payload reads also enforce quota.
    pub async fn next_string(&mut self) -> Result<String, FdReaderError> {
        self.check_quota(size_of::<u32>())?;
        let result = self
            .reader
            .next_string_limited::<false>(&mut self.quota)
            .await;
        // The bounded decoder has already charged each successfully read region.
        self.finish_read(result, 0)
    }

    /// Reads a nullable string: length zero means NULL, and length one means empty.
    ///
    /// # Errors
    /// Returns an error for invalid UTF-8, embedded or missing NUL, invalid
    /// lengths, allocation failures, or stream failures. Payload reads also enforce quota.
    pub async fn next_nullable_string(&mut self) -> Result<Option<String>, FdReaderError> {
        self.check_quota(size_of::<u32>())?;
        let result = self
            .reader
            .next_nullable_string_limited::<false>(&mut self.quota)
            .await;
        // The bounded decoder has already charged each successfully read region.
        self.finish_read(result, 0)
    }

    /// Reads an array length, its bytes, and padding to a four-byte boundary.
    /// Validates the length before allocation; the result excludes padding.
    /// Payload reads check the remaining quota; stream reads check the Wayland message limit.
    ///
    /// # Errors
    /// Returns an error for invalid lengths, allocation failures, or stream
    /// failures. Payload reads reject lengths exceeding their remaining quota.
    pub async fn next_array(&mut self) -> Result<Vec<u8>, FdReaderError> {
        self.check_quota(size_of::<u32>())?;
        let result = self
            .reader
            .next_array_limited::<false>(&mut self.quota)
            .await;
        // The bounded decoder has already charged each successfully read region.
        self.finish_read(result, 0)
    }

    /// Starts a region at the reader's current position without consuming data.
    ///
    /// Requires an open reader with no unfinished numeric read. Zero quota is
    /// allowed. A rejected construction leaves the reader unchanged.
    ///
    /// # Errors
    /// Returns an error if the reader is not open or a numeric read is unfinished.
    pub fn new(reader: &'a mut FdReader, quota: usize) -> Result<Self, FdReaderError> {
        reader.check_state()?;
        if reader.number_len != 0 {
            return Err(FdReaderError::UnfinishedNumber {
                read: reader.number_len,
            });
        }
        Ok(Self {
            reader,
            quota,
            read_in_progress: false,
        })
    }

    /// Returns the quota left after successfully completed reads.
    pub fn remaining_quota(&self) -> usize {
        self.quota
    }

    /// Reads a native-endian `int` within the region.
    ///
    /// Failed or cancelled reads prevent further byte reads from this region.
    ///
    /// # Errors
    /// Returns an error if the stream ends or fails, socket I/O fails, or a
    /// payload read exceeds its quota.
    pub async fn next_i32(&mut self) -> Result<i32, FdReaderError> {
        const SIZE: usize = size_of::<i32>();
        self.check_quota(SIZE)?;
        let result = self.reader.next_i32().await;
        self.finish_read(result, SIZE)
    }

    /// Reads a native-endian `uint` or object ID within the region.
    ///
    /// Partial-read and cancellation behavior is the same as [`Self::next_i32`].
    ///
    /// # Errors
    /// Returns an error if the stream ends or fails, socket I/O fails, or a
    /// payload read exceeds its quota.
    pub async fn next_u32(&mut self) -> Result<u32, FdReaderError> {
        const SIZE: usize = size_of::<u32>();
        self.check_quota(SIZE)?;
        let result = self.reader.next_u32().await;
        self.finish_read(result, SIZE)
    }

    /// Reads a signed Wayland 24.8 number encoded as i32 (four bytes).
    /// The returned f64 is a decoded value, not the socket representation.
    ///
    /// # Errors
    /// Returns an error if the stream ends or fails, socket I/O fails, or a
    /// payload read exceeds its quota.
    pub async fn next_fixed(&mut self) -> Result<f64, FdReaderError> {
        const SIZE: usize = size_of::<i32>();
        self.check_quota(SIZE)?;
        let result = self.reader.next_fixed().await;
        self.finish_read(result, SIZE)
    }

    /// Fills the caller's buffer, rejecting requests larger than remaining quota.
    ///
    /// On error or cancellation, a prefix may already have been consumed. This
    /// region cannot be resumed in that case. An empty read is allowed at zero
    /// quota while the reader is open and no previous read was cancelled.
    ///
    /// # Errors
    /// Returns an error if the stream ends or fails, a numeric read is
    /// unfinished, socket I/O fails, or a payload read exceeds its quota.
    pub async fn next_bytes(&mut self, destination: &mut [u8]) -> Result<(), FdReaderError> {
        let size = destination.len();
        self.check_quota(size)?;
        let result = self.reader.next_bytes(destination).await;
        self.finish_read(result, size)
    }

    /// Takes a queued descriptor without I/O or charging byte quota.
    pub fn next_fd(&mut self) -> Option<OwnedFd> {
        self.reader.next_fd()
    }

    /// Returns the current stream state.
    pub fn state(&self) -> &FdReaderState {
        self.reader.state()
    }

    fn check_quota(&mut self, requested: usize) -> Result<(), FdReaderError> {
        if self.read_in_progress {
            self.reader.state = FdReaderState::Failed;
            return Err(FdReaderError::ReaderFailed);
        }
        if requested > self.quota {
            let remaining = self.quota;
            self.quota = 0;
            self.reader.state = FdReaderState::Failed;
            return Err(FdReaderError::QuotaExceeded {
                requested,
                remaining,
            });
        }
        self.read_in_progress = true;
        Ok(())
    }

    fn finish_read<T>(
        &mut self,
        result: Result<T, FdReaderError>,
        size: usize,
    ) -> Result<T, FdReaderError> {
        self.read_in_progress = false;
        if result.is_ok() {
            self.quota -= size;
        } else {
            self.reader.state = FdReaderState::Failed;
        }
        result
    }
}

impl Drop for FdPayloadReader<'_> {
    fn drop(&mut self) {
        if self.quota != 0 || self.read_in_progress {
            self.reader.state = FdReaderState::Failed;
        }
    }
}
