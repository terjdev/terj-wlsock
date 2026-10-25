use crate::protocol::{ClientState, CommandReader, CommandWriter};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Wire value `zwlr_output_power_v1.mode`.
///
/// Unknown numeric values are preserved.
/// ```xml
/// <enum name="mode">
///       <entry name="off" value="0" summary="Output is turned off." />
///       <entry name="on" value="1" summary="Output is turned on, no power saving" />
///     </enum>
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct ZwlrOutputPowerV1Mode(pub u32);
impl ZwlrOutputPowerV1Mode {
    /// Output is turned off; available since version 1.
    pub const OFF: Self = Self(0);
    /// Output is turned on, no power saving; available since version 1.
    pub const ON: Self = Self(1);
}
impl CommandReader<Self> for ZwlrOutputPowerV1Mode {
    async fn read(
        _object_id: u32,
        _opcode: u16,
        reader: &mut FdPayloadReader<'_>,
        _state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        Ok(Self(reader.next_u32().await?))
    }
}
impl CommandWriter for ZwlrOutputPowerV1Mode {
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
