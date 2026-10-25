use crate::protocol::ext::xdg_shell::ObjectInteface as XdgShellObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, CommandWriterExt, InterfaceReader,
    ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Client request `xdg_positioner.set_reactive`.
///
/// Opcode 7; available since version 3.
/// Wire framing and receiver metadata are checked; protocol semantics belong to the caller.
/// ```xml
/// <request name="set_reactive" since="3">
///       <description summary="continuously reconstrain the surface">
///     When set reactive, the surface is reconstrained if the conditions used
///     for constraining changed, e.g. the parent window moved.
///
///     If the conditions changed and the popup was reconstrained, an
///     xdg_popup.configure event is sent with updated geometry, followed by an
///     xdg_surface.configure event.
///       </description>
///     </request>
/// ```
#[derive(Debug)]
pub struct XdgPositionerSetReactiveRequest {
    /// Wire identifier of the receiver.
    pub object_id: u32,
}
impl XdgPositionerSetReactiveRequest {
    /// Opcode in the request namespace.
    pub const OPCODE: u16 = 7;
    /// Minimum supported interface version.
    pub const SINCE: u32 = 3;
}
impl CommandReader<Self> for XdgPositionerSetReactiveRequest {
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
impl InterfaceReader for XdgPositionerSetReactiveRequest {
    async fn read_with_interface(
        object_id: u32,
        opcode: u16,
        interface: ObjectInteface,
        reader: &mut FdPayloadReader<'_>,
        _state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        if !matches!(
            interface,
            ObjectInteface::XdgShell(XdgShellObjectInteface::XdgPositioner { .. })
        ) || opcode != Self::OPCODE
            || interface.version() < Self::SINCE
        {
            return Err(FdReaderError::InvalidProtocol);
        }
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        Ok(Self { object_id })
    }
}
impl CommandWriter for XdgPositionerSetReactiveRequest {
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
        Self::writer_request_interface(state, self.object_id, "xdg_positioner", Self::SINCE)?;
        if self.payload_size() != writer.remaining_quota() {
            return Err(FdWriterError::InvalidProtocol);
        }
        Ok(())
    }
}
