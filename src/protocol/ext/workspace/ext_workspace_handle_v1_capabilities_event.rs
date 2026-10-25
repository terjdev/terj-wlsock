use super::ExtWorkspaceHandleV1WorkspaceCapabilities;
use crate::protocol::ext::workspace::ObjectInteface as WorkspaceObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, CommandWriterExt, InterfaceReader,
    ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Server event `ext_workspace_handle_v1.capabilities`.
///
/// Opcode 4; available since version 1.
/// Wire framing and receiver metadata are checked; protocol semantics belong to the caller.
/// ```xml
/// <event name="capabilities">
///       <description summary="compositor capabilities">
///         This event advertises the capabilities supported by the compositor. If
///         a capability isn't supported, clients should hide or disable the UI
///         elements that expose this functionality. For instance, if the
///         compositor doesn't advertise support for removing workspaces, a button
///         triggering the remove request should not be displayed.
///
///         The compositor will ignore requests it doesn't support. For instance,
///         a compositor which doesn't advertise support for remove will ignore
///         remove requests.
///
///         Compositors must send this event once after creation of an
///         ext_workspace_handle_v1 . When the capabilities change, compositors
///         must send this event again.
///       </description>
///       <arg name="capabilities" type="uint" summary="capabilities" enum="workspace_capabilities" />
///     </event>
/// ```
#[derive(Debug)]
pub struct ExtWorkspaceHandleV1CapabilitiesEvent {
    /// Wire identifier of the receiver.
    pub object_id: u32,
    /// capabilities.
    pub capabilities: ExtWorkspaceHandleV1WorkspaceCapabilities,
}
impl ExtWorkspaceHandleV1CapabilitiesEvent {
    /// Opcode in the event namespace.
    pub const OPCODE: u16 = 4;
    /// Minimum supported interface version.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for ExtWorkspaceHandleV1CapabilitiesEvent {
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
impl InterfaceReader for ExtWorkspaceHandleV1CapabilitiesEvent {
    async fn read_with_interface(
        object_id: u32,
        opcode: u16,
        interface: ObjectInteface,
        reader: &mut FdPayloadReader<'_>,
        _state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        if !matches!(
            interface,
            ObjectInteface::Workspace(WorkspaceObjectInteface::ExtWorkspaceHandleV1 { .. })
        ) || opcode != Self::OPCODE
            || interface.version() < Self::SINCE
        {
            return Err(FdReaderError::InvalidProtocol);
        }
        let capabilities = ExtWorkspaceHandleV1WorkspaceCapabilities(reader.next_u32().await?);
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        Ok(Self {
            object_id,
            capabilities,
        })
    }
}
impl CommandWriter for ExtWorkspaceHandleV1CapabilitiesEvent {
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
        Self::writer_event_interface(
            state,
            self.object_id,
            "ext_workspace_handle_v1",
            Self::SINCE,
        )?;
        if self.payload_size() != writer.remaining_quota() {
            return Err(FdWriterError::InvalidProtocol);
        }
        writer.next_u32(self.capabilities.0).await?;
        Ok(())
    }
}
