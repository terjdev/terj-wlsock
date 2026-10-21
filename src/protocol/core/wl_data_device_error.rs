use crate::protocol::ClientState;
use crate::protocol::{CommandReader, CommandWriter};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Core-protocol value `wl_data_device.error`.
///
/// The numeric representation preserves unknown values without losing information.
/// Reading and writing encode four bytes; this is an argument value,
/// not a standalone message: object_id and opcode are both zero.
/// ```xml
/// <enum name="error">
///       <entry name="role" value="0" summary="given wl_surface has another role" />
///       <entry name="used_source" value="1" summary="source has already been used" />
///     </enum>
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct WlDataDeviceError(pub u32);
impl WlDataDeviceError {
    /// given wl_surface has another role; XML since=1.
    /// ```xml
    /// <entry name="role" value="0" summary="given wl_surface has another role" />
    /// ```
    pub const ROLE: Self = Self(0);
    /// source has already been used; XML since=1.
    /// ```xml
    /// <entry name="used_source" value="1" summary="source has already been used" />
    /// ```
    pub const USED_SOURCE: Self = Self(1);
}
impl CommandReader<Self> for WlDataDeviceError {
    async fn read(
        _object_id: u32,
        _opcode: u16,
        reader: &mut FdPayloadReader<'_>,
        _state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        Ok(Self(reader.next_u32().await?))
    }
}
impl CommandWriter for WlDataDeviceError {
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
