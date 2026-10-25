use crate::protocol::ext::xdg_shell::ObjectInteface as XdgShellObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, CommandWriterExt, InterfaceReader,
    ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Client request `xdg_surface.set_window_geometry`.
///
/// Opcode 3; available since version 1.
/// Wire framing and receiver metadata are checked; protocol semantics belong to the caller.
/// ```xml
/// <request name="set_window_geometry">
///       <description summary="set the new window geometry">
///     The window geometry of a surface is its "visible bounds" from the
///     user's perspective. Client-side decorations often have invisible
///     portions like drop-shadows which should be ignored for the
///     purposes of aligning, placing and constraining windows. Note that
///     in some situations, compositors may clip rendering to the window
///     geometry, so the client should avoid putting functional elements
///     outside of it.
///
///     The window geometry is double-buffered state, see wl_surface.commit.
///
///     When maintaining a position, the compositor should treat the (x, y)
///     coordinate of the window geometry as the top left corner of the window.
///     A client changing the (x, y) window geometry coordinate should in
///     general not alter the position of the window.
///
///     Once the window geometry of the surface is set, it is not possible to
///     unset it, and it will remain the same until set_window_geometry is
///     called again, even if a new subsurface or buffer is attached.
///
///     If never set, the value is the full bounds of the surface,
///     including any subsurfaces. This updates dynamically on every
///     commit. This unset is meant for extremely simple clients.
///
///     The arguments are given in the surface-local coordinate space of
///     the wl_surface associated with this xdg_surface, and may extend outside
///     of the wl_surface itself to mark parts of the subsurface tree as part of
///     the window geometry.
///
///     When applied, the effective window geometry will be the set window
///     geometry clamped to the bounding rectangle of the combined
///     geometry of the surface of the xdg_surface and the associated
///     subsurfaces.
///
///     The effective geometry will not be recalculated unless a new call to
///     set_window_geometry is done and the new pending surface state is
///     subsequently applied.
///
///     The width and height of the effective window geometry must be
///     greater than zero. Setting an invalid size will raise an
///     invalid_size error.
///       </description>
///       <arg name="x" type="int" summary="x coordinate of the top-left corner of the window inside this surface" />
///       <arg name="y" type="int" summary="y coordinate of the top-left corner of the window inside this surface" />
///       <arg name="width" type="int" summary="width of the window" />
///       <arg name="height" type="int" summary="height of the window" />
///     </request>
/// ```
#[derive(Debug)]
pub struct XdgSurfaceSetWindowGeometryRequest {
    /// Wire identifier of the receiver.
    pub object_id: u32,
    /// x coordinate of the top-left corner of the window inside this surface.
    pub x: i32,
    /// y coordinate of the top-left corner of the window inside this surface.
    pub y: i32,
    /// width of the window.
    pub width: i32,
    /// height of the window.
    pub height: i32,
}
impl XdgSurfaceSetWindowGeometryRequest {
    /// Opcode in the request namespace.
    pub const OPCODE: u16 = 3;
    /// Minimum supported interface version.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for XdgSurfaceSetWindowGeometryRequest {
    async fn read(
        object_id: u32,
        opcode: u16,
        reader: &mut FdPayloadReader<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        let interface = Self::reader_state(state)?
            .object_interface(object_id)
            .ok_or(FdReaderError::InvalidProtocol)?;
        Self::read_with_interface(object_id, opcode, interface, reader, state).await
    }
}
impl InterfaceReader for XdgSurfaceSetWindowGeometryRequest {
    async fn read_with_interface(
        object_id: u32,
        opcode: u16,
        interface: ObjectInteface,
        reader: &mut FdPayloadReader<'_>,
        _state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        if !matches!(
            interface,
            ObjectInteface::XdgShell(XdgShellObjectInteface::XdgSurface { .. })
        ) || opcode != Self::OPCODE
            || interface.version() < Self::SINCE
        {
            return Err(FdReaderError::InvalidProtocol);
        }
        let x = reader.next_i32().await?;
        let y = reader.next_i32().await?;
        let width = reader.next_i32().await?;
        let height = reader.next_i32().await?;
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        Ok(Self {
            object_id,
            x,
            y,
            width,
            height,
        })
    }
}
impl CommandWriter for XdgSurfaceSetWindowGeometryRequest {
    fn object_id(&self) -> u32 {
        self.object_id
    }
    fn opcode(&self) -> u16 {
        Self::OPCODE
    }
    fn payload_size(&self) -> usize {
        Self::total_payload_size(&[Some(4), Some(4), Some(4), Some(4)])
    }
    async fn write(
        self,
        writer: &mut FdPayloadWriter<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<(), FdWriterError> {
        Self::writer_request_interface(state, self.object_id, "xdg_surface", Self::SINCE)?;
        if self.payload_size() != writer.remaining_quota() {
            return Err(FdWriterError::InvalidProtocol);
        }
        writer.next_i32(self.x).await?;
        writer.next_i32(self.y).await?;
        writer.next_i32(self.width).await?;
        writer.next_i32(self.height).await?;
        Ok(())
    }
}
