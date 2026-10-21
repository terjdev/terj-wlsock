use crate::protocol::ClientState;
use crate::protocol::{CommandWriter, CommandWriterExt};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Server event `wl_data_device.selection`.
///
/// Opcode 5; available since version 1. The structure contains the receiver and arguments in XML order.
/// Encoding checks wire values and receiver metadata; compositor semantics are not validated.
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
        Self::event_interface(state, self.object_id, "wl_data_device", Self::SINCE)?;
        if self.payload_size() != writer.remaining_quota() {
            return Err(FdWriterError::InvalidProtocol);
        }
        writer.next_u32(self.id.unwrap_or(0)).await?;
        Ok(())
    }
}
