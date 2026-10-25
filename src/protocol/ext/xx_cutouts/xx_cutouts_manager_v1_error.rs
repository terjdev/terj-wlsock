use crate::protocol::{ClientState, CommandReader, CommandWriter};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Wire value `xx_cutouts_manager_v1.error`.
///
/// Unknown numeric values are preserved.
/// ```xml
/// <enum name="error">
///       <entry name="invalid_role" value="0" summary="given wl_surface has incorrect role" />
///       <entry name="defunct_cutouts_object" value="1" summary="wl_surface or surface role was destroyed before the cutouts object" />
///     </enum>
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct XxCutoutsManagerV1Error(pub u32);
impl XxCutoutsManagerV1Error {
    /// given wl_surface has incorrect role; available since version 1.
    pub const INVALID_ROLE: Self = Self(0);
    /// wl_surface or surface role was destroyed before the cutouts object; available since version 1.
    pub const DEFUNCT_CUTOUTS_OBJECT: Self = Self(1);
}
impl CommandReader<Self> for XxCutoutsManagerV1Error {
    async fn read(
        _object_id: u32,
        _opcode: u16,
        reader: &mut FdPayloadReader<'_>,
        _state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        Ok(Self(reader.next_u32().await?))
    }
}
impl CommandWriter for XxCutoutsManagerV1Error {
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
