use super::WlDataDeviceManagerDndAction;
use crate::protocol::ClientState;
use crate::protocol::{CommandReader, CommandReaderExt};
use crate::protocol::{CommandWriter, CommandWriterExt};
use crate::protocol::{InterfaceReader, ObjectInteface};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Server event `wl_data_offer.action`.
///
/// Opcode 2; available since version 3. The structure contains the receiver and arguments in XML order.
/// Encoding and decoding check wire values and receiver metadata; compositor semantics are not validated.
/// Decoding consumes the complete payload and updates wire object metadata when needed.
/// The caller handles compositor policy and message contents.
/// ```xml
/// <event name="action" since="3">
///       <description summary="notify the selected action">
///         This event indicates the action selected by the compositor after
///         matching the source/destination side actions. Only one action (or
///         none) will be offered here.
///
///         This event can be emitted multiple times during the drag-and-drop
///         operation in response to destination side action changes through
///         wl_data_offer.set_actions.
///
///         This event will no longer be emitted after wl_data_device.drop
///         happened on the drag-and-drop destination, the client must
///         honor the last action received, or the last preferred one set
///         through wl_data_offer.set_actions when handling an "ask" action.
///
///         Compositors may also change the selected action on the fly, mainly
///         in response to keyboard modifier changes during the drag-and-drop
///         operation.
///
///         The most recent action received is always the valid one. Prior to
///         receiving wl_data_device.drop, the chosen action may change (e.g.
///         due to keyboard modifiers being pressed). At the time of receiving
///         wl_data_device.drop the drag-and-drop destination must honor the
///         last action received.
///
///         Action changes may still happen after wl_data_device.drop,
///         especially on "ask" actions, where the drag-and-drop destination
///         may choose another action afterwards. Action changes happening
///         at this stage are always the result of inter-client negotiation, the
///         compositor shall no longer be able to induce a different action.
///
///         Upon "ask" actions, it is expected that the drag-and-drop destination
///         may potentially choose a different action and/or mime type,
///         based on wl_data_offer.source_actions and finally chosen by the
///         user (e.g. popping up a menu with the available options). The
///         final wl_data_offer.set_actions and wl_data_offer.accept requests
///         must happen before the call to wl_data_offer.finish.
///       </description>
///       <arg name="dnd_action" type="uint" summary="action selected by the compositor" enum="wl_data_device_manager.dnd_action" />
///     </event>
/// ```
#[derive(Debug)]
pub struct WlDataOfferActionEvent {
    /// Wire identifier of the object receiving the message.
    pub object_id: u32,
    /// Argument `dnd_action`: action selected by the compositor.
    pub dnd_action: WlDataDeviceManagerDndAction,
}
impl WlDataOfferActionEvent {
    /// Opcode in the event namespace of interface wl_data_offer.
    pub const OPCODE: u16 = 2;
    /// Minimum interface version supporting this message.
    pub const SINCE: u32 = 3;
}
impl CommandWriter for WlDataOfferActionEvent {
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
        Self::writer_event_interface(state, self.object_id, "wl_data_offer", Self::SINCE)?;
        if self.payload_size() != writer.remaining_quota() {
            return Err(FdWriterError::InvalidProtocol);
        }
        writer.next_u32(self.dnd_action.0).await?;
        Ok(())
    }
}

impl CommandReader<Self> for WlDataOfferActionEvent {
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

impl InterfaceReader for WlDataOfferActionEvent {
    async fn read_with_interface(
        object_id: u32,
        opcode: u16,
        interface: ObjectInteface,
        reader: &mut FdPayloadReader<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        if !matches!(interface, ObjectInteface::WlDataOffer { .. })
            || opcode != Self::OPCODE
            || interface.version() < Self::SINCE
        {
            return Err(FdReaderError::InvalidProtocol);
        }
        let dnd_action =
            WlDataDeviceManagerDndAction::read(object_id, opcode, reader, state).await?;
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        Ok(Self {
            object_id,
            dnd_action,
        })
    }
}
