use super::ObjectInteface as XxZonesObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, CommandWriterExt, InterfaceReader,
    ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::FdWriter;
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Client request `xx_zone_manager_v1.get_zone_from_handle`.
///
/// Framing, receiver protocol and version are checked; semantics belong to the caller.
/// New IDs are registered before writes; close the connection after a failed or cancelled write.
/// ```xml
/// <request name="get_zone_from_handle">
///       <description summary="join a zone via its handle">
///         Create a new zone object using the zone's handle.
///         For the returned zone, the same rules as described in
///         'get_zone' apply.
///
///         This request returns a reference to an existing or remembered zone
///         that is represented by 'handle'.
///         The zone may potentially have been created by a different client.
///
///         This allows cooperating clients to share the same coordinate space.
///
///         If the zone handle was invalid or unknown, a new zone must
///         be created and returned instead, following the rules outlined
///         in 'get_zone' and assuming no output preference.
///
///         Every new zone object created by this request emits its initial event
///         sequence, including the 'handle' event, which must return a different
///         handle from the one passed to this request in case the existing zone
///         could not be joined.
///       </description>
///       <arg name="id" type="new_id" interface="xx_zone_v1" />
///       <arg name="handle" type="string" summary="the handle of a zone" />
///     </request>
/// ```
#[derive(Debug)]
pub struct XxZoneManagerV1GetZoneFromHandleRequest {
    /// Wire identifier of the receiver.
    pub object_id: u32,
    /// Wire argument `id`.
    pub id: u32,
    /// the handle of a zone.
    pub handle: String,
}
impl XxZoneManagerV1GetZoneFromHandleRequest {
    /// Opcode in the request namespace.
    pub const OPCODE: u16 = 3;
    /// Minimum interface version.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for XxZoneManagerV1GetZoneFromHandleRequest {
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
impl InterfaceReader for XxZoneManagerV1GetZoneFromHandleRequest {
    async fn read_with_interface(
        object_id: u32,
        opcode: u16,
        interface: ObjectInteface,
        reader: &mut FdPayloadReader<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        if !matches!(
            interface,
            ObjectInteface::XxZones(XxZonesObjectInteface::XxZoneManagerV1 { .. })
        ) || opcode != Self::OPCODE
            || interface.version() < Self::SINCE
        {
            return Err(FdReaderError::InvalidProtocol);
        }
        let id = reader.next_u32().await?;
        let handle = reader.next_string().await?;
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        {
            let new_interface = ObjectInteface::XxZones(XxZonesObjectInteface::XxZoneV1 {
                version: interface.version().min(1),
            });
            Self::reader_state(state)?
                .register_object(id, new_interface, false)
                .map_err(|_| FdReaderError::InvalidProtocol)?;
        }
        Ok(Self {
            object_id,
            id,
            handle,
        })
    }
}
impl CommandWriter for XxZoneManagerV1GetZoneFromHandleRequest {
    fn object_id(&self) -> u32 {
        self.object_id
    }
    fn opcode(&self) -> u16 {
        Self::OPCODE
    }
    fn payload_size(&self) -> usize {
        Self::total_payload_size(&[Some(4), FdWriter::encoded_string_size(Some(&self.handle))])
    }
    async fn write(
        self,
        writer: &mut FdPayloadWriter<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<(), FdWriterError> {
        let interface = Self::writer_request_interface(
            state,
            self.object_id,
            "xx_zone_manager_v1",
            Self::SINCE,
        )?;
        if !matches!(
            interface,
            ObjectInteface::XxZones(XxZonesObjectInteface::XxZoneManagerV1 { .. })
        ) || self.payload_size() != writer.remaining_quota()
        {
            return Err(FdWriterError::InvalidProtocol);
        }
        if !FdWriter::is_valid_string(&self.handle) {
            return Err(FdWriterError::InvalidProtocol);
        }
        {
            let new_interface = ObjectInteface::XxZones(XxZonesObjectInteface::XxZoneV1 {
                version: interface.version().min(1),
            });
            Self::writer_state(state)?
                .register_object(self.id, new_interface, false)
                .map_err(|_| FdWriterError::InvalidProtocol)?;
        }
        writer.next_u32(self.id).await?;
        writer.next_string(&self.handle).await?;
        Ok(())
    }
}
