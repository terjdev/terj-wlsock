use crate::protocol::{ClientState, CommandReader, CommandWriter};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Wire value `zwlr_virtual_pointer_v1.error`.
///
/// Unknown numeric values and flag bits are preserved.
/// ```xml
/// <enum name="error">
///       <entry name="invalid_axis" value="0" summary="client sent invalid axis enumeration value" />
///       <entry name="invalid_axis_source" value="1" summary="client sent invalid axis source enumeration value" />
///     </enum>
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct ZwlrVirtualPointerV1Error(pub u32);
impl ZwlrVirtualPointerV1Error {
    /// client sent invalid axis enumeration value; available since version 1.
    pub const INVALID_AXIS: Self = Self(0);
    /// client sent invalid axis source enumeration value; available since version 1.
    pub const INVALID_AXIS_SOURCE: Self = Self(1);
}
impl CommandReader<Self> for ZwlrVirtualPointerV1Error {
    async fn read(
        _object_id: u32,
        _opcode: u16,
        reader: &mut FdPayloadReader<'_>,
        _state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        Ok(Self(reader.next_u32().await?))
    }
}
impl CommandWriter for ZwlrVirtualPointerV1Error {
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
