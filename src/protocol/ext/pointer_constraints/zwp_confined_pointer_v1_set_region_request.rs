use super::ObjectInteface as PointerConstraintsObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, CommandWriterExt, InterfaceReader,
    ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Client request `zwp_confined_pointer_v1.set_region`.
///
/// Framing, receiver protocol and version are checked; semantics belong to the caller.
/// ```xml
/// <request name="set_region">
///       <description summary="set a new confine region">
///     Set a new region used to confine the pointer.
///
///     The new confine region is double-buffered, see wl_surface.commit.
///
///     If the confinement is active when the new confinement region is applied
///     and the pointer ends up outside of newly applied region, the pointer may
///     warped to a position within the new confinement region. If warped, a
///     wl_pointer.motion event will be emitted, but no
///     wp_relative_pointer.relative_motion event.
///
///     The compositor may also, instead of using the new region, unconfine the
///     pointer.
///
///     For details about the confine region, see wp_confined_pointer.
///       </description>
///       <arg name="region" type="object" interface="wl_region" allow-null="true" summary="region of surface" />
///     </request>
/// ```
#[derive(Debug)]
pub struct ZwpConfinedPointerV1SetRegionRequest {
    /// Wire identifier of the receiver.
    pub object_id: u32,
    /// region of surface.
    pub region: Option<u32>,
}
impl ZwpConfinedPointerV1SetRegionRequest {
    /// Opcode in the request namespace.
    pub const OPCODE: u16 = 1;
    /// Minimum interface version.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for ZwpConfinedPointerV1SetRegionRequest {
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
impl InterfaceReader for ZwpConfinedPointerV1SetRegionRequest {
    async fn read_with_interface(
        object_id: u32,
        opcode: u16,
        interface: ObjectInteface,
        reader: &mut FdPayloadReader<'_>,
        _state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        if !matches!(
            interface,
            ObjectInteface::PointerConstraints(
                PointerConstraintsObjectInteface::ZwpConfinedPointerV1 { .. }
            )
        ) || opcode != Self::OPCODE
            || interface.version() < Self::SINCE
        {
            return Err(FdReaderError::InvalidProtocol);
        }
        let region = {
            let id = reader.next_u32().await?;
            (id != 0).then_some(id)
        };
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        Ok(Self { object_id, region })
    }
}
impl CommandWriter for ZwpConfinedPointerV1SetRegionRequest {
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
        let interface = Self::writer_request_interface(
            state,
            self.object_id,
            "zwp_confined_pointer_v1",
            Self::SINCE,
        )?;
        if !matches!(
            interface,
            ObjectInteface::PointerConstraints(
                PointerConstraintsObjectInteface::ZwpConfinedPointerV1 { .. }
            )
        ) || self.payload_size() != writer.remaining_quota()
        {
            return Err(FdWriterError::InvalidProtocol);
        }
        writer.next_u32(self.region.unwrap_or(0)).await?;
        Ok(())
    }
}
