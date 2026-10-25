use crate::protocol::{ClientState, CommandReader, CommandWriter};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Wire value `zwp_linux_surface_synchronization_v1.error`.
///
/// Unknown numeric values are preserved.
/// ```xml
/// <enum name="error">
///       <entry name="invalid_fence" value="0" summary="the fence specified by the client could not be imported" />
///       <entry name="duplicate_fence" value="1" summary="multiple fences added for a single surface commit" />
///       <entry name="duplicate_release" value="2" summary="multiple releases added for a single surface commit" />
///       <entry name="no_surface" value="3" summary="the associated wl_surface was destroyed" />
///       <entry name="unsupported_buffer" value="4" summary="the buffer does not support explicit synchronization" />
///       <entry name="no_buffer" value="5" summary="no buffer was attached" />
///     </enum>
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct ZwpLinuxSurfaceSynchronizationV1Error(pub u32);
impl ZwpLinuxSurfaceSynchronizationV1Error {
    /// the fence specified by the client could not be imported; available since version 1.
    pub const INVALID_FENCE: Self = Self(0);
    /// multiple fences added for a single surface commit; available since version 1.
    pub const DUPLICATE_FENCE: Self = Self(1);
    /// multiple releases added for a single surface commit; available since version 1.
    pub const DUPLICATE_RELEASE: Self = Self(2);
    /// the associated wl_surface was destroyed; available since version 1.
    pub const NO_SURFACE: Self = Self(3);
    /// the buffer does not support explicit synchronization; available since version 1.
    pub const UNSUPPORTED_BUFFER: Self = Self(4);
    /// no buffer was attached; available since version 1.
    pub const NO_BUFFER: Self = Self(5);
}
impl CommandReader<Self> for ZwpLinuxSurfaceSynchronizationV1Error {
    async fn read(
        _object_id: u32,
        _opcode: u16,
        reader: &mut FdPayloadReader<'_>,
        _state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        Ok(Self(reader.next_u32().await?))
    }
}
impl CommandWriter for ZwpLinuxSurfaceSynchronizationV1Error {
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
