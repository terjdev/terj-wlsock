use super::ObjectInteface as XxZonesObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, CommandWriterExt, InterfaceReader,
    ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Client request `xx_zone_item_v1.set_position`.
///
/// Framing, receiver protocol and version are checked; semantics belong to the caller.
/// ```xml
/// <request name="set_position">
///       <description summary="set a preferred item position">
///         Request a preferred position (x, y) for the frame of the specified
///         item to be placed at, relative to its associated zone.
///         This state is double-buffered and is applied on the next
///         wl_surface.commit of the surface represented by 'item'.
///
///         X and Y coordinates are relative to the zone this item is associated
///         with, and must not be larger than the dimensions set by the zone size.
///         They may be smaller than zero, if the item's top-left edge is to be
///         placed beyond the zone's top-left sides, but clients should expect the
///         compositor to more aggressively sanitize the coordinate values in that
///         case.
///         If a coordinate exceeds the zone's maximum bounds, the compositor must
///         sanitize it to more appropriate values (e.g. by clamping the values to
///         the maximum size).
///         For infinite zones, the client may pick any coordinate.
///
///         Compositors implementing this protocol should try to place an item
///         at the requested coordinates relative to the item's zone, unless doing
///         so is not allowed by compositor policy (because e.g. the user has set
///         custom rules for the surface represented by the respective item, the
///         surface overlaps with a protected shell component, session management
///         has loaded previous surface positions or the placement request would
///         send the item out of bounds).
///
///         Clients should be aware that their placement preferences might not
///         always be followed and must be prepared to handle the case where the
///         item is placed at a different position by the compositor.
///
///         Once an item has been mapped, a change to its preferred placement can
///         still be requested and should be applied, but must not be followed
///         by the compositor while the user is interacting with the affected item
///         surface (e.g. clicking &amp; dragging within the window, or resizing it).
///
///         After a call to this request, a 'position' event must be emitted with the
///         item's new actual position, once the request has been applied. If the
///         item already is at the requested (or sanitized) position, the request
///         is considered successful and a 'position' event must still be emitted.
///         If the current item has no zone associated with it, a 'position_failed'
///         event must be emitted instead.
///         If the compositor refused to move the item at all, not even to sanitized
///         coordinates (e.g. because compositor policy does not allow the item to
///         be moved in its current state), a 'position_failed' event must be
///         emitted as well.
///       </description>
///       <arg name="x" type="int" summary="x position relative to zone" />
///       <arg name="y" type="int" summary="y position relative to zone" />
///     </request>
/// ```
#[derive(Debug)]
pub struct XxZoneItemV1SetPositionRequest {
    /// Wire identifier of the receiver.
    pub object_id: u32,
    /// x position relative to zone.
    pub x: i32,
    /// y position relative to zone.
    pub y: i32,
}
impl XxZoneItemV1SetPositionRequest {
    /// Opcode in the request namespace.
    pub const OPCODE: u16 = 1;
    /// Minimum interface version.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for XxZoneItemV1SetPositionRequest {
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
impl InterfaceReader for XxZoneItemV1SetPositionRequest {
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
        let x = reader.next_i32().await?;
        let y = reader.next_i32().await?;
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        Ok(Self { object_id, x, y })
    }
}
impl CommandWriter for XxZoneItemV1SetPositionRequest {
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
            Self::writer_request_interface(state, self.object_id, "xx_zone_item_v1", Self::SINCE)?;
        if !matches!(
            interface,
            ObjectInteface::XxZones(XxZonesObjectInteface::XxZoneItemV1 { .. })
        ) || self.payload_size() != writer.remaining_quota()
        {
            return Err(FdWriterError::InvalidProtocol);
        }
        writer.next_i32(self.x).await?;
        writer.next_i32(self.y).await?;
        Ok(())
    }
}
