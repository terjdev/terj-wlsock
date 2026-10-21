use crate::protocol::ClientState;
use crate::protocol::{CommandReader, CommandWriter};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Core-protocol value `wl_output.subpixel`.
///
/// The numeric representation preserves unknown values without losing information.
/// Reading and writing encode four bytes; this is an argument value,
/// not a standalone message: object_id and opcode are both zero.
/// ```xml
/// <enum name="subpixel">
///       <description summary="subpixel geometry information">
///         This enumeration describes how the physical
///         pixels on an output are laid out.
///       </description>
///       <entry name="unknown" value="0" summary="unknown geometry" />
///       <entry name="none" value="1" summary="no geometry" />
///       <entry name="horizontal_rgb" value="2" summary="horizontal RGB" />
///       <entry name="horizontal_bgr" value="3" summary="horizontal BGR" />
///       <entry name="vertical_rgb" value="4" summary="vertical RGB" />
///       <entry name="vertical_bgr" value="5" summary="vertical BGR" />
///     </enum>
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct WlOutputSubpixel(pub u32);
impl WlOutputSubpixel {
    /// unknown geometry; XML since=1.
    /// ```xml
    /// <entry name="unknown" value="0" summary="unknown geometry" />
    /// ```
    pub const UNKNOWN: Self = Self(0);
    /// no geometry; XML since=1.
    /// ```xml
    /// <entry name="none" value="1" summary="no geometry" />
    /// ```
    pub const NONE: Self = Self(1);
    /// horizontal RGB; XML since=1.
    /// ```xml
    /// <entry name="horizontal_rgb" value="2" summary="horizontal RGB" />
    /// ```
    pub const HORIZONTAL_RGB: Self = Self(2);
    /// horizontal BGR; XML since=1.
    /// ```xml
    /// <entry name="horizontal_bgr" value="3" summary="horizontal BGR" />
    /// ```
    pub const HORIZONTAL_BGR: Self = Self(3);
    /// vertical RGB; XML since=1.
    /// ```xml
    /// <entry name="vertical_rgb" value="4" summary="vertical RGB" />
    /// ```
    pub const VERTICAL_RGB: Self = Self(4);
    /// vertical BGR; XML since=1.
    /// ```xml
    /// <entry name="vertical_bgr" value="5" summary="vertical BGR" />
    /// ```
    pub const VERTICAL_BGR: Self = Self(5);
}
impl CommandReader<Self> for WlOutputSubpixel {
    async fn read(
        _object_id: u32,
        _opcode: u16,
        reader: &mut FdPayloadReader<'_>,
        _state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        Ok(Self(reader.next_u32().await?))
    }
}
impl CommandWriter for WlOutputSubpixel {
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
