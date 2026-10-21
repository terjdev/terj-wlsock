use crate::protocol::ClientState;
use crate::protocol::{CommandReader, CommandWriter};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Core-protocol value `wl_pointer.axis_relative_direction`.
///
/// The numeric representation preserves unknown values without losing information.
/// Reading and writing encode four bytes; this is an argument value,
/// not a standalone message: object_id and opcode are both zero.
/// ```xml
/// <enum name="axis_relative_direction">
///       <description summary="axis relative direction">
///         This specifies the direction of the physical motion that caused a
///         wl_pointer.axis event, relative to the wl_pointer.axis direction.
///       </description>
///       <entry name="identical" value="0" summary="physical motion matches axis direction" />
///       <entry name="inverted" value="1" summary="physical motion is the inverse of the axis direction" />
///     </enum>
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct WlPointerAxisRelativeDirection(pub u32);
impl WlPointerAxisRelativeDirection {
    /// physical motion matches axis direction; XML since=1.
    /// ```xml
    /// <entry name="identical" value="0" summary="physical motion matches axis direction" />
    /// ```
    pub const IDENTICAL: Self = Self(0);
    /// physical motion is the inverse of the axis direction; XML since=1.
    /// ```xml
    /// <entry name="inverted" value="1" summary="physical motion is the inverse of the axis direction" />
    /// ```
    pub const INVERTED: Self = Self(1);
}
impl CommandReader<Self> for WlPointerAxisRelativeDirection {
    async fn read(
        _object_id: u32,
        _opcode: u16,
        reader: &mut FdPayloadReader<'_>,
        _state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        Ok(Self(reader.next_u32().await?))
    }
}
impl CommandWriter for WlPointerAxisRelativeDirection {
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
