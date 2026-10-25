use crate::protocol::{ClientState, CommandReader, CommandWriter};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Wire value `xx_input_method_v1.protocol_compat`.
///
/// Unknown numeric values are preserved.
/// ```xml
/// <enum name="protocol_compat" since="4">
///       <description summary="protocol compatibility value">
///         Tells the input method client what kinds of events the text input client supports.
///       </description>
///       <entry name="text_input_v3" value="0x0" summary="zwp-text-input-v3 semantics" />
///       <entry name="xx_text_input" value="0x1">
///         <description summary="xx-text-input semantics">
///           Changes the meaning of serial compared to v3.
///           The text input client now applies requested updates on a "best-effort" basis.
///         </description>
///       </entry>
///
///     </enum>
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct XxInputMethodV1ProtocolCompat(pub u32);
impl XxInputMethodV1ProtocolCompat {
    /// zwp-text-input-v3 semantics; available since version 4.
    pub const TEXT_INPUT_V3: Self = Self(0x0);
    /// xx-text-input semantics; available since version 4.
    pub const XX_TEXT_INPUT: Self = Self(0x1);
}
impl CommandReader<Self> for XxInputMethodV1ProtocolCompat {
    async fn read(
        _object_id: u32,
        _opcode: u16,
        reader: &mut FdPayloadReader<'_>,
        _state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        Ok(Self(reader.next_u32().await?))
    }
}
impl CommandWriter for XxInputMethodV1ProtocolCompat {
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
