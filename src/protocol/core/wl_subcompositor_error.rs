use crate::protocol::ClientState;
use crate::protocol::{CommandReader, CommandWriter};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Core-protocol value `wl_subcompositor.error`.
///
/// The numeric representation preserves unknown values without losing information.
/// Reading and writing encode four bytes; this is an argument value,
/// not a standalone message: object_id and opcode are both zero.
/// ```xml
/// <enum name="error">
///       <entry name="bad_surface" value="0" summary="the to-be sub-surface is invalid" />
///       <entry name="bad_parent" value="1" summary="the to-be sub-surface parent is invalid" />
///     </enum>
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct WlSubcompositorError(pub u32);
impl WlSubcompositorError {
    /// the to-be sub-surface is invalid; XML since=1.
    /// ```xml
    /// <entry name="bad_surface" value="0" summary="the to-be sub-surface is invalid" />
    /// ```
    pub const BAD_SURFACE: Self = Self(0);
    /// the to-be sub-surface parent is invalid; XML since=1.
    /// ```xml
    /// <entry name="bad_parent" value="1" summary="the to-be sub-surface parent is invalid" />
    /// ```
    pub const BAD_PARENT: Self = Self(1);
}
impl CommandReader<Self> for WlSubcompositorError {
    async fn read(
        _object_id: u32,
        _opcode: u16,
        reader: &mut FdPayloadReader<'_>,
        _state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        Ok(Self(reader.next_u32().await?))
    }
}
impl CommandWriter for WlSubcompositorError {
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
