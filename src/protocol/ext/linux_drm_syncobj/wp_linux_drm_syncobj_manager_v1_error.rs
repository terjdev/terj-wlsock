use crate::protocol::{ClientState, CommandReader, CommandWriter};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Wire value `wp_linux_drm_syncobj_manager_v1.error`.
///
/// Unknown numeric values and flag bits are preserved.
/// ```xml
/// <enum name="error">
///       <entry name="surface_exists" value="0" summary="the surface already has a synchronization object associated" />
///       <entry name="invalid_timeline" value="1" summary="the timeline object could not be imported" />
///     </enum>
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct WpLinuxDrmSyncobjManagerV1Error(pub u32);
impl WpLinuxDrmSyncobjManagerV1Error {
    /// the surface already has a synchronization object associated; available since version 1.
    pub const SURFACE_EXISTS: Self = Self(0);
    /// the timeline object could not be imported; available since version 1.
    pub const INVALID_TIMELINE: Self = Self(1);
}
impl CommandReader<Self> for WpLinuxDrmSyncobjManagerV1Error {
    async fn read(
        _object_id: u32,
        _opcode: u16,
        reader: &mut FdPayloadReader<'_>,
        _state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        Ok(Self(reader.next_u32().await?))
    }
}
impl CommandWriter for WpLinuxDrmSyncobjManagerV1Error {
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
