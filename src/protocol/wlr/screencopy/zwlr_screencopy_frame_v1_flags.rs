use crate::protocol::{ClientState, CommandReader, CommandWriter};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Wire value `zwlr_screencopy_frame_v1.flags`.
///
/// Unknown numeric values and flag bits are preserved.
/// ```xml
/// <enum name="flags" bitfield="true">
///       <entry name="y_invert" value="1" summary="contents are y-inverted" />
///     </enum>
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct ZwlrScreencopyFrameV1Flags(pub u32);
impl ZwlrScreencopyFrameV1Flags {
    /// contents are y-inverted; available since version 1.
    pub const Y_INVERT: Self = Self(1);
}
impl std::ops::BitOr for ZwlrScreencopyFrameV1Flags {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl std::ops::BitAnd for ZwlrScreencopyFrameV1Flags {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl CommandReader<Self> for ZwlrScreencopyFrameV1Flags {
    async fn read(
        _object_id: u32,
        _opcode: u16,
        reader: &mut FdPayloadReader<'_>,
        _state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        Ok(Self(reader.next_u32().await?))
    }
}
impl CommandWriter for ZwlrScreencopyFrameV1Flags {
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
