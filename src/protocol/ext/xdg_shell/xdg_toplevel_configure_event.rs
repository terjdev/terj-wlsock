use crate::protocol::ext::xdg_shell::ObjectInteface as XdgShellObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, CommandWriterExt, InterfaceReader,
    ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::FdWriter;
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Server event `xdg_toplevel.configure`.
///
/// Opcode 0; available since version 1.
/// Wire framing and receiver metadata are checked; protocol semantics belong to the caller.
/// ```xml
/// <event name="configure">
///       <description summary="suggest a surface change">
///     This configure event asks the client to resize its toplevel surface or
///     to change its state. The configured state should not be applied
///     immediately. See xdg_surface.configure for details.
///
///     The width and height arguments specify a hint to the window
///     about how its surface should be resized in window geometry
///     coordinates. See set_window_geometry.
///
///     If the width or height arguments are zero, it means the client
///     should decide its own window dimension. This may happen when the
///     compositor needs to configure the state of the surface but doesn't
///     have any information about any previous or expected dimension.
///
///     The states listed in the event specify how the width/height
///     arguments should be interpreted, and possibly how it should be
///     drawn.
///
///     The states are sent as an array of 32-bit unsigned integers in
///     native endianness. State values are defined in the state enum.
///
///     Clients must send an ack_configure in response to this event. See
///     xdg_surface.configure and xdg_surface.ack_configure for details.
///       </description>
///       <arg name="width" type="int" summary="suggested width of window" />
///       <arg name="height" type="int" summary="suggested height of window" />
///       <arg name="states" type="array" summary="suggested states of the window" />
///     </event>
/// ```
#[derive(Debug)]
pub struct XdgToplevelConfigureEvent {
    /// Wire identifier of the receiver.
    pub object_id: u32,
    /// suggested width of window.
    pub width: i32,
    /// suggested height of window.
    pub height: i32,
    /// suggested states of the window.
    pub states: Vec<u8>,
}
impl XdgToplevelConfigureEvent {
    /// Opcode in the event namespace.
    pub const OPCODE: u16 = 0;
    /// Minimum supported interface version.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for XdgToplevelConfigureEvent {
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
impl InterfaceReader for XdgToplevelConfigureEvent {
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
        let states = reader.next_array().await?;
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        Ok(Self {
            object_id,
            width,
            height,
            states,
        })
    }
}
impl CommandWriter for XdgToplevelConfigureEvent {
    fn object_id(&self) -> u32 {
        self.object_id
    }
    fn opcode(&self) -> u16 {
        Self::OPCODE
    }
    fn payload_size(&self) -> usize {
        Self::total_payload_size(&[
            Some(4),
            Some(4),
            FdWriter::encoded_array_size(self.states.len()),
        ])
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
        writer.next_array(&self.states).await?;
        Ok(())
    }
}
