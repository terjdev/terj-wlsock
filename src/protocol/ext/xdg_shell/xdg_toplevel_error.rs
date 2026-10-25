use crate::protocol::{ClientState, CommandReader, CommandWriter};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Wire value `xdg_toplevel.error`.
///
/// Unknown numeric values and flag bits are preserved.
/// ```xml
/// <enum name="error">
///       <entry name="invalid_resize_edge" value="0" summary="provided value is         not a valid variant of the resize_edge enum" />
///       <entry name="invalid_parent" value="1" summary="invalid parent toplevel" />
///       <entry name="invalid_size" value="2" summary="client provided an invalid min or max size" />
///     </enum>
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct XdgToplevelError(pub u32);
impl XdgToplevelError {
    /// provided value is not a valid variant of the resize_edge enum; available since version 1.
    pub const INVALID_RESIZE_EDGE: Self = Self(0);
    /// invalid parent toplevel; available since version 1.
    pub const INVALID_PARENT: Self = Self(1);
    /// client provided an invalid min or max size; available since version 1.
    pub const INVALID_SIZE: Self = Self(2);
}
impl CommandReader<Self> for XdgToplevelError {
    async fn read(
        _object_id: u32,
        _opcode: u16,
        reader: &mut FdPayloadReader<'_>,
        _state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        Ok(Self(reader.next_u32().await?))
    }
}
impl CommandWriter for XdgToplevelError {
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
