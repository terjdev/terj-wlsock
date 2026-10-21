use crate::protocol::ClientState;
use crate::protocol::{CommandReader, CommandWriter};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Core-protocol value `wl_display.error`.
///
/// The numeric representation preserves unknown values without losing information.
/// Reading and writing encode four bytes; this is an argument value,
/// not a standalone message: object_id and opcode are both zero.
/// ```xml
/// <enum name="error">
///       <description summary="global error values">
///         These errors are global and can be emitted in response to any
///         server request.
///       </description>
///       <entry name="invalid_object" value="0" summary="server couldn't find object" />
///       <entry name="invalid_method" value="1" summary="method doesn't exist on the specified interface or malformed request" />
///       <entry name="no_memory" value="2" summary="server is out of memory" />
///       <entry name="implementation" value="3" summary="implementation error in compositor" />
///     </enum>
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct WlDisplayError(pub u32);
impl WlDisplayError {
    /// server couldn't find object; XML since=1.
    /// ```xml
    /// <entry name="invalid_object" value="0" summary="server couldn't find object" />
    /// ```
    pub const INVALID_OBJECT: Self = Self(0);
    /// method doesn't exist on the specified interface or malformed request; XML since=1.
    /// ```xml
    /// <entry name="invalid_method" value="1" summary="method doesn't exist on the specified interface or malformed request" />
    /// ```
    pub const INVALID_METHOD: Self = Self(1);
    /// server is out of memory; XML since=1.
    /// ```xml
    /// <entry name="no_memory" value="2" summary="server is out of memory" />
    /// ```
    pub const NO_MEMORY: Self = Self(2);
    /// implementation error in compositor; XML since=1.
    /// ```xml
    /// <entry name="implementation" value="3" summary="implementation error in compositor" />
    /// ```
    pub const IMPLEMENTATION: Self = Self(3);
}
impl CommandReader<Self> for WlDisplayError {
    async fn read(
        _object_id: u32,
        _opcode: u16,
        reader: &mut FdPayloadReader<'_>,
        _state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        Ok(Self(reader.next_u32().await?))
    }
}
impl CommandWriter for WlDisplayError {
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
