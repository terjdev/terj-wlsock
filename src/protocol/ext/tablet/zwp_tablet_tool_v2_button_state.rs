use crate::protocol::{ClientState, CommandReader, CommandWriter};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Wire value `zwp_tablet_tool_v2.button_state`.
///
/// Unknown numeric values and flag bits are preserved.
/// ```xml
/// <enum name="button_state">
///       <description summary="physical button state">
///     Describes the physical state of a button that produced the button event.
///       </description>
///       <entry name="released" value="0" summary="button is not pressed" />
///       <entry name="pressed" value="1" summary="button is pressed" />
///     </enum>
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct ZwpTabletToolV2ButtonState(pub u32);
impl ZwpTabletToolV2ButtonState {
    /// button is not pressed; available since version 1.
    pub const RELEASED: Self = Self(0);
    /// button is pressed; available since version 1.
    pub const PRESSED: Self = Self(1);
}
impl CommandReader<Self> for ZwpTabletToolV2ButtonState {
    async fn read(
        _object_id: u32,
        _opcode: u16,
        reader: &mut FdPayloadReader<'_>,
        _state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        Ok(Self(reader.next_u32().await?))
    }
}
impl CommandWriter for ZwpTabletToolV2ButtonState {
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
