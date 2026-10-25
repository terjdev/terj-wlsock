use crate::protocol::ext::xdg_shell::ObjectInteface as XdgShellObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, CommandWriterExt, InterfaceReader,
    ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Server event `xdg_popup.repositioned`.
///
/// Opcode 2; available since version 3.
/// Wire framing and receiver metadata are checked; protocol semantics belong to the caller.
/// ```xml
/// <event name="repositioned" since="3">
///       <description summary="signal the completion of a repositioned request">
///     The repositioned event is sent as part of a popup configuration
///     sequence, together with xdg_popup.configure and lastly
///     xdg_surface.configure to notify the completion of a reposition request.
///
///     The repositioned event is to notify about the completion of a
///     xdg_popup.reposition request. The token argument is the token passed
///     in the xdg_popup.reposition request.
///
///     Immediately after this event is emitted, xdg_popup.configure and
///     xdg_surface.configure will be sent with the updated size and position,
///     as well as a new configure serial.
///
///     The client should optionally update the content of the popup, but must
///     acknowledge the new popup configuration for the new position to take
///     effect. See xdg_surface.ack_configure for details.
///       </description>
///       <arg name="token" type="uint" summary="reposition request token" />
///     </event>
/// ```
#[derive(Debug)]
pub struct XdgPopupRepositionedEvent {
    /// Wire identifier of the receiver.
    pub object_id: u32,
    /// reposition request token.
    pub token: u32,
}
impl XdgPopupRepositionedEvent {
    /// Opcode in the event namespace.
    pub const OPCODE: u16 = 2;
    /// Minimum supported interface version.
    pub const SINCE: u32 = 3;
}
impl CommandReader<Self> for XdgPopupRepositionedEvent {
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
impl InterfaceReader for XdgPopupRepositionedEvent {
    async fn read_with_interface(
        object_id: u32,
        opcode: u16,
        interface: ObjectInteface,
        reader: &mut FdPayloadReader<'_>,
        _state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        if !matches!(
            interface,
            ObjectInteface::XdgShell(XdgShellObjectInteface::XdgPopup { .. })
        ) || opcode != Self::OPCODE
            || interface.version() < Self::SINCE
        {
            return Err(FdReaderError::InvalidProtocol);
        }
        let token = reader.next_u32().await?;
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        Ok(Self { object_id, token })
    }
}
impl CommandWriter for XdgPopupRepositionedEvent {
    fn object_id(&self) -> u32 {
        self.object_id
    }
    fn opcode(&self) -> u16 {
        Self::OPCODE
    }
    fn payload_size(&self) -> usize {
        Self::total_payload_size(&[Some(4)])
    }
    async fn write(
        self,
        writer: &mut FdPayloadWriter<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<(), FdWriterError> {
        Self::writer_event_interface(state, self.object_id, "xdg_popup", Self::SINCE)?;
        if self.payload_size() != writer.remaining_quota() {
            return Err(FdWriterError::InvalidProtocol);
        }
        writer.next_u32(self.token).await?;
        Ok(())
    }
}
