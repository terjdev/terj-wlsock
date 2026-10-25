use crate::protocol::ext::tablet::ObjectInteface as TabletObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, CommandWriterExt, InterfaceReader,
    ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Server event `zwp_tablet_tool_v2.proximity_in`.
///
/// Opcode 6; available since version 1.
/// Wire framing and receiver metadata are checked; protocol semantics belong to the caller.
/// ```xml
/// <event name="proximity_in">
///       <description summary="proximity in event">
///     Notification that this tool is focused on a certain surface.
///
///     This event can be received when the tool has moved from one surface to
///     another, or when the tool has come back into proximity above the
///     surface.
///
///     If any button is logically down when the tool comes into proximity,
///     the respective button event is sent after the proximity_in event but
///     within the same frame as the proximity_in event.
///       </description>
///       <arg name="serial" type="uint" />
///       <arg name="tablet" type="object" interface="zwp_tablet_v2" summary="The tablet the tool is in proximity of" />
///       <arg name="surface" type="object" interface="wl_surface" summary="The current surface the tablet tool is over" />
///     </event>
/// ```
#[derive(Debug)]
pub struct ZwpTabletToolV2ProximityInEvent {
    /// Wire identifier of the receiver.
    pub object_id: u32,
    /// Wire argument `serial`.
    pub serial: u32,
    /// The tablet the tool is in proximity of.
    pub tablet: u32,
    /// The current surface the tablet tool is over.
    pub surface: u32,
}
impl ZwpTabletToolV2ProximityInEvent {
    /// Opcode in the event namespace.
    pub const OPCODE: u16 = 6;
    /// Minimum supported interface version.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for ZwpTabletToolV2ProximityInEvent {
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
impl InterfaceReader for ZwpTabletToolV2ProximityInEvent {
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
        let tablet = reader.next_u32().await?;
        let surface = reader.next_u32().await?;
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        Ok(Self {
            object_id,
            serial,
            tablet,
            surface,
        })
    }
}
impl CommandWriter for ZwpTabletToolV2ProximityInEvent {
    fn object_id(&self) -> u32 {
        self.object_id
    }
    fn opcode(&self) -> u16 {
        Self::OPCODE
    }
    fn payload_size(&self) -> usize {
        Self::total_payload_size(&[Some(4), Some(4), Some(4)])
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
        writer.next_u32(self.tablet).await?;
        writer.next_u32(self.surface).await?;
        Ok(())
    }
}
