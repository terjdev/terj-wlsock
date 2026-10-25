use crate::protocol::ext::tablet::ObjectInteface as TabletObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, CommandWriterExt, InterfaceReader,
    ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Server event `zwp_tablet_pad_group_v2.strip`.
///
/// Opcode 2; available since version 1.
/// Wire framing and receiver metadata are checked; protocol semantics belong to the caller.
/// New IDs are registered before writes; close the connection after a failed or cancelled write.
/// ```xml
/// <event name="strip">
///       <description summary="strip announced">
///     Sent on zwp_tablet_pad_v2 initialization to announce available strips.
///     One event is sent for each strip available on this pad group.
///
///     This event is sent in the initial burst of events before the
///     zwp_tablet_pad_group_v2.done event.
///       </description>
///       <arg name="strip" type="new_id" interface="zwp_tablet_pad_strip_v2" />
///     </event>
/// ```
#[derive(Debug)]
pub struct ZwpTabletPadGroupV2StripEvent {
    /// Wire identifier of the receiver.
    pub object_id: u32,
    /// Wire argument `strip`.
    pub strip: u32,
}
impl ZwpTabletPadGroupV2StripEvent {
    /// Opcode in the event namespace.
    pub const OPCODE: u16 = 2;
    /// Minimum supported interface version.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for ZwpTabletPadGroupV2StripEvent {
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
impl InterfaceReader for ZwpTabletPadGroupV2StripEvent {
    async fn read_with_interface(
        object_id: u32,
        opcode: u16,
        interface: ObjectInteface,
        reader: &mut FdPayloadReader<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        if !matches!(
            interface,
            ObjectInteface::Tablet(TabletObjectInteface::ZwpTabletPadGroupV2 { .. })
        ) || opcode != Self::OPCODE
            || interface.version() < Self::SINCE
        {
            return Err(FdReaderError::InvalidProtocol);
        }
        let strip = reader.next_u32().await?;
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        {
            let new_interface =
                ObjectInteface::from_name("zwp_tablet_pad_strip_v2", interface.version())
                    .ok_or(FdReaderError::InvalidProtocol)?;
            Self::reader_state(state)?
                .register_object(strip, new_interface, true)
                .map_err(|_| FdReaderError::InvalidProtocol)?;
        }
        Ok(Self { object_id, strip })
    }
}
impl CommandWriter for ZwpTabletPadGroupV2StripEvent {
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
        let interface = Self::writer_event_interface(
            state,
            self.object_id,
            "zwp_tablet_pad_group_v2",
            Self::SINCE,
        )?;
        if self.payload_size() != writer.remaining_quota() {
            return Err(FdWriterError::InvalidProtocol);
        }
        {
            let new_interface =
                ObjectInteface::from_name("zwp_tablet_pad_strip_v2", interface.version())
                    .ok_or(FdWriterError::InvalidProtocol)?;
            Self::writer_state(state)?
                .register_object(self.strip, new_interface, true)
                .map_err(|_| FdWriterError::InvalidProtocol)?;
        }
        writer.next_u32(self.strip).await?;
        Ok(())
    }
}
