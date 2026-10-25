use crate::protocol::{ClientState, CommandReader, CommandWriter};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Wire value `xx_hotkey_v1.error`.
///
/// Unknown numeric values are preserved.
/// ```xml
/// <enum name="error">
///       <description summary="illegal request sequences">
///         Fatal protocol errors. A trigger that is merely unacceptable to the
///         compositor is reported through "denied" and is not an error.
///       </description>
///       <entry name="invalid_trigger" value="0" summary="a null keysym, or unknown modifier bits" />
///       <entry name="no_trigger" value="1" summary="commit was sent before any trigger was described" />
///     </enum>
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct XxHotkeyV1Error(pub u32);
impl XxHotkeyV1Error {
    /// a null keysym, or unknown modifier bits; available since version 1.
    pub const INVALID_TRIGGER: Self = Self(0);
    /// commit was sent before any trigger was described; available since version 1.
    pub const NO_TRIGGER: Self = Self(1);
}
impl CommandReader<Self> for XxHotkeyV1Error {
    async fn read(
        _object_id: u32,
        _opcode: u16,
        reader: &mut FdPayloadReader<'_>,
        _state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        Ok(Self(reader.next_u32().await?))
    }
}
impl CommandWriter for XxHotkeyV1Error {
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
