use crate::protocol::ClientState;
use crate::protocol::{CommandWriter, CommandWriterExt};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Server event `wl_touch.cancel`.
///
/// Opcode 4; available since version 1. The structure contains the receiver and arguments in XML order.
/// Encoding checks wire values and receiver metadata; compositor semantics are not validated.
/// The caller handles compositor policy and message contents.
/// ```xml
/// <event name="cancel">
///       <description summary="touch session cancelled">
///         Sent if the compositor decides the touch stream is a global
///         gesture. No further events are sent to the clients from that
///         particular gesture. Touch cancellation applies to all touch points
///         currently active on this client's surface. The client is
///         responsible for finalizing the touch points, future touch points on
///         this surface may reuse the touch point ID.
///
///         No frame event is required after the cancel event.
///       </description>
///     </event>
/// ```
#[derive(Debug)]
pub struct WlTouchCancelEvent {
    /// Wire identifier of the object receiving the message.
    pub object_id: u32,
}
impl WlTouchCancelEvent {
    /// Opcode in the event namespace of interface wl_touch.
    pub const OPCODE: u16 = 4;
    /// Minimum interface version supporting this message.
    pub const SINCE: u32 = 1;
}
impl CommandWriter for WlTouchCancelEvent {
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
        Self::event_interface(state, self.object_id, "wl_touch", Self::SINCE)?;
        if self.payload_size() != writer.remaining_quota() {
            return Err(FdWriterError::InvalidProtocol);
        }
        Ok(())
    }
}
