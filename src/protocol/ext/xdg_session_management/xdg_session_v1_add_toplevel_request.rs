use crate::protocol::ext::xdg_session_management::ObjectInteface as XdgXxSessionManagementObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, CommandWriterExt, InterfaceReader,
    ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::FdWriter;
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Client request `xdg_session_v1.add_toplevel`.
///
/// Opcode 2; available since version 1.
/// Wire framing and receiver metadata are checked; protocol semantics belong to the caller.
/// New IDs are registered before writes; close the connection after a failed or cancelled write.
/// ```xml
/// <request name="add_toplevel">
///       <description summary="add a new surface to the session">
///         Attempt to add a given surface to the session. The passed name is used
///         to identify what window is being restored, and may be used to store
///         window specific state within the session.
///
///         The name given to the toplevel must not correspond to any previously
///         existing toplevel names in the session. If the name matches an already
///         known toplevel name in the session, a 'name_in_use' protocol error will
///         be raised.
///
///         The toplevel object must not be added more than once to any session
///         created by the client, otherwise the 'already_added' protocol error
///         will be raised.
///
///         This request will return a xdg_toplevel_session_v1 for later
///         manipulation. As this resource is created from an empty initial state,
///         compositors must not emit a xdg_toplevel_session_v1.restored event for
///         resources created through this request.
///
///         The name string must be UTF-8 encoded. It is also limited by the maximum
///         length of wayland messages (around 4KB). The 'invalid_name' protocol
///         error will be raised if an invalid string is provided.
///       </description>
///       <arg name="id" type="new_id" interface="xdg_toplevel_session_v1" />
///       <arg name="toplevel" type="object" interface="xdg_toplevel" />
///       <arg name="name" type="string" summary="name identifying the toplevel" />
///     </request>
/// ```
#[derive(Debug)]
pub struct XdgSessionV1AddToplevelRequest {
    /// Wire identifier of the receiver.
    pub object_id: u32,
    /// Wire argument `id`.
    pub id: u32,
    /// Wire argument `toplevel`.
    pub toplevel: u32,
    /// name identifying the toplevel.
    pub name: String,
}
impl XdgSessionV1AddToplevelRequest {
    /// Opcode in the request namespace.
    pub const OPCODE: u16 = 2;
    /// Minimum supported interface version.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for XdgSessionV1AddToplevelRequest {
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
impl InterfaceReader for XdgSessionV1AddToplevelRequest {
    async fn read_with_interface(
        object_id: u32,
        opcode: u16,
        interface: ObjectInteface,
        reader: &mut FdPayloadReader<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        if !matches!(
            interface,
            ObjectInteface::XdgSessionManagement(
                XdgXxSessionManagementObjectInteface::XdgSessionV1 { .. }
            )
        ) || opcode != Self::OPCODE
            || interface.version() < Self::SINCE
        {
            return Err(FdReaderError::InvalidProtocol);
        }
        let id = reader.next_u32().await?;
        let toplevel = reader.next_u32().await?;
        let name = reader.next_string().await?;
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        {
            let new_interface =
                ObjectInteface::from_name("xdg_toplevel_session_v1", interface.version().min(1))
                    .ok_or(FdReaderError::InvalidProtocol)?;
            Self::reader_state(state)?
                .register_object(id, new_interface, false)
                .map_err(|_| FdReaderError::InvalidProtocol)?;
        }
        Ok(Self {
            object_id,
            id,
            toplevel,
            name,
        })
    }
}
impl CommandWriter for XdgSessionV1AddToplevelRequest {
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
            FdWriter::encoded_string_size(Some(&self.name)),
        ])
    }
    async fn write(
        self,
        writer: &mut FdPayloadWriter<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<(), FdWriterError> {
        let interface =
            Self::writer_request_interface(state, self.object_id, "xdg_session_v1", Self::SINCE)?;
        if self.payload_size() != writer.remaining_quota() {
            return Err(FdWriterError::InvalidProtocol);
        }
        if !FdWriter::is_valid_string(&self.name) {
            return Err(FdWriterError::InvalidProtocol);
        }
        {
            let new_interface =
                ObjectInteface::from_name("xdg_toplevel_session_v1", interface.version().min(1))
                    .ok_or(FdWriterError::InvalidProtocol)?;
            Self::writer_state(state)?
                .register_object(self.id, new_interface, false)
                .map_err(|_| FdWriterError::InvalidProtocol)?;
        }
        writer.next_u32(self.id).await?;
        writer.next_u32(self.toplevel).await?;
        writer.next_string(&self.name).await?;
        Ok(())
    }
}
