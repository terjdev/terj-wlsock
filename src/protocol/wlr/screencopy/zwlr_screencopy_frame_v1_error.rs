use crate::protocol::{ClientState, CommandReader, CommandWriter};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Wire value `zwlr_screencopy_frame_v1.error`.
///
/// Unknown numeric values and flag bits are preserved.
/// ```xml
/// <enum name="error">
///       <entry name="already_used" value="0" summary="the object has already been used to copy a wl_buffer" />
///       <entry name="invalid_buffer" value="1" summary="buffer attributes are invalid" />
///     </enum>
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct ZwlrScreencopyFrameV1Error(pub u32);
impl ZwlrScreencopyFrameV1Error {
    /// the object has already been used to copy a wl_buffer; available since version 1.
    pub const ALREADY_USED: Self = Self(0);
    /// buffer attributes are invalid; available since version 1.
    pub const INVALID_BUFFER: Self = Self(1);
}
impl CommandReader<Self> for ZwlrScreencopyFrameV1Error {
    async fn read(
        _object_id: u32,
        _opcode: u16,
        reader: &mut FdPayloadReader<'_>,
        _state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        Ok(Self(reader.next_u32().await?))
    }
}
impl CommandWriter for ZwlrScreencopyFrameV1Error {
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
