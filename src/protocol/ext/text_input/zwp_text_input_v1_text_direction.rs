use crate::protocol::{ClientState, CommandReader, CommandWriter};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Wire value `zwp_text_input_v1.text_direction`.
///
/// Unknown numeric values are preserved.
/// ```xml
/// <enum name="text_direction">
///       <entry name="auto" value="0" summary="automatic text direction based on text and language" />
///       <entry name="ltr" value="1" summary="left-to-right" />
///       <entry name="rtl" value="2" summary="right-to-left" />
///     </enum>
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct ZwpTextInputV1TextDirection(pub u32);
impl ZwpTextInputV1TextDirection {
    /// automatic text direction based on text and language; available since version 1.
    pub const AUTO: Self = Self(0);
    /// left-to-right; available since version 1.
    pub const LTR: Self = Self(1);
    /// right-to-left; available since version 1.
    pub const RTL: Self = Self(2);
}
impl CommandReader<Self> for ZwpTextInputV1TextDirection {
    async fn read(
        _object_id: u32,
        _opcode: u16,
        reader: &mut FdPayloadReader<'_>,
        _state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        Ok(Self(reader.next_u32().await?))
    }
}
impl CommandWriter for ZwpTextInputV1TextDirection {
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
