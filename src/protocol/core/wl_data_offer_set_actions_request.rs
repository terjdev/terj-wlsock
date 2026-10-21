use super::WlDataDeviceManagerDndAction;
use crate::protocol::ClientState;
use crate::protocol::{CommandReader, CommandReaderExt};
use crate::protocol::{CommandWriter, CommandWriterExt};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Client request `wl_data_offer.set_actions`.
///
/// Opcode 4; available since version 3. The structure contains the receiver and arguments in XML order.
/// Encoding and decoding check wire framing and receiver metadata; object semantics belong to the compositor.
/// The caller handles compositor policy and message contents.
/// Client encoding checks the complete payload quota before writing arguments.
/// New object IDs are registered before payload I/O; client IDs remain reserved
/// after destruction until the server acknowledges them with delete_id.
/// ```xml
/// <request name="set_actions" since="3">
///       <description summary="set the available/preferred drag-and-drop actions">
///         Sets the actions that the destination side client supports for
///         this operation. This request may trigger the emission of
///         wl_data_source.action and wl_data_offer.action events if the compositor
///         needs to change the selected action.
///
///         This request can be called multiple times throughout the
///         drag-and-drop operation, typically in response to wl_data_device.enter
///         or wl_data_device.motion events.
///
///         This request determines the final result of the drag-and-drop
///         operation. If the end result is that no action is accepted,
///         the drag source will receive wl_data_source.cancelled.
///
///         The dnd_actions argument must contain only values expressed in the
///         wl_data_device_manager.dnd_actions enum, and the preferred_action
///         argument must only contain one of those values set, otherwise it
///         will result in a protocol error.
///
///         While managing an "ask" action, the destination drag-and-drop client
///         may perform further wl_data_offer.receive requests, and is expected
///         to perform one last wl_data_offer.set_actions request with a preferred
///         action other than "ask" (and optionally wl_data_offer.accept) before
///         requesting wl_data_offer.finish, in order to convey the action selected
///         by the user. If the preferred action is not in the
///         wl_data_offer.source_actions mask, an error will be raised.
///
///         If the "ask" action is dismissed (e.g. user cancellation), the client
///         is expected to perform wl_data_offer.destroy right away.
///
///         This request can only be made on drag-and-drop offers, a protocol error
///         will be raised otherwise.
///       </description>
///       <arg name="dnd_actions" type="uint" summary="actions supported by the destination client" enum="wl_data_device_manager.dnd_action" />
///       <arg name="preferred_action" type="uint" summary="action preferred by the destination client" enum="wl_data_device_manager.dnd_action" />
///     </request>
/// ```
#[derive(Debug)]
pub struct WlDataOfferSetActionsRequest {
    /// Wire identifier of the object receiving the message.
    pub object_id: u32,
    /// Argument `dnd_actions`: actions supported by the destination client.
    pub dnd_actions: WlDataDeviceManagerDndAction,
    /// Argument `preferred_action`: action preferred by the destination client.
    pub preferred_action: WlDataDeviceManagerDndAction,
}
impl WlDataOfferSetActionsRequest {
    /// Opcode in the request namespace of interface wl_data_offer.
    pub const OPCODE: u16 = 4;
    /// Minimum interface version supporting this message.
    pub const SINCE: u32 = 3;
}
impl CommandReader<Self> for WlDataOfferSetActionsRequest {
    async fn read(
        object_id: u32,
        opcode: u16,
        reader: &mut FdPayloadReader<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        Self::reader_request_interface(
            state,
            object_id,
            opcode,
            Self::OPCODE,
            "wl_data_offer",
            Self::SINCE,
        )?;
        let dnd_actions = WlDataDeviceManagerDndAction(reader.next_u32().await?);
        let preferred_action = WlDataDeviceManagerDndAction(reader.next_u32().await?);
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        Ok(Self {
            object_id,
            dnd_actions,
            preferred_action,
        })
    }
}

impl CommandWriter for WlDataOfferSetActionsRequest {
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
        Self::writer_request_interface(state, self.object_id, "wl_data_offer", Self::SINCE)?;
        if self.payload_size() != writer.remaining_quota() {
            return Err(FdWriterError::InvalidProtocol);
        }
        writer.next_u32(self.dnd_actions.0).await?;
        writer.next_u32(self.preferred_action.0).await?;
        Ok(())
    }
}
