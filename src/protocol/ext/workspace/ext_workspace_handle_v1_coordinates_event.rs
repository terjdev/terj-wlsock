use crate::protocol::ext::workspace::ObjectInteface as WorkspaceObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, CommandWriterExt, InterfaceReader,
    ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::FdWriter;
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Server event `ext_workspace_handle_v1.coordinates`.
///
/// Opcode 2; available since version 1.
/// Wire framing and receiver metadata are checked; protocol semantics belong to the caller.
/// ```xml
/// <event name="coordinates">
///       <description summary="workspace coordinates changed">
///         This event is used to organize workspaces into an N-dimensional grid
///         within a workspace group, and if supported, is emitted immediately after
///         the ext_workspace_handle_v1 is created and whenever the coordinates of
///         the workspace change. Compositors may not send this event if they do not
///         conceptually arrange workspaces in this way. If compositors simply
///         number workspaces, without any geometric interpretation, they may send
///         1D coordinates, which clients should not interpret as implying any
///         geometry. Sending an empty array means that the compositor no longer
///         orders the workspace geometrically.
///
///         Coordinates have an arbitrary number of dimensions N with an uint32
///         position along each dimension. By convention if N &gt; 1, the first
///         dimension is X, the second Y, the third Z, and so on. The compositor may
///         chose to utilize these events for a more novel workspace layout
///         convention, however. No guarantee is made about the grid being filled or
///         bounded; there may be a workspace at coordinate 1 and another at
///         coordinate 1000 and none in between. Within a workspace group, however,
///         workspaces must have unique coordinates of equal dimensionality.
///       </description>
///       <arg name="coordinates" type="array" />
///     </event>
/// ```
#[derive(Debug)]
pub struct ExtWorkspaceHandleV1CoordinatesEvent {
    /// Wire identifier of the receiver.
    pub object_id: u32,
    /// Wire argument `coordinates`.
    pub coordinates: Vec<u8>,
}
impl ExtWorkspaceHandleV1CoordinatesEvent {
    /// Opcode in the event namespace.
    pub const OPCODE: u16 = 2;
    /// Minimum supported interface version.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for ExtWorkspaceHandleV1CoordinatesEvent {
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
impl InterfaceReader for ExtWorkspaceHandleV1CoordinatesEvent {
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
        let coordinates = reader.next_array().await?;
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        Ok(Self {
            object_id,
            coordinates,
        })
    }
}
impl CommandWriter for ExtWorkspaceHandleV1CoordinatesEvent {
    fn object_id(&self) -> u32 {
        self.object_id
    }
    fn opcode(&self) -> u16 {
        Self::OPCODE
    }
    fn payload_size(&self) -> usize {
        Self::total_payload_size(&[FdWriter::encoded_array_size(self.coordinates.len())])
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
        writer.next_array(&self.coordinates).await?;
        Ok(())
    }
}
