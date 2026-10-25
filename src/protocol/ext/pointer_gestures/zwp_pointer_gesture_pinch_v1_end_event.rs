use super::ObjectInteface as PointerGesturesObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, CommandWriterExt, InterfaceReader,
    ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Server event `zwp_pointer_gesture_pinch_v1.end`.
///
/// Framing, receiver protocol and version are checked; semantics belong to the caller.
/// ```xml
/// <event name="end">
///       <description summary="multi-finger pinch end">
///     This event is sent when a multi-finger pinch gesture ceases to
///     be valid. This may happen when one or more fingers are lifted or
///     the gesture is cancelled.
///
///     When a gesture is cancelled, the client should undo state changes
///     caused by this gesture. What causes a gesture to be cancelled is
///     implementation-dependent.
///       </description>
///       <arg name="serial" type="uint" />
///       <arg name="time" type="uint" summary="timestamp with millisecond granularity" />
///       <arg name="cancelled" type="int" summary="1 if the gesture was cancelled, 0 otherwise" />
///     </event>
/// ```
#[derive(Debug)]
pub struct ZwpPointerGesturePinchV1EndEvent {
    /// Wire identifier of the receiver.
    pub object_id: u32,
    /// Wire argument `serial`.
    pub serial: u32,
    /// timestamp with millisecond granularity.
    pub time: u32,
    /// 1 if the gesture was cancelled, 0 otherwise.
    pub cancelled: i32,
}
impl ZwpPointerGesturePinchV1EndEvent {
    /// Opcode in the event namespace.
    pub const OPCODE: u16 = 2;
    /// Minimum interface version.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for ZwpPointerGesturePinchV1EndEvent {
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
impl InterfaceReader for ZwpPointerGesturePinchV1EndEvent {
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
        let serial = reader.next_u32().await?;
        let time = reader.next_u32().await?;
        let cancelled = reader.next_i32().await?;
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        Ok(Self {
            object_id,
            serial,
            time,
            cancelled,
        })
    }
}
impl CommandWriter for ZwpPointerGesturePinchV1EndEvent {
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
        writer.next_u32(self.serial).await?;
        writer.next_u32(self.time).await?;
        writer.next_i32(self.cancelled).await?;
        Ok(())
    }
}
