use super::ObjectInteface as TabletUnstableObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, CommandWriterExt, InterfaceReader,
    ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Server event `zwp_tablet_tool_v1.pressure`.
///
/// Framing, receiver protocol and version are checked; semantics belong to the caller.
/// ```xml
/// <event name="pressure">
///       <description summary="pressure change event">
///     Sent whenever the pressure axis on a tool changes. The value of this
///     event is normalized to a value between 0 and 65535.
///
///     Note that pressure may be nonzero even when a tool is not in logical
///     contact. See the down and up events for more details.
///       </description>
///       <arg name="pressure" type="uint" summary="The current pressure value" />
///     </event>
/// ```
#[derive(Debug)]
pub struct ZwpTabletToolV1PressureEvent {
    /// Wire identifier of the receiver.
    pub object_id: u32,
    /// The current pressure value.
    pub pressure: u32,
}
impl ZwpTabletToolV1PressureEvent {
    /// Opcode in the event namespace.
    pub const OPCODE: u16 = 11;
    /// Minimum interface version.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for ZwpTabletToolV1PressureEvent {
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
impl InterfaceReader for ZwpTabletToolV1PressureEvent {
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
        let pressure = reader.next_u32().await?;
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        Ok(Self {
            object_id,
            pressure,
        })
    }
}
impl CommandWriter for ZwpTabletToolV1PressureEvent {
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
        let interface =
            Self::writer_event_interface(state, self.object_id, "zwp_tablet_tool_v1", Self::SINCE)?;
        if !matches!(
            interface,
            ObjectInteface::TabletUnstable(TabletUnstableObjectInteface::ZwpTabletToolV1 { .. })
        ) || self.payload_size() != writer.remaining_quota()
        {
            return Err(FdWriterError::InvalidProtocol);
        }
        writer.next_u32(self.pressure).await?;
        Ok(())
    }
}
