use crate::protocol::ClientState;
use crate::protocol::{CommandReader, CommandWriter};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Core-protocol value `wl_seat.capability`.
///
/// The numeric representation preserves unknown values without losing information.
/// The bitmask supports combining values with `|`.
/// Reading and writing encode four bytes; this is an argument value,
/// not a standalone message: object_id and opcode are both zero.
/// ```xml
/// <enum name="capability" bitfield="true">
///       <description summary="seat capability bitmask">
///         This is a bitmask of capabilities this seat has; if a member is
///         set, then it is present on the seat.
///       </description>
///       <entry name="pointer" value="1" summary="the seat has pointer devices" />
///       <entry name="keyboard" value="2" summary="the seat has one or more keyboards" />
///       <entry name="touch" value="4" summary="the seat has touch devices" />
///     </enum>
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct WlSeatCapability(pub u32);
impl WlSeatCapability {
    /// the seat has pointer devices; XML since=1.
    /// ```xml
    /// <entry name="pointer" value="1" summary="the seat has pointer devices" />
    /// ```
    pub const POINTER: Self = Self(1);
    /// the seat has one or more keyboards; XML since=1.
    /// ```xml
    /// <entry name="keyboard" value="2" summary="the seat has one or more keyboards" />
    /// ```
    pub const KEYBOARD: Self = Self(2);
    /// the seat has touch devices; XML since=1.
    /// ```xml
    /// <entry name="touch" value="4" summary="the seat has touch devices" />
    /// ```
    pub const TOUCH: Self = Self(4);
}
impl std::ops::BitOr for WlSeatCapability {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl std::ops::BitAnd for WlSeatCapability {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl CommandReader<Self> for WlSeatCapability {
    async fn read(
        _object_id: u32,
        _opcode: u16,
        reader: &mut FdPayloadReader<'_>,
        _state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        Ok(Self(reader.next_u32().await?))
    }
}
impl CommandWriter for WlSeatCapability {
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
