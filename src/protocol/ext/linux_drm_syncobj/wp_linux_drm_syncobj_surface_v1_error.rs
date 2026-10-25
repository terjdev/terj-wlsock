use crate::protocol::{ClientState, CommandReader, CommandWriter};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Wire value `wp_linux_drm_syncobj_surface_v1.error`.
///
/// Unknown numeric values and flag bits are preserved.
/// ```xml
/// <enum name="error">
///       <entry name="no_surface" value="1" summary="the associated wl_surface was destroyed" />
///       <entry name="unsupported_buffer" value="2" summary="the buffer does not support explicit synchronization" />
///       <entry name="no_buffer" value="3" summary="no buffer was attached" />
///       <entry name="no_acquire_point" value="4" summary="no acquire timeline point was set" />
///       <entry name="no_release_point" value="5" summary="no release timeline point was set" />
///       <entry name="conflicting_points" value="6" summary="acquire and release timeline points are in conflict" />
///     </enum>
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct WpLinuxDrmSyncobjSurfaceV1Error(pub u32);
impl WpLinuxDrmSyncobjSurfaceV1Error {
    /// the associated wl_surface was destroyed; available since version 1.
    pub const NO_SURFACE: Self = Self(1);
    /// the buffer does not support explicit synchronization; available since version 1.
    pub const UNSUPPORTED_BUFFER: Self = Self(2);
    /// no buffer was attached; available since version 1.
    pub const NO_BUFFER: Self = Self(3);
    /// no acquire timeline point was set; available since version 1.
    pub const NO_ACQUIRE_POINT: Self = Self(4);
    /// no release timeline point was set; available since version 1.
    pub const NO_RELEASE_POINT: Self = Self(5);
    /// acquire and release timeline points are in conflict; available since version 1.
    pub const CONFLICTING_POINTS: Self = Self(6);
}
impl CommandReader<Self> for WpLinuxDrmSyncobjSurfaceV1Error {
    async fn read(
        _object_id: u32,
        _opcode: u16,
        reader: &mut FdPayloadReader<'_>,
        _state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        Ok(Self(reader.next_u32().await?))
    }
}
impl CommandWriter for WpLinuxDrmSyncobjSurfaceV1Error {
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
