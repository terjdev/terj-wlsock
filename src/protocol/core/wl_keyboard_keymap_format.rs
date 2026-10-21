use crate::protocol::ClientState;
use crate::protocol::{CommandReader, CommandWriter};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Core-protocol value `wl_keyboard.keymap_format`.
///
/// The numeric representation preserves unknown values without losing information.
/// Reading and writing encode four bytes; this is an argument value,
/// not a standalone message: object_id and opcode are both zero.
/// ```xml
/// <enum name="keymap_format">
///       <description summary="keyboard mapping format">
///         This specifies the format of the keymap provided to the
///         client with the wl_keyboard.keymap event.
///       </description>
///       <entry name="no_keymap" value="0" summary="no keymap; client must understand how to interpret the raw keycode" />
///       <entry name="xkb_v1" value="1" summary="libxkbcommon compatible, null-terminated string; to determine the xkb keycode, clients must add 8 to the key event keycode" />
///     </enum>
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct WlKeyboardKeymapFormat(pub u32);
impl WlKeyboardKeymapFormat {
    /// no keymap; client must understand how to interpret the raw keycode; XML since=1.
    /// ```xml
    /// <entry name="no_keymap" value="0" summary="no keymap; client must understand how to interpret the raw keycode" />
    /// ```
    pub const NO_KEYMAP: Self = Self(0);
    /// libxkbcommon compatible, null-terminated string; to determine the xkb keycode, clients must add 8 to the key event keycode; XML since=1.
    /// ```xml
    /// <entry name="xkb_v1" value="1" summary="libxkbcommon compatible, null-terminated string; to determine the xkb keycode, clients must add 8 to the key event keycode" />
    /// ```
    pub const XKB_V1: Self = Self(1);
}
impl CommandReader<Self> for WlKeyboardKeymapFormat {
    async fn read(
        _object_id: u32,
        _opcode: u16,
        reader: &mut FdPayloadReader<'_>,
        _state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        Ok(Self(reader.next_u32().await?))
    }
}
impl CommandWriter for WlKeyboardKeymapFormat {
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
