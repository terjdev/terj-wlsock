use crate::protocol::{ClientState, CommandReader, CommandWriter};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Wire value `zwp_text_input_v3.change_cause`.
///
/// Unknown numeric values are preserved.
/// ```xml
/// <enum name="change_cause">
///       <description summary="text change reason">
///         Reason for the change of surrounding text or cursor position.
///       </description>
///       <entry name="input_method" value="0" summary="input method caused the change" />
///       <entry name="other" value="1" summary="something else than the input method caused the change" />
///     </enum>
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct ZwpTextInputV3ChangeCause(pub u32);
impl ZwpTextInputV3ChangeCause {
    /// input method caused the change; available since version 1.
    pub const INPUT_METHOD: Self = Self(0);
    /// something else than the input method caused the change; available since version 1.
    pub const OTHER: Self = Self(1);
}
impl CommandReader<Self> for ZwpTextInputV3ChangeCause {
    async fn read(
        _object_id: u32,
        _opcode: u16,
        reader: &mut FdPayloadReader<'_>,
        _state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        Ok(Self(reader.next_u32().await?))
    }
}
impl CommandWriter for ZwpTextInputV3ChangeCause {
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
