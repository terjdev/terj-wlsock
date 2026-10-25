use crate::protocol::{ClientState, CommandReader, CommandWriter};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Wire value `zwlr_layer_shell_v1.error`.
///
/// Unknown numeric values and flag bits are preserved.
/// ```xml
/// <enum name="error">
///       <entry name="role" value="0" summary="wl_surface has another role" />
///       <entry name="invalid_layer" value="1" summary="layer value is invalid" />
///       <entry name="already_constructed" value="2" summary="wl_surface has a buffer attached or committed" />
///     </enum>
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct ZwlrLayerShellV1Error(pub u32);
impl ZwlrLayerShellV1Error {
    /// wl_surface has another role; available since version 1.
    pub const ROLE: Self = Self(0);
    /// layer value is invalid; available since version 1.
    pub const INVALID_LAYER: Self = Self(1);
    /// wl_surface has a buffer attached or committed; available since version 1.
    pub const ALREADY_CONSTRUCTED: Self = Self(2);
}
impl CommandReader<Self> for ZwlrLayerShellV1Error {
    async fn read(
        _object_id: u32,
        _opcode: u16,
        reader: &mut FdPayloadReader<'_>,
        _state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        Ok(Self(reader.next_u32().await?))
    }
}
impl CommandWriter for ZwlrLayerShellV1Error {
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
