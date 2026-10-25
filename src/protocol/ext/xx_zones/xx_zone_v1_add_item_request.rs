use super::ObjectInteface as XxZonesObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, CommandWriterExt, InterfaceReader,
    ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Client request `xx_zone_v1.add_item`.
///
/// Framing, receiver protocol and version are checked; semantics belong to the caller.
/// ```xml
/// <request name="add_item">
///       <description summary="associate an item with this zone">
///         Make 'item' a member of this zone.
///         This state is double-buffered and is applied on the next
///         'wl_surface.commit' of the surface represented by 'item'.
///
///         This request associates an item with this zone.
///         If this request is called on an item that already has a zone
///         association with a different zone, the item must leave its old zone
///         (with 'item_left' being emitted on its old zone) and will instead
///         be associated with this zone.
///
///         Upon receiving this request and if the target zone is allowed for 'item',
///         a compositor must emit 'item_entered' to confirm the zone association.
///         It must even emit this event if the item was already associated with this
///         zone before.
///
///         The compositor must move the surface represented by 'item' into the
///         boundary of this zone upon receiving this request and accepting it
///         (either by extending the zone size, or by moving the item surface).
///
///         If the compositor does not allow the item to switch zone associations,
///         and wants it to remain in its previous zone, it must emit
///         'item_blocked' instead.
///         Compositors might want to prevent zone associations if they
///         perform specialized window management (e.g. autotiling) that would
///         make clients moving items between certain zones undesirable.
///
///         Once the 'item' is added to its zone, the compositor must first send
///         a 'frame_extents' event on the item, followed by an initial 'position'
///         event with the item's current position.
///         The compositor must then send 'position' events when the position
///         of the item in its zone is changed, for as long as the item is
///         associated with a zone.
///
///         If the zone is invalid, an 'invalid' error must be raised and the item
///         must not be associated with the invalid zone.
///         If the referenced item is inert (its underlying surface has been
///         destroyed), the request must be silently ignored.
///       </description>
///       <arg name="item" type="object" interface="xx_zone_item_v1" summary="the zone item" />
///     </request>
/// ```
#[derive(Debug)]
pub struct XxZoneV1AddItemRequest {
    /// Wire identifier of the receiver.
    pub object_id: u32,
    /// the zone item.
    pub item: u32,
}
impl XxZoneV1AddItemRequest {
    /// Opcode in the request namespace.
    pub const OPCODE: u16 = 1;
    /// Minimum interface version.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for XxZoneV1AddItemRequest {
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
impl InterfaceReader for XxZoneV1AddItemRequest {
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
        let item = reader.next_u32().await?;
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        Ok(Self { object_id, item })
    }
}
impl CommandWriter for XxZoneV1AddItemRequest {
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
            Self::writer_request_interface(state, self.object_id, "xx_zone_v1", Self::SINCE)?;
        if !matches!(
            interface,
            ObjectInteface::XxZones(XxZonesObjectInteface::XxZoneV1 { .. })
        ) || self.payload_size() != writer.remaining_quota()
        {
            return Err(FdWriterError::InvalidProtocol);
        }
        writer.next_u32(self.item).await?;
        Ok(())
    }
}
