use super::ObjectInteface as PointerGesturesObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, CommandWriterExt, InterfaceReader,
    ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Server event `zwp_pointer_gesture_hold_v1.begin`.
///
/// Framing, receiver protocol and version are checked; semantics belong to the caller.
/// ```xml
/// <event name="begin" since="3">
///       <description summary="multi-finger hold begin">
///     This event is sent when a hold gesture is detected on the device.
///       </description>
///       <arg name="serial" type="uint" />
///       <arg name="time" type="uint" summary="timestamp with millisecond granularity" />
///       <arg name="surface" type="object" interface="wl_surface" />
///       <arg name="fingers" type="uint" summary="number of fingers" />
///     </event>
/// ```
#[derive(Debug)]
pub struct ZwpPointerGestureHoldV1BeginEvent {
    /// Wire identifier of the receiver.
    pub object_id: u32,
    /// Wire argument `serial`.
    pub serial: u32,
    /// timestamp with millisecond granularity.
    pub time: u32,
    /// Wire argument `surface`.
    pub surface: u32,
    /// number of fingers.
    pub fingers: u32,
}
impl ZwpPointerGestureHoldV1BeginEvent {
    /// Opcode in the event namespace.
    pub const OPCODE: u16 = 0;
    /// Minimum interface version.
    pub const SINCE: u32 = 3;
}
impl CommandReader<Self> for ZwpPointerGestureHoldV1BeginEvent {
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
impl InterfaceReader for ZwpPointerGestureHoldV1BeginEvent {
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
                PointerGesturesObjectInteface::ZwpPointerGestureHoldV1 { .. }
            )
        ) || opcode != Self::OPCODE
            || interface.version() < Self::SINCE
        {
            return Err(FdReaderError::InvalidProtocol);
        }
        let serial = reader.next_u32().await?;
        let time = reader.next_u32().await?;
        let surface = reader.next_u32().await?;
        let fingers = reader.next_u32().await?;
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        Ok(Self {
            object_id,
            serial,
            time,
            surface,
            fingers,
        })
    }
}
impl CommandWriter for ZwpPointerGestureHoldV1BeginEvent {
    fn object_id(&self) -> u32 {
        self.object_id
    }
    fn opcode(&self) -> u16 {
        Self::OPCODE
    }
    fn payload_size(&self) -> usize {
        Self::total_payload_size(&[Some(4), Some(4), Some(4), Some(4)])
    }
    async fn write(
        self,
        writer: &mut FdPayloadWriter<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<(), FdWriterError> {
        let interface = Self::writer_event_interface(
            state,
            self.object_id,
            "zwp_pointer_gesture_hold_v1",
            Self::SINCE,
        )?;
        if !matches!(
            interface,
            ObjectInteface::PointerGestures(
                PointerGesturesObjectInteface::ZwpPointerGestureHoldV1 { .. }
            )
        ) || self.payload_size() != writer.remaining_quota()
        {
            return Err(FdWriterError::InvalidProtocol);
        }
        writer.next_u32(self.serial).await?;
        writer.next_u32(self.time).await?;
        writer.next_u32(self.surface).await?;
        writer.next_u32(self.fingers).await?;
        Ok(())
    }
}
