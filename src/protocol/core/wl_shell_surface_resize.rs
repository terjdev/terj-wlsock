use crate::protocol::ClientState;
use crate::protocol::{CommandReader, CommandWriter};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Core-protocol value `wl_shell_surface.resize`.
///
/// The numeric representation preserves unknown values without losing information.
/// The bitmask supports combining values with `|`.
/// Reading and writing encode four bytes; this is an argument value,
/// not a standalone message: object_id and opcode are both zero.
/// ```xml
/// <enum name="resize" bitfield="true">
///       <description summary="edge values for resizing">
///         These values are used to indicate which edge of a surface
///         is being dragged in a resize operation. The server may
///         use this information to adapt its behavior, e.g. choose
///         an appropriate cursor image.
///       </description>
///       <entry name="none" value="0" summary="no edge" />
///       <entry name="top" value="1" summary="top edge" />
///       <entry name="bottom" value="2" summary="bottom edge" />
///       <entry name="left" value="4" summary="left edge" />
///       <entry name="top_left" value="5" summary="top and left edges" />
///       <entry name="bottom_left" value="6" summary="bottom and left edges" />
///       <entry name="right" value="8" summary="right edge" />
///       <entry name="top_right" value="9" summary="top and right edges" />
///       <entry name="bottom_right" value="10" summary="bottom and right edges" />
///     </enum>
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct WlShellSurfaceResize(pub u32);
impl WlShellSurfaceResize {
    /// no edge; XML since=1.
    /// ```xml
    /// <entry name="none" value="0" summary="no edge" />
    /// ```
    pub const NONE: Self = Self(0);
    /// top edge; XML since=1.
    /// ```xml
    /// <entry name="top" value="1" summary="top edge" />
    /// ```
    pub const TOP: Self = Self(1);
    /// bottom edge; XML since=1.
    /// ```xml
    /// <entry name="bottom" value="2" summary="bottom edge" />
    /// ```
    pub const BOTTOM: Self = Self(2);
    /// left edge; XML since=1.
    /// ```xml
    /// <entry name="left" value="4" summary="left edge" />
    /// ```
    pub const LEFT: Self = Self(4);
    /// top and left edges; XML since=1.
    /// ```xml
    /// <entry name="top_left" value="5" summary="top and left edges" />
    /// ```
    pub const TOP_LEFT: Self = Self(5);
    /// bottom and left edges; XML since=1.
    /// ```xml
    /// <entry name="bottom_left" value="6" summary="bottom and left edges" />
    /// ```
    pub const BOTTOM_LEFT: Self = Self(6);
    /// right edge; XML since=1.
    /// ```xml
    /// <entry name="right" value="8" summary="right edge" />
    /// ```
    pub const RIGHT: Self = Self(8);
    /// top and right edges; XML since=1.
    /// ```xml
    /// <entry name="top_right" value="9" summary="top and right edges" />
    /// ```
    pub const TOP_RIGHT: Self = Self(9);
    /// bottom and right edges; XML since=1.
    /// ```xml
    /// <entry name="bottom_right" value="10" summary="bottom and right edges" />
    /// ```
    pub const BOTTOM_RIGHT: Self = Self(10);
}
impl std::ops::BitOr for WlShellSurfaceResize {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl std::ops::BitAnd for WlShellSurfaceResize {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl CommandReader<Self> for WlShellSurfaceResize {
    async fn read(
        _object_id: u32,
        _opcode: u16,
        reader: &mut FdPayloadReader<'_>,
        _state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        Ok(Self(reader.next_u32().await?))
    }
}
impl CommandWriter for WlShellSurfaceResize {
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
