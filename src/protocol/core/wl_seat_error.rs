use crate::protocol::ClientState;
use crate::protocol::{CommandReader, CommandWriter};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Core-protocol value `wl_seat.error`.
///
/// The numeric representation preserves unknown values without losing information.
/// Reading and writing encode four bytes; this is an argument value,
/// not a standalone message: object_id and opcode are both zero.
/// ```xml
/// <enum name="error">
///       <description summary="wl_seat error values">
///         These errors can be emitted in response to wl_seat requests.
///       </description>
///       <entry name="missing_capability" value="0" summary="get_pointer, get_keyboard or get_touch called on seat without the matching capability" />
///     </enum>
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct WlSeatError(pub u32);
impl WlSeatError {
    /// get_pointer, get_keyboard or get_touch called on seat without the matching capability; XML since=1.
    /// ```xml
    /// <entry name="missing_capability" value="0" summary="get_pointer, get_keyboard or get_touch called on seat without the matching capability" />
    /// ```
    pub const MISSING_CAPABILITY: Self = Self(0);
}
impl CommandReader<Self> for WlSeatError {
    async fn read(
        _object_id: u32,
        _opcode: u16,
        reader: &mut FdPayloadReader<'_>,
        _state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        Ok(Self(reader.next_u32().await?))
    }
}
impl CommandWriter for WlSeatError {
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
