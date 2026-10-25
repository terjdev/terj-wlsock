use super::XdgPositionerGravity;
use crate::protocol::ext::xdg_shell::ObjectInteface as XdgShellObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, CommandWriterExt, InterfaceReader,
    ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Client request `xdg_positioner.set_gravity`.
///
/// Opcode 4; available since version 1.
/// Wire framing and receiver metadata are checked; protocol semantics belong to the caller.
/// ```xml
/// <request name="set_gravity">
///       <description summary="set child surface gravity">
///     Defines in what direction a surface should be positioned, relative to
///     the anchor point of the parent surface. If a corner gravity is
///     specified (e.g. 'bottom_right' or 'top_left'), then the child surface
///     will be placed towards the specified gravity; otherwise, the child
///     surface will be centered over the anchor point on any axis that had no
///     gravity specified. If the gravity is not in the ‘gravity’ enum, an
///     invalid_input error is raised.
///       </description>
///       <arg name="gravity" type="uint" enum="gravity" summary="gravity direction" />
///     </request>
/// ```
#[derive(Debug)]
pub struct XdgPositionerSetGravityRequest {
    /// Wire identifier of the receiver.
    pub object_id: u32,
    /// gravity direction.
    pub gravity: XdgPositionerGravity,
}
impl XdgPositionerSetGravityRequest {
    /// Opcode in the request namespace.
    pub const OPCODE: u16 = 4;
    /// Minimum supported interface version.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for XdgPositionerSetGravityRequest {
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
impl InterfaceReader for XdgPositionerSetGravityRequest {
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
        let gravity = XdgPositionerGravity(reader.next_u32().await?);
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        Ok(Self { object_id, gravity })
    }
}
impl CommandWriter for XdgPositionerSetGravityRequest {
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
        Self::writer_request_interface(state, self.object_id, "xdg_positioner", Self::SINCE)?;
        if self.payload_size() != writer.remaining_quota() {
            return Err(FdWriterError::InvalidProtocol);
        }
        writer.next_u32(self.gravity.0).await?;
        Ok(())
    }
}
