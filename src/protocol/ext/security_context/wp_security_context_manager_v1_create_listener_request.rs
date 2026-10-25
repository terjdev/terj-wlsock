use crate::protocol::ext::security_context::ObjectInteface as SecurityContextObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, CommandWriterExt, InterfaceReader,
    ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Client request `wp_security_context_manager_v1.create_listener`.
///
/// Opcode 1; available since version 1.
/// Wire framing and receiver metadata are checked; protocol semantics belong to the caller.
/// New IDs are registered before writes; close the connection after a failed or cancelled write.
/// ```xml
/// <request name="create_listener">
///       <description summary="create a new security context">
///         Creates a new security context with a socket listening FD.
///
///         The compositor will accept new client connections on listen_fd.
///         listen_fd must be ready to accept new connections when this request is
///         sent by the client. In other words, the client must call bind(2) and
///         listen(2) before sending the FD.
///
///         close_fd is a FD that will signal hangup when the compositor should stop
///         accepting new connections on listen_fd.
///
///         The compositor must continue to accept connections on listen_fd when
///         the Wayland client which created the security context disconnects.
///
///         After sending this request, closing listen_fd and close_fd remains the
///         only valid operation on them.
///       </description>
///       <arg name="id" type="new_id" interface="wp_security_context_v1" />
///       <arg name="listen_fd" type="fd" summary="listening socket FD" />
///       <arg name="close_fd" type="fd" summary="FD signaling when done" />
///     </request>
/// ```
#[derive(Debug)]
pub struct WpSecurityContextManagerV1CreateListenerRequest {
    /// Wire identifier of the receiver.
    pub object_id: u32,
    /// Wire argument `id`.
    pub id: u32,
    /// listening socket FD.
    pub listen_fd: std::os::fd::OwnedFd,
    /// FD signaling when done.
    pub close_fd: std::os::fd::OwnedFd,
}
impl WpSecurityContextManagerV1CreateListenerRequest {
    /// Opcode in the request namespace.
    pub const OPCODE: u16 = 1;
    /// Minimum supported interface version.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for WpSecurityContextManagerV1CreateListenerRequest {
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
impl InterfaceReader for WpSecurityContextManagerV1CreateListenerRequest {
    async fn read_with_interface(
        object_id: u32,
        opcode: u16,
        interface: ObjectInteface,
        reader: &mut FdPayloadReader<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        if !matches!(
            interface,
            ObjectInteface::SecurityContext(
                SecurityContextObjectInteface::WpSecurityContextManagerV1 { .. }
            )
        ) || opcode != Self::OPCODE
            || interface.version() < Self::SINCE
        {
            return Err(FdReaderError::InvalidProtocol);
        }
        let id = reader.next_u32().await?;
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        let listen_fd = reader.next_fd().ok_or(FdReaderError::InvalidProtocol)?;
        let close_fd = reader.next_fd().ok_or(FdReaderError::InvalidProtocol)?;
        {
            let new_interface =
                ObjectInteface::from_name("wp_security_context_v1", interface.version().min(1))
                    .ok_or(FdReaderError::InvalidProtocol)?;
            Self::reader_state(state)?
                .register_object(id, new_interface, false)
                .map_err(|_| FdReaderError::InvalidProtocol)?;
        }
        Ok(Self {
            object_id,
            id,
            listen_fd,
            close_fd,
        })
    }
}
impl CommandWriter for WpSecurityContextManagerV1CreateListenerRequest {
    fn object_id(&self) -> u32 {
        self.object_id
    }
    fn opcode(&self) -> u16 {
        Self::OPCODE
    }
    fn payload_size(&self) -> usize {
        Self::total_payload_size(&[Some(4), Some(0), Some(0)])
    }
    async fn write(
        self,
        writer: &mut FdPayloadWriter<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<(), FdWriterError> {
        let interface = Self::writer_request_interface(
            state,
            self.object_id,
            "wp_security_context_manager_v1",
            Self::SINCE,
        )?;
        if self.payload_size() != writer.remaining_quota() {
            return Err(FdWriterError::InvalidProtocol);
        }
        {
            let new_interface =
                ObjectInteface::from_name("wp_security_context_v1", interface.version().min(1))
                    .ok_or(FdWriterError::InvalidProtocol)?;
            Self::writer_state(state)?
                .register_object(self.id, new_interface, false)
                .map_err(|_| FdWriterError::InvalidProtocol)?;
        }
        writer.next_u32(self.id).await?;
        writer.next_fd(self.listen_fd).await?;
        writer.next_fd(self.close_fd).await?;
        Ok(())
    }
}
