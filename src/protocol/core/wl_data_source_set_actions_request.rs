use super::WlDataDeviceManagerDndAction;
use crate::protocol::ClientState;
use crate::protocol::{CommandReader, CommandReaderExt};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use std::sync::{Arc, Mutex};

/// Client request `wl_data_source.set_actions`.
///
/// Opcode 2; available since version 3. The structure contains the receiver and arguments in XML order.
/// Decoding checks wire framing and receiver metadata; object arguments belong to the compositor.
/// The caller handles compositor policy and message contents.
/// ```xml
/// <request name="set_actions" since="3">
///       <description summary="set the available drag-and-drop actions">
///         Sets the actions that the source side client supports for this
///         operation. This request may trigger wl_data_source.action and
///         wl_data_offer.action events if the compositor needs to change the
///         selected action.
///
///         The dnd_actions argument must contain only values expressed in the
///         wl_data_device_manager.dnd_actions enum, otherwise it will result
///         in a protocol error.
///
///         This request must be made once only, and can only be made on sources
///         used in drag-and-drop, so it must be performed before
///         wl_data_device.start_drag. Attempting to use the source other than
///         for drag-and-drop will raise a protocol error.
///       </description>
///       <arg name="dnd_actions" type="uint" summary="actions supported by the data source" enum="wl_data_device_manager.dnd_action" />
///     </request>
/// ```
#[derive(Debug)]
pub struct WlDataSourceSetActionsRequest {
    /// Wire identifier of the object receiving the message.
    pub object_id: u32,
    /// Argument `dnd_actions`: actions supported by the data source.
    pub dnd_actions: WlDataDeviceManagerDndAction,
}
impl WlDataSourceSetActionsRequest {
    /// Opcode in the request namespace of interface wl_data_source.
    pub const OPCODE: u16 = 2;
    /// Minimum interface version supporting this message.
    pub const SINCE: u32 = 3;
}
impl CommandReader<Self> for WlDataSourceSetActionsRequest {
    async fn read(
        object_id: u32,
        opcode: u16,
        reader: &mut FdPayloadReader<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        Self::request_interface(
            state,
            object_id,
            opcode,
            Self::OPCODE,
            "wl_data_source",
            Self::SINCE,
        )?;
        let dnd_actions = WlDataDeviceManagerDndAction(reader.next_u32().await?);
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        Ok(Self {
            object_id,
            dnd_actions,
        })
    }
}
