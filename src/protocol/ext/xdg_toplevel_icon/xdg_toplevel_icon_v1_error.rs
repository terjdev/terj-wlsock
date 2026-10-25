use crate::protocol::{ClientState, CommandReader, CommandWriter};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Wire value `xdg_toplevel_icon_v1.error`.
///
/// Unknown numeric values and flag bits are preserved.
/// ```xml
/// <enum name="error">
///       <entry name="invalid_buffer" summary="the provided buffer does not satisfy requirements" value="1" />
///       <entry name="immutable" summary="the icon has already been assigned to a toplevel and must not be changed" value="2" />
///       <entry name="no_buffer" summary="the provided buffer has been destroyed before the toplevel icon" value="3" />
///     </enum>
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct XdgToplevelIconV1Error(pub u32);
impl XdgToplevelIconV1Error {
    /// the provided buffer does not satisfy requirements; available since version 1.
    pub const INVALID_BUFFER: Self = Self(1);
    /// the icon has already been assigned to a toplevel and must not be changed; available since version 1.
    pub const IMMUTABLE: Self = Self(2);
    /// the provided buffer has been destroyed before the toplevel icon; available since version 1.
    pub const NO_BUFFER: Self = Self(3);
}
impl CommandReader<Self> for XdgToplevelIconV1Error {
    async fn read(
        _object_id: u32,
        _opcode: u16,
        reader: &mut FdPayloadReader<'_>,
        _state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        Ok(Self(reader.next_u32().await?))
    }
}
impl CommandWriter for XdgToplevelIconV1Error {
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
