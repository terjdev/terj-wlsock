use crate::protocol::ext::xdg_shell::ObjectInteface as XdgShellObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, CommandWriterExt, InterfaceReader,
    ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::FdWriter;
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Server event `xdg_toplevel.wm_capabilities`.
///
/// Opcode 3; available since version 5.
/// Wire framing and receiver metadata are checked; protocol semantics belong to the caller.
/// ```xml
/// <event name="wm_capabilities" since="5">
///       <description summary="compositor capabilities">
///     This event advertises the capabilities supported by the compositor. If
///     a capability isn't supported, clients should hide or disable the UI
///     elements that expose this functionality. For instance, if the
///     compositor doesn't advertise support for minimized toplevels, a button
///     triggering the set_minimized request should not be displayed.
///
///     The compositor will ignore requests it doesn't support. For instance,
///     a compositor which doesn't advertise support for minimized will ignore
///     set_minimized requests.
///
///     Compositors must send this event once before the first
///     xdg_surface.configure event. When the capabilities change, compositors
///     must send this event again and then send an xdg_surface.configure
///     event.
///
///     The configured state should not be applied immediately. See
///     xdg_surface.configure for details.
///
///     The capabilities are sent as an array of 32-bit unsigned integers in
///     native endianness. Capability values are defined in the wm_capabilities enum.
///       </description>
///       <arg name="capabilities" type="array" summary="array of 32-bit capabilities" />
///     </event>
/// ```
#[derive(Debug)]
pub struct XdgToplevelWmCapabilitiesEvent {
    /// Wire identifier of the receiver.
    pub object_id: u32,
    /// array of 32-bit capabilities.
    pub capabilities: Vec<u8>,
}
impl XdgToplevelWmCapabilitiesEvent {
    /// Opcode in the event namespace.
    pub const OPCODE: u16 = 3;
    /// Minimum supported interface version.
    pub const SINCE: u32 = 5;
}
impl CommandReader<Self> for XdgToplevelWmCapabilitiesEvent {
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
impl InterfaceReader for XdgToplevelWmCapabilitiesEvent {
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
        let capabilities = reader.next_array().await?;
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        Ok(Self {
            object_id,
            capabilities,
        })
    }
}
impl CommandWriter for XdgToplevelWmCapabilitiesEvent {
    fn object_id(&self) -> u32 {
        self.object_id
    }
    fn opcode(&self) -> u16 {
        Self::OPCODE
    }
    fn payload_size(&self) -> usize {
        Self::total_payload_size(&[FdWriter::encoded_array_size(self.capabilities.len())])
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
        writer.next_array(&self.capabilities).await?;
        Ok(())
    }
}
