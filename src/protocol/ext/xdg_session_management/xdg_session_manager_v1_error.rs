use crate::protocol::{ClientState, CommandReader, CommandWriter};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Wire value `xdg_session_manager_v1.error`.
///
/// Unknown numeric values and flag bits are preserved.
/// ```xml
/// <enum name="error">
///       <entry name="in_use" summary="a requested session is already in use" value="1" />
///       <entry name="invalid_session_id" summary="invalid session identifier" value="2" />
///       <entry name="invalid_reason" summary="invalid reason" value="3" />
///     </enum>
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct XdgSessionManagerV1Error(pub u32);
impl XdgSessionManagerV1Error {
    /// a requested session is already in use; available since version 1.
    pub const IN_USE: Self = Self(1);
    /// invalid session identifier; available since version 1.
    pub const INVALID_SESSION_ID: Self = Self(2);
    /// invalid reason; available since version 1.
    pub const INVALID_REASON: Self = Self(3);
}
impl CommandReader<Self> for XdgSessionManagerV1Error {
    async fn read(
        _object_id: u32,
        _opcode: u16,
        reader: &mut FdPayloadReader<'_>,
        _state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        Ok(Self(reader.next_u32().await?))
    }
}
impl CommandWriter for XdgSessionManagerV1Error {
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
