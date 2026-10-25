use crate::protocol::{ClientState, CommandReader, CommandWriter};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Wire value `zwlr_input_inhibit_manager_v1.error`.
///
/// Unknown numeric values are preserved.
/// ```xml
/// <enum name="error">
///       <entry name="already_inhibited" value="0" summary="an input inhibitor is already in use on the compositor" />
///     </enum>
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct ZwlrInputInhibitManagerV1Error(pub u32);
impl ZwlrInputInhibitManagerV1Error {
    /// an input inhibitor is already in use on the compositor; available since version 1.
    pub const ALREADY_INHIBITED: Self = Self(0);
}
impl CommandReader<Self> for ZwlrInputInhibitManagerV1Error {
    async fn read(
        _object_id: u32,
        _opcode: u16,
        reader: &mut FdPayloadReader<'_>,
        _state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        Ok(Self(reader.next_u32().await?))
    }
}
impl CommandWriter for ZwlrInputInhibitManagerV1Error {
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
