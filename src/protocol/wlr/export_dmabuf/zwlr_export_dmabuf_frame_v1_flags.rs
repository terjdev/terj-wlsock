use crate::protocol::{ClientState, CommandReader, CommandWriter};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Wire value `zwlr_export_dmabuf_frame_v1.flags`.
///
/// Unknown numeric values are preserved.
/// ```xml
/// <enum name="flags">
///       <description summary="frame flags">
///         Special flags that should be respected by the client.
///       </description>
///       <entry name="transient" value="0x1" summary="clients should copy frame before processing" />
///     </enum>
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct ZwlrExportDmabufFrameV1Flags(pub u32);
impl ZwlrExportDmabufFrameV1Flags {
    /// clients should copy frame before processing; available since version 1.
    pub const TRANSIENT: Self = Self(0x1);
}
impl CommandReader<Self> for ZwlrExportDmabufFrameV1Flags {
    async fn read(
        _object_id: u32,
        _opcode: u16,
        reader: &mut FdPayloadReader<'_>,
        _state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        Ok(Self(reader.next_u32().await?))
    }
}
impl CommandWriter for ZwlrExportDmabufFrameV1Flags {
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
