use crate::protocol::ext::tablet::ObjectInteface as TabletObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, CommandWriterExt, InterfaceReader,
    ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Server event `zwp_tablet_v2.id`.
///
/// Opcode 1; available since version 1.
/// Wire framing and receiver metadata are checked; protocol semantics belong to the caller.
/// ```xml
/// <event name="id">
///       <description summary="tablet device vendor/product id">
///     The vendor and product IDs for the tablet device.
///
///     The interpretation of the id depends on the zwp_tablet_v2.bustype.
///     Prior to version v2 of this protocol, the id was implied to be a USB
///     vendor and product ID. If no zwp_tablet_v2.bustype is sent, the ID
///     is to be interpreted as USB vendor and product ID.
///
///     If the device has no vendor/product ID, this event is not sent.
///     This can happen for virtual devices or non-USB devices, for instance.
///
///     This event is sent in the initial burst of events before the
///     zwp_tablet_v2.done event.
///       </description>
///       <arg name="vid" type="uint" summary="vendor id" />
///       <arg name="pid" type="uint" summary="product id" />
///     </event>
/// ```
#[derive(Debug)]
pub struct ZwpTabletV2IdEvent {
    /// Wire identifier of the receiver.
    pub object_id: u32,
    /// vendor id.
    pub vid: u32,
    /// product id.
    pub pid: u32,
}
impl ZwpTabletV2IdEvent {
    /// Opcode in the event namespace.
    pub const OPCODE: u16 = 1;
    /// Minimum supported interface version.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for ZwpTabletV2IdEvent {
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
impl InterfaceReader for ZwpTabletV2IdEvent {
    async fn read_with_interface(
        object_id: u32,
        opcode: u16,
        interface: ObjectInteface,
        reader: &mut FdPayloadReader<'_>,
        _state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        if !matches!(
            interface,
            ObjectInteface::Tablet(TabletObjectInteface::ZwpTabletV2 { .. })
        ) || opcode != Self::OPCODE
            || interface.version() < Self::SINCE
        {
            return Err(FdReaderError::InvalidProtocol);
        }
        let vid = reader.next_u32().await?;
        let pid = reader.next_u32().await?;
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        Ok(Self {
            object_id,
            vid,
            pid,
        })
    }
}
impl CommandWriter for ZwpTabletV2IdEvent {
    fn object_id(&self) -> u32 {
        self.object_id
    }
    fn opcode(&self) -> u16 {
        Self::OPCODE
    }
    fn payload_size(&self) -> usize {
        Self::total_payload_size(&[Some(4), Some(4)])
    }
    async fn write(
        self,
        writer: &mut FdPayloadWriter<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<(), FdWriterError> {
        Self::writer_event_interface(state, self.object_id, "zwp_tablet_v2", Self::SINCE)?;
        if self.payload_size() != writer.remaining_quota() {
            return Err(FdWriterError::InvalidProtocol);
        }
        writer.next_u32(self.vid).await?;
        writer.next_u32(self.pid).await?;
        Ok(())
    }
}
