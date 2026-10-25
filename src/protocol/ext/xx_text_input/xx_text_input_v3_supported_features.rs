use crate::protocol::{ClientState, CommandReader, CommandWriter};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Wire value `xx_text_input_v3.supported_features`.
///
/// Unknown numeric values are preserved.
/// ```xml
/// <enum name="supported_features" bitfield="true" since="2">
///       <description summary="possible supported features">
///         Client functionality over the baseline that isn't indicated implicitly.
///
///         This does not include events coming with .enable: when the input method receives such an event, it is clear the text input supports it, e.g. content_type, available_actions.
///
///         Baseline functionality like commit_string, set_preedit_string must always be supported for the protocol to be useful.
///
///         The flags match text-input protocol versions, but should be kept general enough to support other protocols.
///       </description>
///       <entry name="none" value="0x0" summary="no extra functionality supported" />
///       <entry name="move_cursor" value="0x1" summary="the move_cursor request" />
///     </enum>
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct XxTextInputV3SupportedFeatures(pub u32);
impl XxTextInputV3SupportedFeatures {
    /// no extra functionality supported; available since version 2.
    pub const NONE: Self = Self(0x0);
    /// the move_cursor request; available since version 2.
    pub const MOVE_CURSOR: Self = Self(0x1);
}
impl std::ops::BitOr for XxTextInputV3SupportedFeatures {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl std::ops::BitAnd for XxTextInputV3SupportedFeatures {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl CommandReader<Self> for XxTextInputV3SupportedFeatures {
    async fn read(
        _object_id: u32,
        _opcode: u16,
        reader: &mut FdPayloadReader<'_>,
        _state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        Ok(Self(reader.next_u32().await?))
    }
}
impl CommandWriter for XxTextInputV3SupportedFeatures {
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
