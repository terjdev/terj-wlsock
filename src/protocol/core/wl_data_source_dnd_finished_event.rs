use crate::protocol::ClientState;
use crate::protocol::{CommandWriter, CommandWriterExt};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Server event `wl_data_source.dnd_finished`.
///
/// Opcode 4; available since version 3. The structure contains the receiver and arguments in XML order.
/// Encoding checks wire values and receiver metadata; compositor semantics are not validated.
/// The caller handles compositor policy and message contents.
/// ```xml
/// <event name="dnd_finished" since="3">
///       <description summary="the drag-and-drop operation concluded">
///         The drop destination finished interoperating with this data
///         source, so the client is now free to destroy this data source and
///         free all associated data.
///
///         If the action used to perform the operation was "move", the
///         source can now delete the transferred data.
///       </description>
///     </event>
/// ```
#[derive(Debug)]
pub struct WlDataSourceDndFinishedEvent {
    /// Wire identifier of the object receiving the message.
    pub object_id: u32,
}
impl WlDataSourceDndFinishedEvent {
    /// Opcode in the event namespace of interface wl_data_source.
    pub const OPCODE: u16 = 4;
    /// Minimum interface version supporting this message.
    pub const SINCE: u32 = 3;
}
impl CommandWriter for WlDataSourceDndFinishedEvent {
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
