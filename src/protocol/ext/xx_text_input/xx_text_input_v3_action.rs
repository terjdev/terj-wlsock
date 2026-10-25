use crate::protocol::{ClientState, CommandReader, CommandWriter};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Wire value `xx_text_input_v3.action`.
///
/// Unknown numeric values are preserved.
/// ```xml
/// <enum name="action" since="2">
///       <description summary="action">
///         A possible action to perform on a text input.
///       </description>
///       <entry name="none" value="1">
///         <description summary="no action">
///           This enum value exists only to provide a default to make double-buffering implementation easier. It should not be used explicitly.
///         </description>
///       </entry>
///
///
///
///       <entry name="finish" value="0">
///         <description summary="trigger appropriate action for the completion of editing">
///           This should be triggered when the user is done with editing the field and wants to move on. For example, the query was typed and the user wants the search result. Or the name was entered and the address needs to be typed next.
///
///           The action to perform depends on the application, and should match the value of the current content_purpose.
///
///           All clients SHOULD implement this action. Without it, on-screen keyboards don't work as expected.
///         </description>
///       </entry>
///     </enum>
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct XxTextInputV3Action(pub u32);
impl XxTextInputV3Action {
    /// no action; available since version 2.
    pub const NONE: Self = Self(1);
    /// trigger appropriate action for the completion of editing; available since version 2.
    pub const FINISH: Self = Self(0);
}
impl CommandReader<Self> for XxTextInputV3Action {
    async fn read(
        _object_id: u32,
        _opcode: u16,
        reader: &mut FdPayloadReader<'_>,
        _state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        Ok(Self(reader.next_u32().await?))
    }
}
impl CommandWriter for XxTextInputV3Action {
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
