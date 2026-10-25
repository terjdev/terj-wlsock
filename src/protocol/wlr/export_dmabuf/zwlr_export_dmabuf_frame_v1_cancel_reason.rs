use crate::protocol::{ClientState, CommandReader, CommandWriter};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Wire value `zwlr_export_dmabuf_frame_v1.cancel_reason`.
///
/// Unknown numeric values are preserved.
/// ```xml
/// <enum name="cancel_reason">
///       <description summary="cancel reason">
///         Indicates reason for cancelling the frame.
///       </description>
///       <entry name="temporary" value="0" summary="temporary error, source will produce more frames" />
///       <entry name="permanent" value="1" summary="fatal error, source will not produce frames" />
///       <entry name="resizing" value="2" summary="temporary error, source will produce more frames" />
///     </enum>
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct ZwlrExportDmabufFrameV1CancelReason(pub u32);
impl ZwlrExportDmabufFrameV1CancelReason {
    /// temporary error, source will produce more frames; available since version 1.
    pub const TEMPORARY: Self = Self(0);
    /// fatal error, source will not produce frames; available since version 1.
    pub const PERMANENT: Self = Self(1);
    /// temporary error, source will produce more frames; available since version 1.
    pub const RESIZING: Self = Self(2);
}
impl CommandReader<Self> for ZwlrExportDmabufFrameV1CancelReason {
    async fn read(
        _object_id: u32,
        _opcode: u16,
        reader: &mut FdPayloadReader<'_>,
        _state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        Ok(Self(reader.next_u32().await?))
    }
}
impl CommandWriter for ZwlrExportDmabufFrameV1CancelReason {
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
