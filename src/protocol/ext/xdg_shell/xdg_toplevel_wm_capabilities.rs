use crate::protocol::{ClientState, CommandReader, CommandWriter};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Wire value `xdg_toplevel.wm_capabilities`.
///
/// Unknown numeric values and flag bits are preserved.
/// ```xml
/// <enum name="wm_capabilities" since="5">
///       <entry name="window_menu" value="1" summary="show_window_menu is available" />
///       <entry name="maximize" value="2" summary="set_maximized and unset_maximized are available" />
///       <entry name="fullscreen" value="3" summary="set_fullscreen and unset_fullscreen are available" />
///       <entry name="minimize" value="4" summary="set_minimized is available" />
///     </enum>
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct XdgToplevelWmCapabilities(pub u32);
impl XdgToplevelWmCapabilities {
    /// show_window_menu is available; available since version 5.
    pub const WINDOW_MENU: Self = Self(1);
    /// set_maximized and unset_maximized are available; available since version 5.
    pub const MAXIMIZE: Self = Self(2);
    /// set_fullscreen and unset_fullscreen are available; available since version 5.
    pub const FULLSCREEN: Self = Self(3);
    /// set_minimized is available; available since version 5.
    pub const MINIMIZE: Self = Self(4);
}
impl CommandReader<Self> for XdgToplevelWmCapabilities {
    async fn read(
        _object_id: u32,
        _opcode: u16,
        reader: &mut FdPayloadReader<'_>,
        _state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        Ok(Self(reader.next_u32().await?))
    }
}
impl CommandWriter for XdgToplevelWmCapabilities {
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
