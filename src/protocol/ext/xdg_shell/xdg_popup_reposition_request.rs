use crate::protocol::ext::xdg_shell::ObjectInteface as XdgShellObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, CommandWriterExt, InterfaceReader,
    ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Client request `xdg_popup.reposition`.
///
/// Opcode 2; available since version 3.
/// Wire framing and receiver metadata are checked; protocol semantics belong to the caller.
/// ```xml
/// <request name="reposition" since="3">
///       <description summary="recalculate the popup's location">
///     Reposition an already-mapped popup. The popup will be placed given the
///     details in the passed xdg_positioner object, and a
///     xdg_popup.repositioned followed by xdg_popup.configure and
///     xdg_surface.configure will be emitted in response. Any parameters set
///     by the previous positioner will be discarded.
///
///     The passed token will be sent in the corresponding
///     xdg_popup.repositioned event. The new popup position will not take
///     effect until the corresponding configure event is acknowledged by the
///     client. See xdg_popup.repositioned for details. The token itself is
///     opaque, and has no other special meaning.
///
///     If multiple reposition requests are sent, the compositor may skip all
///     but the last one.
///
///     If the popup is repositioned in response to a configure event for its
///     parent, the client should send an xdg_positioner.set_parent_configure
///     and possibly an xdg_positioner.set_parent_size request to allow the
///     compositor to properly constrain the popup.
///
///     If the popup is repositioned together with a parent that is being
///     resized, but not in response to a configure event, the client should
///     send an xdg_positioner.set_parent_size request.
///       </description>
///       <arg name="positioner" type="object" interface="xdg_positioner" />
///       <arg name="token" type="uint" summary="reposition request token" />
///     </request>
/// ```
#[derive(Debug)]
pub struct XdgPopupRepositionRequest {
    /// Wire identifier of the receiver.
    pub object_id: u32,
    /// Wire argument `positioner`.
    pub positioner: u32,
    /// reposition request token.
    pub token: u32,
}
impl XdgPopupRepositionRequest {
    /// Opcode in the request namespace.
    pub const OPCODE: u16 = 2;
    /// Minimum supported interface version.
    pub const SINCE: u32 = 3;
}
impl CommandReader<Self> for XdgPopupRepositionRequest {
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
impl InterfaceReader for XdgPopupRepositionRequest {
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
        let positioner = reader.next_u32().await?;
        let token = reader.next_u32().await?;
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        Ok(Self {
            object_id,
            positioner,
            token,
        })
    }
}
impl CommandWriter for XdgPopupRepositionRequest {
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
        Self::writer_request_interface(state, self.object_id, "xdg_popup", Self::SINCE)?;
        if self.payload_size() != writer.remaining_quota() {
            return Err(FdWriterError::InvalidProtocol);
        }
        writer.next_u32(self.positioner).await?;
        writer.next_u32(self.token).await?;
        Ok(())
    }
}
