use crate::protocol::ext::tablet::ObjectInteface as TabletObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, CommandWriterExt, InterfaceReader,
    ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Server event `zwp_tablet_seat_v2.pad_added`.
///
/// Opcode 2; available since version 1.
/// Wire framing and receiver metadata are checked; protocol semantics belong to the caller.
/// New IDs are registered before writes; close the connection after a failed or cancelled write.
/// ```xml
/// <event name="pad_added">
///       <description summary="new pad notification">
///     This event is sent whenever a new pad is known to the system. Typically,
///     pads are physically attached to tablets and a pad_added event is
///     sent immediately after the zwp_tablet_seat_v2.tablet_added.
///     However, some standalone pad devices logically attach to tablets at
///     runtime, and the client must wait for zwp_tablet_pad_v2.enter to know
///     the tablet a pad is attached to.
///
///     This event only provides the object id of the pad. All further
///     features (buttons, strips, rings) are sent through the zwp_tablet_pad_v2
///     interface.
///       </description>
///       <arg name="id" type="new_id" interface="zwp_tablet_pad_v2" summary="the newly added pad" />
///     </event>
/// ```
#[derive(Debug)]
pub struct ZwpTabletSeatV2PadAddedEvent {
    /// Wire identifier of the receiver.
    pub object_id: u32,
    /// the newly added pad.
    pub id: u32,
}
impl ZwpTabletSeatV2PadAddedEvent {
    /// Opcode in the event namespace.
    pub const OPCODE: u16 = 2;
    /// Minimum supported interface version.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for ZwpTabletSeatV2PadAddedEvent {
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
impl InterfaceReader for ZwpTabletSeatV2PadAddedEvent {
    async fn read_with_interface(
        object_id: u32,
        opcode: u16,
        interface: ObjectInteface,
        reader: &mut FdPayloadReader<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        if !matches!(
            interface,
            ObjectInteface::Tablet(TabletObjectInteface::ZwpTabletSeatV2 { .. })
        ) || opcode != Self::OPCODE
            || interface.version() < Self::SINCE
        {
            return Err(FdReaderError::InvalidProtocol);
        }
        let id = reader.next_u32().await?;
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        {
            let new_interface = ObjectInteface::from_name("zwp_tablet_pad_v2", interface.version())
                .ok_or(FdReaderError::InvalidProtocol)?;
            Self::reader_state(state)?
                .register_object(id, new_interface, true)
                .map_err(|_| FdReaderError::InvalidProtocol)?;
        }
        Ok(Self { object_id, id })
    }
}
impl CommandWriter for ZwpTabletSeatV2PadAddedEvent {
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
        let interface =
            Self::writer_event_interface(state, self.object_id, "zwp_tablet_seat_v2", Self::SINCE)?;
        if self.payload_size() != writer.remaining_quota() {
            return Err(FdWriterError::InvalidProtocol);
        }
        {
            let new_interface = ObjectInteface::from_name("zwp_tablet_pad_v2", interface.version())
                .ok_or(FdWriterError::InvalidProtocol)?;
            Self::writer_state(state)?
                .register_object(self.id, new_interface, true)
                .map_err(|_| FdWriterError::InvalidProtocol)?;
        }
        writer.next_u32(self.id).await?;
        Ok(())
    }
}
