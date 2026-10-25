use crate::protocol::{ClientState, CommandReader, CommandWriter};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Wire value `zwlr_foreign_toplevel_handle_v1.error`.
///
/// Unknown numeric values are preserved.
/// ```xml
/// <enum name="error">
///       <entry name="invalid_rectangle" value="0" summary="the provided rectangle is invalid" />
///     </enum>
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct ZwlrForeignToplevelHandleV1Error(pub u32);
impl ZwlrForeignToplevelHandleV1Error {
    /// the provided rectangle is invalid; available since version 1.
    pub const INVALID_RECTANGLE: Self = Self(0);
}
impl CommandReader<Self> for ZwlrForeignToplevelHandleV1Error {
    async fn read(
        _object_id: u32,
        _opcode: u16,
        reader: &mut FdPayloadReader<'_>,
        _state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        Ok(Self(reader.next_u32().await?))
    }
}
impl CommandWriter for ZwlrForeignToplevelHandleV1Error {
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
