use crate::protocol::ClientState;
use crate::protocol::{CommandReader, CommandWriter};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Core-protocol value `wl_pointer.axis`.
///
/// The numeric representation preserves unknown values without losing information.
/// Reading and writing encode four bytes; this is an argument value,
/// not a standalone message: object_id and opcode are both zero.
/// ```xml
/// <enum name="axis">
///       <description summary="axis types">
///         Describes the axis types of scroll events.
///       </description>
///       <entry name="vertical_scroll" value="0" summary="vertical axis" />
///       <entry name="horizontal_scroll" value="1" summary="horizontal axis" />
///     </enum>
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct WlPointerAxis(pub u32);
impl WlPointerAxis {
    /// vertical axis; XML since=1.
    /// ```xml
    /// <entry name="vertical_scroll" value="0" summary="vertical axis" />
    /// ```
    pub const VERTICAL_SCROLL: Self = Self(0);
    /// horizontal axis; XML since=1.
    /// ```xml
    /// <entry name="horizontal_scroll" value="1" summary="horizontal axis" />
    /// ```
    pub const HORIZONTAL_SCROLL: Self = Self(1);
}
impl CommandReader<Self> for WlPointerAxis {
    async fn read(
        _object_id: u32,
        _opcode: u16,
        reader: &mut FdPayloadReader<'_>,
        _state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        Ok(Self(reader.next_u32().await?))
    }
}
impl CommandWriter for WlPointerAxis {
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
