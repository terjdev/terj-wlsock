use crate::protocol::ClientState;
use crate::protocol::{CommandReader, CommandWriter};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Core-protocol value `wl_surface.error`.
///
/// The numeric representation preserves unknown values without losing information.
/// Reading and writing encode four bytes; this is an argument value,
/// not a standalone message: object_id and opcode are both zero.
/// ```xml
/// <enum name="error">
///       <description summary="wl_surface error values">
///         These errors can be emitted in response to wl_surface requests.
///       </description>
///       <entry name="invalid_scale" value="0" summary="buffer scale value is invalid" />
///       <entry name="invalid_transform" value="1" summary="buffer transform value is invalid" />
///       <entry name="invalid_size" value="2" summary="buffer size is invalid" />
///       <entry name="invalid_offset" value="3" summary="buffer offset is invalid" />
///       <entry name="defunct_role_object" value="4" summary="surface was destroyed before its role object" />
///       <entry name="no_buffer" value="5" summary="no buffer was attached" />
///     </enum>
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct WlSurfaceError(pub u32);
impl WlSurfaceError {
    /// buffer scale value is invalid; XML since=1.
    /// ```xml
    /// <entry name="invalid_scale" value="0" summary="buffer scale value is invalid" />
    /// ```
    pub const INVALID_SCALE: Self = Self(0);
    /// buffer transform value is invalid; XML since=1.
    /// ```xml
    /// <entry name="invalid_transform" value="1" summary="buffer transform value is invalid" />
    /// ```
    pub const INVALID_TRANSFORM: Self = Self(1);
    /// buffer size is invalid; XML since=1.
    /// ```xml
    /// <entry name="invalid_size" value="2" summary="buffer size is invalid" />
    /// ```
    pub const INVALID_SIZE: Self = Self(2);
    /// buffer offset is invalid; XML since=1.
    /// ```xml
    /// <entry name="invalid_offset" value="3" summary="buffer offset is invalid" />
    /// ```
    pub const INVALID_OFFSET: Self = Self(3);
    /// surface was destroyed before its role object; XML since=1.
    /// ```xml
    /// <entry name="defunct_role_object" value="4" summary="surface was destroyed before its role object" />
    /// ```
    pub const DEFUNCT_ROLE_OBJECT: Self = Self(4);
    /// no buffer was attached; XML since=1.
    /// ```xml
    /// <entry name="no_buffer" value="5" summary="no buffer was attached" />
    /// ```
    pub const NO_BUFFER: Self = Self(5);
}
impl CommandReader<Self> for WlSurfaceError {
    async fn read(
        _object_id: u32,
        _opcode: u16,
        reader: &mut FdPayloadReader<'_>,
        _state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        Ok(Self(reader.next_u32().await?))
    }
}
impl CommandWriter for WlSurfaceError {
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
