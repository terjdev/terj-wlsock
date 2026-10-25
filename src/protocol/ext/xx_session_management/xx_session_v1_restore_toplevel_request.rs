use super::ObjectInteface as XxSessionManagementObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, CommandWriterExt, InterfaceReader,
    ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::FdWriter;
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Client request `xx_session_v1.restore_toplevel`.
///
/// Framing, receiver protocol and version are checked; semantics belong to the caller.
/// New IDs are registered before writes; close the connection after a failed or cancelled write.
/// ```xml
/// <request name="restore_toplevel">
///       <description summary="restore a surface state">
///         Inform the compositor that the toplevel associated with the passed name
///         should have its window management state restored.
///
///         Calling this with a toplevel that is already managed by the session with
///         the same associated will raise an in_use error.
///
///         This request must be called prior to the first commit on the associated
///         wl_surface, otherwise an already_mapped error is raised.
///
///         As part of the initial configure sequence, if the toplevel was
///         successfully restored, a xx_toplevel_session_v1.restored event is
///         emitted. See the xx_toplevel_session_v1.restored event for further
///         details.
///       </description>
///       <arg name="id" type="new_id" interface="xx_toplevel_session_v1" />
///       <arg name="toplevel" type="object" interface="xdg_toplevel" />
///       <arg name="name" type="string" />
///     </request>
/// ```
#[derive(Debug)]
pub struct XxSessionV1RestoreToplevelRequest {
    /// Wire identifier of the receiver.
    pub object_id: u32,
    /// Wire argument `id`.
    pub id: u32,
    /// Wire argument `toplevel`.
    pub toplevel: u32,
    /// Wire argument `name`.
    pub name: String,
}
impl XxSessionV1RestoreToplevelRequest {
    /// Opcode in the request namespace.
    pub const OPCODE: u16 = 3;
    /// Minimum interface version.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for XxSessionV1RestoreToplevelRequest {
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
impl InterfaceReader for XxSessionV1RestoreToplevelRequest {
    async fn read_with_interface(
        object_id: u32,
        opcode: u16,
        interface: ObjectInteface,
        reader: &mut FdPayloadReader<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        if !matches!(
            interface,
            ObjectInteface::XxSessionManagement(
                XxSessionManagementObjectInteface::XxSessionV1 { .. }
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
            let new_interface = ObjectInteface::XxSessionManagement(
                XxSessionManagementObjectInteface::XxToplevelSessionV1 {
                    version: interface.version().min(1),
                },
            );
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
impl CommandWriter for XxSessionV1RestoreToplevelRequest {
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
            Self::writer_request_interface(state, self.object_id, "xx_session_v1", Self::SINCE)?;
        if !matches!(
            interface,
            ObjectInteface::XxSessionManagement(
                XxSessionManagementObjectInteface::XxSessionV1 { .. }
            )
        ) || self.payload_size() != writer.remaining_quota()
        {
            return Err(FdWriterError::InvalidProtocol);
        }
        if !FdWriter::is_valid_string(&self.name) {
            return Err(FdWriterError::InvalidProtocol);
        }
        {
            let new_interface = ObjectInteface::XxSessionManagement(
                XxSessionManagementObjectInteface::XxToplevelSessionV1 {
                    version: interface.version().min(1),
                },
            );
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
