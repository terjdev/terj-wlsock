use crate::protocol::ext::xdg_shell::ObjectInteface as XdgShellObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, CommandWriterExt, InterfaceReader,
    ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Client request `xdg_toplevel.move`.
///
/// Opcode 5; available since version 1.
/// Wire framing and receiver metadata are checked; protocol semantics belong to the caller.
/// ```xml
/// <request name="move">
///       <description summary="start an interactive move">
///     Start an interactive, user-driven move of the surface.
///
///     This request must be used in response to some sort of user action
///     like a button press, key press, or touch down event. The passed
///     serial is used to determine the type of interactive move (touch,
///     pointer, etc).
///
///     The server may ignore move requests depending on the state of
///     the surface (e.g. fullscreen or maximized), or if the passed serial
///     is no longer valid.
///
///     If triggered, the surface will lose the focus of the device
///     (wl_pointer, wl_touch, etc) used for the move. It is up to the
///     compositor to visually indicate that the move is taking place, such as
///     updating a pointer cursor, during the move. There is no guarantee
///     that the device focus will return when the move is completed.
///       </description>
///       <arg name="seat" type="object" interface="wl_seat" summary="the wl_seat of the user event" />
///       <arg name="serial" type="uint" summary="the serial of the user event" />
///     </request>
/// ```
#[derive(Debug)]
pub struct XdgToplevelMoveRequest {
    /// Wire identifier of the receiver.
    pub object_id: u32,
    /// the wl_seat of the user event.
    pub seat: u32,
    /// the serial of the user event.
    pub serial: u32,
}
impl XdgToplevelMoveRequest {
    /// Opcode in the request namespace.
    pub const OPCODE: u16 = 5;
    /// Minimum supported interface version.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for XdgToplevelMoveRequest {
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
impl InterfaceReader for XdgToplevelMoveRequest {
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
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        Ok(Self {
            object_id,
            seat,
            serial,
        })
    }
}
impl CommandWriter for XdgToplevelMoveRequest {
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
        writer.next_u32(self.seat).await?;
        writer.next_u32(self.serial).await?;
        Ok(())
    }
}
