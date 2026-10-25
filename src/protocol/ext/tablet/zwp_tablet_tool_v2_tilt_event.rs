use crate::protocol::ext::tablet::ObjectInteface as TabletObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, CommandWriterExt, InterfaceReader,
    ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::FdWriter;
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Server event `zwp_tablet_tool_v2.tilt`.
///
/// Opcode 13; available since version 1.
/// Wire framing and receiver metadata are checked; protocol semantics belong to the caller.
/// ```xml
/// <event name="tilt">
///       <description summary="tilt change event">
///     Sent whenever one or both of the tilt axes on a tool change. Each tilt
///     value is in degrees, relative to the z-axis of the tablet.
///     The angle is positive when the top of a tool tilts along the
///     positive x or y axis.
///       </description>
///       <arg name="tilt_x" type="fixed" summary="The current value of the X tilt axis" />
///       <arg name="tilt_y" type="fixed" summary="The current value of the Y tilt axis" />
///     </event>
/// ```
#[derive(Debug)]
pub struct ZwpTabletToolV2TiltEvent {
    /// Wire identifier of the receiver.
    pub object_id: u32,
    /// The current value of the X tilt axis.
    pub tilt_x: f64,
    /// The current value of the Y tilt axis.
    pub tilt_y: f64,
}
impl ZwpTabletToolV2TiltEvent {
    /// Opcode in the event namespace.
    pub const OPCODE: u16 = 13;
    /// Minimum supported interface version.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for ZwpTabletToolV2TiltEvent {
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
impl InterfaceReader for ZwpTabletToolV2TiltEvent {
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
        let tilt_x = reader.next_fixed().await?;
        let tilt_y = reader.next_fixed().await?;
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        Ok(Self {
            object_id,
            tilt_x,
            tilt_y,
        })
    }
}
impl CommandWriter for ZwpTabletToolV2TiltEvent {
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
        if !FdWriter::is_valid_fixed(self.tilt_x) {
            return Err(FdWriterError::InvalidProtocol);
        }
        if !FdWriter::is_valid_fixed(self.tilt_y) {
            return Err(FdWriterError::InvalidProtocol);
        }
        writer.next_fixed(self.tilt_x).await?;
        writer.next_fixed(self.tilt_y).await?;
        Ok(())
    }
}
