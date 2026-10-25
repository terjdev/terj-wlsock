use super::ObjectInteface as XxZonesObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, CommandWriterExt, InterfaceReader,
    ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Server event `xx_zone_v1.size`.
///
/// Framing, receiver protocol and version are checked; semantics belong to the caller.
/// ```xml
/// <event name="size">
///       <description summary="size of the zone">
///         The 'size' event describes the size of this zone.
///
///         It is a rectangle with its origin in the top-left corner, using
///         the surface coordinate space (device pixels divided by the scaling
///         factor of the output this zone is attached to).
///
///         If a width or height value is zero, the zone is infinite
///         in that direction.
///
///         If the width and height values are negative, the zone is considered
///         "invalid" and must not be used.
///         A size event declaring the zone invalid may only be emitted immediately
///         after the zone was created.
///         A zone must not become invalid at a later time by sending a negative
///         'size' after the zone has been established.
///
///         The 'size' event is sent immediately after creating an 'xx_zone_v1',
///         and whenever the size of the zone changes. A zone size can change at
///         any time, for any reason, for example due to output size or scaling
///         changes, or by compositor policy.
///
///         Upon subsequent emissions of 'size' after 'xx_zone' has already
///         been created, the 'done' event does not have to be sent again.
///       </description>
///       <arg name="width" type="int" summary="zone width in logical pixels" />
///       <arg name="height" type="int" summary="zone height in logical pixels" />
///     </event>
/// ```
#[derive(Debug)]
pub struct XxZoneV1SizeEvent {
    /// Wire identifier of the receiver.
    pub object_id: u32,
    /// zone width in logical pixels.
    pub width: i32,
    /// zone height in logical pixels.
    pub height: i32,
}
impl XxZoneV1SizeEvent {
    /// Opcode in the event namespace.
    pub const OPCODE: u16 = 0;
    /// Minimum interface version.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for XxZoneV1SizeEvent {
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
impl InterfaceReader for XxZoneV1SizeEvent {
    async fn read_with_interface(
        object_id: u32,
        opcode: u16,
        interface: ObjectInteface,
        reader: &mut FdPayloadReader<'_>,
        _state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        if !matches!(
            interface,
            ObjectInteface::XxZones(XxZonesObjectInteface::XxZoneV1 { .. })
        ) || opcode != Self::OPCODE
            || interface.version() < Self::SINCE
        {
            return Err(FdReaderError::InvalidProtocol);
        }
        let width = reader.next_i32().await?;
        let height = reader.next_i32().await?;
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        Ok(Self {
            object_id,
            width,
            height,
        })
    }
}
impl CommandWriter for XxZoneV1SizeEvent {
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
        let interface =
            Self::writer_event_interface(state, self.object_id, "xx_zone_v1", Self::SINCE)?;
        if !matches!(
            interface,
            ObjectInteface::XxZones(XxZonesObjectInteface::XxZoneV1 { .. })
        ) || self.payload_size() != writer.remaining_quota()
        {
            return Err(FdWriterError::InvalidProtocol);
        }
        writer.next_i32(self.width).await?;
        writer.next_i32(self.height).await?;
        Ok(())
    }
}
