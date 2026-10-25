use crate::protocol::{ClientState, CommandReader, CommandWriter};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Wire value `zwp_tablet_pad_strip_v2.source`.
///
/// Unknown numeric values and flag bits are preserved.
/// ```xml
/// <enum name="source">
///       <description summary="strip axis source">
///     Describes the source types for strip events. This indicates to the
///     client how a strip event was physically generated; a client may
///     adjust the user interface accordingly. For example, events
///     from a "finger" source may trigger kinetic scrolling.
///       </description>
///       <entry name="finger" value="1" summary="finger" />
///     </enum>
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct ZwpTabletPadStripV2Source(pub u32);
impl ZwpTabletPadStripV2Source {
    /// finger; available since version 1.
    pub const FINGER: Self = Self(1);
}
impl CommandReader<Self> for ZwpTabletPadStripV2Source {
    async fn read(
        _object_id: u32,
        _opcode: u16,
        reader: &mut FdPayloadReader<'_>,
        _state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        Ok(Self(reader.next_u32().await?))
    }
}
impl CommandWriter for ZwpTabletPadStripV2Source {
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
