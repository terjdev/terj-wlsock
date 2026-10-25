use super::ObjectInteface as XxSessionManagementObjectInteface;
use super::XxSessionManagerV1Reason;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, CommandWriterExt, InterfaceReader,
    ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::FdWriter;
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Client request `xx_session_manager_v1.get_session`.
///
/// Framing, receiver protocol and version are checked; semantics belong to the caller.
/// New IDs are registered before writes; close the connection after a failed or cancelled write.
/// ```xml
/// <request name="get_session">
///       <description summary="create or restore a session">
///         Create a session object corresponding to either an existing session
///         identified by the given session identifier string or a new session.
///         While the session object exists, the session is considered to be "in
///         use".
///
///         If a identifier string represents a session that is currently actively
///         in use by the same client, an 'in_use' error is raised. If some
///         other client is currently using the same session, the new session will
///         replace managing the associated state.
///
///         NULL is passed to initiate a new session. If an id is passed which does
///         not represent a valid session, the compositor treats it as if NULL had
///         been passed.
///
///         A client is allowed to have any number of in use sessions at the same
///         time.
///       </description>
///       <arg name="id" type="new_id" interface="xx_session_v1" />
///       <arg name="reason" type="uint" enum="reason" summary="reason for session" />
///       <arg name="session" type="string" summary="the session to restore" allow-null="true" />
///     </request>
/// ```
#[derive(Debug)]
pub struct XxSessionManagerV1GetSessionRequest {
    /// Wire identifier of the receiver.
    pub object_id: u32,
    /// Wire argument `id`.
    pub id: u32,
    /// reason for session.
    pub reason: XxSessionManagerV1Reason,
    /// the session to restore.
    pub session: Option<String>,
}
impl XxSessionManagerV1GetSessionRequest {
    /// Opcode in the request namespace.
    pub const OPCODE: u16 = 1;
    /// Minimum interface version.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for XxSessionManagerV1GetSessionRequest {
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
impl InterfaceReader for XxSessionManagerV1GetSessionRequest {
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
                XxSessionManagementObjectInteface::XxSessionManagerV1 { .. }
            )
        ) || opcode != Self::OPCODE
            || interface.version() < Self::SINCE
        {
            return Err(FdReaderError::InvalidProtocol);
        }
        let id = reader.next_u32().await?;
        let reason = XxSessionManagerV1Reason(reader.next_u32().await?);
        let session = reader.next_nullable_string().await?;
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        {
            let new_interface = ObjectInteface::XxSessionManagement(
                XxSessionManagementObjectInteface::XxSessionV1 {
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
            reason,
            session,
        })
    }
}
impl CommandWriter for XxSessionManagerV1GetSessionRequest {
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
            FdWriter::encoded_string_size(self.session.as_deref()),
        ])
    }
    async fn write(
        self,
        writer: &mut FdPayloadWriter<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<(), FdWriterError> {
        let interface = Self::writer_request_interface(
            state,
            self.object_id,
            "xx_session_manager_v1",
            Self::SINCE,
        )?;
        if !matches!(
            interface,
            ObjectInteface::XxSessionManagement(
                XxSessionManagementObjectInteface::XxSessionManagerV1 { .. }
            )
        ) || self.payload_size() != writer.remaining_quota()
        {
            return Err(FdWriterError::InvalidProtocol);
        }
        if self
            .session
            .as_deref()
            .is_some_and(|v| !FdWriter::is_valid_string(v))
        {
            return Err(FdWriterError::InvalidProtocol);
        }
        {
            let new_interface = ObjectInteface::XxSessionManagement(
                XxSessionManagementObjectInteface::XxSessionV1 {
                    version: interface.version().min(1),
                },
            );
            Self::writer_state(state)?
                .register_object(self.id, new_interface, false)
                .map_err(|_| FdWriterError::InvalidProtocol)?;
        }
        writer.next_u32(self.id).await?;
        writer.next_u32(self.reason.0).await?;
        writer.next_nullable_string(self.session.as_deref()).await?;
        Ok(())
    }
}
