use crate::protocol::ext::xdg_shell::ObjectInteface as XdgShellObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, CommandWriterExt, InterfaceReader,
    ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Client request `xdg_toplevel.unset_fullscreen`.
///
/// Opcode 12; available since version 1.
/// Wire framing and receiver metadata are checked; protocol semantics belong to the caller.
/// ```xml
/// <request name="unset_fullscreen">
///       <description summary="unset the window as fullscreen">
///     Make the surface no longer fullscreen.
///
///     After requesting that the surface should be unfullscreened, the
///     compositor will respond by emitting a configure event.
///     Whether this actually removes the fullscreen state of the client is
///     subject to compositor policies.
///
///     Making a surface unfullscreen sets states for the surface based on the following:
///     * the state(s) it may have had before becoming fullscreen
///     * any state(s) decided by the compositor
///     * any state(s) requested by the client while the surface was fullscreen
///
///     The compositor may include the previous window geometry dimensions in
///     the configure event, if applicable.
///
///     The client must also acknowledge the configure when committing the new
///     content (see ack_configure).
///       </description>
///     </request>
/// ```
#[derive(Debug)]
pub struct XdgToplevelUnsetFullscreenRequest {
    /// Wire identifier of the receiver.
    pub object_id: u32,
}
impl XdgToplevelUnsetFullscreenRequest {
    /// Opcode in the request namespace.
    pub const OPCODE: u16 = 12;
    /// Minimum supported interface version.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for XdgToplevelUnsetFullscreenRequest {
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
impl InterfaceReader for XdgToplevelUnsetFullscreenRequest {
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
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        Ok(Self { object_id })
    }
}
impl CommandWriter for XdgToplevelUnsetFullscreenRequest {
    fn object_id(&self) -> u32 {
        self.object_id
    }
    fn opcode(&self) -> u16 {
        Self::OPCODE
    }
    fn payload_size(&self) -> usize {
        Self::total_payload_size(&[])
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
        Ok(())
    }
}
