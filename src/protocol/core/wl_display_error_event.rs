use crate::protocol::ClientState;
use crate::protocol::core::ObjectInteface as CoreObjectInteface;
use crate::protocol::{CommandReader, CommandReaderExt};
use crate::protocol::{CommandWriter, CommandWriterExt};
use crate::protocol::{InterfaceReader, ObjectInteface};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Server event `wl_display.error`.
///
/// Opcode 0; available since version 1. The structure contains the receiver and arguments in XML order.
/// Encoding and decoding check wire values and receiver metadata; compositor semantics are not validated.
/// Decoding consumes the complete payload and updates wire object metadata when needed.
/// The caller handles compositor policy and message contents.
/// ```xml
/// <event name="error">
///       <description summary="fatal error event">
///         The error event is sent out when a fatal (non-recoverable)
///         error has occurred.  The object_id argument is the object
///         where the error occurred, most often in response to a request
///         to that object.  The code identifies the error and is defined
///         by the object interface.  As such, each interface defines its
///         own set of error codes.  The message is a brief description
///         of the error, for (debugging) convenience.
///       </description>
///       <arg name="object_id" type="object" summary="object where the error occurred" />
///       <arg name="code" type="uint" summary="error code" />
///       <arg name="message" type="string" summary="error description" />
///     </event>
/// ```
#[derive(Debug)]
pub struct WlDisplayErrorEvent {
    /// Wire identifier of the object receiving the message.
    pub object_id: u32,
    /// Argument `object_id`: object where the error occurred.
    pub error_object_id: u32,
    /// Argument `code`: error code.
    pub code: u32,
    /// Argument `message`: error description.
    pub message: String,
}
impl WlDisplayErrorEvent {
    /// Opcode in the event namespace of interface wl_display.
    pub const OPCODE: u16 = 0;
    /// Minimum interface version supporting this message.
    pub const SINCE: u32 = 1;
}
impl CommandWriter for WlDisplayErrorEvent {
    fn object_id(&self) -> u32 {
        self.object_id
    }
    fn opcode(&self) -> u16 {
        Self::OPCODE
    }
    fn payload_size(&self) -> usize {
        Self::total_payload_size(&[
            Some(4),
            Some(4),
            FdWriter::encoded_string_size(Some(&self.message)),
        ])
    }
    async fn write(
        self,
        writer: &mut FdPayloadWriter<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<(), FdWriterError> {
        Self::writer_event_interface(state, self.object_id, "wl_display", Self::SINCE)?;
        if self.payload_size() != writer.remaining_quota() {
            return Err(FdWriterError::InvalidProtocol);
        }
        if !FdWriter::is_valid_string(&self.message) {
            return Err(FdWriterError::InvalidProtocol);
        }
        writer.next_u32(self.error_object_id).await?;
        writer.next_u32(self.code).await?;
        writer.next_string(&self.message).await?;
        Ok(())
    }
}

impl CommandReader<Self> for WlDisplayErrorEvent {
    async fn read(
        object_id: u32,
        opcode: u16,
        reader: &mut FdPayloadReader<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        let interface = Self::reader_state(state)?
            .object_interface(object_id)
            .ok_or(FdReaderError::InvalidProtocol)?;
        Self::read_with_interface(object_id, opcode, interface, reader, state).await
    }
}

impl InterfaceReader for WlDisplayErrorEvent {
    async fn read_with_interface(
        object_id: u32,
        opcode: u16,
        interface: ObjectInteface,
        reader: &mut FdPayloadReader<'_>,
        _state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        if !matches!(
            interface,
            ObjectInteface::Core(CoreObjectInteface::WlDisplay { .. })
        ) || opcode != Self::OPCODE
            || interface.version() < Self::SINCE
        {
            return Err(FdReaderError::InvalidProtocol);
        }
        let error_object_id = reader.next_u32().await?;
        let code = reader.next_u32().await?;
        let message = reader.next_string().await?;
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        Ok(Self {
            object_id,
            error_object_id,
            code,
            message,
        })
    }
}
