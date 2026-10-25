use super::ObjectInteface as PointerGesturesObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, CommandWriterExt, InterfaceReader,
    ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::FdWriter;
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Server event `zwp_pointer_gesture_pinch_v1.update`.
///
/// Framing, receiver protocol and version are checked; semantics belong to the caller.
/// ```xml
/// <event name="update">
///       <description summary="multi-finger pinch motion">
///     This event is sent when a multi-finger pinch gesture changes the
///     position of the logical center, the rotation or the relative scale.
///
///     The dx and dy coordinates are relative coordinates in the
///     surface coordinate space of the logical center of the gesture.
///
///     The scale factor is an absolute scale compared to the
///     pointer_gesture_pinch.begin event, e.g. a scale of 2 means the fingers
///     are now twice as far apart as on pointer_gesture_pinch.begin.
///
///     The rotation is the relative angle in degrees clockwise compared to the previous
///     pointer_gesture_pinch.begin or pointer_gesture_pinch.update event.
///       </description>
///       <arg name="time" type="uint" summary="timestamp with millisecond granularity" />
///       <arg name="dx" type="fixed" summary="delta x coordinate in surface coordinate space" />
///       <arg name="dy" type="fixed" summary="delta y coordinate in surface coordinate space" />
///       <arg name="scale" type="fixed" summary="scale relative to the initial finger position" />
///       <arg name="rotation" type="fixed" summary="angle in degrees cw relative to the previous event" />
///     </event>
/// ```
#[derive(Debug)]
pub struct ZwpPointerGesturePinchV1UpdateEvent {
    /// Wire identifier of the receiver.
    pub object_id: u32,
    /// timestamp with millisecond granularity.
    pub time: u32,
    /// delta x coordinate in surface coordinate space.
    pub dx: f64,
    /// delta y coordinate in surface coordinate space.
    pub dy: f64,
    /// scale relative to the initial finger position.
    pub scale: f64,
    /// angle in degrees cw relative to the previous event.
    pub rotation: f64,
}
impl ZwpPointerGesturePinchV1UpdateEvent {
    /// Opcode in the event namespace.
    pub const OPCODE: u16 = 1;
    /// Minimum interface version.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for ZwpPointerGesturePinchV1UpdateEvent {
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
impl InterfaceReader for ZwpPointerGesturePinchV1UpdateEvent {
    async fn read_with_interface(
        object_id: u32,
        opcode: u16,
        interface: ObjectInteface,
        reader: &mut FdPayloadReader<'_>,
        _state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        if !matches!(
            interface,
            ObjectInteface::PointerGestures(
                PointerGesturesObjectInteface::ZwpPointerGesturePinchV1 { .. }
            )
        ) || opcode != Self::OPCODE
            || interface.version() < Self::SINCE
        {
            return Err(FdReaderError::InvalidProtocol);
        }
        let time = reader.next_u32().await?;
        let dx = reader.next_fixed().await?;
        let dy = reader.next_fixed().await?;
        let scale = reader.next_fixed().await?;
        let rotation = reader.next_fixed().await?;
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        Ok(Self {
            object_id,
            time,
            dx,
            dy,
            scale,
            rotation,
        })
    }
}
impl CommandWriter for ZwpPointerGesturePinchV1UpdateEvent {
    fn object_id(&self) -> u32 {
        self.object_id
    }
    fn opcode(&self) -> u16 {
        Self::OPCODE
    }
    fn payload_size(&self) -> usize {
        Self::total_payload_size(&[Some(4), Some(4), Some(4), Some(4), Some(4)])
    }
    async fn write(
        self,
        writer: &mut FdPayloadWriter<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<(), FdWriterError> {
        let interface = Self::writer_event_interface(
            state,
            self.object_id,
            "zwp_pointer_gesture_pinch_v1",
            Self::SINCE,
        )?;
        if !matches!(
            interface,
            ObjectInteface::PointerGestures(
                PointerGesturesObjectInteface::ZwpPointerGesturePinchV1 { .. }
            )
        ) || self.payload_size() != writer.remaining_quota()
        {
            return Err(FdWriterError::InvalidProtocol);
        }
        if !FdWriter::is_valid_fixed(self.dx) {
            return Err(FdWriterError::InvalidProtocol);
        }
        if !FdWriter::is_valid_fixed(self.dy) {
            return Err(FdWriterError::InvalidProtocol);
        }
        if !FdWriter::is_valid_fixed(self.scale) {
            return Err(FdWriterError::InvalidProtocol);
        }
        if !FdWriter::is_valid_fixed(self.rotation) {
            return Err(FdWriterError::InvalidProtocol);
        }
        writer.next_u32(self.time).await?;
        writer.next_fixed(self.dx).await?;
        writer.next_fixed(self.dy).await?;
        writer.next_fixed(self.scale).await?;
        writer.next_fixed(self.rotation).await?;
        Ok(())
    }
}
