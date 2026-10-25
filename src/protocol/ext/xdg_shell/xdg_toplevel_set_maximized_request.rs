use crate::protocol::ext::xdg_shell::ObjectInteface as XdgShellObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, CommandWriterExt, InterfaceReader,
    ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Client request `xdg_toplevel.set_maximized`.
///
/// Opcode 9; available since version 1.
/// Wire framing and receiver metadata are checked; protocol semantics belong to the caller.
/// ```xml
/// <request name="set_maximized">
///       <description summary="maximize the window">
///     Maximize the surface.
///
///     After requesting that the surface should be maximized, the compositor
///     will respond by emitting a configure event. Whether this configure
///     actually sets the window maximized is subject to compositor policies.
///     The client must then update its content, drawing in the configured
///     state. The client must also acknowledge the configure when committing
///     the new content (see ack_configure).
///
///     It is up to the compositor to decide how and where to maximize the
///     surface, for example which output and what region of the screen should
///     be used.
///
///     If the surface was already maximized, the compositor will still emit
///     a configure event with the "maximized" state.
///
///     If the surface is in a fullscreen state, this request has no direct
///     effect. It may alter the state the surface is returned to when
///     unmaximized unless overridden by the compositor.
///       </description>
///     </request>
/// ```
#[derive(Debug)]
pub struct XdgToplevelSetMaximizedRequest {
    /// Wire identifier of the receiver.
    pub object_id: u32,
}
impl XdgToplevelSetMaximizedRequest {
    /// Opcode in the request namespace.
    pub const OPCODE: u16 = 9;
    /// Minimum supported interface version.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for XdgToplevelSetMaximizedRequest {
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
impl InterfaceReader for XdgToplevelSetMaximizedRequest {
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
impl CommandWriter for XdgToplevelSetMaximizedRequest {
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
