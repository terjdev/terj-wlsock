use crate::protocol::{ClientState, CommandReader, CommandWriter};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Wire value `wp_commit_timer_v1.error`.
///
/// Unknown numeric values and flag bits are preserved.
/// ```xml
/// <enum name="error">
///       <entry name="invalid_timestamp" value="0" summary="timestamp contains an invalid value" />
///       <entry name="timestamp_exists" value="1" summary="timestamp exists" />
///       <entry name="surface_destroyed" value="2" summary="the associated surface no longer exists" />
///     </enum>
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct WpCommitTimerV1Error(pub u32);
impl WpCommitTimerV1Error {
    /// timestamp contains an invalid value; available since version 1.
    pub const INVALID_TIMESTAMP: Self = Self(0);
    /// timestamp exists; available since version 1.
    pub const TIMESTAMP_EXISTS: Self = Self(1);
    /// the associated surface no longer exists; available since version 1.
    pub const SURFACE_DESTROYED: Self = Self(2);
}
impl CommandReader<Self> for WpCommitTimerV1Error {
    async fn read(
        _object_id: u32,
        _opcode: u16,
        reader: &mut FdPayloadReader<'_>,
        _state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        Ok(Self(reader.next_u32().await?))
    }
}
impl CommandWriter for WpCommitTimerV1Error {
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
