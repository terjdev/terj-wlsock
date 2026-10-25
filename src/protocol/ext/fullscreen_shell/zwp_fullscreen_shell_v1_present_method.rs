use crate::protocol::{ClientState, CommandReader, CommandWriter};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Wire value `zwp_fullscreen_shell_v1.present_method`.
///
/// Unknown numeric values are preserved.
/// ```xml
/// <enum name="present_method">
///       <description summary="different method to set the surface fullscreen">
///     Hints to indicate to the compositor how to deal with a conflict
///     between the dimensions of the surface and the dimensions of the
///     output. The compositor is free to ignore this parameter.
///       </description>
///       <entry name="default" value="0" summary="no preference, apply default policy" />
///       <entry name="center" value="1" summary="center the surface on the output" />
///       <entry name="zoom" value="2" summary="scale the surface, preserving aspect ratio, to the largest size that will fit on the output" />
///       <entry name="zoom_crop" value="3" summary="scale the surface, preserving aspect ratio, to fully fill the output cropping if needed" />
///       <entry name="stretch" value="4" summary="scale the surface to the size of the output ignoring aspect ratio" />
///     </enum>
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct ZwpFullscreenShellV1PresentMethod(pub u32);
impl ZwpFullscreenShellV1PresentMethod {
    /// no preference, apply default policy; available since version 1.
    pub const DEFAULT: Self = Self(0);
    /// center the surface on the output; available since version 1.
    pub const CENTER: Self = Self(1);
    /// scale the surface, preserving aspect ratio, to the largest size that will fit on the output; available since version 1.
    pub const ZOOM: Self = Self(2);
    /// scale the surface, preserving aspect ratio, to fully fill the output cropping if needed; available since version 1.
    pub const ZOOM_CROP: Self = Self(3);
    /// scale the surface to the size of the output ignoring aspect ratio; available since version 1.
    pub const STRETCH: Self = Self(4);
}
impl CommandReader<Self> for ZwpFullscreenShellV1PresentMethod {
    async fn read(
        _object_id: u32,
        _opcode: u16,
        reader: &mut FdPayloadReader<'_>,
        _state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        Ok(Self(reader.next_u32().await?))
    }
}
impl CommandWriter for ZwpFullscreenShellV1PresentMethod {
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
