use crate::protocol::{ClientState, CommandReader, CommandWriter};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Wire value `xx_image_capture_color_manager_v1.error`.
///
/// Unknown numeric values are preserved.
/// ```xml
/// <enum name="error">
///       <entry name="image_description_not_ready" value="1" summary="the image description is not ready" />
///       <entry name="already_captured" value="2" summary="capture request has been sent" />
///     </enum>
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct XxImageCaptureColorManagerV1Error(pub u32);
impl XxImageCaptureColorManagerV1Error {
    /// the image description is not ready; available since version 1.
    pub const IMAGE_DESCRIPTION_NOT_READY: Self = Self(1);
    /// capture request has been sent; available since version 1.
    pub const ALREADY_CAPTURED: Self = Self(2);
}
impl CommandReader<Self> for XxImageCaptureColorManagerV1Error {
    async fn read(
        _object_id: u32,
        _opcode: u16,
        reader: &mut FdPayloadReader<'_>,
        _state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        Ok(Self(reader.next_u32().await?))
    }
}
impl CommandWriter for XxImageCaptureColorManagerV1Error {
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
