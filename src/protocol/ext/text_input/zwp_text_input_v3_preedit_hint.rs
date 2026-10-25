use crate::protocol::{ClientState, CommandReader, CommandWriter};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Wire value `zwp_text_input_v3.preedit_hint`.
///
/// Unknown numeric values are preserved.
/// ```xml
/// <enum name="preedit_hint">
///       <description summary="preedit style hint">
///     Style hints for the preedit string.
///       </description>
///       <entry name="whole" value="1" summary="simple pre-edit text style, typically underlined" />
///       <entry name="selection" value="2" summary="hint for a selected piece of text, e.g. per-character navigation and composition" />
///       <entry name="prediction" value="3" summary="predicted text, not typed by the user" />
///       <entry name="prefix" value="4" summary="prefixed text not being currently edited, e.g. prior to a 'selection' section" />
///       <entry name="suffix" value="5" summary="suffixed text not being currently edited, e.g. after a 'selection' section" />
///       <entry name="spelling_error" value="6" summary="spelling error" />
///       <entry name="compose_error" value="7" summary="wrong composition, e.g. user input that can not be transliterated" />
///     </enum>
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct ZwpTextInputV3PreeditHint(pub u32);
impl ZwpTextInputV3PreeditHint {
    /// simple pre-edit text style, typically underlined; available since version 1.
    pub const WHOLE: Self = Self(1);
    /// hint for a selected piece of text, e.g. per-character navigation and composition; available since version 1.
    pub const SELECTION: Self = Self(2);
    /// predicted text, not typed by the user; available since version 1.
    pub const PREDICTION: Self = Self(3);
    /// prefixed text not being currently edited, e.g. prior to a 'selection' section; available since version 1.
    pub const PREFIX: Self = Self(4);
    /// suffixed text not being currently edited, e.g. after a 'selection' section; available since version 1.
    pub const SUFFIX: Self = Self(5);
    /// spelling error; available since version 1.
    pub const SPELLING_ERROR: Self = Self(6);
    /// wrong composition, e.g. user input that can not be transliterated; available since version 1.
    pub const COMPOSE_ERROR: Self = Self(7);
}
impl CommandReader<Self> for ZwpTextInputV3PreeditHint {
    async fn read(
        _object_id: u32,
        _opcode: u16,
        reader: &mut FdPayloadReader<'_>,
        _state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        Ok(Self(reader.next_u32().await?))
    }
}
impl CommandWriter for ZwpTextInputV3PreeditHint {
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
