use crate::protocol::ClientState;
use crate::protocol::{CommandReader, CommandWriter};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Core-protocol value `wl_data_source.error`.
///
/// The numeric representation preserves unknown values without losing information.
/// Reading and writing encode four bytes; this is an argument value,
/// not a standalone message: object_id and opcode are both zero.
/// ```xml
/// <enum name="error">
///       <entry name="invalid_action_mask" value="0" summary="action mask contains invalid values" />
///       <entry name="invalid_source" value="1" summary="source doesn't accept this request" />
///     </enum>
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct WlDataSourceError(pub u32);
impl WlDataSourceError {
    /// action mask contains invalid values; XML since=1.
    /// ```xml
    /// <entry name="invalid_action_mask" value="0" summary="action mask contains invalid values" />
    /// ```
    pub const INVALID_ACTION_MASK: Self = Self(0);
    /// source doesn't accept this request; XML since=1.
    /// ```xml
    /// <entry name="invalid_source" value="1" summary="source doesn't accept this request" />
    /// ```
    pub const INVALID_SOURCE: Self = Self(1);
}
impl CommandReader<Self> for WlDataSourceError {
    async fn read(
        _object_id: u32,
        _opcode: u16,
        reader: &mut FdPayloadReader<'_>,
        _state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        Ok(Self(reader.next_u32().await?))
    }
}
impl CommandWriter for WlDataSourceError {
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
