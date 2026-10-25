use crate::protocol::ext::idle_notify::ObjectInteface as IdleNotifyObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, CommandWriterExt, InterfaceReader,
    ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Client request `ext_idle_notifier_v1.get_idle_notification`.
///
/// Opcode 1; available since version 1.
/// Wire framing and receiver metadata are checked; protocol semantics belong to the caller.
/// New IDs are registered before writes; close the connection after a failed or cancelled write.
/// ```xml
/// <request name="get_idle_notification">
///       <description summary="create a notification object">
///         Create a new idle notification object.
///
///         The notification object has a minimum timeout duration and is tied to a
///         seat. The client will be notified if the seat is inactive for at least
///         the provided timeout. See ext_idle_notification_v1 for more details.
///
///         A zero timeout is valid and means the client wants to be notified as
///         soon as possible when the seat is inactive.
///       </description>
///       <arg name="id" type="new_id" interface="ext_idle_notification_v1" />
///       <arg name="timeout" type="uint" summary="minimum idle timeout in msec" />
///       <arg name="seat" type="object" interface="wl_seat" />
///     </request>
/// ```
#[derive(Debug)]
pub struct ExtIdleNotifierV1GetIdleNotificationRequest {
    /// Wire identifier of the receiver.
    pub object_id: u32,
    /// Wire argument `id`.
    pub id: u32,
    /// minimum idle timeout in msec.
    pub timeout: u32,
    /// Wire argument `seat`.
    pub seat: u32,
}
impl ExtIdleNotifierV1GetIdleNotificationRequest {
    /// Opcode in the request namespace.
    pub const OPCODE: u16 = 1;
    /// Minimum supported interface version.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for ExtIdleNotifierV1GetIdleNotificationRequest {
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
impl InterfaceReader for ExtIdleNotifierV1GetIdleNotificationRequest {
    async fn read_with_interface(
        object_id: u32,
        opcode: u16,
        interface: ObjectInteface,
        reader: &mut FdPayloadReader<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        if !matches!(
            interface,
            ObjectInteface::IdleNotify(IdleNotifyObjectInteface::ExtIdleNotifierV1 { .. })
        ) || opcode != Self::OPCODE
            || interface.version() < Self::SINCE
        {
            return Err(FdReaderError::InvalidProtocol);
        }
        let id = reader.next_u32().await?;
        let timeout = reader.next_u32().await?;
        let seat = reader.next_u32().await?;
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        {
            let new_interface =
                ObjectInteface::from_name("ext_idle_notification_v1", interface.version().min(2))
                    .ok_or(FdReaderError::InvalidProtocol)?;
            Self::reader_state(state)?
                .register_object(id, new_interface, false)
                .map_err(|_| FdReaderError::InvalidProtocol)?;
        }
        Ok(Self {
            object_id,
            id,
            timeout,
            seat,
        })
    }
}
impl CommandWriter for ExtIdleNotifierV1GetIdleNotificationRequest {
    fn object_id(&self) -> u32 {
        self.object_id
    }
    fn opcode(&self) -> u16 {
        Self::OPCODE
    }
    fn payload_size(&self) -> usize {
        Self::total_payload_size(&[Some(4), Some(4), Some(4)])
    }
    async fn write(
        self,
        writer: &mut FdPayloadWriter<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<(), FdWriterError> {
        let interface = Self::writer_request_interface(
            state,
            self.object_id,
            "ext_idle_notifier_v1",
            Self::SINCE,
        )?;
        if self.payload_size() != writer.remaining_quota() {
            return Err(FdWriterError::InvalidProtocol);
        }
        {
            let new_interface =
                ObjectInteface::from_name("ext_idle_notification_v1", interface.version().min(2))
                    .ok_or(FdWriterError::InvalidProtocol)?;
            Self::writer_state(state)?
                .register_object(self.id, new_interface, false)
                .map_err(|_| FdWriterError::InvalidProtocol)?;
        }
        writer.next_u32(self.id).await?;
        writer.next_u32(self.timeout).await?;
        writer.next_u32(self.seat).await?;
        Ok(())
    }
}
