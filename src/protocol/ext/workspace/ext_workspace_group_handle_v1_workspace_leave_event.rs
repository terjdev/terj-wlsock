use crate::protocol::ext::workspace::ObjectInteface as WorkspaceObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, CommandWriterExt, InterfaceReader,
    ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Server event `ext_workspace_group_handle_v1.workspace_leave`.
///
/// Opcode 4; available since version 1.
/// Wire framing and receiver metadata are checked; protocol semantics belong to the caller.
/// ```xml
/// <event name="workspace_leave">
///       <description summary="workspace removed from workspace group">
///         This event is emitted whenever a workspace is removed from this group.
///       </description>
///       <arg name="workspace" type="object" interface="ext_workspace_handle_v1" />
///     </event>
/// ```
#[derive(Debug)]
pub struct ExtWorkspaceGroupHandleV1WorkspaceLeaveEvent {
    /// Wire identifier of the receiver.
    pub object_id: u32,
    /// Wire argument `workspace`.
    pub workspace: u32,
}
impl ExtWorkspaceGroupHandleV1WorkspaceLeaveEvent {
    /// Opcode in the event namespace.
    pub const OPCODE: u16 = 4;
    /// Minimum supported interface version.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for ExtWorkspaceGroupHandleV1WorkspaceLeaveEvent {
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
impl InterfaceReader for ExtWorkspaceGroupHandleV1WorkspaceLeaveEvent {
    async fn read_with_interface(
        object_id: u32,
        opcode: u16,
        interface: ObjectInteface,
        reader: &mut FdPayloadReader<'_>,
        _state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        if !matches!(
            interface,
            ObjectInteface::Workspace(WorkspaceObjectInteface::ExtWorkspaceGroupHandleV1 { .. })
        ) || opcode != Self::OPCODE
            || interface.version() < Self::SINCE
        {
            return Err(FdReaderError::InvalidProtocol);
        }
        let workspace = reader.next_u32().await?;
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        Ok(Self {
            object_id,
            workspace,
        })
    }
}
impl CommandWriter for ExtWorkspaceGroupHandleV1WorkspaceLeaveEvent {
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
            "ext_workspace_group_handle_v1",
            Self::SINCE,
        )?;
        if self.payload_size() != writer.remaining_quota() {
            return Err(FdWriterError::InvalidProtocol);
        }
        writer.next_u32(self.workspace).await?;
        Ok(())
    }
}
