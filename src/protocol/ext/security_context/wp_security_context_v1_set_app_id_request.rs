use crate::protocol::ext::security_context::ObjectInteface as SecurityContextObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, CommandWriterExt, InterfaceReader,
    ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::FdWriter;
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Client request `wp_security_context_v1.set_app_id`.
///
/// Opcode 2; available since version 1.
/// Wire framing and receiver metadata are checked; protocol semantics belong to the caller.
/// ```xml
/// <request name="set_app_id">
///       <description summary="set the application ID">
///         Attach an application ID to the security context.
///
///         The application ID is an opaque, sandbox-specific identifier for an
///         application. See the well-known engines document for more details:
///         https://gitlab.freedesktop.org/wayland/wayland-protocols/-/blob/main/staging/security-context/engines.md
///
///         The compositor may use the application ID to group clients belonging to
///         the same security context application.
///
///         Whether this request is optional or not depends on the sandbox engine used.
///
///         It is a protocol error to call this request twice. The already_set
///         error is sent in this case.
///       </description>
///       <arg name="app_id" type="string" summary="the application ID" />
///     </request>
/// ```
#[derive(Debug)]
pub struct WpSecurityContextV1SetAppIdRequest {
    /// Wire identifier of the receiver.
    pub object_id: u32,
    /// the application ID.
    pub app_id: String,
}
impl WpSecurityContextV1SetAppIdRequest {
    /// Opcode in the request namespace.
    pub const OPCODE: u16 = 2;
    /// Minimum supported interface version.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for WpSecurityContextV1SetAppIdRequest {
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
impl InterfaceReader for WpSecurityContextV1SetAppIdRequest {
    async fn read_with_interface(
        object_id: u32,
        opcode: u16,
        interface: ObjectInteface,
        reader: &mut FdPayloadReader<'_>,
        _state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        if !matches!(
            interface,
            ObjectInteface::SecurityContext(
                SecurityContextObjectInteface::WpSecurityContextV1 { .. }
            )
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
impl CommandWriter for WpSecurityContextV1SetAppIdRequest {
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
        Self::writer_request_interface(
            state,
            self.object_id,
            "wp_security_context_v1",
            Self::SINCE,
        )?;
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
