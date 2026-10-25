use crate::protocol::ClientState;
use crate::protocol::core::ObjectInteface as CoreObjectInteface;
use crate::protocol::{CommandReader, CommandReaderExt};
use crate::protocol::{CommandWriter, CommandWriterExt};
use crate::protocol::{InterfaceReader, ObjectInteface};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Server event `wl_data_device.selection`.
///
/// Opcode 5; available since version 1. The structure contains the receiver and arguments in XML order.
/// Encoding and decoding check wire values and receiver metadata; compositor semantics are not validated.
/// Decoding consumes the complete payload and updates wire object metadata when needed.
/// The caller handles compositor policy and message contents.
/// ```xml
/// <event name="selection">
///       <description summary="advertise new selection">
///         The selection event is sent out to notify the client of a new
///         wl_data_offer for the selection for this device.  The
///         data_device.data_offer and the data_offer.offer events are
///         sent out immediately before this event to introduce the data
///         offer object.  The selection event is sent to a client
///         immediately before receiving keyboard focus and when a new
///         selection is set while the client has keyboard focus.  The
///         data_offer is valid until a new data_offer or NULL is received
///         or until the client loses keyboard focus.  Switching surface with
///         keyboard focus within the same client doesn't mean a new selection
///         will be sent.  The client must destroy the previous selection
///         data_offer, if any, upon receiving this event.
///       </description>
///       <arg name="id" type="object" interface="wl_data_offer" allow-null="true" summary="selection data_offer object" />
///     </event>
/// ```
#[derive(Debug)]
pub struct WlDataDeviceSelectionEvent {
    /// Wire identifier of the object receiving the message.
    pub object_id: u32,
    /// Argument `id`: selection data_offer object.
    /// The protocol permits NULL.
    /// Object implementing interface wl_data_offer.
    pub id: Option<u32>,
}
impl WlDataDeviceSelectionEvent {
    /// Opcode in the event namespace of interface wl_data_device.
    pub const OPCODE: u16 = 5;
    /// Minimum interface version supporting this message.
    pub const SINCE: u32 = 1;
}
impl CommandWriter for WlDataDeviceSelectionEvent {
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
        Self::writer_event_interface(state, self.object_id, "wl_data_device", Self::SINCE)?;
        if self.payload_size() != writer.remaining_quota() {
            return Err(FdWriterError::InvalidProtocol);
        }
        writer.next_u32(self.id.unwrap_or(0)).await?;
        Ok(())
    }
}

impl CommandReader<Self> for WlDataDeviceSelectionEvent {
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

impl InterfaceReader for WlDataDeviceSelectionEvent {
    async fn read_with_interface(
        object_id: u32,
        opcode: u16,
        interface: ObjectInteface,
        reader: &mut FdPayloadReader<'_>,
        _state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        if !matches!(
            interface,
            ObjectInteface::Core(CoreObjectInteface::WlDataDevice { .. })
        ) || opcode != Self::OPCODE
            || interface.version() < Self::SINCE
        {
            return Err(FdReaderError::InvalidProtocol);
        }
        let id = {
            let id = reader.next_u32().await?;
            if id == 0 { None } else { Some(id) }
        };
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        Ok(Self { object_id, id })
    }
}
