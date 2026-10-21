use crate::protocol::ClientState;
use crate::protocol::{CommandReader, CommandWriter};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Core-protocol value `wl_output.mode`.
///
/// The numeric representation preserves unknown values without losing information.
/// The bitmask supports combining values with `|`.
/// Reading and writing encode four bytes; this is an argument value,
/// not a standalone message: object_id and opcode are both zero.
/// ```xml
/// <enum name="mode" bitfield="true">
///       <description summary="mode information">
///         These flags describe properties of an output mode.
///         They are used in the flags bitfield of the mode event.
///       </description>
///       <entry name="current" value="0x1" summary="indicates this is the current mode" />
///       <entry name="preferred" value="0x2" summary="indicates this is the preferred mode" />
///     </enum>
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct WlOutputMode(pub u32);
impl WlOutputMode {
    /// indicates this is the current mode; XML since=1.
    /// ```xml
    /// <entry name="current" value="0x1" summary="indicates this is the current mode" />
    /// ```
    pub const CURRENT: Self = Self(0x1);
    /// indicates this is the preferred mode; XML since=1.
    /// ```xml
    /// <entry name="preferred" value="0x2" summary="indicates this is the preferred mode" />
    /// ```
    pub const PREFERRED: Self = Self(0x2);
}
impl std::ops::BitOr for WlOutputMode {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl std::ops::BitAnd for WlOutputMode {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl CommandReader<Self> for WlOutputMode {
    async fn read(
        _object_id: u32,
        _opcode: u16,
        reader: &mut FdPayloadReader<'_>,
        _state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        Ok(Self(reader.next_u32().await?))
    }
}
impl CommandWriter for WlOutputMode {
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
