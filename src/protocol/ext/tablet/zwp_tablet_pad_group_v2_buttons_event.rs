use crate::protocol::ext::tablet::ObjectInteface as TabletObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, CommandWriterExt, InterfaceReader,
    ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::FdWriter;
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Server event `zwp_tablet_pad_group_v2.buttons`.
///
/// Opcode 0; available since version 1.
/// Wire framing and receiver metadata are checked; protocol semantics belong to the caller.
/// ```xml
/// <event name="buttons">
///       <description summary="buttons announced">
///     Sent on zwp_tablet_pad_group_v2 initialization to announce the available
///     buttons in the group. Button indices start at 0, a button may only be
///     in one group at a time.
///
///     This event is first sent in the initial burst of events before the
///     zwp_tablet_pad_group_v2.done event.
///
///     Some buttons are reserved by the compositor. These buttons may not be
///     assigned to any zwp_tablet_pad_group_v2. Compositors may broadcast this
///     event in the case of changes to the mapping of these reserved buttons.
///     If the compositor happens to reserve all buttons in a group, this event
///     will be sent with an empty array.
///       </description>
///       <arg name="buttons" type="array" summary="buttons in this group" />
///     </event>
/// ```
#[derive(Debug)]
pub struct ZwpTabletPadGroupV2ButtonsEvent {
    /// Wire identifier of the receiver.
    pub object_id: u32,
    /// buttons in this group.
    pub buttons: Vec<u8>,
}
impl ZwpTabletPadGroupV2ButtonsEvent {
    /// Opcode in the event namespace.
    pub const OPCODE: u16 = 0;
    /// Minimum supported interface version.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for ZwpTabletPadGroupV2ButtonsEvent {
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
impl InterfaceReader for ZwpTabletPadGroupV2ButtonsEvent {
    async fn read_with_interface(
        object_id: u32,
        opcode: u16,
        interface: ObjectInteface,
        reader: &mut FdPayloadReader<'_>,
        _state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        if !matches!(
            interface,
            ObjectInteface::Tablet(TabletObjectInteface::ZwpTabletPadGroupV2 { .. })
        ) || opcode != Self::OPCODE
            || interface.version() < Self::SINCE
        {
            return Err(FdReaderError::InvalidProtocol);
        }
        let buttons = reader.next_array().await?;
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        Ok(Self { object_id, buttons })
    }
}
impl CommandWriter for ZwpTabletPadGroupV2ButtonsEvent {
    fn object_id(&self) -> u32 {
        self.object_id
    }
    fn opcode(&self) -> u16 {
        Self::OPCODE
    }
    fn payload_size(&self) -> usize {
        Self::total_payload_size(&[FdWriter::encoded_array_size(self.buttons.len())])
    }
    async fn write(
        self,
        writer: &mut FdPayloadWriter<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<(), FdWriterError> {
        Self::writer_event_interface(
            state,
            self.object_id,
            "zwp_tablet_pad_group_v2",
            Self::SINCE,
        )?;
        if self.payload_size() != writer.remaining_quota() {
            return Err(FdWriterError::InvalidProtocol);
        }
        writer.next_array(&self.buttons).await?;
        Ok(())
    }
}
