use crate::protocol::ext::xdg_shell::ObjectInteface as XdgShellObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, CommandWriterExt, InterfaceReader,
    ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Client request `xdg_surface.ack_configure`.
///
/// Opcode 4; available since version 1.
/// Wire framing and receiver metadata are checked; protocol semantics belong to the caller.
/// ```xml
/// <request name="ack_configure">
///       <description summary="ack a configure event">
///     When a configure event is received, if a client commits the
///     surface in response to the configure event, then the client
///     must make an ack_configure request sometime before the commit
///     request, passing along the serial of the configure event.
///
///     For instance, for toplevel surfaces the compositor might use this
///     information to move a surface to the top left only when the client has
///     drawn itself for the maximized or fullscreen state.
///
///     If the client receives multiple configure events before it
///     can respond to one, it only has to ack the last configure event.
///     Acking a configure event that was never sent raises an invalid_serial
///     error.
///
///     A client is not required to commit immediately after sending
///     an ack_configure request - it may even ack_configure several times
///     before its next surface commit.
///
///     A client may send multiple ack_configure requests before committing, but
///     only the last request sent before a commit indicates which configure
///     event the client really is responding to.
///
///     Sending an ack_configure request consumes the serial number sent with
///     the request, as well as serial numbers sent by all configure events
///     sent on this xdg_surface prior to the configure event referenced by
///     the committed serial.
///
///     It is an error to issue multiple ack_configure requests referencing a
///     serial from the same configure event, or to issue an ack_configure
///     request referencing a serial from a configure event issued before the
///     event identified by the last ack_configure request for the same
///     xdg_surface. Doing so will raise an invalid_serial error.
///       </description>
///       <arg name="serial" type="uint" summary="the serial from the configure event" />
///     </request>
/// ```
#[derive(Debug)]
pub struct XdgSurfaceAckConfigureRequest {
    /// Wire identifier of the receiver.
    pub object_id: u32,
    /// the serial from the configure event.
    pub serial: u32,
}
impl XdgSurfaceAckConfigureRequest {
    /// Opcode in the request namespace.
    pub const OPCODE: u16 = 4;
    /// Minimum supported interface version.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for XdgSurfaceAckConfigureRequest {
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
impl InterfaceReader for XdgSurfaceAckConfigureRequest {
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
        let serial = reader.next_u32().await?;
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        Ok(Self { object_id, serial })
    }
}
impl CommandWriter for XdgSurfaceAckConfigureRequest {
    fn object_id(&self) -> u32 {
        self.object_id
    }
    fn opcode(&self) -> u16 {
        Self::OPCODE
    }
    fn payload_size(&self) -> usize {
        Self::total_payload_size(&[Some(4)])
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
        writer.next_u32(self.serial).await?;
        Ok(())
    }
}
