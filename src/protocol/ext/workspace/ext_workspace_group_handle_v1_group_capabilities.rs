use crate::protocol::{ClientState, CommandReader, CommandWriter};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Wire value `ext_workspace_group_handle_v1.group_capabilities`.
///
/// Unknown numeric values and flag bits are preserved.
/// ```xml
/// <enum name="group_capabilities" bitfield="true">
///       <entry name="create_workspace" value="1" summary="create_workspace request is available" />
///     </enum>
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct ExtWorkspaceGroupHandleV1GroupCapabilities(pub u32);
impl ExtWorkspaceGroupHandleV1GroupCapabilities {
    /// create_workspace request is available; available since version 1.
    pub const CREATE_WORKSPACE: Self = Self(1);
}
impl std::ops::BitOr for ExtWorkspaceGroupHandleV1GroupCapabilities {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl std::ops::BitAnd for ExtWorkspaceGroupHandleV1GroupCapabilities {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl CommandReader<Self> for ExtWorkspaceGroupHandleV1GroupCapabilities {
    async fn read(
        _object_id: u32,
        _opcode: u16,
        reader: &mut FdPayloadReader<'_>,
        _state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        Ok(Self(reader.next_u32().await?))
    }
}
impl CommandWriter for ExtWorkspaceGroupHandleV1GroupCapabilities {
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
