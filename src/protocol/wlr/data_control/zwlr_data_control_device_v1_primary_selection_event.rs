use crate::protocol::wlr::data_control::ObjectInteface as WlrDataControlObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, CommandWriterExt, InterfaceReader,
    ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Server event `zwlr_data_control_device_v1.primary_selection`.
///
/// Opcode 3; available since version 2.
/// Wire framing and receiver metadata are checked; protocol semantics belong to the caller.
/// ```xml
/// <event name="primary_selection" since="2">
///       <description summary="advertise new primary selection">
///         The primary_selection event is sent out to notify the client of a new
///         wlr_data_control_offer for the primary selection for this device. The
///         wlr_data_control_device.data_offer and the wlr_data_control_offer.offer
///         events are sent out immediately before this event to introduce the data
///         offer object. The primary_selection event is sent to a client when a
///         new primary selection is set. The wlr_data_control_offer is valid until
///         a new wlr_data_control_offer or NULL is received. The client must
///         destroy the previous primary selection wlr_data_control_offer, if any,
///         upon receiving this event.
///
///         If the compositor supports primary selection, the first
///         primary_selection event is sent upon binding the
///         wlr_data_control_device object.
///       </description>
///       <arg name="id" type="object" interface="zwlr_data_control_offer_v1" allow-null="true" />
///     </event>
/// ```
#[derive(Debug)]
pub struct ZwlrDataControlDeviceV1PrimarySelectionEvent {
    /// Wire identifier of the receiver.
    pub object_id: u32,
    /// Wire argument `id`.
    pub id: Option<u32>,
}
impl ZwlrDataControlDeviceV1PrimarySelectionEvent {
    /// Opcode in the event namespace.
    pub const OPCODE: u16 = 3;
    /// Minimum supported interface version.
    pub const SINCE: u32 = 2;
}
impl CommandReader<Self> for ZwlrDataControlDeviceV1PrimarySelectionEvent {
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
impl InterfaceReader for ZwlrDataControlDeviceV1PrimarySelectionEvent {
    async fn read_with_interface(
        object_id: u32,
        opcode: u16,
        interface: ObjectInteface,
        reader: &mut FdPayloadReader<'_>,
        _state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        if !matches!(
            interface,
            ObjectInteface::WlrDataControl(
                WlrDataControlObjectInteface::ZwlrDataControlDeviceV1 { .. }
            )
        ) || opcode != Self::OPCODE
            || interface.version() < Self::SINCE
        {
            return Err(FdReaderError::InvalidProtocol);
        }
        let id = {
            let id = reader.next_u32().await?;
            (id != 0).then_some(id)
        };
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        Ok(Self { object_id, id })
    }
}
impl CommandWriter for ZwlrDataControlDeviceV1PrimarySelectionEvent {
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
        Self::writer_event_interface(
            state,
            self.object_id,
            "zwlr_data_control_device_v1",
            Self::SINCE,
        )?;
        if self.payload_size() != writer.remaining_quota() {
            return Err(FdWriterError::InvalidProtocol);
        }
        writer.next_u32(self.id.unwrap_or(0)).await?;
        Ok(())
    }
}
