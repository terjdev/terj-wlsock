use crate::protocol::ClientState;
use crate::protocol::{CommandWriter, CommandWriterExt};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Server event `wl_pointer.leave`.
///
/// Opcode 1; available since version 1. The structure contains the receiver and arguments in XML order.
/// Encoding checks wire values and receiver metadata; compositor semantics are not validated.
/// The caller handles compositor policy and message contents.
/// ```xml
/// <event name="leave">
///       <description summary="leave event">
///         Notification that this seat's pointer is no longer focused on
///         a certain surface.
///         Unless there is an enter event on another surface in the same frame,
///         the client should assume the button state is reset, in other words
///         that no buttons are pressed.
///         Note that this is different from the buttons being released, the
///         client should avoid triggering actions in response to this.
///
///         The leave notification is sent before the enter notification
///         for the new focus.
///       </description>
///       <arg name="serial" type="uint" summary="serial number of the leave event" />
///       <arg name="surface" type="object" interface="wl_surface" summary="surface left by the pointer" />
///     </event>
/// ```
#[derive(Debug)]
pub struct WlPointerLeaveEvent {
    /// Wire identifier of the object receiving the message.
    pub object_id: u32,
    /// Argument `serial`: serial number of the leave event.
    pub serial: u32,
    /// Argument `surface`: surface left by the pointer.
    /// Object implementing interface wl_surface.
    pub surface: u32,
}
impl WlPointerLeaveEvent {
    /// Opcode in the event namespace of interface wl_pointer.
    pub const OPCODE: u16 = 1;
    /// Minimum interface version supporting this message.
    pub const SINCE: u32 = 1;
}
impl CommandWriter for WlPointerLeaveEvent {
    fn object_id(&self) -> u32 {
        self.object_id
    }
    fn opcode(&self) -> u16 {
        Self::OPCODE
    }
    fn payload_size(&self) -> usize {
        Self::total_payload_size(&[Some(4), Some(4)])
    }
    async fn write(
        self,
        writer: &mut FdPayloadWriter<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<(), FdWriterError> {
        Self::event_interface(state, self.object_id, "wl_pointer", Self::SINCE)?;
        if self.payload_size() != writer.remaining_quota() {
            return Err(FdWriterError::InvalidProtocol);
        }
        writer.next_u32(self.serial).await?;
        writer.next_u32(self.surface).await?;
        Ok(())
    }
}
