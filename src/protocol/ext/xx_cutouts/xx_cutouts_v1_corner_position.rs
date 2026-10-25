use crate::protocol::{ClientState, CommandReader, CommandWriter};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Wire value `xx_cutouts_v1.corner_position`.
///
/// Unknown numeric values are preserved.
/// ```xml
/// <enum name="corner_position">
///       <description summary="Corner position">
///         The position of a corner on a surface
///       </description>
///
///       <entry name="top_left" value="0" />
///       <entry name="top_right" value="1" />
///       <entry name="bottom_right" value="2" />
///       <entry name="bottom_left" value="3" />
///     </enum>
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct XxCutoutsV1CornerPosition(pub u32);
impl XxCutoutsV1CornerPosition {
    /// top left; available since version 1.
    pub const TOP_LEFT: Self = Self(0);
    /// top right; available since version 1.
    pub const TOP_RIGHT: Self = Self(1);
    /// bottom right; available since version 1.
    pub const BOTTOM_RIGHT: Self = Self(2);
    /// bottom left; available since version 1.
    pub const BOTTOM_LEFT: Self = Self(3);
}
impl CommandReader<Self> for XxCutoutsV1CornerPosition {
    async fn read(
        _object_id: u32,
        _opcode: u16,
        reader: &mut FdPayloadReader<'_>,
        _state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        Ok(Self(reader.next_u32().await?))
    }
}
impl CommandWriter for XxCutoutsV1CornerPosition {
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
