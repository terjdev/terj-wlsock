use crate::protocol::{ClientState, CommandReader, CommandWriter};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Wire value `zwp_text_input_v1.content_hint`.
///
/// Unknown numeric values are preserved.
/// ```xml
/// <enum name="content_hint" bitfield="true">
///       <description summary="content hint">
///     Content hint is a bitmask to allow to modify the behavior of the text
///     input.
///       </description>
///       <entry name="none" value="0x0" summary="no special behaviour" />
///       <entry name="default" value="0x7" summary="auto completion, correction and capitalization" />
///       <entry name="password" value="0xc0" summary="hidden and sensitive text" />
///       <entry name="auto_completion" value="0x1" summary="suggest word completions" />
///       <entry name="auto_correction" value="0x2" summary="suggest word corrections" />
///       <entry name="auto_capitalization" value="0x4" summary="switch to uppercase letters at the start of a sentence" />
///       <entry name="lowercase" value="0x8" summary="prefer lowercase letters" />
///       <entry name="uppercase" value="0x10" summary="prefer uppercase letters" />
///       <entry name="titlecase" value="0x20" summary="prefer casing for titles and headings (can be language dependent)" />
///       <entry name="hidden_text" value="0x40" summary="characters should be hidden" />
///       <entry name="sensitive_data" value="0x80" summary="typed text should not be stored" />
///       <entry name="latin" value="0x100" summary="just latin characters should be entered" />
///       <entry name="multiline" value="0x200" summary="the text input is multiline" />
///     </enum>
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct ZwpTextInputV1ContentHint(pub u32);
impl ZwpTextInputV1ContentHint {
    /// no special behaviour; available since version 1.
    pub const NONE: Self = Self(0x0);
    /// auto completion, correction and capitalization; available since version 1.
    pub const DEFAULT: Self = Self(0x7);
    /// hidden and sensitive text; available since version 1.
    pub const PASSWORD: Self = Self(0xc0);
    /// suggest word completions; available since version 1.
    pub const AUTO_COMPLETION: Self = Self(0x1);
    /// suggest word corrections; available since version 1.
    pub const AUTO_CORRECTION: Self = Self(0x2);
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
    /// just latin characters should be entered; available since version 1.
    pub const LATIN: Self = Self(0x100);
    /// the text input is multiline; available since version 1.
    pub const MULTILINE: Self = Self(0x200);
}
impl std::ops::BitOr for ZwpTextInputV1ContentHint {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl std::ops::BitAnd for ZwpTextInputV1ContentHint {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl CommandReader<Self> for ZwpTextInputV1ContentHint {
    async fn read(
        _object_id: u32,
        _opcode: u16,
        reader: &mut FdPayloadReader<'_>,
        _state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        Ok(Self(reader.next_u32().await?))
    }
}
impl CommandWriter for ZwpTextInputV1ContentHint {
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
