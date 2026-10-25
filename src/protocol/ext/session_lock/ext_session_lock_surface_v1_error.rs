use crate::protocol::{ClientState, CommandReader, CommandWriter};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Wire value `ext_session_lock_surface_v1.error`.
///
/// Unknown numeric values and flag bits are preserved.
/// ```xml
/// <enum name="error">
///       <entry name="commit_before_first_ack" value="0" summary="surface committed before first ack_configure request" />
///       <entry name="null_buffer" value="1" summary="surface committed with a null buffer" />
///       <entry name="dimensions_mismatch" value="2" summary="failed to match ack'd width/height" />
///       <entry name="invalid_serial" value="3" summary="serial provided in ack_configure is invalid" />
///     </enum>
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct ExtSessionLockSurfaceV1Error(pub u32);
impl ExtSessionLockSurfaceV1Error {
    /// surface committed before first ack_configure request; available since version 1.
    pub const COMMIT_BEFORE_FIRST_ACK: Self = Self(0);
    /// surface committed with a null buffer; available since version 1.
    pub const NULL_BUFFER: Self = Self(1);
    /// failed to match ack'd width/height; available since version 1.
    pub const DIMENSIONS_MISMATCH: Self = Self(2);
    /// serial provided in ack_configure is invalid; available since version 1.
    pub const INVALID_SERIAL: Self = Self(3);
}
impl CommandReader<Self> for ExtSessionLockSurfaceV1Error {
    async fn read(
        _object_id: u32,
        _opcode: u16,
        reader: &mut FdPayloadReader<'_>,
        _state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        Ok(Self(reader.next_u32().await?))
    }
}
impl CommandWriter for ExtSessionLockSurfaceV1Error {
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
