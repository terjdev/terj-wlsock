use crate::protocol::ext::workspace::ObjectInteface as WorkspaceObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, CommandWriterExt, InterfaceReader,
    ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::FdWriter;
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Server event `ext_workspace_handle_v1.id`.
///
/// Opcode 0; available since version 1.
/// Wire framing and receiver metadata are checked; protocol semantics belong to the caller.
/// ```xml
/// <event name="id">
///       <description summary="workspace id">
///         If this event is emitted, it will be send immediately after the
///         ext_workspace_handle_v1 is created or when an id is assigned to
///         a workspace (at most once during its lifetime).
///
///         An id will never change during the lifetime of the `ext_workspace_handle_v1`
///         and is guaranteed to be unique during its lifetime.
///
///         Ids are not human-readable and shouldn't be displayed, use `name` for that purpose.
///
///         Compositors are expected to only send ids for workspaces likely stable across multiple
///         sessions and can be used by clients to store preferences for workspaces. Workspaces without
///         ids should be considered temporary and any data associated with them should be deleted once
///         the respective object is lost.
///       </description>
///       <arg name="id" type="string" />
///     </event>
/// ```
#[derive(Debug)]
pub struct ExtWorkspaceHandleV1IdEvent {
    /// Wire identifier of the receiver.
    pub object_id: u32,
    /// Wire argument `id`.
    pub id: String,
}
impl ExtWorkspaceHandleV1IdEvent {
    /// Opcode in the event namespace.
    pub const OPCODE: u16 = 0;
    /// Minimum supported interface version.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for ExtWorkspaceHandleV1IdEvent {
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
impl InterfaceReader for ExtWorkspaceHandleV1IdEvent {
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
        let id = reader.next_string().await?;
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        Ok(Self { object_id, id })
    }
}
impl CommandWriter for ExtWorkspaceHandleV1IdEvent {
    fn object_id(&self) -> u32 {
        self.object_id
    }
    fn opcode(&self) -> u16 {
        Self::OPCODE
    }
    fn payload_size(&self) -> usize {
        Self::total_payload_size(&[FdWriter::encoded_string_size(Some(&self.id))])
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
        if !FdWriter::is_valid_string(&self.id) {
            return Err(FdWriterError::InvalidProtocol);
        }
        writer.next_string(&self.id).await?;
        Ok(())
    }
}
