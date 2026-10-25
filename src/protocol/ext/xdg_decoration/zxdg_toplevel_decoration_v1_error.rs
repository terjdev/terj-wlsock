use crate::protocol::{ClientState, CommandReader, CommandWriter};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Wire value `zxdg_toplevel_decoration_v1.error`.
///
/// Unknown numeric values are preserved.
/// ```xml
/// <enum name="error">
///       <entry name="unconfigured_buffer" value="0" summary="xdg_toplevel has a buffer attached before configure" />
///       <entry name="already_constructed" value="1" summary="xdg_toplevel already has a decoration object" />
///       <entry name="orphaned" value="2" summary="xdg_toplevel destroyed before the decoration object" />
///       <entry name="invalid_mode" value="3" summary="invalid mode" />
///     </enum>
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct ZxdgToplevelDecorationV1Error(pub u32);
impl ZxdgToplevelDecorationV1Error {
    /// xdg_toplevel has a buffer attached before configure; available since version 1.
    pub const UNCONFIGURED_BUFFER: Self = Self(0);
    /// xdg_toplevel already has a decoration object; available since version 1.
    pub const ALREADY_CONSTRUCTED: Self = Self(1);
    /// xdg_toplevel destroyed before the decoration object; available since version 1.
    pub const ORPHANED: Self = Self(2);
    /// invalid mode; available since version 1.
    pub const INVALID_MODE: Self = Self(3);
}
impl CommandReader<Self> for ZxdgToplevelDecorationV1Error {
    async fn read(
        _object_id: u32,
        _opcode: u16,
        reader: &mut FdPayloadReader<'_>,
        _state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        Ok(Self(reader.next_u32().await?))
    }
}
impl CommandWriter for ZxdgToplevelDecorationV1Error {
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
