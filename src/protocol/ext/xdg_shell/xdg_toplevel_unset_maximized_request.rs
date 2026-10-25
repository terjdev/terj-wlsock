use crate::protocol::ext::xdg_shell::ObjectInteface as XdgShellObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, CommandWriterExt, InterfaceReader,
    ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Client request `xdg_toplevel.unset_maximized`.
///
/// Opcode 10; available since version 1.
/// Wire framing and receiver metadata are checked; protocol semantics belong to the caller.
/// ```xml
/// <request name="unset_maximized">
///       <description summary="unmaximize the window">
///     Unmaximize the surface.
///
///     After requesting that the surface should be unmaximized, the compositor
///     will respond by emitting a configure event. Whether this actually
///     un-maximizes the window is subject to compositor policies.
///     If available and applicable, the compositor will include the window
///     geometry dimensions the window had prior to being maximized in the
///     configure event. The client must then update its content, drawing it in
///     the configured state. The client must also acknowledge the configure
///     when committing the new content (see ack_configure).
///
///     It is up to the compositor to position the surface after it was
///     unmaximized; usually the position the surface had before maximizing, if
///     applicable.
///
///     If the surface was already not maximized, the compositor will still
///     emit a configure event without the "maximized" state.
///
///     If the surface is in a fullscreen state, this request has no direct
///     effect. It may alter the state the surface is returned to when
///     unmaximized unless overridden by the compositor.
///       </description>
///     </request>
/// ```
#[derive(Debug)]
pub struct XdgToplevelUnsetMaximizedRequest {
    /// Wire identifier of the receiver.
    pub object_id: u32,
}
impl XdgToplevelUnsetMaximizedRequest {
    /// Opcode in the request namespace.
    pub const OPCODE: u16 = 10;
    /// Minimum supported interface version.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for XdgToplevelUnsetMaximizedRequest {
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
impl InterfaceReader for XdgToplevelUnsetMaximizedRequest {
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
impl CommandWriter for XdgToplevelUnsetMaximizedRequest {
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
