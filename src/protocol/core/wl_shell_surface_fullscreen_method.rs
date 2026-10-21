use crate::protocol::ClientState;
use crate::protocol::{CommandReader, CommandWriter};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Core-protocol value `wl_shell_surface.fullscreen_method`.
///
/// The numeric representation preserves unknown values without losing information.
/// Reading and writing encode four bytes; this is an argument value,
/// not a standalone message: object_id and opcode are both zero.
/// ```xml
/// <enum name="fullscreen_method">
///       <description summary="different method to set the surface fullscreen">
///         Hints to indicate to the compositor how to deal with a conflict
///         between the dimensions of the surface and the dimensions of the
///         output. The compositor is free to ignore this parameter.
///       </description>
///       <entry name="default" value="0" summary="no preference, apply default policy" />
///       <entry name="scale" value="1" summary="scale, preserve the surface's aspect ratio and center on output" />
///       <entry name="driver" value="2" summary="switch output mode to the smallest mode that can fit the surface, add black borders to compensate size mismatch" />
///       <entry name="fill" value="3" summary="no upscaling, center on output and add black borders to compensate size mismatch" />
///     </enum>
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct WlShellSurfaceFullscreenMethod(pub u32);
impl WlShellSurfaceFullscreenMethod {
    /// no preference, apply default policy; XML since=1.
    /// ```xml
    /// <entry name="default" value="0" summary="no preference, apply default policy" />
    /// ```
    pub const DEFAULT: Self = Self(0);
    /// scale, preserve the surface's aspect ratio and center on output; XML since=1.
    /// ```xml
    /// <entry name="scale" value="1" summary="scale, preserve the surface's aspect ratio and center on output" />
    /// ```
    pub const SCALE: Self = Self(1);
    /// switch output mode to the smallest mode that can fit the surface, add black borders to compensate size mismatch; XML since=1.
    /// ```xml
    /// <entry name="driver" value="2" summary="switch output mode to the smallest mode that can fit the surface, add black borders to compensate size mismatch" />
    /// ```
    pub const DRIVER: Self = Self(2);
    /// no upscaling, center on output and add black borders to compensate size mismatch; XML since=1.
    /// ```xml
    /// <entry name="fill" value="3" summary="no upscaling, center on output and add black borders to compensate size mismatch" />
    /// ```
    pub const FILL: Self = Self(3);
}
impl CommandReader<Self> for WlShellSurfaceFullscreenMethod {
    async fn read(
        _object_id: u32,
        _opcode: u16,
        reader: &mut FdPayloadReader<'_>,
        _state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        Ok(Self(reader.next_u32().await?))
    }
}
impl CommandWriter for WlShellSurfaceFullscreenMethod {
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
