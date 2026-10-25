use crate::protocol::{ClientState, CommandReader, CommandWriter};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Wire value `zwp_tablet_tool_v2.type`.
///
/// Unknown numeric values and flag bits are preserved.
/// ```xml
/// <enum name="type">
///       <description summary="a physical tool type">
///     Describes the physical type of a tool. The physical type of a tool
///     generally defines its base usage.
///
///     The mouse tool represents a mouse-shaped tool that is not a relative
///     device but bound to the tablet's surface, providing absolute
///     coordinates.
///
///     The lens tool is a mouse-shaped tool with an attached lens to
///     provide precision focus.
///       </description>
///       <entry name="pen" value="0x140" summary="Pen" />
///       <entry name="eraser" value="0x141" summary="Eraser" />
///       <entry name="brush" value="0x142" summary="Brush" />
///       <entry name="pencil" value="0x143" summary="Pencil" />
///       <entry name="airbrush" value="0x144" summary="Airbrush" />
///       <entry name="finger" value="0x145" summary="Finger" />
///       <entry name="mouse" value="0x146" summary="Mouse" />
///       <entry name="lens" value="0x147" summary="Lens" />
///     </enum>
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct ZwpTabletToolV2Type(pub u32);
impl ZwpTabletToolV2Type {
    /// Pen; available since version 1.
    pub const PEN: Self = Self(0x140);
    /// Eraser; available since version 1.
    pub const ERASER: Self = Self(0x141);
    /// Brush; available since version 1.
    pub const BRUSH: Self = Self(0x142);
    /// Pencil; available since version 1.
    pub const PENCIL: Self = Self(0x143);
    /// Airbrush; available since version 1.
    pub const AIRBRUSH: Self = Self(0x144);
    /// Finger; available since version 1.
    pub const FINGER: Self = Self(0x145);
    /// Mouse; available since version 1.
    pub const MOUSE: Self = Self(0x146);
    /// Lens; available since version 1.
    pub const LENS: Self = Self(0x147);
}
impl CommandReader<Self> for ZwpTabletToolV2Type {
    async fn read(
        _object_id: u32,
        _opcode: u16,
        reader: &mut FdPayloadReader<'_>,
        _state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        Ok(Self(reader.next_u32().await?))
    }
}
impl CommandWriter for ZwpTabletToolV2Type {
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
