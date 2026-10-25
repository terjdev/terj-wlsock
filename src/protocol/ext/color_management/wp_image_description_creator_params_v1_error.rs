use crate::protocol::{ClientState, CommandReader, CommandWriter};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Wire value `wp_image_description_creator_params_v1.error`.
///
/// Unknown numeric values and flag bits are preserved.
/// ```xml
/// <enum name="error">
///       <description summary="protocol errors" />
///
///       <entry name="incomplete_set" value="0" summary="incomplete parameter set" />
///       <entry name="already_set" value="1" summary="property already set" />
///       <entry name="unsupported_feature" value="2" summary="request not supported" />
///       <entry name="invalid_tf" value="3" summary="invalid transfer characteristic" />
///       <entry name="invalid_primaries_named" value="4" summary="invalid primaries named" />
///       <entry name="invalid_luminance" value="5" summary="invalid luminance value or range" />
///     </enum>
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct WpImageDescriptionCreatorParamsV1Error(pub u32);
impl WpImageDescriptionCreatorParamsV1Error {
    /// incomplete parameter set; available since version 1.
    pub const INCOMPLETE_SET: Self = Self(0);
    /// property already set; available since version 1.
    pub const ALREADY_SET: Self = Self(1);
    /// request not supported; available since version 1.
    pub const UNSUPPORTED_FEATURE: Self = Self(2);
    /// invalid transfer characteristic; available since version 1.
    pub const INVALID_TF: Self = Self(3);
    /// invalid primaries named; available since version 1.
    pub const INVALID_PRIMARIES_NAMED: Self = Self(4);
    /// invalid luminance value or range; available since version 1.
    pub const INVALID_LUMINANCE: Self = Self(5);
}
impl CommandReader<Self> for WpImageDescriptionCreatorParamsV1Error {
    async fn read(
        _object_id: u32,
        _opcode: u16,
        reader: &mut FdPayloadReader<'_>,
        _state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        Ok(Self(reader.next_u32().await?))
    }
}
impl CommandWriter for WpImageDescriptionCreatorParamsV1Error {
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
