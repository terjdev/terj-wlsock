use crate::protocol::{ClientState, CommandReader, CommandWriter};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Wire value `wp_color_representation_surface_v1.range`.
///
/// Unknown numeric values and flag bits are preserved.
/// ```xml
/// <enum name="range">
///       <description summary="Color range values">
///         Possible color range values.
///
///         A value of 0 is invalid and will never be present in the list of enums.
///       </description>
///       <entry name="full" value="1" summary="Full color range" />
///       <entry name="limited" value="2" summary="Limited color range" />
///     </enum>
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct WpColorRepresentationSurfaceV1Range(pub u32);
impl WpColorRepresentationSurfaceV1Range {
    /// Full color range; available since version 1.
    pub const FULL: Self = Self(1);
    /// Limited color range; available since version 1.
    pub const LIMITED: Self = Self(2);
}
impl CommandReader<Self> for WpColorRepresentationSurfaceV1Range {
    async fn read(
        _object_id: u32,
        _opcode: u16,
        reader: &mut FdPayloadReader<'_>,
        _state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        Ok(Self(reader.next_u32().await?))
    }
}
impl CommandWriter for WpColorRepresentationSurfaceV1Range {
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
