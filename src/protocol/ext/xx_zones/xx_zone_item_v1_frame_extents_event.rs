use super::ObjectInteface as XxZonesObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, CommandWriterExt, InterfaceReader,
    ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Server event `xx_zone_item_v1.frame_extents`.
///
/// Framing, receiver protocol and version are checked; semantics belong to the caller.
/// ```xml
/// <event name="frame_extents">
///       <description summary="the extents of the frame bordering the item">
///         The 'frame_extents' event describes the current extents of the frame
///         bordering the item's content area.
///
///         This event is sent immediately after the item joins a zone, or if
///         the item frame extents have been changed by other means (e.g. toggled
///         by a client request, or compositor involvement). The dimensions are in
///         the same coordinate space as the item's zone (the surface coordinate
///         space).
///
///         This event must be followed by a 'position' event, even if the item's
///         coordinates did not change as a result of the frame extents changing.
///
///         If the item has no associated frame, the event should still be sent,
///         but extents must be set to zero.
///
///         This event can only be emitted if the item is currently associated
///         with a zone.
///       </description>
///       <arg name="top" type="int" summary="current height of the frame bordering the top of the item" />
///       <arg name="bottom" type="int" summary="current height of the frame bordering the bottom of the item" />
///       <arg name="left" type="int" summary="current width of the frame bordering the left of the item" />
///       <arg name="right" type="int" summary="current width of the frame bordering the right of the item" />
///     </event>
/// ```
#[derive(Debug)]
pub struct XxZoneItemV1FrameExtentsEvent {
    /// Wire identifier of the receiver.
    pub object_id: u32,
    /// current height of the frame bordering the top of the item.
    pub top: i32,
    /// current height of the frame bordering the bottom of the item.
    pub bottom: i32,
    /// current width of the frame bordering the left of the item.
    pub left: i32,
    /// current width of the frame bordering the right of the item.
    pub right: i32,
}
impl XxZoneItemV1FrameExtentsEvent {
    /// Opcode in the event namespace.
    pub const OPCODE: u16 = 0;
    /// Minimum interface version.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for XxZoneItemV1FrameExtentsEvent {
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
impl InterfaceReader for XxZoneItemV1FrameExtentsEvent {
    async fn read_with_interface(
        object_id: u32,
        opcode: u16,
        interface: ObjectInteface,
        reader: &mut FdPayloadReader<'_>,
        _state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        if !matches!(
            interface,
            ObjectInteface::XxZones(XxZonesObjectInteface::XxZoneItemV1 { .. })
        ) || opcode != Self::OPCODE
            || interface.version() < Self::SINCE
        {
            return Err(FdReaderError::InvalidProtocol);
        }
        let top = reader.next_i32().await?;
        let bottom = reader.next_i32().await?;
        let left = reader.next_i32().await?;
        let right = reader.next_i32().await?;
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        Ok(Self {
            object_id,
            top,
            bottom,
            left,
            right,
        })
    }
}
impl CommandWriter for XxZoneItemV1FrameExtentsEvent {
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
        let interface =
            Self::writer_event_interface(state, self.object_id, "xx_zone_item_v1", Self::SINCE)?;
        if !matches!(
            interface,
            ObjectInteface::XxZones(XxZonesObjectInteface::XxZoneItemV1 { .. })
        ) || self.payload_size() != writer.remaining_quota()
        {
            return Err(FdWriterError::InvalidProtocol);
        }
        writer.next_i32(self.top).await?;
        writer.next_i32(self.bottom).await?;
        writer.next_i32(self.left).await?;
        writer.next_i32(self.right).await?;
        Ok(())
    }
}
