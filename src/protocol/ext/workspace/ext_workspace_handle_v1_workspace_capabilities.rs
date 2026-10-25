use crate::protocol::{ClientState, CommandReader, CommandWriter};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Wire value `ext_workspace_handle_v1.workspace_capabilities`.
///
/// Unknown numeric values and flag bits are preserved.
/// ```xml
/// <enum name="workspace_capabilities" bitfield="true">
///       <entry name="activate" value="1" summary="activate request is available" />
///       <entry name="deactivate" value="2" summary="deactivate request is available" />
///       <entry name="remove" value="4" summary="remove request is available" />
///       <entry name="assign" value="8" summary="assign request is available" />
///     </enum>
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct ExtWorkspaceHandleV1WorkspaceCapabilities(pub u32);
impl ExtWorkspaceHandleV1WorkspaceCapabilities {
    /// activate request is available; available since version 1.
    pub const ACTIVATE: Self = Self(1);
    /// deactivate request is available; available since version 1.
    pub const DEACTIVATE: Self = Self(2);
    /// remove request is available; available since version 1.
    pub const REMOVE: Self = Self(4);
    /// assign request is available; available since version 1.
    pub const ASSIGN: Self = Self(8);
}
impl std::ops::BitOr for ExtWorkspaceHandleV1WorkspaceCapabilities {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl std::ops::BitAnd for ExtWorkspaceHandleV1WorkspaceCapabilities {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl CommandReader<Self> for ExtWorkspaceHandleV1WorkspaceCapabilities {
    async fn read(
        _object_id: u32,
        _opcode: u16,
        reader: &mut FdPayloadReader<'_>,
        _state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        Ok(Self(reader.next_u32().await?))
    }
}
impl CommandWriter for ExtWorkspaceHandleV1WorkspaceCapabilities {
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
