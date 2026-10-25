use crate::protocol::{ClientState, CommandReader, CommandWriter};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Wire value `xwayland_surface_v1.error`.
///
/// Unknown numeric values and flag bits are preserved.
/// ```xml
/// <enum name="error">
///       <entry name="already_associated" value="0" summary="given wl_surface is already associated with an X11 window" />
///       <entry name="invalid_serial" value="1" summary="serial was not valid" />
///     </enum>
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct XwaylandSurfaceV1Error(pub u32);
impl XwaylandSurfaceV1Error {
    /// given wl_surface is already associated with an X11 window; available since version 1.
    pub const ALREADY_ASSOCIATED: Self = Self(0);
    /// serial was not valid; available since version 1.
    pub const INVALID_SERIAL: Self = Self(1);
}
impl CommandReader<Self> for XwaylandSurfaceV1Error {
    async fn read(
        _object_id: u32,
        _opcode: u16,
        reader: &mut FdPayloadReader<'_>,
        _state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        Ok(Self(reader.next_u32().await?))
    }
}
impl CommandWriter for XwaylandSurfaceV1Error {
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
