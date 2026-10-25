use crate::protocol::{ClientState, CommandReader, CommandWriter};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Wire value `xx_keyboard_filter_v1.filter_action`.
///
/// Unknown numeric values are preserved.
/// ```xml
/// <enum name="filter_action">
///       <entry name="consume" value="0" summary="consume the key event" />
///       <entry name="passthrough" value="1" summary="pass the key event to the text input client" />
///     </enum>
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct XxKeyboardFilterV1FilterAction(pub u32);
impl XxKeyboardFilterV1FilterAction {
    /// consume the key event; available since version 1.
    pub const CONSUME: Self = Self(0);
    /// pass the key event to the text input client; available since version 1.
    pub const PASSTHROUGH: Self = Self(1);
}
impl CommandReader<Self> for XxKeyboardFilterV1FilterAction {
    async fn read(
        _object_id: u32,
        _opcode: u16,
        reader: &mut FdPayloadReader<'_>,
        _state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        Ok(Self(reader.next_u32().await?))
    }
}
impl CommandWriter for XxKeyboardFilterV1FilterAction {
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
