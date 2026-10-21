use crate::protocol::ClientState;
use crate::protocol::{CommandReader, CommandReaderExt};
use crate::protocol::{CommandWriter, CommandWriterExt};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Server event `wl_seat.name`.
///
/// Opcode 1; available since version 2. The structure contains the receiver and arguments in XML order.
/// Encoding and decoding check wire values and receiver metadata; compositor semantics are not validated.
/// Decoding consumes the complete payload and updates wire object metadata when needed.
/// The caller handles compositor policy and message contents.
/// ```xml
/// <event name="name" since="2">
///       <description summary="unique identifier for this seat">
///         In a multi-seat configuration the seat name can be used by clients to
///         help identify which physical devices the seat represents.
///
///         The seat name is a UTF-8 string with no convention defined for its
///         contents. Each name is unique among all wl_seat globals. The name is
///         only guaranteed to be unique for the current compositor instance.
///
///         The same seat names are used for all clients. Thus, the name can be
///         shared across processes to refer to a specific wl_seat global.
///
///         The name event is sent after binding to the seat global, and should be sent
///         before announcing capabilities. This event is only sent once per seat object,
///         and the name does not change over the lifetime of the wl_seat global.
///
///         Compositors may re-use the same seat name if the wl_seat global is
///         destroyed and re-created later.
///       </description>
///       <arg name="name" type="string" summary="seat identifier" />
///     </event>
/// ```
#[derive(Debug)]
pub struct WlSeatNameEvent {
    /// Wire identifier of the object receiving the message.
    pub object_id: u32,
    /// Argument `name`: seat identifier.
    pub name: String,
}
impl WlSeatNameEvent {
    /// Opcode in the event namespace of interface wl_seat.
    pub const OPCODE: u16 = 1;
    /// Minimum interface version supporting this message.
    pub const SINCE: u32 = 2;
}
impl CommandWriter for WlSeatNameEvent {
    fn object_id(&self) -> u32 {
        self.object_id
    }
    fn opcode(&self) -> u16 {
        Self::OPCODE
    }
    fn payload_size(&self) -> usize {
        Self::total_payload_size(&[FdWriter::encoded_string_size(Some(&self.name))])
    }
    async fn write(
        self,
        writer: &mut FdPayloadWriter<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<(), FdWriterError> {
        Self::writer_event_interface(state, self.object_id, "wl_seat", Self::SINCE)?;
        if self.payload_size() != writer.remaining_quota() {
            return Err(FdWriterError::InvalidProtocol);
        }
        if !FdWriter::is_valid_string(&self.name) {
            return Err(FdWriterError::InvalidProtocol);
        }
        writer.next_string(&self.name).await?;
        Ok(())
    }
}

impl CommandReader<Self> for WlSeatNameEvent {
    async fn read(
        object_id: u32,
        opcode: u16,
        reader: &mut FdPayloadReader<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        Self::reader_event_interface(
            state,
            object_id,
            opcode,
            Self::OPCODE,
            "wl_seat",
            Self::SINCE,
        )?;
        let name = reader.next_string().await?;
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        Ok(Self { object_id, name })
    }
}
