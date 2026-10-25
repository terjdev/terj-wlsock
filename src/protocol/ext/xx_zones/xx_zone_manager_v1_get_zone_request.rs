use super::ObjectInteface as XxZonesObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, CommandWriterExt, InterfaceReader,
    ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Client request `xx_zone_manager_v1.get_zone`.
///
/// Framing, receiver protocol and version are checked; semantics belong to the caller.
/// New IDs are registered before writes; close the connection after a failed or cancelled write.
/// ```xml
/// <request name="get_zone">
///       <description summary="join a zone or request a new one">
///         Create a new zone. While the zone object exists, the compositor
///         must consider it "used" and keep track of it.
///
///         A zone is represented by a string 'handle'.
///
///         The compositor must keep zone handles valid while any client is
///         referencing the corresponding zone.
///         The compositor may always give a client the same zone for a given
///         output, and remember its position and size for the client, but
///         clients should not rely on this behavior.
///
///         A client can request a zone to be placed on a specific
///         output by passing a wl_output as 'output'. If a valid output
///         is set, the compositor should place the zone on that output.
///         If NULL is passed, the compositor decides the output.
///
///         The compositor should provide the biggest reasonable zone space
///         for the client, governed by its own policy.
///
///         If the compositor wants to deny zone creation (e.g. on a specific
///         output), the returned zone must be "invalid". A zone is invalid
///         if it has a negative size, in which case the client is forbidden
///         to place items in it.
///       </description>
///       <arg name="id" type="new_id" interface="xx_zone_v1" />
///       <arg name="output" type="object" interface="wl_output" summary="the preferred output to place the zone on, or NULL" allow-null="true" />
///     </request>
/// ```
#[derive(Debug)]
pub struct XxZoneManagerV1GetZoneRequest {
    /// Wire identifier of the receiver.
    pub object_id: u32,
    /// Wire argument `id`.
    pub id: u32,
    /// the preferred output to place the zone on, or NULL.
    pub output: Option<u32>,
}
impl XxZoneManagerV1GetZoneRequest {
    /// Opcode in the request namespace.
    pub const OPCODE: u16 = 2;
    /// Minimum interface version.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for XxZoneManagerV1GetZoneRequest {
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
impl InterfaceReader for XxZoneManagerV1GetZoneRequest {
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
        let output = {
            let id = reader.next_u32().await?;
            (id != 0).then_some(id)
        };
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
            output,
        })
    }
}
impl CommandWriter for XxZoneManagerV1GetZoneRequest {
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
        {
            let new_interface = ObjectInteface::XxZones(XxZonesObjectInteface::XxZoneV1 {
                version: interface.version().min(1),
            });
            Self::writer_state(state)?
                .register_object(self.id, new_interface, false)
                .map_err(|_| FdWriterError::InvalidProtocol)?;
        }
        writer.next_u32(self.id).await?;
        writer.next_u32(self.output.unwrap_or(0)).await?;
        Ok(())
    }
}
