use crate::protocol::ClientState;
use crate::protocol::InterfaceReader;
use crate::protocol::core::ObjectInteface as CoreObjectInteface;
use crate::protocol::{CommandReader, CommandReaderExt};
use crate::protocol::{CommandWriter, CommandWriterExt, ObjectInteface};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Server event `wl_data_device.data_offer`.
///
/// Opcode 0; available since version 1. The structure contains the receiver and arguments in XML order.
/// Encoding and decoding check wire values and receiver metadata; compositor semantics are not validated.
/// Decoding consumes the complete payload and updates wire object metadata when needed.
/// The caller handles compositor policy and message contents.
/// ```xml
/// <event name="data_offer">
///       <description summary="introduce a new wl_data_offer">
///         The data_offer event introduces a new wl_data_offer object,
///         which will subsequently be used in either the
///         data_device.enter event (for drag-and-drop) or the
///         data_device.selection event (for selections).  Immediately
///         following the data_device.data_offer event, the new data_offer
///         object will send out data_offer.offer events to describe the
///         mime types it offers.
///       </description>
///       <arg name="id" type="new_id" interface="wl_data_offer" summary="the new data_offer object" />
///     </event>
/// ```
#[derive(Debug)]
pub struct WlDataDeviceDataOfferEvent {
    /// Wire identifier of the object receiving the message.
    pub object_id: u32,
    /// Argument `id`: the new data_offer object.
    /// Object implementing interface wl_data_offer.
    pub id: u32,
}
impl WlDataDeviceDataOfferEvent {
    /// Opcode in the event namespace of interface wl_data_device.
    pub const OPCODE: u16 = 0;
    /// Minimum interface version supporting this message.
    pub const SINCE: u32 = 1;
}
impl CommandWriter for WlDataDeviceDataOfferEvent {
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
        let parent =
            Self::writer_event_interface(state, self.object_id, "wl_data_device", Self::SINCE)?;
        if self.payload_size() != writer.remaining_quota() {
            return Err(FdWriterError::InvalidProtocol);
        }
        writer.next_u32(self.id).await?;
        {
            let mut state = Self::writer_state(state)?;
            let new_interface = ObjectInteface::from_name("wl_data_offer", parent.version().min(4))
                .ok_or(FdWriterError::InvalidProtocol)?;
            state
                .register_object(self.id, new_interface, true)
                .map_err(|_| FdWriterError::InvalidProtocol)?;
        }
        Ok(())
    }
}

impl CommandReader<Self> for WlDataDeviceDataOfferEvent {
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

impl InterfaceReader for WlDataDeviceDataOfferEvent {
    async fn read_with_interface(
        object_id: u32,
        opcode: u16,
        interface: ObjectInteface,
        reader: &mut FdPayloadReader<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        if !matches!(
            interface,
            ObjectInteface::Core(CoreObjectInteface::WlDataDevice { .. })
        ) || opcode != Self::OPCODE
            || interface.version() < Self::SINCE
        {
            return Err(FdReaderError::InvalidProtocol);
        }
        let parent = interface;
        let id = reader.next_u32().await?;
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        {
            let interface = ObjectInteface::from_name("wl_data_offer", parent.version().min(4))
                .ok_or(FdReaderError::InvalidProtocol)?;
            Self::reader_state(state)?
                .register_object(id, interface, true)
                .map_err(|_| FdReaderError::InvalidProtocol)?;
        }
        Ok(Self { object_id, id })
    }
}
