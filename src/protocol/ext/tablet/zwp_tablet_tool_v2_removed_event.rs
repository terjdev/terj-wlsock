use crate::protocol::ext::tablet::ObjectInteface as TabletObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, CommandWriterExt, InterfaceReader,
    ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Server event `zwp_tablet_tool_v2.removed`.
///
/// Opcode 5; available since version 1.
/// Wire framing and receiver metadata are checked; protocol semantics belong to the caller.
/// ```xml
/// <event name="removed">
///       <description summary="tool removed">
///     This event is sent when the tool is removed from the system and will
///     send no further events. Should the physical tool come back into
///     proximity later, a new zwp_tablet_tool_v2 object will be created.
///
///     It is compositor-dependent when a tool is removed. A compositor may
///     remove a tool on proximity out, tablet removal or any other reason.
///     A compositor may also keep a tool alive until shutdown.
///
///     If the tool is currently in proximity, a proximity_out event will be
///     sent before the removed event. See zwp_tablet_tool_v2.proximity_out for
///     the handling of any buttons logically down.
///
///     When this event is received, the client must zwp_tablet_tool_v2.destroy
///     the object.
///       </description>
///     </event>
/// ```
#[derive(Debug)]
pub struct ZwpTabletToolV2RemovedEvent {
    /// Wire identifier of the receiver.
    pub object_id: u32,
}
impl ZwpTabletToolV2RemovedEvent {
    /// Opcode in the event namespace.
    pub const OPCODE: u16 = 5;
    /// Minimum supported interface version.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for ZwpTabletToolV2RemovedEvent {
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
impl InterfaceReader for ZwpTabletToolV2RemovedEvent {
    async fn read_with_interface(
        object_id: u32,
        opcode: u16,
        interface: ObjectInteface,
        reader: &mut FdPayloadReader<'_>,
        _state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        if !matches!(
            interface,
            ObjectInteface::Tablet(TabletObjectInteface::ZwpTabletToolV2 { .. })
        ) || opcode != Self::OPCODE
            || interface.version() < Self::SINCE
        {
            return Err(FdReaderError::InvalidProtocol);
        }
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        Ok(Self { object_id })
    }
}
impl CommandWriter for ZwpTabletToolV2RemovedEvent {
    fn object_id(&self) -> u32 {
        self.object_id
    }
    fn opcode(&self) -> u16 {
        Self::OPCODE
    }
    fn payload_size(&self) -> usize {
        Self::total_payload_size(&[])
    }
    async fn write(
        self,
        writer: &mut FdPayloadWriter<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<(), FdWriterError> {
        Self::writer_event_interface(state, self.object_id, "zwp_tablet_tool_v2", Self::SINCE)?;
        if self.payload_size() != writer.remaining_quota() {
            return Err(FdWriterError::InvalidProtocol);
        }
        Ok(())
    }
}
