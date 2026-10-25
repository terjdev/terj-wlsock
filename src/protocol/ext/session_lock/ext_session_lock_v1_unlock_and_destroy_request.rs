use crate::protocol::ext::session_lock::ObjectInteface as SessionLockObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, CommandWriterExt, InterfaceReader,
    ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Client request `ext_session_lock_v1.unlock_and_destroy`.
///
/// Opcode 2; available since version 1.
/// Wire framing and receiver metadata are checked; protocol semantics belong to the caller.
/// ```xml
/// <request name="unlock_and_destroy" type="destructor">
///       <description summary="unlock the session, destroying the object">
///         This request indicates that the session should be unlocked, for
///         example because the user has entered their password and it has been
///         verified by the client.
///
///         This request also informs the compositor that the lock object will
///         no longer be used and should be destroyed. Existing objects created
///         through this interface remain valid.
///
///         After this request is made, lock surfaces created through this object
///         should be destroyed by the client as they will no longer be used by
///         the compositor.
///
///         It is a protocol error to make this request if the locked event has
///         not been sent. In that case, the lock object must be destroyed using
///         the destroy request.
///
///         Note that a correct client that wishes to exit directly after unlocking
///         the session must use the wl_display.sync request to ensure the server
///         receives and processes the unlock_and_destroy request. Otherwise
///         there is no guarantee that the server has unlocked the session due
///         to the asynchronous nature of the Wayland protocol. For example,
///         the server might terminate the client with a protocol error before
///         it processes the unlock_and_destroy request.
///       </description>
///     </request>
/// ```
#[derive(Debug)]
pub struct ExtSessionLockV1UnlockAndDestroyRequest {
    /// Wire identifier of the receiver.
    pub object_id: u32,
}
impl ExtSessionLockV1UnlockAndDestroyRequest {
    /// Opcode in the request namespace.
    pub const OPCODE: u16 = 2;
    /// Minimum supported interface version.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for ExtSessionLockV1UnlockAndDestroyRequest {
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
impl InterfaceReader for ExtSessionLockV1UnlockAndDestroyRequest {
    async fn read_with_interface(
        object_id: u32,
        opcode: u16,
        interface: ObjectInteface,
        reader: &mut FdPayloadReader<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        if !matches!(
            interface,
            ObjectInteface::SessionLock(SessionLockObjectInteface::ExtSessionLockV1 { .. })
        ) || opcode != Self::OPCODE
            || interface.version() < Self::SINCE
        {
            return Err(FdReaderError::InvalidProtocol);
        }
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        Self::reader_state(state)?.release_server_id(object_id);
        Ok(Self { object_id })
    }
}
impl CommandWriter for ExtSessionLockV1UnlockAndDestroyRequest {
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
        Self::writer_request_interface(state, self.object_id, "ext_session_lock_v1", Self::SINCE)?;
        if self.payload_size() != writer.remaining_quota() {
            return Err(FdWriterError::InvalidProtocol);
        }
        Self::writer_state(state)?.release_server_id(self.object_id);
        Ok(())
    }
}
