use crate::protocol::ClientState;
use crate::protocol::{CommandWriter, CommandWriterExt};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Server event `wl_data_device.drop`.
///
/// Opcode 4; available since version 1. The structure contains the receiver and arguments in XML order.
/// Encoding checks wire values and receiver metadata; compositor semantics are not validated.
/// The caller handles compositor policy and message contents.
/// ```xml
/// <event name="drop">
///       <description summary="end drag-and-drop session successfully">
///         The event is sent when a drag-and-drop operation is ended
///         because the implicit grab is removed.
///
///         The drag-and-drop destination is expected to honor the last action
///         received through wl_data_offer.action, if the resulting action is
///         "copy" or "move", the destination can still perform
///         wl_data_offer.receive requests, and is expected to end all
///         transfers with a wl_data_offer.finish request.
///
///         If the resulting action is "ask", the action will not be considered
///         final. The drag-and-drop destination is expected to perform one last
///         wl_data_offer.set_actions request, or wl_data_offer.destroy in order
///         to cancel the operation.
///       </description>
///     </event>
/// ```
#[derive(Debug)]
pub struct WlDataDeviceDropEvent {
    /// Wire identifier of the object receiving the message.
    pub object_id: u32,
}
impl WlDataDeviceDropEvent {
    /// Opcode in the event namespace of interface wl_data_device.
    pub const OPCODE: u16 = 4;
    /// Minimum interface version supporting this message.
    pub const SINCE: u32 = 1;
}
impl CommandWriter for WlDataDeviceDropEvent {
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
