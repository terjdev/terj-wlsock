use crate::protocol::{ClientState, CommandReader, CommandWriter};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Wire value `xdg_session_v1.error`.
///
/// Unknown numeric values and flag bits are preserved.
/// ```xml
/// <enum name="error">
///       <entry name="name_in_use" summary="toplevel name is already in use" value="1" />
///       <entry name="already_mapped" summary="toplevel was already mapped when restored" value="2" />
///       <entry name="invalid_name" summary="provided toplevel name is invalid" value="3" />
///       <entry name="already_added" summary="toplevel already added" value="4" />
///     </enum>
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct XdgSessionV1Error(pub u32);
impl XdgSessionV1Error {
    /// toplevel name is already in use; available since version 1.
    pub const NAME_IN_USE: Self = Self(1);
    /// toplevel was already mapped when restored; available since version 1.
    pub const ALREADY_MAPPED: Self = Self(2);
    /// provided toplevel name is invalid; available since version 1.
    pub const INVALID_NAME: Self = Self(3);
    /// toplevel already added; available since version 1.
    pub const ALREADY_ADDED: Self = Self(4);
}
impl CommandReader<Self> for XdgSessionV1Error {
    async fn read(
        _object_id: u32,
        _opcode: u16,
        reader: &mut FdPayloadReader<'_>,
        _state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        Ok(Self(reader.next_u32().await?))
    }
}
impl CommandWriter for XdgSessionV1Error {
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
