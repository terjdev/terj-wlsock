use crate::protocol::ClientState;
use crate::protocol::{CommandWriter, CommandWriterExt};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Server event `wl_data_device.leave`.
///
/// Opcode 2; available since version 1. The structure contains the receiver and arguments in XML order.
/// Encoding checks wire values and receiver metadata; compositor semantics are not validated.
/// The caller handles compositor policy and message contents.
/// ```xml
/// <event name="leave">
///       <description summary="end drag-and-drop session">
///         This event is sent when the drag-and-drop pointer leaves the
///         surface and the session ends.  The client must destroy the
///         wl_data_offer introduced at enter time at this point.
///       </description>
///     </event>
/// ```
#[derive(Debug)]
pub struct WlDataDeviceLeaveEvent {
    /// Wire identifier of the object receiving the message.
    pub object_id: u32,
}
impl WlDataDeviceLeaveEvent {
    /// Opcode in the event namespace of interface wl_data_device.
    pub const OPCODE: u16 = 2;
    /// Minimum interface version supporting this message.
    pub const SINCE: u32 = 1;
}
impl CommandWriter for WlDataDeviceLeaveEvent {
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
        Self::event_interface(state, self.object_id, "wl_data_device", Self::SINCE)?;
        if self.payload_size() != writer.remaining_quota() {
            return Err(FdWriterError::InvalidProtocol);
        }
        Ok(())
    }
}
