use crate::protocol::{ClientState, CommandReader, CommandWriter};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Wire value `zwlr_layer_surface_v1.anchor`.
///
/// Unknown numeric values and flag bits are preserved.
/// ```xml
/// <enum name="anchor" bitfield="true">
///       <entry name="top" value="1" summary="the top edge of the anchor rectangle" />
///       <entry name="bottom" value="2" summary="the bottom edge of the anchor rectangle" />
///       <entry name="left" value="4" summary="the left edge of the anchor rectangle" />
///       <entry name="right" value="8" summary="the right edge of the anchor rectangle" />
///     </enum>
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct ZwlrLayerSurfaceV1Anchor(pub u32);
impl ZwlrLayerSurfaceV1Anchor {
    /// the top edge of the anchor rectangle; available since version 1.
    pub const TOP: Self = Self(1);
    /// the bottom edge of the anchor rectangle; available since version 1.
    pub const BOTTOM: Self = Self(2);
    /// the left edge of the anchor rectangle; available since version 1.
    pub const LEFT: Self = Self(4);
    /// the right edge of the anchor rectangle; available since version 1.
    pub const RIGHT: Self = Self(8);
}
impl std::ops::BitOr for ZwlrLayerSurfaceV1Anchor {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl std::ops::BitAnd for ZwlrLayerSurfaceV1Anchor {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl CommandReader<Self> for ZwlrLayerSurfaceV1Anchor {
    async fn read(
        _object_id: u32,
        _opcode: u16,
        reader: &mut FdPayloadReader<'_>,
        _state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        Ok(Self(reader.next_u32().await?))
    }
}
impl CommandWriter for ZwlrLayerSurfaceV1Anchor {
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
