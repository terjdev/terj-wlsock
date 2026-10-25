use crate::protocol::ext::xdg_shell::ObjectInteface as XdgShellObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, CommandWriterExt, InterfaceReader,
    ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Client request `xdg_toplevel.show_window_menu`.
///
/// Opcode 4; available since version 1.
/// Wire framing and receiver metadata are checked; protocol semantics belong to the caller.
/// ```xml
/// <request name="show_window_menu">
///       <description summary="show the window menu">
///     Clients implementing client-side decorations might want to show
///     a context menu when right-clicking on the decorations, giving the
///     user a menu that they can use to maximize or minimize the window.
///
///     This request asks the compositor to pop up such a window menu at
///     the given position, relative to the local surface coordinates of
///     the parent surface. There are no guarantees as to what menu items
///     the window menu contains, or even if a window menu will be drawn
///     at all.
///
///     This request must be used in response to some sort of user action
///     like a button press, key press, or touch down event.
///       </description>
///       <arg name="seat" type="object" interface="wl_seat" summary="the wl_seat of the user event" />
///       <arg name="serial" type="uint" summary="the serial of the user event" />
///       <arg name="x" type="int" summary="the x position to pop up the window menu at" />
///       <arg name="y" type="int" summary="the y position to pop up the window menu at" />
///     </request>
/// ```
#[derive(Debug)]
pub struct XdgToplevelShowWindowMenuRequest {
    /// Wire identifier of the receiver.
    pub object_id: u32,
    /// the wl_seat of the user event.
    pub seat: u32,
    /// the serial of the user event.
    pub serial: u32,
    /// the x position to pop up the window menu at.
    pub x: i32,
    /// the y position to pop up the window menu at.
    pub y: i32,
}
impl XdgToplevelShowWindowMenuRequest {
    /// Opcode in the request namespace.
    pub const OPCODE: u16 = 4;
    /// Minimum supported interface version.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for XdgToplevelShowWindowMenuRequest {
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
impl InterfaceReader for XdgToplevelShowWindowMenuRequest {
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
        let seat = reader.next_u32().await?;
        let serial = reader.next_u32().await?;
        let x = reader.next_i32().await?;
        let y = reader.next_i32().await?;
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        Ok(Self {
            object_id,
            seat,
            serial,
            x,
            y,
        })
    }
}
impl CommandWriter for XdgToplevelShowWindowMenuRequest {
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
        Self::writer_request_interface(state, self.object_id, "xdg_toplevel", Self::SINCE)?;
        if self.payload_size() != writer.remaining_quota() {
            return Err(FdWriterError::InvalidProtocol);
        }
        writer.next_u32(self.seat).await?;
        writer.next_u32(self.serial).await?;
        writer.next_i32(self.x).await?;
        writer.next_i32(self.y).await?;
        Ok(())
    }
}
