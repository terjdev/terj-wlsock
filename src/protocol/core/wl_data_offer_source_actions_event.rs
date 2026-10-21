use super::WlDataDeviceManagerDndAction;
use crate::protocol::ClientState;
use crate::protocol::{CommandReader, CommandReaderExt};
use crate::protocol::{CommandWriter, CommandWriterExt};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Server event `wl_data_offer.source_actions`.
///
/// Opcode 1; available since version 3. The structure contains the receiver and arguments in XML order.
/// Encoding and decoding check wire values and receiver metadata; compositor semantics are not validated.
/// Decoding consumes the complete payload and updates wire object metadata when needed.
/// The caller handles compositor policy and message contents.
/// ```xml
/// <event name="source_actions" since="3">
///       <description summary="notify the source-side available actions">
///         This event indicates the actions offered by the data source. It
///         will be sent immediately after creating the wl_data_offer object,
///         or anytime the source side changes its offered actions through
///         wl_data_source.set_actions.
///       </description>
///       <arg name="source_actions" type="uint" summary="actions offered by the data source" enum="wl_data_device_manager.dnd_action" />
///     </event>
/// ```
#[derive(Debug)]
pub struct WlDataOfferSourceActionsEvent {
    /// Wire identifier of the object receiving the message.
    pub object_id: u32,
    /// Argument `source_actions`: actions offered by the data source.
    pub source_actions: WlDataDeviceManagerDndAction,
}
impl WlDataOfferSourceActionsEvent {
    /// Opcode in the event namespace of interface wl_data_offer.
    pub const OPCODE: u16 = 1;
    /// Minimum interface version supporting this message.
    pub const SINCE: u32 = 3;
}
impl CommandWriter for WlDataOfferSourceActionsEvent {
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
        writer.next_u32(self.source_actions.0).await?;
        Ok(())
    }
}

impl CommandReader<Self> for WlDataOfferSourceActionsEvent {
    async fn read(
        object_id: u32,
        opcode: u16,
        reader: &mut FdPayloadReader<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        Self::reader_event_interface(
            state,
            object_id,
            opcode,
            Self::OPCODE,
            "wl_data_offer",
            Self::SINCE,
        )?;
        let source_actions =
            WlDataDeviceManagerDndAction::read(object_id, opcode, reader, state).await?;
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        Ok(Self {
            object_id,
            source_actions,
        })
    }
}
