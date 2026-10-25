use crate::protocol::ext::tablet::ObjectInteface as TabletObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, CommandWriterExt, InterfaceReader,
    ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Server event `zwp_tablet_pad_ring_v2.frame`.
///
/// Opcode 3; available since version 1.
/// Wire framing and receiver metadata are checked; protocol semantics belong to the caller.
/// ```xml
/// <event name="frame">
///       <description summary="end of a ring event sequence">
///     Indicates the end of a set of ring events that logically belong
///     together. A client is expected to accumulate the data in all events
///     within the frame before proceeding.
///
///     All zwp_tablet_pad_ring_v2 events before a zwp_tablet_pad_ring_v2.frame event belong
///     logically together. For example, on termination of a finger interaction
///     on a ring the compositor will send a zwp_tablet_pad_ring_v2.source event,
///     a zwp_tablet_pad_ring_v2.stop event and a zwp_tablet_pad_ring_v2.frame event.
///
///     A zwp_tablet_pad_ring_v2.frame event is sent for every logical event
///     group, even if the group only contains a single zwp_tablet_pad_ring_v2
///     event. Specifically, a client may get a sequence: angle, frame,
///     angle, frame, etc.
///       </description>
///       <arg name="time" type="uint" summary="timestamp with millisecond granularity" />
///     </event>
/// ```
#[derive(Debug)]
pub struct ZwpTabletPadRingV2FrameEvent {
    /// Wire identifier of the receiver.
    pub object_id: u32,
    /// timestamp with millisecond granularity.
    pub time: u32,
}
impl ZwpTabletPadRingV2FrameEvent {
    /// Opcode in the event namespace.
    pub const OPCODE: u16 = 3;
    /// Minimum supported interface version.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for ZwpTabletPadRingV2FrameEvent {
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
impl InterfaceReader for ZwpTabletPadRingV2FrameEvent {
    async fn read_with_interface(
        object_id: u32,
        opcode: u16,
        interface: ObjectInteface,
        reader: &mut FdPayloadReader<'_>,
        _state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        if !matches!(
            interface,
            ObjectInteface::Tablet(TabletObjectInteface::ZwpTabletPadRingV2 { .. })
        ) || opcode != Self::OPCODE
            || interface.version() < Self::SINCE
        {
            return Err(FdReaderError::InvalidProtocol);
        }
        let time = reader.next_u32().await?;
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        Ok(Self { object_id, time })
    }
}
impl CommandWriter for ZwpTabletPadRingV2FrameEvent {
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
        Self::writer_event_interface(state, self.object_id, "zwp_tablet_pad_ring_v2", Self::SINCE)?;
        if self.payload_size() != writer.remaining_quota() {
            return Err(FdWriterError::InvalidProtocol);
        }
        writer.next_u32(self.time).await?;
        Ok(())
    }
}
