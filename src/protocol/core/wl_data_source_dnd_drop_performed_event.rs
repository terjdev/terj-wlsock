use crate::protocol::ClientState;
use crate::protocol::{CommandReader, CommandReaderExt};
use crate::protocol::{CommandWriter, CommandWriterExt};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Server event `wl_data_source.dnd_drop_performed`.
///
/// Opcode 3; available since version 3. The structure contains the receiver and arguments in XML order.
/// Encoding and decoding check wire values and receiver metadata; compositor semantics are not validated.
/// Decoding consumes the complete payload and updates wire object metadata when needed.
/// The caller handles compositor policy and message contents.
/// ```xml
/// <event name="dnd_drop_performed" since="3">
///       <description summary="the drag-and-drop operation physically finished">
///         The user performed the drop action. This event does not indicate
///         acceptance, wl_data_source.cancelled may still be emitted afterwards
///         if the drop destination does not accept any mime type.
///
///         However, this event might not be received if the compositor cancelled
///         the drag-and-drop operation before this event could happen.
///
///         Note that the data_source may still be used in the future and should
///         not be destroyed here.
///       </description>
///     </event>
/// ```
#[derive(Debug)]
pub struct WlDataSourceDndDropPerformedEvent {
    /// Wire identifier of the object receiving the message.
    pub object_id: u32,
}
impl WlDataSourceDndDropPerformedEvent {
    /// Opcode in the event namespace of interface wl_data_source.
    pub const OPCODE: u16 = 3;
    /// Minimum interface version supporting this message.
    pub const SINCE: u32 = 3;
}
impl CommandWriter for WlDataSourceDndDropPerformedEvent {
    fn object_id(&self) -> u32 {
        self.object_id
    }
    fn opcode(&self) -> u16 {
        Self::OPCODE
    }
    fn payload_size(&self) -> usize {
        Self::total_payload_size(&[])
    }
    async fn write(
        self,
        writer: &mut FdPayloadWriter<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<(), FdWriterError> {
        Self::event_interface(state, self.object_id, "wl_data_source", Self::SINCE)?;
        if self.payload_size() != writer.remaining_quota() {
            return Err(FdWriterError::InvalidProtocol);
        }
        Ok(())
    }
}

impl CommandReader<Self> for WlDataSourceDndDropPerformedEvent {
    async fn read(
        object_id: u32,
        opcode: u16,
        reader: &mut FdPayloadReader<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        Self::event_interface(
            state,
            object_id,
            opcode,
            Self::OPCODE,
            "wl_data_source",
            Self::SINCE,
        )?;
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        Ok(Self { object_id })
    }
}
