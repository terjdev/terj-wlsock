use crate::protocol::ClientState;
use crate::protocol::{CommandReader, CommandWriter};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Core-protocol value `wl_pointer.button_state`.
///
/// The numeric representation preserves unknown values without losing information.
/// Reading and writing encode four bytes; this is an argument value,
/// not a standalone message: object_id and opcode are both zero.
/// ```xml
/// <enum name="button_state">
///       <description summary="physical button state">
///         Describes the physical state of a button that produced the button
///         event.
///       </description>
///       <entry name="released" value="0" summary="the button is not pressed" />
///       <entry name="pressed" value="1" summary="the button is pressed" />
///     </enum>
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct WlPointerButtonState(pub u32);
impl WlPointerButtonState {
    /// the button is not pressed; XML since=1.
    /// ```xml
    /// <entry name="released" value="0" summary="the button is not pressed" />
    /// ```
    pub const RELEASED: Self = Self(0);
    /// the button is pressed; XML since=1.
    /// ```xml
    /// <entry name="pressed" value="1" summary="the button is pressed" />
    /// ```
    pub const PRESSED: Self = Self(1);
}
impl CommandReader<Self> for WlPointerButtonState {
    async fn read(
        _object_id: u32,
        _opcode: u16,
        reader: &mut FdPayloadReader<'_>,
        _state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        Ok(Self(reader.next_u32().await?))
    }
}
impl CommandWriter for WlPointerButtonState {
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
