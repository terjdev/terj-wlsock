use crate::protocol::{ClientState, CommandReader, CommandWriter};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Wire value `zwlr_foreign_toplevel_handle_v1.state`.
///
/// Unknown numeric values are preserved.
/// ```xml
/// <enum name="state">
///       <description summary="types of states on the toplevel">
///         The different states that a toplevel can have. These have the same meaning
///         as the states with the same names defined in xdg-toplevel
///       </description>
///
///       <entry name="maximized" value="0" summary="the toplevel is maximized" />
///       <entry name="minimized" value="1" summary="the toplevel is minimized" />
///       <entry name="activated" value="2" summary="the toplevel is active" />
///       <entry name="fullscreen" value="3" summary="the toplevel is fullscreen" since="2" />
///     </enum>
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct ZwlrForeignToplevelHandleV1State(pub u32);
impl ZwlrForeignToplevelHandleV1State {
    /// the toplevel is maximized; available since version 1.
    pub const MAXIMIZED: Self = Self(0);
    /// the toplevel is minimized; available since version 1.
    pub const MINIMIZED: Self = Self(1);
    /// the toplevel is active; available since version 1.
    pub const ACTIVATED: Self = Self(2);
    /// the toplevel is fullscreen; available since version 2.
    pub const FULLSCREEN: Self = Self(3);
}
impl CommandReader<Self> for ZwlrForeignToplevelHandleV1State {
    async fn read(
        _object_id: u32,
        _opcode: u16,
        reader: &mut FdPayloadReader<'_>,
        _state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        Ok(Self(reader.next_u32().await?))
    }
}
impl CommandWriter for ZwlrForeignToplevelHandleV1State {
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
