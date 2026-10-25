use crate::protocol::ClientState;
use crate::protocol::{CommandReader, CommandWriter};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Presentation-time value `wp_presentation.error`.
///
/// Unknown numeric values are preserved. This value encodes four bytes.
/// ```xml
/// <enum name="error">
///       <description summary="fatal presentation errors">
///         These fatal protocol errors may be emitted in response to
///         illegal presentation requests.
///       </description>
///       <entry name="invalid_timestamp" value="0" summary="invalid value in tv_nsec" />
///       <entry name="invalid_flag" value="1" summary="invalid flag" />
///     </enum>
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct WpPresentationError(pub u32);
impl WpPresentationError {
    /// invalid value in tv_nsec; XML since=1.
    pub const INVALID_TIMESTAMP: Self = Self(0);
    /// invalid flag; XML since=1.
    pub const INVALID_FLAG: Self = Self(1);
}
impl CommandReader<Self> for WpPresentationError {
    async fn read(
        _object_id: u32,
        _opcode: u16,
        reader: &mut FdPayloadReader<'_>,
        _state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        Ok(Self(reader.next_u32().await?))
    }
}
impl CommandWriter for WpPresentationError {
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
