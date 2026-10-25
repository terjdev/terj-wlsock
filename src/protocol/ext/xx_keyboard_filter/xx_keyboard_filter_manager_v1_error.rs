use crate::protocol::{ClientState, CommandReader, CommandWriter};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Wire value `xx_keyboard_filter_manager_v1.error`.
///
/// Unknown numeric values are preserved.
/// ```xml
/// <enum name="error">
///       <entry name="already_bound" value="0x1" summary="an argument is already bound" />
///       <entry name="wrong_seat" value="0x2" summary="the keyboard i attached to the wrong seat for this operation" />
///     </enum>
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct XxKeyboardFilterManagerV1Error(pub u32);
impl XxKeyboardFilterManagerV1Error {
    /// an argument is already bound; available since version 1.
    pub const ALREADY_BOUND: Self = Self(0x1);
    /// the keyboard i attached to the wrong seat for this operation; available since version 1.
    pub const WRONG_SEAT: Self = Self(0x2);
}
impl CommandReader<Self> for XxKeyboardFilterManagerV1Error {
    async fn read(
        _object_id: u32,
        _opcode: u16,
        reader: &mut FdPayloadReader<'_>,
        _state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        Ok(Self(reader.next_u32().await?))
    }
}
impl CommandWriter for XxKeyboardFilterManagerV1Error {
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
