use crate::protocol::{ClientState, CommandReader, CommandWriter};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Wire value `zwlr_output_head_v1.adaptive_sync_state`.
///
/// Unknown numeric values are preserved.
/// ```xml
/// <enum name="adaptive_sync_state" since="4">
///       <entry name="disabled" value="0" summary="adaptive sync is disabled" />
///       <entry name="enabled" value="1" summary="adaptive sync is enabled" />
///     </enum>
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct ZwlrOutputHeadV1AdaptiveSyncState(pub u32);
impl ZwlrOutputHeadV1AdaptiveSyncState {
    /// adaptive sync is disabled; available since version 4.
    pub const DISABLED: Self = Self(0);
    /// adaptive sync is enabled; available since version 4.
    pub const ENABLED: Self = Self(1);
}
impl CommandReader<Self> for ZwlrOutputHeadV1AdaptiveSyncState {
    async fn read(
        _object_id: u32,
        _opcode: u16,
        reader: &mut FdPayloadReader<'_>,
        _state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        Ok(Self(reader.next_u32().await?))
    }
}
impl CommandWriter for ZwlrOutputHeadV1AdaptiveSyncState {
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
