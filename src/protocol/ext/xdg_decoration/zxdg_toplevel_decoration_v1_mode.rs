use crate::protocol::{ClientState, CommandReader, CommandWriter};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Wire value `zxdg_toplevel_decoration_v1.mode`.
///
/// Unknown numeric values are preserved.
/// ```xml
/// <enum name="mode">
///       <description summary="window decoration modes">
///         These values describe window decoration modes.
///       </description>
///       <entry name="client_side" value="1" summary="no server-side window decoration" />
///       <entry name="server_side" value="2" summary="server-side window decoration" />
///     </enum>
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct ZxdgToplevelDecorationV1Mode(pub u32);
impl ZxdgToplevelDecorationV1Mode {
    /// no server-side window decoration; available since version 1.
    pub const CLIENT_SIDE: Self = Self(1);
    /// server-side window decoration; available since version 1.
    pub const SERVER_SIDE: Self = Self(2);
}
impl CommandReader<Self> for ZxdgToplevelDecorationV1Mode {
    async fn read(
        _object_id: u32,
        _opcode: u16,
        reader: &mut FdPayloadReader<'_>,
        _state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        Ok(Self(reader.next_u32().await?))
    }
}
impl CommandWriter for ZxdgToplevelDecorationV1Mode {
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
