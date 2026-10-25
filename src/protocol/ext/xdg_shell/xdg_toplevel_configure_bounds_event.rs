use crate::protocol::ext::xdg_shell::ObjectInteface as XdgShellObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, CommandWriterExt, InterfaceReader,
    ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Server event `xdg_toplevel.configure_bounds`.
///
/// Opcode 2; available since version 4.
/// Wire framing and receiver metadata are checked; protocol semantics belong to the caller.
/// ```xml
/// <event name="configure_bounds" since="4">
///       <description summary="recommended window geometry bounds">
///     The configure_bounds event may be sent prior to a xdg_toplevel.configure
///     event to communicate the bounds a window geometry size is recommended
///     to constrain to.
///
///     The passed width and height are in surface coordinate space. If width
///     and height are 0, it means bounds is unknown and equivalent to as if no
///     configure_bounds event was ever sent for this surface.
///
///     The bounds can for example correspond to the size of a monitor excluding
///     any panels or other shell components, so that a surface isn't created in
///     a way that it cannot fit.
///
///     The bounds may change at any point, and in such a case, a new
///     xdg_toplevel.configure_bounds will be sent, followed by
///     xdg_toplevel.configure and xdg_surface.configure.
///       </description>
///       <arg name="width" type="int" summary="suggested maximum width of surface" />
///       <arg name="height" type="int" summary="suggested maximum height of surface" />
///     </event>
/// ```
#[derive(Debug)]
pub struct XdgToplevelConfigureBoundsEvent {
    /// Wire identifier of the receiver.
    pub object_id: u32,
    /// suggested maximum width of surface.
    pub width: i32,
    /// suggested maximum height of surface.
    pub height: i32,
}
impl XdgToplevelConfigureBoundsEvent {
    /// Opcode in the event namespace.
    pub const OPCODE: u16 = 2;
    /// Minimum supported interface version.
    pub const SINCE: u32 = 4;
}
impl CommandReader<Self> for XdgToplevelConfigureBoundsEvent {
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
impl InterfaceReader for XdgToplevelConfigureBoundsEvent {
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
impl CommandWriter for XdgToplevelConfigureBoundsEvent {
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
        Self::writer_event_interface(state, self.object_id, "xdg_toplevel", Self::SINCE)?;
        if self.payload_size() != writer.remaining_quota() {
            return Err(FdWriterError::InvalidProtocol);
        }
        writer.next_i32(self.width).await?;
        writer.next_i32(self.height).await?;
        Ok(())
    }
}
