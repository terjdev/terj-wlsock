use crate::protocol::ext::data_control::ObjectInteface as DataControlObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, CommandWriterExt, InterfaceReader,
    ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Server event `ext_data_control_device_v1.selection`.
///
/// Opcode 1; available since version 1.
/// Wire framing and receiver metadata are checked; protocol semantics belong to the caller.
/// ```xml
/// <event name="selection">
///       <description summary="advertise new selection">
///         The selection event is sent out to notify the client of a new
///         ext_data_control_offer for the selection for this device. The
///         ext_data_control_device.data_offer and the ext_data_control_offer.offer
///         events are sent out immediately before this event to introduce the data
///         offer object. The selection event is sent to a client when a new
///         selection is set. The ext_data_control_offer is valid until a new
///         ext_data_control_offer or NULL is received. The client must destroy the
///         previous selection ext_data_control_offer, if any, upon receiving this
///         event. Regardless, the previous selection will be ignored once a new
///         selection ext_data_control_offer is received.
///
///         The first selection event is sent upon binding the
///         ext_data_control_device object.
///       </description>
///       <arg name="id" type="object" interface="ext_data_control_offer_v1" allow-null="true" />
///     </event>
/// ```
#[derive(Debug)]
pub struct ExtDataControlDeviceV1SelectionEvent {
    /// Wire identifier of the receiver.
    pub object_id: u32,
    /// Wire argument `id`.
    pub id: Option<u32>,
}
impl ExtDataControlDeviceV1SelectionEvent {
    /// Opcode in the event namespace.
    pub const OPCODE: u16 = 1;
    /// Minimum supported interface version.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for ExtDataControlDeviceV1SelectionEvent {
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
impl InterfaceReader for ExtDataControlDeviceV1SelectionEvent {
    async fn read_with_interface(
        object_id: u32,
        opcode: u16,
        interface: ObjectInteface,
        reader: &mut FdPayloadReader<'_>,
        _state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        if !matches!(
            interface,
            ObjectInteface::DataControl(DataControlObjectInteface::ExtDataControlDeviceV1 { .. })
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
impl CommandWriter for ExtDataControlDeviceV1SelectionEvent {
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
            "ext_data_control_device_v1",
            Self::SINCE,
        )?;
        if self.payload_size() != writer.remaining_quota() {
            return Err(FdWriterError::InvalidProtocol);
        }
        writer.next_u32(self.id.unwrap_or(0)).await?;
        Ok(())
    }
}
