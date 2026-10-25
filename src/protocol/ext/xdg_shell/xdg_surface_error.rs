use crate::protocol::{ClientState, CommandReader, CommandWriter};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Wire value `xdg_surface.error`.
///
/// Unknown numeric values and flag bits are preserved.
/// ```xml
/// <enum name="error">
///       <entry name="not_constructed" value="1" summary="Surface was not fully constructed" />
///       <entry name="already_constructed" value="2" summary="Surface was already constructed" />
///       <entry name="unconfigured_buffer" value="3" summary="Attaching a buffer to an unconfigured surface" />
///       <entry name="invalid_serial" value="4" summary="Invalid serial number when acking a configure event" />
///       <entry name="invalid_size" value="5" summary="Width or height was zero or negative" />
///       <entry name="defunct_role_object" value="6" summary="Surface was destroyed before its role object" />
///     </enum>
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct XdgSurfaceError(pub u32);
impl XdgSurfaceError {
    /// Surface was not fully constructed; available since version 1.
    pub const NOT_CONSTRUCTED: Self = Self(1);
    /// Surface was already constructed; available since version 1.
    pub const ALREADY_CONSTRUCTED: Self = Self(2);
    /// Attaching a buffer to an unconfigured surface; available since version 1.
    pub const UNCONFIGURED_BUFFER: Self = Self(3);
    /// Invalid serial number when acking a configure event; available since version 1.
    pub const INVALID_SERIAL: Self = Self(4);
    /// Width or height was zero or negative; available since version 1.
    pub const INVALID_SIZE: Self = Self(5);
    /// Surface was destroyed before its role object; available since version 1.
    pub const DEFUNCT_ROLE_OBJECT: Self = Self(6);
}
impl CommandReader<Self> for XdgSurfaceError {
    async fn read(
        _object_id: u32,
        _opcode: u16,
        reader: &mut FdPayloadReader<'_>,
        _state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        Ok(Self(reader.next_u32().await?))
    }
}
impl CommandWriter for XdgSurfaceError {
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
