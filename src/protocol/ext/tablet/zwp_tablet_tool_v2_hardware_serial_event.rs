use crate::protocol::ext::tablet::ObjectInteface as TabletObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, CommandWriterExt, InterfaceReader,
    ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Server event `zwp_tablet_tool_v2.hardware_serial`.
///
/// Opcode 1; available since version 1.
/// Wire framing and receiver metadata are checked; protocol semantics belong to the caller.
/// ```xml
/// <event name="hardware_serial">
///       <description summary="unique hardware serial number of the tool">
///     If the physical tool can be identified by a unique 64-bit serial
///     number, this event notifies the client of this serial number.
///
///     If multiple tablets are available in the same seat and the tool is
///     uniquely identifiable by the serial number, that tool may move
///     between tablets.
///
///     Otherwise, if the tool has no serial number and this event is
///     missing, the tool is tied to the tablet it first comes into
///     proximity with. Even if the physical tool is used on multiple
///     tablets, separate zwp_tablet_tool_v2 objects will be created, one per
///     tablet.
///
///     This event is sent in the initial burst of events before the
///     zwp_tablet_tool_v2.done event.
///       </description>
///       <arg name="hardware_serial_hi" type="uint" summary="the unique serial number of the tool, most significant bits" />
///       <arg name="hardware_serial_lo" type="uint" summary="the unique serial number of the tool, least significant bits" />
///     </event>
/// ```
#[derive(Debug)]
pub struct ZwpTabletToolV2HardwareSerialEvent {
    /// Wire identifier of the receiver.
    pub object_id: u32,
    /// the unique serial number of the tool, most significant bits.
    pub hardware_serial_hi: u32,
    /// the unique serial number of the tool, least significant bits.
    pub hardware_serial_lo: u32,
}
impl ZwpTabletToolV2HardwareSerialEvent {
    /// Opcode in the event namespace.
    pub const OPCODE: u16 = 1;
    /// Minimum supported interface version.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for ZwpTabletToolV2HardwareSerialEvent {
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
impl InterfaceReader for ZwpTabletToolV2HardwareSerialEvent {
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
        let hardware_serial_hi = reader.next_u32().await?;
        let hardware_serial_lo = reader.next_u32().await?;
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        Ok(Self {
            object_id,
            hardware_serial_hi,
            hardware_serial_lo,
        })
    }
}
impl CommandWriter for ZwpTabletToolV2HardwareSerialEvent {
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
        Self::writer_event_interface(state, self.object_id, "zwp_tablet_tool_v2", Self::SINCE)?;
        if self.payload_size() != writer.remaining_quota() {
            return Err(FdWriterError::InvalidProtocol);
        }
        writer.next_u32(self.hardware_serial_hi).await?;
        writer.next_u32(self.hardware_serial_lo).await?;
        Ok(())
    }
}
