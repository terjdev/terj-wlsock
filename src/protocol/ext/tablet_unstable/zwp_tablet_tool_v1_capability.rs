use crate::protocol::{ClientState, CommandReader, CommandWriter};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Wire value `zwp_tablet_tool_v1.capability`.
///
/// Unknown numeric values are preserved.
/// ```xml
/// <enum name="capability">
///       <description summary="capability flags for a tool">
///     Describes extra capabilities on a tablet.
///
///     Any tool must provide x and y values, extra axes are
///     device-specific.
///       </description>
///       <entry name="tilt" value="1" summary="Tilt axes" />
///       <entry name="pressure" value="2" summary="Pressure axis" />
///       <entry name="distance" value="3" summary="Distance axis" />
///       <entry name="rotation" value="4" summary="Z-rotation axis" />
///       <entry name="slider" value="5" summary="Slider axis" />
///       <entry name="wheel" value="6" summary="Wheel axis" />
///     </enum>
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct ZwpTabletToolV1Capability(pub u32);
impl ZwpTabletToolV1Capability {
    /// Tilt axes; available since version 1.
    pub const TILT: Self = Self(1);
    /// Pressure axis; available since version 1.
    pub const PRESSURE: Self = Self(2);
    /// Distance axis; available since version 1.
    pub const DISTANCE: Self = Self(3);
    /// Z-rotation axis; available since version 1.
    pub const ROTATION: Self = Self(4);
    /// Slider axis; available since version 1.
    pub const SLIDER: Self = Self(5);
    /// Wheel axis; available since version 1.
    pub const WHEEL: Self = Self(6);
}
impl CommandReader<Self> for ZwpTabletToolV1Capability {
    async fn read(
        _object_id: u32,
        _opcode: u16,
        reader: &mut FdPayloadReader<'_>,
        _state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        Ok(Self(reader.next_u32().await?))
    }
}
impl CommandWriter for ZwpTabletToolV1Capability {
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
