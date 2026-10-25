//! Defines protocol command interfaces, connection identifiers, and transport messages.

pub mod core;
pub mod ext;
pub mod wlr;

mod object_interface;
#[doc(inline)]
pub use object_interface::ObjectInteface;

mod command_request;
pub use command_request::CommandRequest;

mod command_event;
pub use command_event::CommandEvent;

use std::sync::{Arc, Mutex, MutexGuard};

mod client_state;
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
pub use client_state::ClientState;

/// Decodes a command payload using the connection's object state.
///
/// Implementations read the payload only; the transport has already consumed
/// the message header. Nested value codecs can consume a portion of a payload.
pub trait CommandReader<T> {
    /// Reads the command addressed to `object_id` with the specified `opcode`.
    ///
    /// # Errors
    /// Returns an error for malformed data, unknown or incompatible receivers, insufficient
    /// payload quota, missing descriptors, or underlying socket failures.
    fn read(
        object_id: u32,
        opcode: u16,
        reader: &mut FdPayloadReader<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> impl Future<Output = Result<T, FdReaderError>> + Send;
}

/// Extends message decoders with explicit protocol-message validation.
///
/// Nested value decoders also implement CommandReader, so these helpers must
/// only be called when decoding a message with an actual receiving object.
/// A returned state guard must be released before awaiting socket I/O.
trait CommandReaderExt: CommandReader<Self> + Sized {
    /// Locks connection state, converting lock poisoning into a protocol error.
    fn reader_state(
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<MutexGuard<'_, ClientState>, FdReaderError> {
        state.lock().map_err(|_| FdReaderError::InvalidProtocol)
    }
}

impl<T: CommandReader<T>> CommandReaderExt for T {}

/// Decodes a complete message using previously resolved receiver metadata.
///
/// Dispatchers resolve the receiver once; standalone public decoders perform
/// their own lookup before calling this shared path. Each implementation checks
/// the interface variant, opcode, and minimum version before reading payload.
/// Metadata is a snapshot for this message only; state mutations still acquire
/// their own locks, and no state guard may be retained across socket I/O.
trait InterfaceReader: CommandReader<Self> + Sized {
    /// Reads and validates a payload without looking up its receiver again.
    ///
    /// The caller supplies metadata copied from `state` for `object_id` before
    /// decoding. Payload, descriptor, and object-state errors remain unchanged.
    fn read_with_interface(
        object_id: u32,
        opcode: u16,
        interface: ObjectInteface,
        reader: &mut FdPayloadReader<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> impl Future<Output = Result<Self, FdReaderError>> + Send;
}

/// Encodes a command payload and reports the metadata needed for its header.
///
/// The transport writes the header and flushes the stream. Payload sizes exclude
/// the eight-byte header and descriptors carried in ancillary messages.
pub trait CommandWriter {
    /// Returns the receiving object identifier, or zero for a nested value codec.
    fn object_id(&self) -> u32;
    /// Returns the interface-specific opcode, or zero for a nested value codec.
    fn opcode(&self) -> u16;
    /// Returns the encoded byte count, including length prefixes and padding.
    fn payload_size(&self) -> usize;

    /// Writes the payload and maintains metadata for created or released wire IDs.
    ///
    /// # Errors
    /// Returns an error for invalid wire values or receiver metadata, insufficient quota,
    /// descriptor limits, or underlying socket failures.
    fn write(
        self,
        writer: &mut FdPayloadWriter<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> impl Future<Output = Result<(), FdWriterError>> + Send;
}

/// Extends message encoders with explicit protocol-message validation.
///
/// Nested value encoders also implement CommandWriter, so receiver checks must
/// only be called when encoding a message with an actual receiving object.
/// A returned state guard must be released before awaiting socket I/O.
trait CommandWriterExt: CommandWriter + Sized {
    /// Locks connection state, converting lock poisoning into a protocol error.
    fn writer_state(
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<MutexGuard<'_, ClientState>, FdWriterError> {
        state.lock().map_err(|_| FdWriterError::InvalidProtocol)
    }

    /// Checks a request receiver's interface and minimum negotiated version.
    /// Returns metadata without retaining a lock across socket I/O.
    fn writer_request_interface(
        state: &Arc<Mutex<ClientState>>,
        id: u32,
        name: &str,
        since: u32,
    ) -> Result<ObjectInteface, FdWriterError> {
        let interface = Self::writer_state(state)?
            .object_interface(id)
            .ok_or(FdWriterError::InvalidProtocol)?;
        if interface.name() != name || interface.version() < since {
            return Err(FdWriterError::InvalidProtocol);
        }
        Ok(interface)
    }

    /// Checks the event receiver's interface and minimum negotiated version.
    /// Returns a copy of the interface without retaining a lock across I/O.
    fn writer_event_interface(
        state: &Arc<Mutex<ClientState>>,
        id: u32,
        name: &str,
        since: u32,
    ) -> Result<ObjectInteface, FdWriterError> {
        let ifc = Self::writer_state(state)?
            .object_interface(id)
            .ok_or(FdWriterError::InvalidProtocol)?;
        if ifc.name() != name || ifc.version() < since {
            return Err(FdWriterError::InvalidProtocol);
        }
        Ok(ifc)
    }

    /// Adds checked argument sizes at the existing payload_size API boundary.
    /// An unrepresentable argument or sum yields usize::MAX, which the transport
    /// rejects before writing a message header. Descriptors contribute zero bytes.
    fn total_payload_size(parts: &[Option<usize>]) -> usize {
        parts
            .iter()
            .try_fold(0usize, |sum, part| sum.checked_add((*part)?))
            .unwrap_or(usize::MAX)
    }
}

impl<T: CommandWriter> CommandWriterExt for T {}

/// Identifies an accepted client connection.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ClientId(pub(crate) u64);

impl std::fmt::Display for ClientId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.0, f)
    }
}

/// Identifies why a client connection was closed.
#[derive(Debug)]
pub enum DisconnectReason {
    /// The server requested disconnection or stopped servicing the connection.
    Server,

    /// The client closed its outgoing stream.
    Client,

    /// A transport, protocol, queue, or client-task failure ended the connection.
    Error(ClientError),
}

/// Reports failures that terminate a client connection.
#[derive(thiserror::Error, Debug)]
pub enum ClientError {
    /// An event could not be queued because the client task's event receiver was closed.
    #[error("The client event queue is closed.")]
    SendQueueClosed,

    /// An event could not be queued because the client's outgoing queue was full.
    #[error("The client event queue is full.")]
    SendQueueOverflow,

    /// The connection reader stopped in a failed state.
    #[error("The client connection reader is in a failed state.")]
    ConnectionFailed,

    /// Decoding an incoming message failed; the connection is closed afterwards.
    #[error("Failed to decode a request from client. {0}")]
    Read(#[from] FdReaderError),

    /// Encoding or sending an event failed; the connection is closed afterwards.
    #[error("Failed to encode an event for client. {0}")]
    Write(#[from] FdWriterError),

    /// A client task panicked or was cancelled before completing.
    #[error("Unhandled error in client task. {0}")]
    UnhandledTaskError(#[from] tokio::task::JoinError),
}

/// Reports connection-level protocol and transport failures.
#[derive(thiserror::Error, Debug)]
pub enum ServerError {
    /// The connection identifier space is exhausted.
    #[error("Client index overflow.")]
    ClientIndexOverflow,

    /// No connection exists with the supplied identifier.
    #[error("Client not found with id {0}")]
    ClientNotFound(ClientId),

    /// Accepting or preparing the connection failed.
    #[error("Failed to accept client. {0}")]
    ClientAccept(std::io::Error),
}
