use crate::protocol::ext::tablet::ObjectInteface as TabletObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, CommandWriterExt, InterfaceReader,
    ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Server event `zwp_tablet_tool_v2.hardware_id_wacom`.
///
/// Opcode 2; available since version 1.
/// Wire framing and receiver metadata are checked; protocol semantics belong to the caller.
/// ```xml
/// <event name="hardware_id_wacom">
///       <description summary="hardware id notification in Wacom's format">
///     This event notifies the client of a hardware id available on this tool.
///
///     The hardware id is a device-specific 64-bit id that provides extra
///     information about the tool in use, beyond the wl_tool.type
///     enumeration. The format of the id is specific to tablets made by
///     Wacom Inc. For example, the hardware id of a Wacom Grip
///     Pen (a stylus) is 0x802.
///
///     This event is sent in the initial burst of events before the
///     zwp_tablet_tool_v2.done event.
///       </description>
///       <arg name="hardware_id_hi" type="uint" summary="the hardware id, most significant bits" />
///       <arg name="hardware_id_lo" type="uint" summary="the hardware id, least significant bits" />
///     </event>
/// ```
#[derive(Debug)]
pub struct ZwpTabletToolV2HardwareIdWacomEvent {
    /// Wire identifier of the receiver.
    pub object_id: u32,
    /// the hardware id, most significant bits.
    pub hardware_id_hi: u32,
    /// the hardware id, least significant bits.
    pub hardware_id_lo: u32,
}
impl ZwpTabletToolV2HardwareIdWacomEvent {
    /// Opcode in the event namespace.
    pub const OPCODE: u16 = 2;
    /// Minimum supported interface version.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for ZwpTabletToolV2HardwareIdWacomEvent {
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
impl InterfaceReader for ZwpTabletToolV2HardwareIdWacomEvent {
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
        let hardware_id_hi = reader.next_u32().await?;
        let hardware_id_lo = reader.next_u32().await?;
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        Ok(Self {
            object_id,
            hardware_id_hi,
            hardware_id_lo,
        })
    }
}
impl CommandWriter for ZwpTabletToolV2HardwareIdWacomEvent {
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
        writer.next_u32(self.hardware_id_hi).await?;
        writer.next_u32(self.hardware_id_lo).await?;
        Ok(())
    }
}
