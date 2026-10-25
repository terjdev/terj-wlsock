use crate::protocol::ext::xdg_shell::ObjectInteface as XdgShellObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, CommandWriterExt, InterfaceReader,
    ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Client request `xdg_toplevel.set_parent`.
///
/// Opcode 1; available since version 1.
/// Wire framing and receiver metadata are checked; protocol semantics belong to the caller.
/// ```xml
/// <request name="set_parent">
///       <description summary="set the parent of this surface">
///     Set the "parent" of this surface. This surface should be stacked
///     above the parent surface and all other ancestor surfaces.
///
///     Parent surfaces should be set on dialogs, toolboxes, or other
///     "auxiliary" surfaces, so that the parent is raised when the dialog
///     is raised.
///
///     Setting a null parent for a child surface unsets its parent. Setting
///     a null parent for a surface which currently has no parent is a no-op.
///
///     Only mapped surfaces can have child surfaces. Setting a parent which
///     is not mapped is equivalent to setting a null parent. If a surface
///     becomes unmapped, its children's parent is set to the parent of
///     the now-unmapped surface. If the now-unmapped surface has no parent,
///     its children's parent is unset. If the now-unmapped surface becomes
///     mapped again, its parent-child relationship is not restored.
///
///     The parent toplevel must not be one of the child toplevel's
///     descendants, and the parent must be different from the child toplevel,
///     otherwise the invalid_parent protocol error is raised.
///       </description>
///       <arg name="parent" type="object" interface="xdg_toplevel" allow-null="true" summary="parent surface for this surface" />
///     </request>
/// ```
#[derive(Debug)]
pub struct XdgToplevelSetParentRequest {
    /// Wire identifier of the receiver.
    pub object_id: u32,
    /// parent surface for this surface.
    pub parent: Option<u32>,
}
impl XdgToplevelSetParentRequest {
    /// Opcode in the request namespace.
    pub const OPCODE: u16 = 1;
    /// Minimum supported interface version.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for XdgToplevelSetParentRequest {
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
impl InterfaceReader for XdgToplevelSetParentRequest {
    async fn read_with_interface(
        object_id: u32,
        opcode: u16,
        interface: ObjectInteface,
        reader: &mut FdPayloadReader<'_>,
        _state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        if !matches!(
            interface,
            ObjectInteface::XdgShell(XdgShellObjectInteface::XdgToplevel { .. })
        ) || opcode != Self::OPCODE
            || interface.version() < Self::SINCE
        {
            return Err(FdReaderError::InvalidProtocol);
        }
        let parent = {
            let id = reader.next_u32().await?;
            (id != 0).then_some(id)
        };
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        Ok(Self { object_id, parent })
    }
}
impl CommandWriter for XdgToplevelSetParentRequest {
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
        Self::writer_request_interface(state, self.object_id, "xdg_toplevel", Self::SINCE)?;
        if self.payload_size() != writer.remaining_quota() {
            return Err(FdWriterError::InvalidProtocol);
        }
        writer.next_u32(self.parent.unwrap_or(0)).await?;
        Ok(())
    }
}
