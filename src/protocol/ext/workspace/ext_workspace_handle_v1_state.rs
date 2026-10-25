use crate::protocol::{ClientState, CommandReader, CommandWriter};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Wire value `ext_workspace_handle_v1.state`.
///
/// Unknown numeric values and flag bits are preserved.
/// ```xml
/// <enum name="state" bitfield="true">
///       <description summary="types of states on the workspace">
///         The different states that a workspace can have.
///       </description>
///
///       <entry name="active" value="1" summary="the workspace is active" />
///       <entry name="urgent" value="2" summary="the workspace requests attention" />
///       <entry name="hidden" value="4">
///         <description summary="the workspace is not visible">
///           The workspace is not visible in its workspace group, and clients
///           attempting to visualize the compositor workspace state should not
///           display such workspaces.
///         </description>
///       </entry>
///     </enum>
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct ExtWorkspaceHandleV1State(pub u32);
impl ExtWorkspaceHandleV1State {
    /// the workspace is active; available since version 1.
    pub const ACTIVE: Self = Self(1);
    /// the workspace requests attention; available since version 1.
    pub const URGENT: Self = Self(2);
    /// the workspace is not visible; available since version 1.
    pub const HIDDEN: Self = Self(4);
}
impl std::ops::BitOr for ExtWorkspaceHandleV1State {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl std::ops::BitAnd for ExtWorkspaceHandleV1State {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl CommandReader<Self> for ExtWorkspaceHandleV1State {
    async fn read(
        _object_id: u32,
        _opcode: u16,
        reader: &mut FdPayloadReader<'_>,
        _state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        Ok(Self(reader.next_u32().await?))
    }
}
impl CommandWriter for ExtWorkspaceHandleV1State {
    fn object_id(&self) -> u32 {
        0
    }
    fn opcode(&self) -> u16 {
        0
    }
    fn payload_size(&self) -> usize {
        4
    }
    async fn write(
        self,
        writer: &mut FdPayloadWriter<'_>,
        _state: &Arc<Mutex<ClientState>>,
    ) -> Result<(), FdWriterError> {
        writer.next_u32(self.0).await
    }
}
