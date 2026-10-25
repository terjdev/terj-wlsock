use super::ObjectInteface as TabletUnstableObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, CommandWriterExt, InterfaceReader,
    ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Server event `zwp_tablet_tool_v1.proximity_out`.
///
/// Framing, receiver protocol and version are checked; semantics belong to the caller.
/// ```xml
/// <event name="proximity_out">
///       <description summary="proximity out event">
///     Notification that this tool has either left proximity, or is no
///     longer focused on a certain surface.
///
///     When the tablet tool leaves proximity of the tablet, button release
///     events are sent for each button that was held down at the time of
///     leaving proximity. These events are sent before the proximity_out
///     event but within the same wp_tablet.frame.
///
///     If the tool stays within proximity of the tablet, but the focus
///     changes from one surface to another, a button release event may not
///     be sent until the button is actually released or the tool leaves the
///     proximity of the tablet.
///       </description>
///     </event>
/// ```
#[derive(Debug)]
pub struct ZwpTabletToolV1ProximityOutEvent {
    /// Wire identifier of the receiver.
    pub object_id: u32,
}
impl ZwpTabletToolV1ProximityOutEvent {
    /// Opcode in the event namespace.
    pub const OPCODE: u16 = 7;
    /// Minimum interface version.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for ZwpTabletToolV1ProximityOutEvent {
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
impl InterfaceReader for ZwpTabletToolV1ProximityOutEvent {
    async fn read_with_interface(
        object_id: u32,
        opcode: u16,
        interface: ObjectInteface,
        reader: &mut FdPayloadReader<'_>,
        _state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        if !matches!(
            interface,
            ObjectInteface::TabletUnstable(TabletUnstableObjectInteface::ZwpTabletToolV1 { .. })
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
impl CommandWriter for ZwpTabletToolV1ProximityOutEvent {
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
        let interface =
            Self::writer_event_interface(state, self.object_id, "zwp_tablet_tool_v1", Self::SINCE)?;
        if !matches!(
            interface,
            ObjectInteface::TabletUnstable(TabletUnstableObjectInteface::ZwpTabletToolV1 { .. })
        ) || self.payload_size() != writer.remaining_quota()
        {
            return Err(FdWriterError::InvalidProtocol);
        }
        Ok(())
    }
}
