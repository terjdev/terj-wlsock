use crate::protocol::ext::xdg_shell::ObjectInteface as XdgShellObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, CommandWriterExt, InterfaceReader,
    ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::FdWriter;
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Client request `xdg_toplevel.set_app_id`.
///
/// Opcode 3; available since version 1.
/// Wire framing and receiver metadata are checked; protocol semantics belong to the caller.
/// ```xml
/// <request name="set_app_id">
///       <description summary="set application ID">
///     Set an application identifier for the surface.
///
///     The app ID identifies the general class of applications to which
///     the surface belongs. The compositor can use this to group multiple
///     surfaces together, or to determine how to launch a new application.
///
///     For D-Bus activatable applications, the app ID is used as the D-Bus
///     service name.
///
///     The compositor shell will try to group application surfaces together
///     by their app ID. As a best practice, it is suggested to select app
///     ID's that match the basename of the application's .desktop file.
///     For example, "org.freedesktop.FooViewer" where the .desktop file is
///     "org.freedesktop.FooViewer.desktop".
///
///     Like other properties, a set_app_id request can be sent after the
///     xdg_toplevel has been mapped to update the property.
///
///     See the desktop-entry specification [0] for more details on
///     application identifiers and how they relate to well-known D-Bus
///     names and .desktop files.
///
///     [0] https://standards.freedesktop.org/desktop-entry-spec/
///       </description>
///       <arg name="app_id" type="string" summary="application identifier surface belongs to" />
///     </request>
/// ```
#[derive(Debug)]
pub struct XdgToplevelSetAppIdRequest {
    /// Wire identifier of the receiver.
    pub object_id: u32,
    /// application identifier surface belongs to.
    pub app_id: String,
}
impl XdgToplevelSetAppIdRequest {
    /// Opcode in the request namespace.
    pub const OPCODE: u16 = 3;
    /// Minimum supported interface version.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for XdgToplevelSetAppIdRequest {
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
impl InterfaceReader for XdgToplevelSetAppIdRequest {
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
        let app_id = reader.next_string().await?;
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        Ok(Self { object_id, app_id })
    }
}
impl CommandWriter for XdgToplevelSetAppIdRequest {
    fn object_id(&self) -> u32 {
        self.object_id
    }
    fn opcode(&self) -> u16 {
        Self::OPCODE
    }
    fn payload_size(&self) -> usize {
        Self::total_payload_size(&[FdWriter::encoded_string_size(Some(&self.app_id))])
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
        if !FdWriter::is_valid_string(&self.app_id) {
            return Err(FdWriterError::InvalidProtocol);
        }
        writer.next_string(&self.app_id).await?;
        Ok(())
    }
}
