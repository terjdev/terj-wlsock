use crate::protocol::{ClientState, CommandReader, CommandWriter};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Wire value `wp_security_context_v1.error`.
///
/// Unknown numeric values and flag bits are preserved.
/// ```xml
/// <enum name="error">
///       <entry name="already_used" value="1" summary="security context has already been committed" />
///       <entry name="already_set" value="2" summary="metadata has already been set" />
///       <entry name="invalid_metadata" value="3" summary="metadata is invalid" />
///     </enum>
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct WpSecurityContextV1Error(pub u32);
impl WpSecurityContextV1Error {
    /// security context has already been committed; available since version 1.
    pub const ALREADY_USED: Self = Self(1);
    /// metadata has already been set; available since version 1.
    pub const ALREADY_SET: Self = Self(2);
    /// metadata is invalid; available since version 1.
    pub const INVALID_METADATA: Self = Self(3);
}
impl CommandReader<Self> for WpSecurityContextV1Error {
    async fn read(
        _object_id: u32,
        _opcode: u16,
        reader: &mut FdPayloadReader<'_>,
        _state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        Ok(Self(reader.next_u32().await?))
    }
}
impl CommandWriter for WpSecurityContextV1Error {
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
