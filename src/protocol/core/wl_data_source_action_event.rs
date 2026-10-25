use super::WlDataDeviceManagerDndAction;
use crate::protocol::ClientState;
use crate::protocol::core::ObjectInteface as CoreObjectInteface;
use crate::protocol::{CommandReader, CommandReaderExt};
use crate::protocol::{CommandWriter, CommandWriterExt};
use crate::protocol::{InterfaceReader, ObjectInteface};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Server event `wl_data_source.action`.
///
/// Opcode 5; available since version 3. The structure contains the receiver and arguments in XML order.
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
///         operation, mainly in response to destination side changes through
///         wl_data_offer.set_actions, and as the data device enters/leaves
///         surfaces.
///
///         It is only possible to receive this event after
///         wl_data_source.dnd_drop_performed if the drag-and-drop operation
///         ended in an "ask" action, in which case the final wl_data_source.action
///         event will happen immediately before wl_data_source.dnd_finished.
///
///         Compositors may also change the selected action on the fly, mainly
///         in response to keyboard modifier changes during the drag-and-drop
///         operation.
///
///         The most recent action received is always the valid one. The chosen
///         action may change alongside negotiation (e.g. an "ask" action can turn
///         into a "move" operation), so the effects of the final action must
///         always be applied in wl_data_source.dnd_finished.
///
///         Clients can trigger cursor surface changes from this point, so
///         they reflect the current action.
///       </description>
///       <arg name="dnd_action" type="uint" summary="action selected by the compositor" enum="wl_data_device_manager.dnd_action" />
///     </event>
/// ```
#[derive(Debug)]
pub struct WlDataSourceActionEvent {
    /// Wire identifier of the object receiving the message.
    pub object_id: u32,
    /// Argument `dnd_action`: action selected by the compositor.
    pub dnd_action: WlDataDeviceManagerDndAction,
}
impl WlDataSourceActionEvent {
    /// Opcode in the event namespace of interface wl_data_source.
    pub const OPCODE: u16 = 5;
    /// Minimum interface version supporting this message.
    pub const SINCE: u32 = 3;
}
impl CommandWriter for WlDataSourceActionEvent {
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
        Self::writer_event_interface(state, self.object_id, "wl_data_source", Self::SINCE)?;
        if self.payload_size() != writer.remaining_quota() {
            return Err(FdWriterError::InvalidProtocol);
        }
        writer.next_u32(self.dnd_action.0).await?;
        Ok(())
    }
}

impl CommandReader<Self> for WlDataSourceActionEvent {
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

impl InterfaceReader for WlDataSourceActionEvent {
    async fn read_with_interface(
        object_id: u32,
        opcode: u16,
        interface: ObjectInteface,
        reader: &mut FdPayloadReader<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        if !matches!(
            interface,
            ObjectInteface::Core(CoreObjectInteface::WlDataSource { .. })
        ) || opcode != Self::OPCODE
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
