use crate::protocol::{ClientState, CommandReader, CommandWriter};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Wire value `zwp_fullscreen_shell_v1.error`.
///
/// Unknown numeric values are preserved.
/// ```xml
/// <enum name="error">
///       <description summary="wl_fullscreen_shell error values">
///     These errors can be emitted in response to wl_fullscreen_shell requests.
///       </description>
///       <entry name="invalid_method" value="0" summary="present_method is not known" />
///       <entry name="role" value="1" summary="given wl_surface has another role" />
///     </enum>
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct ZwpFullscreenShellV1Error(pub u32);
impl ZwpFullscreenShellV1Error {
    /// present_method is not known; available since version 1.
    pub const INVALID_METHOD: Self = Self(0);
    /// given wl_surface has another role; available since version 1.
    pub const ROLE: Self = Self(1);
}
impl CommandReader<Self> for ZwpFullscreenShellV1Error {
    async fn read(
        _object_id: u32,
        _opcode: u16,
        reader: &mut FdPayloadReader<'_>,
        _state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        Ok(Self(reader.next_u32().await?))
    }
}
impl CommandWriter for ZwpFullscreenShellV1Error {
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
