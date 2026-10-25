use crate::protocol::{ClientState, CommandReader, CommandWriter};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Wire value `zwp_text_input_v1.preedit_style`.
///
/// Unknown numeric values are preserved.
/// ```xml
/// <enum name="preedit_style">
///       <entry name="default" value="0" summary="default style for composing text" />
///       <entry name="none" value="1" summary="style should be the same as in non-composing text" />
///       <entry name="active" value="2" />
///       <entry name="inactive" value="3" />
///       <entry name="highlight" value="4" />
///       <entry name="underline" value="5" />
///       <entry name="selection" value="6" />
///       <entry name="incorrect" value="7" />
///     </enum>
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct ZwpTextInputV1PreeditStyle(pub u32);
impl ZwpTextInputV1PreeditStyle {
    /// default style for composing text; available since version 1.
    pub const DEFAULT: Self = Self(0);
    /// style should be the same as in non-composing text; available since version 1.
    pub const NONE: Self = Self(1);
    /// active; available since version 1.
    pub const ACTIVE: Self = Self(2);
    /// inactive; available since version 1.
    pub const INACTIVE: Self = Self(3);
    /// highlight; available since version 1.
    pub const HIGHLIGHT: Self = Self(4);
    /// underline; available since version 1.
    pub const UNDERLINE: Self = Self(5);
    /// selection; available since version 1.
    pub const SELECTION: Self = Self(6);
    /// incorrect; available since version 1.
    pub const INCORRECT: Self = Self(7);
}
impl CommandReader<Self> for ZwpTextInputV1PreeditStyle {
    async fn read(
        _object_id: u32,
        _opcode: u16,
        reader: &mut FdPayloadReader<'_>,
        _state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        Ok(Self(reader.next_u32().await?))
    }
}
impl CommandWriter for ZwpTextInputV1PreeditStyle {
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
