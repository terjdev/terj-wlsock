use crate::protocol::ext::tablet::ObjectInteface as TabletObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, CommandWriterExt, InterfaceReader,
    ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::FdWriter;
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Server event `zwp_tablet_tool_v2.wheel`.
///
/// Opcode 16; available since version 1.
/// Wire framing and receiver metadata are checked; protocol semantics belong to the caller.
/// ```xml
/// <event name="wheel">
///       <description summary="Wheel delta event">
///     Sent whenever the wheel on the tool emits an event. This event
///     contains two values for the same axis change. The degrees value is
///     in the same orientation as the wl_pointer.vertical_scroll axis. The
///     clicks value is in discrete logical clicks of the mouse wheel. This
///     value may be zero if the movement of the wheel was less
///     than one logical click.
///
///     Clients should choose either value and avoid mixing degrees and
///     clicks. The compositor may accumulate values smaller than a logical
///     click and emulate click events when a certain threshold is met.
///     Thus, zwp_tablet_tool_v2.wheel events with non-zero clicks values may
///     have different degrees values.
///       </description>
///       <arg name="degrees" type="fixed" summary="The wheel delta in degrees" />
///       <arg name="clicks" type="int" summary="The wheel delta in discrete clicks" />
///     </event>
/// ```
#[derive(Debug)]
pub struct ZwpTabletToolV2WheelEvent {
    /// Wire identifier of the receiver.
    pub object_id: u32,
    /// The wheel delta in degrees.
    pub degrees: f64,
    /// The wheel delta in discrete clicks.
    pub clicks: i32,
}
impl ZwpTabletToolV2WheelEvent {
    /// Opcode in the event namespace.
    pub const OPCODE: u16 = 16;
    /// Minimum supported interface version.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for ZwpTabletToolV2WheelEvent {
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
impl InterfaceReader for ZwpTabletToolV2WheelEvent {
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
        let degrees = reader.next_fixed().await?;
        let clicks = reader.next_i32().await?;
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        Ok(Self {
            object_id,
            degrees,
            clicks,
        })
    }
}
impl CommandWriter for ZwpTabletToolV2WheelEvent {
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
        if !FdWriter::is_valid_fixed(self.degrees) {
            return Err(FdWriterError::InvalidProtocol);
        }
        writer.next_fixed(self.degrees).await?;
        writer.next_i32(self.clicks).await?;
        Ok(())
    }
}
