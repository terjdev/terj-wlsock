use super::XdgToplevelResizeEdge;
use crate::protocol::ext::xdg_shell::ObjectInteface as XdgShellObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, CommandWriterExt, InterfaceReader,
    ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Client request `xdg_toplevel.resize`.
///
/// Opcode 6; available since version 1.
/// Wire framing and receiver metadata are checked; protocol semantics belong to the caller.
/// ```xml
/// <request name="resize">
///       <description summary="start an interactive resize">
///     Start a user-driven, interactive resize of the surface.
///
///     This request must be used in response to some sort of user action
///     like a button press, key press, or touch down event. The passed
///     serial is used to determine the type of interactive resize (touch,
///     pointer, etc).
///
///     The server may ignore resize requests depending on the state of
///     the surface (e.g. fullscreen or maximized).
///
///     If triggered, the client will receive configure events with the
///     "resize" state enum value and the expected sizes. See the "resize"
///     enum value for more details about what is required. The client
///     must also acknowledge configure events using "ack_configure". After
///     the resize is completed, the client will receive another "configure"
///     event without the resize state.
///
///     If triggered, the surface also will lose the focus of the device
///     (wl_pointer, wl_touch, etc) used for the resize. It is up to the
///     compositor to visually indicate that the resize is taking place,
///     such as updating a pointer cursor, during the resize. There is no
///     guarantee that the device focus will return when the resize is
///     completed.
///
///     The edges parameter specifies how the surface should be resized, and
///     is one of the values of the resize_edge enum. Values not matching
///     a variant of the enum will cause the invalid_resize_edge protocol error.
///     The compositor may use this information to update the surface position
///     for example when dragging the top left corner. The compositor may also
///     use this information to adapt its behavior, e.g. choose an appropriate
///     cursor image.
///       </description>
///       <arg name="seat" type="object" interface="wl_seat" summary="the wl_seat of the user event" />
///       <arg name="serial" type="uint" summary="the serial of the user event" />
///       <arg name="edges" type="uint" enum="resize_edge" summary="which edge or corner is being dragged" />
///     </request>
/// ```
#[derive(Debug)]
pub struct XdgToplevelResizeRequest {
    /// Wire identifier of the receiver.
    pub object_id: u32,
    /// the wl_seat of the user event.
    pub seat: u32,
    /// the serial of the user event.
    pub serial: u32,
    /// which edge or corner is being dragged.
    pub edges: XdgToplevelResizeEdge,
}
impl XdgToplevelResizeRequest {
    /// Opcode in the request namespace.
    pub const OPCODE: u16 = 6;
    /// Minimum supported interface version.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for XdgToplevelResizeRequest {
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
impl InterfaceReader for XdgToplevelResizeRequest {
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
        let edges = XdgToplevelResizeEdge(reader.next_u32().await?);
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        Ok(Self {
            object_id,
            seat,
            serial,
            edges,
        })
    }
}
impl CommandWriter for XdgToplevelResizeRequest {
    fn object_id(&self) -> u32 {
        self.object_id
    }
    fn opcode(&self) -> u16 {
        Self::OPCODE
    }
    fn payload_size(&self) -> usize {
        Self::total_payload_size(&[Some(4), Some(4), Some(4)])
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
        writer.next_u32(self.edges.0).await?;
        Ok(())
    }
}
