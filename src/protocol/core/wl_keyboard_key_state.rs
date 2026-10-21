use crate::protocol::ClientState;
use crate::protocol::{CommandReader, CommandWriter};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Core-protocol value `wl_keyboard.key_state`.
///
/// The numeric representation preserves unknown values without losing information.
/// Reading and writing encode four bytes; this is an argument value,
/// not a standalone message: object_id and opcode are both zero.
/// ```xml
/// <enum name="key_state">
///       <description summary="physical key state">
///         Describes the physical state of a key that produced the key event.
///
///         Since version 10, the key can be in a "repeated" pseudo-state which
///         means the same as "pressed", but is used to signal repetition in the
///         key event.
///
///         The key may only enter the repeated state after entering the pressed
///         state and before entering the released state. This event may be
///         generated multiple times while the key is down.
///       </description>
///       <entry name="released" value="0" summary="key is not pressed" />
///       <entry name="pressed" value="1" summary="key is pressed" />
///       <entry name="repeated" value="2" summary="key was repeated" since="10" />
///     </enum>
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct WlKeyboardKeyState(pub u32);
impl WlKeyboardKeyState {
    /// key is not pressed; XML since=1.
    /// ```xml
    /// <entry name="released" value="0" summary="key is not pressed" />
    /// ```
    pub const RELEASED: Self = Self(0);
    /// key is pressed; XML since=1.
    /// ```xml
    /// <entry name="pressed" value="1" summary="key is pressed" />
    /// ```
    pub const PRESSED: Self = Self(1);
    /// key was repeated; XML since=10.
    /// ```xml
    /// <entry name="repeated" value="2" summary="key was repeated" since="10" />
    /// ```
    pub const REPEATED: Self = Self(2);
}
impl CommandReader<Self> for WlKeyboardKeyState {
    async fn read(
        _object_id: u32,
        _opcode: u16,
        reader: &mut FdPayloadReader<'_>,
        _state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        Ok(Self(reader.next_u32().await?))
    }
}
impl CommandWriter for WlKeyboardKeyState {
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
