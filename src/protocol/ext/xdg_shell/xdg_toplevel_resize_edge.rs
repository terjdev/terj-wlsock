use crate::protocol::{ClientState, CommandReader, CommandWriter};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Wire value `xdg_toplevel.resize_edge`.
///
/// Unknown numeric values and flag bits are preserved.
/// ```xml
/// <enum name="resize_edge">
///       <description summary="edge values for resizing">
///     These values are used to indicate which edge of a surface
///     is being dragged in a resize operation.
///       </description>
///       <entry name="none" value="0" />
///       <entry name="top" value="1" />
///       <entry name="bottom" value="2" />
///       <entry name="left" value="4" />
///       <entry name="top_left" value="5" />
///       <entry name="bottom_left" value="6" />
///       <entry name="right" value="8" />
///       <entry name="top_right" value="9" />
///       <entry name="bottom_right" value="10" />
///     </enum>
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct XdgToplevelResizeEdge(pub u32);
impl XdgToplevelResizeEdge {
    /// none; available since version 1.
    pub const NONE: Self = Self(0);
    /// top; available since version 1.
    pub const TOP: Self = Self(1);
    /// bottom; available since version 1.
    pub const BOTTOM: Self = Self(2);
    /// left; available since version 1.
    pub const LEFT: Self = Self(4);
    /// top left; available since version 1.
    pub const TOP_LEFT: Self = Self(5);
    /// bottom left; available since version 1.
    pub const BOTTOM_LEFT: Self = Self(6);
    /// right; available since version 1.
    pub const RIGHT: Self = Self(8);
    /// top right; available since version 1.
    pub const TOP_RIGHT: Self = Self(9);
    /// bottom right; available since version 1.
    pub const BOTTOM_RIGHT: Self = Self(10);
}
impl CommandReader<Self> for XdgToplevelResizeEdge {
    async fn read(
        _object_id: u32,
        _opcode: u16,
        reader: &mut FdPayloadReader<'_>,
        _state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        Ok(Self(reader.next_u32().await?))
    }
}
impl CommandWriter for XdgToplevelResizeEdge {
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
