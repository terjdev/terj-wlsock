use crate::protocol::{ClientState, CommandReader, CommandWriter};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Wire value `zxdg_imported_v2.error`.
///
/// Unknown numeric values are preserved.
/// ```xml
/// <enum name="error">
///       <description summary="error values">
///         These errors can be emitted in response to invalid xdg_imported
///         requests.
///       </description>
///       <entry name="invalid_surface" value="0" summary="surface is not an xdg_toplevel" />
///     </enum>
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct ZxdgImportedV2Error(pub u32);
impl ZxdgImportedV2Error {
    /// surface is not an xdg_toplevel; available since version 1.
    pub const INVALID_SURFACE: Self = Self(0);
}
impl CommandReader<Self> for ZxdgImportedV2Error {
    async fn read(
        _object_id: u32,
        _opcode: u16,
        reader: &mut FdPayloadReader<'_>,
        _state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        Ok(Self(reader.next_u32().await?))
    }
}
impl CommandWriter for ZxdgImportedV2Error {
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
