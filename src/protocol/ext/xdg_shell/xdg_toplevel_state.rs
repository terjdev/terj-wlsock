use crate::protocol::{ClientState, CommandReader, CommandWriter};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Wire value `xdg_toplevel.state`.
///
/// Unknown numeric values and flag bits are preserved.
/// ```xml
/// <enum name="state">
///       <description summary="types of state on the surface">
///     The different state values used on the surface. This is designed for
///     state values like maximized, fullscreen. It is paired with the
///     configure event to ensure that both the client and the compositor
///     setting the state can be synchronized.
///
///     States set in this way are double-buffered, see wl_surface.commit.
///       </description>
///       <entry name="maximized" value="1" summary="the surface is maximized">
///     <description summary="the surface is maximized">
///       The surface is maximized. The window geometry specified in the configure
///       event must be obeyed by the client, or the xdg_wm_base.invalid_surface_state
///       error is raised.
///
///       The client should draw without shadow or other
///       decoration outside of the window geometry.
///     </description>
///       </entry>
///       <entry name="fullscreen" value="2" summary="the surface is fullscreen">
///     <description summary="the surface is fullscreen">
///       The surface is fullscreen. The window geometry specified in the
///       configure event is a maximum; the client cannot resize beyond it. For
///       a surface to cover the whole fullscreened area, the geometry
///       dimensions must be obeyed by the client. For more details, see
///       xdg_toplevel.set_fullscreen.
///     </description>
///       </entry>
///       <entry name="resizing" value="3" summary="the surface is being resized">
///     <description summary="the surface is being resized">
///       The surface is being resized. The window geometry specified in the
///       configure event is a maximum; the client cannot resize beyond it.
///       Clients that have aspect ratio or cell sizing configuration can use
///       a smaller size, however.
///     </description>
///       </entry>
///       <entry name="activated" value="4" summary="the surface is now activated">
///     <description summary="the surface is now activated">
///       Client window decorations should be painted as if the window is
///       active. Do not assume this means that the window actually has
///       keyboard or pointer focus.
///     </description>
///       </entry>
///       <entry name="tiled_left" value="5" since="2">
///     <description summary="the surface’s left edge is tiled">
///       The window is currently in a tiled layout and the left edge is
///       considered to be adjacent to another part of the tiling grid.
///
///       The client should draw without shadow or other decoration outside of
///       the window geometry on the left edge.
///     </description>
///       </entry>
///       <entry name="tiled_right" value="6" since="2">
///     <description summary="the surface’s right edge is tiled">
///       The window is currently in a tiled layout and the right edge is
///       considered to be adjacent to another part of the tiling grid.
///
///       The client should draw without shadow or other decoration outside of
///       the window geometry on the right edge.
///     </description>
///       </entry>
///       <entry name="tiled_top" value="7" since="2">
///     <description summary="the surface’s top edge is tiled">
///       The window is currently in a tiled layout and the top edge is
///       considered to be adjacent to another part of the tiling grid.
///
///       The client should draw without shadow or other decoration outside of
///       the window geometry on the top edge.
///     </description>
///       </entry>
///       <entry name="tiled_bottom" value="8" since="2">
///     <description summary="the surface’s bottom edge is tiled">
///       The window is currently in a tiled layout and the bottom edge is
///       considered to be adjacent to another part of the tiling grid.
///
///       The client should draw without shadow or other decoration outside of
///       the window geometry on the bottom edge.
///     </description>
///       </entry>
///       <entry name="suspended" value="9" since="6">
///         <description summary="surface repaint is suspended">
///       The surface is currently not ordinarily being repainted; for
///       example because its content is occluded by another window, or its
///       outputs are switched off due to screen locking.
///     </description>
///       </entry>
///       <entry name="constrained_left" value="10" since="7">
///     <description summary="the surface’s left edge is constrained">
///           The left edge of the window is currently constrained, meaning it
///           shouldn't attempt to resize from that edge. It can for example mean
///           it's tiled next to a monitor edge on the constrained side of the
///           window.
///     </description>
///       </entry>
///       <entry name="constrained_right" value="11" since="7">
///     <description summary="the surface’s right edge is constrained">
///           The right edge of the window is currently constrained, meaning it
///           shouldn't attempt to resize from that edge. It can for example mean
///           it's tiled next to a monitor edge on the constrained side of the
///           window.
///     </description>
///       </entry>
///       <entry name="constrained_top" value="12" since="7">
///     <description summary="the surface’s top edge is constrained">
///           The top edge of the window is currently constrained, meaning it
///           shouldn't attempt to resize from that edge. It can for example mean
///           it's tiled next to a monitor edge on the constrained side of the
///           window.
///     </description>
///       </entry>
///       <entry name="constrained_bottom" value="13" since="7">
///     <description summary="the surface’s bottom edge is constrained">
///           The bottom edge of the window is currently constrained, meaning it
///           shouldn't attempt to resize from that edge. It can for example mean
///           it's tiled next to a monitor edge on the constrained side of the
///           window.
///     </description>
///       </entry>
///     </enum>
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct XdgToplevelState(pub u32);
impl XdgToplevelState {
    /// the surface is maximized; available since version 1.
    pub const MAXIMIZED: Self = Self(1);
    /// the surface is fullscreen; available since version 1.
    pub const FULLSCREEN: Self = Self(2);
    /// the surface is being resized; available since version 1.
    pub const RESIZING: Self = Self(3);
    /// the surface is now activated; available since version 1.
    pub const ACTIVATED: Self = Self(4);
    /// the surface’s left edge is tiled; available since version 2.
    pub const TILED_LEFT: Self = Self(5);
    /// the surface’s right edge is tiled; available since version 2.
    pub const TILED_RIGHT: Self = Self(6);
    /// the surface’s top edge is tiled; available since version 2.
    pub const TILED_TOP: Self = Self(7);
    /// the surface’s bottom edge is tiled; available since version 2.
    pub const TILED_BOTTOM: Self = Self(8);
    /// surface repaint is suspended; available since version 6.
    pub const SUSPENDED: Self = Self(9);
    /// the surface’s left edge is constrained; available since version 7.
    pub const CONSTRAINED_LEFT: Self = Self(10);
    /// the surface’s right edge is constrained; available since version 7.
    pub const CONSTRAINED_RIGHT: Self = Self(11);
    /// the surface’s top edge is constrained; available since version 7.
    pub const CONSTRAINED_TOP: Self = Self(12);
    /// the surface’s bottom edge is constrained; available since version 7.
    pub const CONSTRAINED_BOTTOM: Self = Self(13);
}
impl CommandReader<Self> for XdgToplevelState {
    async fn read(
        _object_id: u32,
        _opcode: u16,
        reader: &mut FdPayloadReader<'_>,
        _state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        Ok(Self(reader.next_u32().await?))
    }
}
impl CommandWriter for XdgToplevelState {
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
