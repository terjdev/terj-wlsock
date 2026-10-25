use crate::protocol::{ClientState, CommandReader, CommandWriter};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Wire value `wp_color_management_surface_feedback_v1.error`.
///
/// Unknown numeric values and flag bits are preserved.
/// ```xml
/// <enum name="error">
///       <description summary="protocol errors" />
///       <entry name="inert" value="0" summary="forbidden request on inert object" />
///       <entry name="unsupported_feature" value="1" summary="attempted to use an unsupported feature" />
///     </enum>
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct WpColorManagementSurfaceFeedbackV1Error(pub u32);
impl WpColorManagementSurfaceFeedbackV1Error {
    /// forbidden request on inert object; available since version 1.
    pub const INERT: Self = Self(0);
    /// attempted to use an unsupported feature; available since version 1.
    pub const UNSUPPORTED_FEATURE: Self = Self(1);
}
impl CommandReader<Self> for WpColorManagementSurfaceFeedbackV1Error {
    async fn read(
        _object_id: u32,
        _opcode: u16,
        reader: &mut FdPayloadReader<'_>,
        _state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        Ok(Self(reader.next_u32().await?))
    }
}
impl CommandWriter for WpColorManagementSurfaceFeedbackV1Error {
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
