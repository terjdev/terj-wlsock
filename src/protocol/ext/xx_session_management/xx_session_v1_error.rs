use crate::protocol::{ClientState, CommandReader, CommandWriter};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Wire value `xx_session_v1.error`.
///
/// Unknown numeric values are preserved.
/// ```xml
/// <enum name="error">
///       <entry name="invalid_restore" summary="restore cannot be performed after initial toplevel commit" value="1" />
///       <entry name="name_in_use" summary="toplevel name is already in used" value="2" />
///       <entry name="already_mapped" summary="toplevel was already mapped when restored" value="3" />
///     </enum>
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct XxSessionV1Error(pub u32);
impl XxSessionV1Error {
    /// restore cannot be performed after initial toplevel commit; available since version 1.
    pub const INVALID_RESTORE: Self = Self(1);
    /// toplevel name is already in used; available since version 1.
    pub const NAME_IN_USE: Self = Self(2);
    /// toplevel was already mapped when restored; available since version 1.
    pub const ALREADY_MAPPED: Self = Self(3);
}
impl CommandReader<Self> for XxSessionV1Error {
    async fn read(
        _object_id: u32,
        _opcode: u16,
        reader: &mut FdPayloadReader<'_>,
        _state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        Ok(Self(reader.next_u32().await?))
    }
}
impl CommandWriter for XxSessionV1Error {
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
