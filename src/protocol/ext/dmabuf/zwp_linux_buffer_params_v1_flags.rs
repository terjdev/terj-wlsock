use crate::protocol::{ClientState, CommandReader, CommandWriter};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Wire value `zwp_linux_buffer_params_v1.flags`.
///
/// Unknown numeric values and flag bits are preserved.
/// ```xml
/// <enum name="flags" bitfield="true">
///       <entry name="y_invert" value="1" summary="contents are y-inverted" />
///       <entry name="interlaced" value="2" summary="content is interlaced" />
///       <entry name="bottom_first" value="4" summary="bottom field first" />
///     </enum>
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct ZwpLinuxBufferParamsV1Flags(pub u32);
impl ZwpLinuxBufferParamsV1Flags {
    /// contents are y-inverted; available since version 1.
    pub const Y_INVERT: Self = Self(1);
    /// content is interlaced; available since version 1.
    pub const INTERLACED: Self = Self(2);
    /// bottom field first; available since version 1.
    pub const BOTTOM_FIRST: Self = Self(4);
}
impl std::ops::BitOr for ZwpLinuxBufferParamsV1Flags {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl std::ops::BitAnd for ZwpLinuxBufferParamsV1Flags {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl CommandReader<Self> for ZwpLinuxBufferParamsV1Flags {
    async fn read(
        _object_id: u32,
        _opcode: u16,
        reader: &mut FdPayloadReader<'_>,
        _state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        Ok(Self(reader.next_u32().await?))
    }
}
impl CommandWriter for ZwpLinuxBufferParamsV1Flags {
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
