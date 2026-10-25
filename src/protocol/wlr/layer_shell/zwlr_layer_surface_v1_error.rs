use crate::protocol::{ClientState, CommandReader, CommandWriter};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Wire value `zwlr_layer_surface_v1.error`.
///
/// Unknown numeric values and flag bits are preserved.
/// ```xml
/// <enum name="error">
///       <entry name="invalid_surface_state" value="0" summary="provided surface state is invalid" />
///       <entry name="invalid_size" value="1" summary="size is invalid" />
///       <entry name="invalid_anchor" value="2" summary="anchor bitfield is invalid" />
///       <entry name="invalid_keyboard_interactivity" value="3" summary="keyboard interactivity is invalid" />
///       <entry name="invalid_exclusive_edge" value="4" summary="exclusive edge is invalid given the surface anchors" />
///     </enum>
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct ZwlrLayerSurfaceV1Error(pub u32);
impl ZwlrLayerSurfaceV1Error {
    /// provided surface state is invalid; available since version 1.
    pub const INVALID_SURFACE_STATE: Self = Self(0);
    /// size is invalid; available since version 1.
    pub const INVALID_SIZE: Self = Self(1);
    /// anchor bitfield is invalid; available since version 1.
    pub const INVALID_ANCHOR: Self = Self(2);
    /// keyboard interactivity is invalid; available since version 1.
    pub const INVALID_KEYBOARD_INTERACTIVITY: Self = Self(3);
    /// exclusive edge is invalid given the surface anchors; available since version 1.
    pub const INVALID_EXCLUSIVE_EDGE: Self = Self(4);
}
impl CommandReader<Self> for ZwlrLayerSurfaceV1Error {
    async fn read(
        _object_id: u32,
        _opcode: u16,
        reader: &mut FdPayloadReader<'_>,
        _state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        Ok(Self(reader.next_u32().await?))
    }
}
impl CommandWriter for ZwlrLayerSurfaceV1Error {
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
