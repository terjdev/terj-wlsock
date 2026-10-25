use crate::protocol::{ClientState, CommandReader, CommandWriter};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Wire value `zwp_text_input_v3.action`.
///
/// Unknown numeric values are preserved.
/// ```xml
/// <enum name="action" since="2">
///       <description summary="action">
///         A possible action to perform on a text input.
///
///         The submit action is intended for input entries that expect some sort of
///         activation after user interaction, e.g. the URL entry in a browser.
///       </description>
///       <entry name="none" value="0" summary="no action" />
///       <entry name="submit" value="1" summary="the action is submitted" />
///     </enum>
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct ZwpTextInputV3Action(pub u32);
impl ZwpTextInputV3Action {
    /// no action; available since version 2.
    pub const NONE: Self = Self(0);
    /// the action is submitted; available since version 2.
    pub const SUBMIT: Self = Self(1);
}
impl CommandReader<Self> for ZwpTextInputV3Action {
    async fn read(
        _object_id: u32,
        _opcode: u16,
        reader: &mut FdPayloadReader<'_>,
        _state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        Ok(Self(reader.next_u32().await?))
    }
}
impl CommandWriter for ZwpTextInputV3Action {
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
