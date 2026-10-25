use crate::protocol::{ClientState, CommandReader, CommandWriter};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Wire value `zwlr_output_configuration_head_v1.error`.
///
/// Unknown numeric values are preserved.
/// ```xml
/// <enum name="error">
///       <entry name="already_set" value="1" summary="property has already been set" />
///       <entry name="invalid_mode" value="2" summary="mode doesn't belong to head" />
///       <entry name="invalid_custom_mode" value="3" summary="mode is invalid" />
///       <entry name="invalid_transform" value="4" summary="transform value outside enum" />
///       <entry name="invalid_scale" value="5" summary="scale negative or zero" />
///       <entry name="invalid_adaptive_sync_state" value="6" since="4" summary="invalid enum value used in the set_adaptive_sync request" />
///     </enum>
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct ZwlrOutputConfigurationHeadV1Error(pub u32);
impl ZwlrOutputConfigurationHeadV1Error {
    /// property has already been set; available since version 1.
    pub const ALREADY_SET: Self = Self(1);
    /// mode doesn't belong to head; available since version 1.
    pub const INVALID_MODE: Self = Self(2);
    /// mode is invalid; available since version 1.
    pub const INVALID_CUSTOM_MODE: Self = Self(3);
    /// transform value outside enum; available since version 1.
    pub const INVALID_TRANSFORM: Self = Self(4);
    /// scale negative or zero; available since version 1.
    pub const INVALID_SCALE: Self = Self(5);
    /// invalid enum value used in the set_adaptive_sync request; available since version 4.
    pub const INVALID_ADAPTIVE_SYNC_STATE: Self = Self(6);
}
impl CommandReader<Self> for ZwlrOutputConfigurationHeadV1Error {
    async fn read(
        _object_id: u32,
        _opcode: u16,
        reader: &mut FdPayloadReader<'_>,
        _state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        Ok(Self(reader.next_u32().await?))
    }
}
impl CommandWriter for ZwlrOutputConfigurationHeadV1Error {
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
