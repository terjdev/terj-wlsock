use crate::protocol::{ClientState, CommandReader, CommandWriter};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Wire value `zwlr_output_configuration_v1.error`.
///
/// Unknown numeric values are preserved.
/// ```xml
/// <enum name="error">
///       <entry name="already_configured_head" value="1" summary="head has been configured twice" />
///       <entry name="unconfigured_head" value="2" summary="head has not been configured" />
///       <entry name="already_used" value="3" summary="request sent after configuration has been applied or tested" />
///     </enum>
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct ZwlrOutputConfigurationV1Error(pub u32);
impl ZwlrOutputConfigurationV1Error {
    /// head has been configured twice; available since version 1.
    pub const ALREADY_CONFIGURED_HEAD: Self = Self(1);
    /// head has not been configured; available since version 1.
    pub const UNCONFIGURED_HEAD: Self = Self(2);
    /// request sent after configuration has been applied or tested; available since version 1.
    pub const ALREADY_USED: Self = Self(3);
}
impl CommandReader<Self> for ZwlrOutputConfigurationV1Error {
    async fn read(
        _object_id: u32,
        _opcode: u16,
        reader: &mut FdPayloadReader<'_>,
        _state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        Ok(Self(reader.next_u32().await?))
    }
}
impl CommandWriter for ZwlrOutputConfigurationV1Error {
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
