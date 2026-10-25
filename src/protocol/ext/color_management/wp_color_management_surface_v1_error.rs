use crate::protocol::{ClientState, CommandReader, CommandWriter};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Wire value `wp_color_management_surface_v1.error`.
///
/// Unknown numeric values and flag bits are preserved.
/// ```xml
/// <enum name="error">
///       <description summary="protocol errors" />
///       <entry name="render_intent" value="0" summary="unsupported rendering intent" />
///       <entry name="image_description" value="1" summary="invalid image description" />
///       <entry name="inert" value="2" summary="forbidden request on inert object" />
///     </enum>
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct WpColorManagementSurfaceV1Error(pub u32);
impl WpColorManagementSurfaceV1Error {
    /// unsupported rendering intent; available since version 1.
    pub const RENDER_INTENT: Self = Self(0);
    /// invalid image description; available since version 1.
    pub const IMAGE_DESCRIPTION: Self = Self(1);
    /// forbidden request on inert object; available since version 1.
    pub const INERT: Self = Self(2);
}
impl CommandReader<Self> for WpColorManagementSurfaceV1Error {
    async fn read(
        _object_id: u32,
        _opcode: u16,
        reader: &mut FdPayloadReader<'_>,
        _state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        Ok(Self(reader.next_u32().await?))
    }
}
impl CommandWriter for WpColorManagementSurfaceV1Error {
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
