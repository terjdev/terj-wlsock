use crate::protocol::{ClientState, CommandReader, CommandWriter};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Wire value `zwlr_layer_shell_v1.layer`.
///
/// Unknown numeric values and flag bits are preserved.
/// ```xml
/// <enum name="layer">
///       <description summary="available layers for surfaces">
///         These values indicate which layers a surface can be rendered in. They
///         are ordered by z depth, bottom-most first. Traditional shell surfaces
///         will typically be rendered between the bottom and top layers.
///         Fullscreen shell surfaces are typically rendered at the top layer.
///         Multiple surfaces can share a single layer, and ordering within a
///         single layer is undefined.
///       </description>
///
///       <entry name="background" value="0" />
///       <entry name="bottom" value="1" />
///       <entry name="top" value="2" />
///       <entry name="overlay" value="3" />
///     </enum>
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct ZwlrLayerShellV1Layer(pub u32);
impl ZwlrLayerShellV1Layer {
    /// background; available since version 1.
    pub const BACKGROUND: Self = Self(0);
    /// bottom; available since version 1.
    pub const BOTTOM: Self = Self(1);
    /// top; available since version 1.
    pub const TOP: Self = Self(2);
    /// overlay; available since version 1.
    pub const OVERLAY: Self = Self(3);
}
impl CommandReader<Self> for ZwlrLayerShellV1Layer {
    async fn read(
        _object_id: u32,
        _opcode: u16,
        reader: &mut FdPayloadReader<'_>,
        _state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        Ok(Self(reader.next_u32().await?))
    }
}
impl CommandWriter for ZwlrLayerShellV1Layer {
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
