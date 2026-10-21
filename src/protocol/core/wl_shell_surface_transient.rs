use crate::protocol::ClientState;
use crate::protocol::{CommandReader, CommandWriter};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Core-protocol value `wl_shell_surface.transient`.
///
/// The numeric representation preserves unknown values without losing information.
/// The bitmask supports combining values with `|`.
/// Reading and writing encode four bytes; this is an argument value,
/// not a standalone message: object_id and opcode are both zero.
/// ```xml
/// <enum name="transient" bitfield="true">
///       <description summary="details of transient behaviour">
///         These flags specify details of the expected behaviour
///         of transient surfaces. Used in the set_transient request.
///       </description>
///       <entry name="inactive" value="0x1" summary="do not set keyboard focus" />
///     </enum>
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct WlShellSurfaceTransient(pub u32);
impl WlShellSurfaceTransient {
    /// do not set keyboard focus; XML since=1.
    /// ```xml
    /// <entry name="inactive" value="0x1" summary="do not set keyboard focus" />
    /// ```
    pub const INACTIVE: Self = Self(0x1);
}
impl std::ops::BitOr for WlShellSurfaceTransient {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl std::ops::BitAnd for WlShellSurfaceTransient {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl CommandReader<Self> for WlShellSurfaceTransient {
    async fn read(
        _object_id: u32,
        _opcode: u16,
        reader: &mut FdPayloadReader<'_>,
        _state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        Ok(Self(reader.next_u32().await?))
    }
}
impl CommandWriter for WlShellSurfaceTransient {
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
