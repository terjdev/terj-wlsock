use crate::protocol::{ClientState, CommandReader, CommandWriter};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Wire value `zwp_text_input_v3.content_hint`.
///
/// Unknown numeric values are preserved.
/// ```xml
/// <enum name="content_hint" bitfield="true">
///       <description summary="content hint">
///         Content hint is a bitmask to allow to modify the behavior of the text
///         input.
///       </description>
///       <entry name="none" value="0x0" summary="no special behavior" />
///       <entry name="completion" value="0x1" summary="suggest word completions" />
///       <entry name="spellcheck" value="0x2" summary="suggest word corrections" />
///       <entry name="auto_capitalization" value="0x4" summary="switch to uppercase letters at the start of a sentence" />
///       <entry name="lowercase" value="0x8" summary="prefer lowercase letters" />
///       <entry name="uppercase" value="0x10" summary="prefer uppercase letters" />
///       <entry name="titlecase" value="0x20" summary="prefer casing for titles and headings (can be language dependent)" />
///       <entry name="hidden_text" value="0x40" summary="characters should be hidden" />
///       <entry name="sensitive_data" value="0x80" summary="typed text should not be stored" />
///       <entry name="latin" value="0x100" summary="just Latin characters should be entered" />
///       <entry name="multiline" value="0x200" summary="the text input is multiline" />
///       <entry name="on_screen_input_provided" value="0x400" since="2" summary="an on-screen way to fill in the input is already provided by the client" />
///       <entry name="no_emoji" value="0x800" since="2" summary="prefer not offering emoji support" />
///       <entry name="preedit_shown" value="0x1000" since="2" summary="the text input will display preedit text in place" />
///     </enum>
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct ZwpTextInputV3ContentHint(pub u32);
impl ZwpTextInputV3ContentHint {
    /// no special behavior; available since version 1.
    pub const NONE: Self = Self(0x0);
    /// suggest word completions; available since version 1.
    pub const COMPLETION: Self = Self(0x1);
    /// suggest word corrections; available since version 1.
    pub const SPELLCHECK: Self = Self(0x2);
    /// switch to uppercase letters at the start of a sentence; available since version 1.
    pub const AUTO_CAPITALIZATION: Self = Self(0x4);
    /// prefer lowercase letters; available since version 1.
    pub const LOWERCASE: Self = Self(0x8);
    /// prefer uppercase letters; available since version 1.
    pub const UPPERCASE: Self = Self(0x10);
    /// prefer casing for titles and headings (can be language dependent); available since version 1.
    pub const TITLECASE: Self = Self(0x20);
    /// characters should be hidden; available since version 1.
    pub const HIDDEN_TEXT: Self = Self(0x40);
    /// typed text should not be stored; available since version 1.
    pub const SENSITIVE_DATA: Self = Self(0x80);
    /// just Latin characters should be entered; available since version 1.
    pub const LATIN: Self = Self(0x100);
    /// the text input is multiline; available since version 1.
    pub const MULTILINE: Self = Self(0x200);
    /// an on-screen way to fill in the input is already provided by the client; available since version 2.
    pub const ON_SCREEN_INPUT_PROVIDED: Self = Self(0x400);
    /// prefer not offering emoji support; available since version 2.
    pub const NO_EMOJI: Self = Self(0x800);
    /// the text input will display preedit text in place; available since version 2.
    pub const PREEDIT_SHOWN: Self = Self(0x1000);
}
impl std::ops::BitOr for ZwpTextInputV3ContentHint {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl std::ops::BitAnd for ZwpTextInputV3ContentHint {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl CommandReader<Self> for ZwpTextInputV3ContentHint {
    async fn read(
        _object_id: u32,
        _opcode: u16,
        reader: &mut FdPayloadReader<'_>,
        _state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        Ok(Self(reader.next_u32().await?))
    }
}
impl CommandWriter for ZwpTextInputV3ContentHint {
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
