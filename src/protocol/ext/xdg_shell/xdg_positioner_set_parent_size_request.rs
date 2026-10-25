use crate::protocol::ext::xdg_shell::ObjectInteface as XdgShellObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, CommandWriterExt, InterfaceReader,
    ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Client request `xdg_positioner.set_parent_size`.
///
/// Opcode 8; available since version 3.
/// Wire framing and receiver metadata are checked; protocol semantics belong to the caller.
/// ```xml
/// <request name="set_parent_size" since="3">
///       <description summary="set parent size">
///     Set the parent window geometry the compositor should use when
///     positioning the popup. The compositor may use this information to
///     determine the future state the popup should be constrained using. If
///     this doesn't match the dimension of the parent the popup is eventually
///     positioned against, the behavior is undefined.
///
///     The arguments are given in the surface-local coordinate space.
///       </description>
///       <arg name="parent_width" type="int" summary="future window geometry width of parent" />
///       <arg name="parent_height" type="int" summary="future window geometry height of parent" />
///     </request>
/// ```
#[derive(Debug)]
pub struct XdgPositionerSetParentSizeRequest {
    /// Wire identifier of the receiver.
    pub object_id: u32,
    /// future window geometry width of parent.
    pub parent_width: i32,
    /// future window geometry height of parent.
    pub parent_height: i32,
}
impl XdgPositionerSetParentSizeRequest {
    /// Opcode in the request namespace.
    pub const OPCODE: u16 = 8;
    /// Minimum supported interface version.
    pub const SINCE: u32 = 3;
}
impl CommandReader<Self> for XdgPositionerSetParentSizeRequest {
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
impl InterfaceReader for XdgPositionerSetParentSizeRequest {
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
        let parent_width = reader.next_i32().await?;
        let parent_height = reader.next_i32().await?;
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        Ok(Self {
            object_id,
            parent_width,
            parent_height,
        })
    }
}
impl CommandWriter for XdgPositionerSetParentSizeRequest {
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
        Self::writer_request_interface(state, self.object_id, "xdg_positioner", Self::SINCE)?;
        if self.payload_size() != writer.remaining_quota() {
            return Err(FdWriterError::InvalidProtocol);
        }
        writer.next_i32(self.parent_width).await?;
        writer.next_i32(self.parent_height).await?;
        Ok(())
    }
}
