use crate::protocol::ClientState;
use crate::protocol::{CommandReader, CommandWriter};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Core-protocol value `wl_shm_pool.error`.
///
/// The numeric representation preserves unknown values without losing information.
/// Reading and writing encode four bytes; this is an argument value,
/// not a standalone message: object_id and opcode are both zero.
/// ```xml
/// <enum name="error" since="3">
///       <description summary="wl_shm_pool error values">
///         These errors can be emitted in response to wl_shm_pool requests.
///       </description>
///       <entry name="invalid_format" value="0" summary="buffer format is not known" />
///       <entry name="invalid_stride" value="1" summary="invalid size or stride during buffer creation" />
///     </enum>
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct WlShmPoolError(pub u32);
impl WlShmPoolError {
    /// buffer format is not known; XML since=1.
    /// ```xml
    /// <entry name="invalid_format" value="0" summary="buffer format is not known" />
    /// ```
    pub const INVALID_FORMAT: Self = Self(0);
    /// invalid size or stride during buffer creation; XML since=1.
    /// ```xml
    /// <entry name="invalid_stride" value="1" summary="invalid size or stride during buffer creation" />
    /// ```
    pub const INVALID_STRIDE: Self = Self(1);
}
impl CommandReader<Self> for WlShmPoolError {
    async fn read(
        _object_id: u32,
        _opcode: u16,
        reader: &mut FdPayloadReader<'_>,
        _state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        Ok(Self(reader.next_u32().await?))
    }
}
impl CommandWriter for WlShmPoolError {
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
