use crate::protocol::ClientState;
use crate::protocol::{CommandReader, CommandReaderExt};
use crate::protocol::{CommandWriter, CommandWriterExt};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Server event `wl_data_source.cancelled`.
///
/// Opcode 2; available since version 1. The structure contains the receiver and arguments in XML order.
/// Encoding and decoding check wire values and receiver metadata; compositor semantics are not validated.
/// Decoding consumes the complete payload and updates wire object metadata when needed.
/// The caller handles compositor policy and message contents.
/// ```xml
/// <event name="cancelled">
///       <description summary="selection was cancelled">
///         This data source is no longer valid. There are several reasons why
///         this could happen:
///
///         - The data source has been replaced by another data source.
///         - The drag-and-drop operation was performed, but the drop destination
///           did not accept any of the mime types offered through
///           wl_data_source.target.
///         - The drag-and-drop operation was performed, but the drop destination
///           did not select any of the actions present in the mask offered through
///           wl_data_source.action.
///         - The drag-and-drop operation was performed but didn't happen over a
///           surface.
///         - The compositor cancelled the drag-and-drop operation (e.g. compositor
///           dependent timeouts to avoid stale drag-and-drop transfers).
///
///         The client should clean up and destroy this data source.
///
///         For objects of version 2 or older, wl_data_source.cancelled will
///         only be emitted if the data source was replaced by another data
///         source.
///       </description>
///     </event>
/// ```
#[derive(Debug)]
pub struct WlDataSourceCancelledEvent {
    /// Wire identifier of the object receiving the message.
    pub object_id: u32,
}
impl WlDataSourceCancelledEvent {
    /// Opcode in the event namespace of interface wl_data_source.
    pub const OPCODE: u16 = 2;
    /// Minimum interface version supporting this message.
    pub const SINCE: u32 = 1;
}
impl CommandWriter for WlDataSourceCancelledEvent {
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

impl CommandReader<Self> for WlDataSourceCancelledEvent {
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
