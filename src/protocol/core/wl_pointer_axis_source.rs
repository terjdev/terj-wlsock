use crate::protocol::ClientState;
use crate::protocol::{CommandReader, CommandWriter};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Core-protocol value `wl_pointer.axis_source`.
///
/// The numeric representation preserves unknown values without losing information.
/// Reading and writing encode four bytes; this is an argument value,
/// not a standalone message: object_id and opcode are both zero.
/// ```xml
/// <enum name="axis_source">
///       <description summary="axis source types">
///         Describes the source types for axis events. This indicates to the
///         client how an axis event was physically generated; a client may
///         adjust the user interface accordingly. For example, scroll events
///         from a "finger" source may be in a smooth coordinate space with
///         kinetic scrolling whereas a "wheel" source may be in discrete steps
///         of a number of lines.
///
///         The "continuous" axis source is a device generating events in a
///         continuous coordinate space, but using something other than a
///         finger. One example for this source is button-based scrolling where
///         the vertical motion of a device is converted to scroll events while
///         a button is held down.
///
///         The "wheel tilt" axis source indicates that the actual device is a
///         wheel but the scroll event is not caused by a rotation but a
///         (usually sideways) tilt of the wheel.
///       </description>
///       <entry name="wheel" value="0" summary="a physical wheel rotation" />
///       <entry name="finger" value="1" summary="finger on a touch surface" />
///       <entry name="continuous" value="2" summary="continuous coordinate space" />
///       <entry name="wheel_tilt" value="3" summary="a physical wheel tilt" since="6" />
///     </enum>
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct WlPointerAxisSource(pub u32);
impl WlPointerAxisSource {
    /// a physical wheel rotation; XML since=1.
    /// ```xml
    /// <entry name="wheel" value="0" summary="a physical wheel rotation" />
    /// ```
    pub const WHEEL: Self = Self(0);
    /// finger on a touch surface; XML since=1.
    /// ```xml
    /// <entry name="finger" value="1" summary="finger on a touch surface" />
    /// ```
    pub const FINGER: Self = Self(1);
    /// continuous coordinate space; XML since=1.
    /// ```xml
    /// <entry name="continuous" value="2" summary="continuous coordinate space" />
    /// ```
    pub const CONTINUOUS: Self = Self(2);
    /// a physical wheel tilt; XML since=6.
    /// ```xml
    /// <entry name="wheel_tilt" value="3" summary="a physical wheel tilt" since="6" />
    /// ```
    pub const WHEEL_TILT: Self = Self(3);
}
impl CommandReader<Self> for WlPointerAxisSource {
    async fn read(
        _object_id: u32,
        _opcode: u16,
        reader: &mut FdPayloadReader<'_>,
        _state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        Ok(Self(reader.next_u32().await?))
    }
}
impl CommandWriter for WlPointerAxisSource {
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
