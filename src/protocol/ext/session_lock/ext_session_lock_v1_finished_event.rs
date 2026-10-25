use crate::protocol::ext::session_lock::ObjectInteface as SessionLockObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, CommandWriterExt, InterfaceReader,
    ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Server event `ext_session_lock_v1.finished`.
///
/// Opcode 1; available since version 1.
/// Wire framing and receiver metadata are checked; protocol semantics belong to the caller.
/// ```xml
/// <event name="finished">
///       <description summary="the session lock object should be destroyed">
///         The compositor has decided that the session lock should be destroyed
///         as it will no longer be used by the compositor. Exactly when this
///         event is sent is compositor policy, but it must never be sent more
///         than once for a given session lock object.
///
///         This might be sent because there is already another ext_session_lock_v1
///         object held by a client, or the compositor has decided to deny the
///         request to lock the session for some other reason. This might also
///         be sent because the compositor implements some alternative, secure
///         way to authenticate and unlock the session.
///
///         The finished event should be sent immediately on creation of this
///         object if the compositor decides that the locked event will not
///         be sent.
///
///         If the locked event is sent on creation of this object the finished
///         event may still be sent at some later time in this object's
///         lifetime. This is compositor policy.
///
///         Upon receiving this event, the client should make either the destroy
///         request or the unlock_and_destroy request, depending on whether or
///         not the locked event was received on this object.
///       </description>
///     </event>
/// ```
#[derive(Debug)]
pub struct ExtSessionLockV1FinishedEvent {
    /// Wire identifier of the receiver.
    pub object_id: u32,
}
impl ExtSessionLockV1FinishedEvent {
    /// Opcode in the event namespace.
    pub const OPCODE: u16 = 1;
    /// Minimum supported interface version.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for ExtSessionLockV1FinishedEvent {
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
impl InterfaceReader for ExtSessionLockV1FinishedEvent {
    async fn read_with_interface(
        object_id: u32,
        opcode: u16,
        interface: ObjectInteface,
        reader: &mut FdPayloadReader<'_>,
        _state: &Arc<Mutex<ClientState>>,
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
        Ok(Self { object_id })
    }
}
impl CommandWriter for ExtSessionLockV1FinishedEvent {
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
        Self::writer_event_interface(state, self.object_id, "ext_session_lock_v1", Self::SINCE)?;
        if self.payload_size() != writer.remaining_quota() {
            return Err(FdWriterError::InvalidProtocol);
        }
        Ok(())
    }
}
