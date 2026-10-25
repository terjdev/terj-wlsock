use crate::protocol::ext::tablet::ObjectInteface as TabletObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, CommandWriterExt, InterfaceReader,
    ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Server event `zwp_tablet_tool_v2.down`.
///
/// Opcode 8; available since version 1.
/// Wire framing and receiver metadata are checked; protocol semantics belong to the caller.
/// ```xml
/// <event name="down">
///       <description summary="tablet tool is making contact">
///     Sent whenever the tablet tool comes in contact with the surface of the
///     tablet.
///
///     If the tool is already in contact with the tablet when entering the
///     input region, the client owning said region will receive a
///     zwp_tablet_v2.proximity_in event, followed by a zwp_tablet_v2.down
///     event and a zwp_tablet_v2.frame event.
///
///     Note that this event describes logical contact, not physical
///     contact. On some devices, a compositor may not consider a tool in
///     logical contact until a minimum physical pressure threshold is
///     exceeded.
///       </description>
///       <arg name="serial" type="uint" />
///     </event>
/// ```
#[derive(Debug)]
pub struct ZwpTabletToolV2DownEvent {
    /// Wire identifier of the receiver.
    pub object_id: u32,
    /// Wire argument `serial`.
    pub serial: u32,
}
impl ZwpTabletToolV2DownEvent {
    /// Opcode in the event namespace.
    pub const OPCODE: u16 = 8;
    /// Minimum supported interface version.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for ZwpTabletToolV2DownEvent {
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
impl InterfaceReader for ZwpTabletToolV2DownEvent {
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
        let serial = reader.next_u32().await?;
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        Ok(Self { object_id, serial })
    }
}
impl CommandWriter for ZwpTabletToolV2DownEvent {
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
        Self::writer_event_interface(state, self.object_id, "zwp_tablet_tool_v2", Self::SINCE)?;
        if self.payload_size() != writer.remaining_quota() {
            return Err(FdWriterError::InvalidProtocol);
        }
        writer.next_u32(self.serial).await?;
        Ok(())
    }
}
