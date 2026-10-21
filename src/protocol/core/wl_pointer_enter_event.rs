use crate::protocol::ClientState;
use crate::protocol::{CommandWriter, CommandWriterExt};
use crate::utils::writer::{FdPayloadWriter, FdWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Server event `wl_pointer.enter`.
///
/// Opcode 0; available since version 1. The structure contains the receiver and arguments in XML order.
/// Encoding checks wire values and receiver metadata; compositor semantics are not validated.
/// The caller handles compositor policy and message contents.
/// ```xml
/// <event name="enter">
///       <description summary="enter event">
///         Notification that this seat's pointer is focused on a certain
///         surface.
///
///         When a seat's focus enters a surface, the pointer image
///         is undefined and a client should respond to this event by setting
///         an appropriate pointer image with the set_cursor request.
///       </description>
///       <arg name="serial" type="uint" summary="serial number of the enter event" />
///       <arg name="surface" type="object" interface="wl_surface" summary="surface entered by the pointer" />
///       <arg name="surface_x" type="fixed" summary="surface-local x coordinate" />
///       <arg name="surface_y" type="fixed" summary="surface-local y coordinate" />
///     </event>
/// ```
#[derive(Debug)]
pub struct WlPointerEnterEvent {
    /// Wire identifier of the object receiving the message.
    pub object_id: u32,
    /// Argument `serial`: serial number of the enter event.
    pub serial: u32,
    /// Argument `surface`: surface entered by the pointer.
    /// Object implementing interface wl_surface.
    pub surface: u32,
    /// Argument `surface_x`: surface-local x coordinate.
    pub surface_x: f64,
    /// Argument `surface_y`: surface-local y coordinate.
    pub surface_y: f64,
}
impl WlPointerEnterEvent {
    /// Opcode in the event namespace of interface wl_pointer.
    pub const OPCODE: u16 = 0;
    /// Minimum interface version supporting this message.
    pub const SINCE: u32 = 1;
}
impl CommandWriter for WlPointerEnterEvent {
    fn object_id(&self) -> u32 {
        self.object_id
    }
    fn opcode(&self) -> u16 {
        Self::OPCODE
    }
    fn payload_size(&self) -> usize {
        Self::total_payload_size(&[Some(4), Some(4), Some(4), Some(4)])
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
        if !FdWriter::is_valid_fixed(self.surface_x) {
            return Err(FdWriterError::InvalidProtocol);
        }
        if !FdWriter::is_valid_fixed(self.surface_y) {
            return Err(FdWriterError::InvalidProtocol);
        }
        writer.next_u32(self.serial).await?;
        writer.next_u32(self.surface).await?;
        writer.next_fixed(self.surface_x).await?;
        writer.next_fixed(self.surface_y).await?;
        Ok(())
    }
}
