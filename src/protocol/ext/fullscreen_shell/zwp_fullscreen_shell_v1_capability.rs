use crate::protocol::{ClientState, CommandReader, CommandWriter};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Wire value `zwp_fullscreen_shell_v1.capability`.
///
/// Unknown numeric values are preserved.
/// ```xml
/// <enum name="capability">
///       <description summary="capabilities advertised by the compositor">
///     Various capabilities that can be advertised by the compositor.  They
///     are advertised one-at-a-time when the wl_fullscreen_shell interface is
///     bound.  See the wl_fullscreen_shell.capability event for more details.
///
///     ARBITRARY_MODES:
///     This is a hint to the client that indicates that the compositor is
///     capable of setting practically any mode on its outputs.  If this
///     capability is provided, wl_fullscreen_shell.present_surface_for_mode
///     will almost never fail and clients should feel free to set whatever
///     mode they like.  If the compositor does not advertise this, it may
///     still support some modes that are not advertised through wl_global.mode
///     but it is less likely.
///
///     CURSOR_PLANE:
///     This is a hint to the client that indicates that the compositor can
///     handle a cursor surface from the client without actually compositing.
///     This may be because of a hardware cursor plane or some other mechanism.
///     If the compositor does not advertise this capability then setting
///     wl_pointer.cursor may degrade performance or be ignored entirely.  If
///     CURSOR_PLANE is not advertised, it is recommended that the client draw
///     its own cursor and set wl_pointer.cursor(NULL).
///       </description>
///       <entry name="arbitrary_modes" value="1" summary="compositor is capable of almost any output mode" />
///       <entry name="cursor_plane" value="2" summary="compositor has a separate cursor plane" />
///     </enum>
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct ZwpFullscreenShellV1Capability(pub u32);
impl ZwpFullscreenShellV1Capability {
    /// compositor is capable of almost any output mode; available since version 1.
    pub const ARBITRARY_MODES: Self = Self(1);
    /// compositor has a separate cursor plane; available since version 1.
    pub const CURSOR_PLANE: Self = Self(2);
}
impl CommandReader<Self> for ZwpFullscreenShellV1Capability {
    async fn read(
        _object_id: u32,
        _opcode: u16,
        reader: &mut FdPayloadReader<'_>,
        _state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        Ok(Self(reader.next_u32().await?))
    }
}
impl CommandWriter for ZwpFullscreenShellV1Capability {
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
