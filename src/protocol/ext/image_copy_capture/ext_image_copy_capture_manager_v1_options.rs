use crate::protocol::{ClientState, CommandReader, CommandWriter};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Wire value `ext_image_copy_capture_manager_v1.options`.
///
/// Unknown numeric values and flag bits are preserved.
/// ```xml
/// <enum name="options" bitfield="true">
///       <entry name="paint_cursors" value="1" summary="paint cursors onto captured frames" />
///     </enum>
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct ExtImageCopyCaptureManagerV1Options(pub u32);
impl ExtImageCopyCaptureManagerV1Options {
    /// paint cursors onto captured frames; available since version 1.
    pub const PAINT_CURSORS: Self = Self(1);
}
impl std::ops::BitOr for ExtImageCopyCaptureManagerV1Options {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl std::ops::BitAnd for ExtImageCopyCaptureManagerV1Options {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl CommandReader<Self> for ExtImageCopyCaptureManagerV1Options {
    async fn read(
        _object_id: u32,
        _opcode: u16,
        reader: &mut FdPayloadReader<'_>,
        _state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        Ok(Self(reader.next_u32().await?))
    }
}
impl CommandWriter for ExtImageCopyCaptureManagerV1Options {
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
