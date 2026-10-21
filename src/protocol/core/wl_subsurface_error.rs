use crate::protocol::ClientState;
use crate::protocol::{CommandReader, CommandWriter};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Core-protocol value `wl_subsurface.error`.
///
/// The numeric representation preserves unknown values without losing information.
/// Reading and writing encode four bytes; this is an argument value,
/// not a standalone message: object_id and opcode are both zero.
/// ```xml
/// <enum name="error">
///       <entry name="bad_surface" value="0" summary="wl_surface is not a sibling or the parent" />
///     </enum>
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct WlSubsurfaceError(pub u32);
impl WlSubsurfaceError {
    /// wl_surface is not a sibling or the parent; XML since=1.
    /// ```xml
    /// <entry name="bad_surface" value="0" summary="wl_surface is not a sibling or the parent" />
    /// ```
    pub const BAD_SURFACE: Self = Self(0);
}
impl CommandReader<Self> for WlSubsurfaceError {
    async fn read(
        _object_id: u32,
        _opcode: u16,
        reader: &mut FdPayloadReader<'_>,
        _state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        Ok(Self(reader.next_u32().await?))
    }
}
impl CommandWriter for WlSubsurfaceError {
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
