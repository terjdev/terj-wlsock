use crate::protocol::ext::xdg_shell::ObjectInteface as XdgShellObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, CommandWriterExt, InterfaceReader,
    ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Client request `xdg_toplevel.set_min_size`.
///
/// Opcode 8; available since version 1.
/// Wire framing and receiver metadata are checked; protocol semantics belong to the caller.
/// ```xml
/// <request name="set_min_size">
///       <description summary="set the minimum size">
///     Set a minimum size for the window.
///
///     The client can specify a minimum size so that the compositor does
///     not try to configure the window below this size.
///
///     The width and height arguments are in window geometry coordinates.
///     See xdg_surface.set_window_geometry.
///
///     Values set in this way are double-buffered, see wl_surface.commit.
///
///     The compositor can use this information to allow or disallow
///     different states like maximize or fullscreen and draw accurate
///     animations.
///
///     Similarly, a tiling window manager may use this information to
///     place and resize client windows in a more effective way.
///
///     The client should not rely on the compositor to obey the minimum
///     size. The compositor may decide to ignore the values set by the
///     client and request a smaller size.
///
///     If never set, or a value of zero in the request, means that the
///     client has no expected minimum size in the given dimension.
///     As a result, a client wishing to reset the minimum size
///     to an unspecified state can use zero for width and height in the
///     request.
///
///     Requesting a minimum size to be larger than the maximum size of
///     a surface is illegal and will result in an invalid_size error.
///
///     The width and height must be greater than or equal to zero. Using
///     strictly negative values for width and height will result in an
///     invalid_size error.
///       </description>
///       <arg name="width" type="int" summary="minimum width of the window" />
///       <arg name="height" type="int" summary="minimum height of the window" />
///     </request>
/// ```
#[derive(Debug)]
pub struct XdgToplevelSetMinSizeRequest {
    /// Wire identifier of the receiver.
    pub object_id: u32,
    /// minimum width of the window.
    pub width: i32,
    /// minimum height of the window.
    pub height: i32,
}
impl XdgToplevelSetMinSizeRequest {
    /// Opcode in the request namespace.
    pub const OPCODE: u16 = 8;
    /// Minimum supported interface version.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for XdgToplevelSetMinSizeRequest {
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
impl InterfaceReader for XdgToplevelSetMinSizeRequest {
    async fn read_with_interface(
        object_id: u32,
        opcode: u16,
        interface: ObjectInteface,
        reader: &mut FdPayloadReader<'_>,
        _state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        if !matches!(
            interface,
            ObjectInteface::XdgShell(XdgShellObjectInteface::XdgToplevel { .. })
        ) || opcode != Self::OPCODE
            || interface.version() < Self::SINCE
        {
            return Err(FdReaderError::InvalidProtocol);
        }
        let width = reader.next_i32().await?;
        let height = reader.next_i32().await?;
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        Ok(Self {
            object_id,
            width,
            height,
        })
    }
}
impl CommandWriter for XdgToplevelSetMinSizeRequest {
    fn object_id(&self) -> u32 {
        self.object_id
    }
    fn opcode(&self) -> u16 {
        Self::OPCODE
    }
    fn payload_size(&self) -> usize {
        Self::total_payload_size(&[Some(4), Some(4)])
    }
    async fn write(
        self,
        writer: &mut FdPayloadWriter<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<(), FdWriterError> {
        Self::writer_request_interface(state, self.object_id, "xdg_toplevel", Self::SINCE)?;
        if self.payload_size() != writer.remaining_quota() {
            return Err(FdWriterError::InvalidProtocol);
        }
        writer.next_i32(self.width).await?;
        writer.next_i32(self.height).await?;
        Ok(())
    }
}
