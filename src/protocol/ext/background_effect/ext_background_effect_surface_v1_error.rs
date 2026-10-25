use crate::protocol::{ClientState, CommandReader, CommandWriter};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Wire value `ext_background_effect_surface_v1.error`.
///
/// Unknown numeric values and flag bits are preserved.
/// ```xml
/// <enum name="error">
///       <entry name="surface_destroyed" value="0" summary="the associated surface has been destroyed" />
///     </enum>
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct ExtBackgroundEffectSurfaceV1Error(pub u32);
impl ExtBackgroundEffectSurfaceV1Error {
    /// the associated surface has been destroyed; available since version 1.
    pub const SURFACE_DESTROYED: Self = Self(0);
}
impl CommandReader<Self> for ExtBackgroundEffectSurfaceV1Error {
    async fn read(
        _object_id: u32,
        _opcode: u16,
        reader: &mut FdPayloadReader<'_>,
        _state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        Ok(Self(reader.next_u32().await?))
    }
}
impl CommandWriter for ExtBackgroundEffectSurfaceV1Error {
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
