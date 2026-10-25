use crate::protocol::ext::session_lock::ObjectInteface as SessionLockObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, CommandWriterExt, InterfaceReader,
    ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Client request `ext_session_lock_surface_v1.ack_configure`.
///
/// Opcode 1; available since version 1.
/// Wire framing and receiver metadata are checked; protocol semantics belong to the caller.
/// ```xml
/// <request name="ack_configure">
///       <description summary="ack a configure event">
///         When a configure event is received, if a client commits the surface
///         in response to the configure event, then the client must make an
///         ack_configure request sometime before the commit request, passing
///         along the serial of the configure event.
///
///         If the client receives multiple configure events before it can
///         respond to one, it only has to ack the last configure event.
///
///         A client is not required to commit immediately after sending an
///         ack_configure request - it may even ack_configure several times
///         before its next surface commit.
///
///         A client may send multiple ack_configure requests before committing,
///         but only the last request sent before a commit indicates which
///         configure event the client really is responding to.
///
///         Sending an ack_configure request consumes the configure event
///         referenced by the given serial, as well as all older configure events
///         sent on this object.
///
///         It is a protocol error to issue multiple ack_configure requests
///         referencing the same configure event or to issue an ack_configure
///         request referencing a configure event older than the last configure
///         event acked for a given lock surface.
///       </description>
///       <arg name="serial" type="uint" summary="serial from the configure event" />
///     </request>
/// ```
#[derive(Debug)]
pub struct ExtSessionLockSurfaceV1AckConfigureRequest {
    /// Wire identifier of the receiver.
    pub object_id: u32,
    /// serial from the configure event.
    pub serial: u32,
}
impl ExtSessionLockSurfaceV1AckConfigureRequest {
    /// Opcode in the request namespace.
    pub const OPCODE: u16 = 1;
    /// Minimum supported interface version.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for ExtSessionLockSurfaceV1AckConfigureRequest {
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
impl InterfaceReader for ExtSessionLockSurfaceV1AckConfigureRequest {
    async fn read_with_interface(
        object_id: u32,
        opcode: u16,
        interface: ObjectInteface,
        reader: &mut FdPayloadReader<'_>,
        _state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        if !matches!(
            interface,
            ObjectInteface::SessionLock(SessionLockObjectInteface::ExtSessionLockSurfaceV1 { .. })
        ) || opcode != Self::OPCODE
            || interface.version() < Self::SINCE
        {
            return Err(FdReaderError::InvalidProtocol);
        }
        let serial = reader.next_u32().await?;
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        Ok(Self { object_id, serial })
    }
}
impl CommandWriter for ExtSessionLockSurfaceV1AckConfigureRequest {
    fn object_id(&self) -> u32 {
        self.object_id
    }
    fn opcode(&self) -> u16 {
        Self::OPCODE
    }
    fn payload_size(&self) -> usize {
        Self::total_payload_size(&[Some(4)])
    }
    async fn write(
        self,
        writer: &mut FdPayloadWriter<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<(), FdWriterError> {
        Self::writer_request_interface(
            state,
            self.object_id,
            "ext_session_lock_surface_v1",
            Self::SINCE,
        )?;
        if self.payload_size() != writer.remaining_quota() {
            return Err(FdWriterError::InvalidProtocol);
        }
        writer.next_u32(self.serial).await?;
        Ok(())
    }
}
