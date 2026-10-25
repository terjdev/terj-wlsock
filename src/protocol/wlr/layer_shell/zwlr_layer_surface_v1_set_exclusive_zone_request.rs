use crate::protocol::wlr::layer_shell::ObjectInteface as WlrLayerShellObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, CommandWriterExt, InterfaceReader,
    ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Client request `zwlr_layer_surface_v1.set_exclusive_zone`.
///
/// Opcode 2; available since version 1.
/// Wire framing and receiver metadata are checked; protocol semantics belong to the caller.
/// ```xml
/// <request name="set_exclusive_zone">
///       <description summary="configures the exclusive geometry of this surface">
///         Requests that the compositor avoids occluding an area with other
///         surfaces. The compositor's use of this information is
///         implementation-dependent - do not assume that this region will not
///         actually be occluded.
///
///         A positive value is only meaningful if the surface is anchored to one
///         edge or an edge and both perpendicular edges. If the surface is not
///         anchored, anchored to only two perpendicular edges (a corner), anchored
///         to only two parallel edges or anchored to all edges, a positive value
///         will be treated the same as zero.
///
///         A positive zone is the distance from the edge in surface-local
///         coordinates to consider exclusive.
///
///         Surfaces that do not wish to have an exclusive zone may instead specify
///         how they should interact with surfaces that do. If set to zero, the
///         surface indicates that it would like to be moved to avoid occluding
///         surfaces with a positive exclusive zone. If set to -1, the surface
///         indicates that it would not like to be moved to accommodate for other
///         surfaces, and the compositor should extend it all the way to the edges
///         it is anchored to.
///
///         For example, a panel might set its exclusive zone to 10, so that
///         maximized shell surfaces are not shown on top of it. A notification
///         might set its exclusive zone to 0, so that it is moved to avoid
///         occluding the panel, but shell surfaces are shown underneath it. A
///         wallpaper or lock screen might set their exclusive zone to -1, so that
///         they stretch below or over the panel.
///
///         The default value is 0.
///
///         Exclusive zone is double-buffered, see wl_surface.commit.
///       </description>
///       <arg name="zone" type="int" />
///     </request>
/// ```
#[derive(Debug)]
pub struct ZwlrLayerSurfaceV1SetExclusiveZoneRequest {
    /// Wire identifier of the receiver.
    pub object_id: u32,
    /// Wire argument `zone`.
    pub zone: i32,
}
impl ZwlrLayerSurfaceV1SetExclusiveZoneRequest {
    /// Opcode in the request namespace.
    pub const OPCODE: u16 = 2;
    /// Minimum supported interface version.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for ZwlrLayerSurfaceV1SetExclusiveZoneRequest {
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
impl InterfaceReader for ZwlrLayerSurfaceV1SetExclusiveZoneRequest {
    async fn read_with_interface(
        object_id: u32,
        opcode: u16,
        interface: ObjectInteface,
        reader: &mut FdPayloadReader<'_>,
        _state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        if !matches!(
            interface,
            ObjectInteface::WlrLayerShell(WlrLayerShellObjectInteface::ZwlrLayerSurfaceV1 { .. })
        ) || opcode != Self::OPCODE
            || interface.version() < Self::SINCE
        {
            return Err(FdReaderError::InvalidProtocol);
        }
        let zone = reader.next_i32().await?;
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        Ok(Self { object_id, zone })
    }
}
impl CommandWriter for ZwlrLayerSurfaceV1SetExclusiveZoneRequest {
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
        Self::writer_request_interface(
            state,
            self.object_id,
            "zwlr_layer_surface_v1",
            Self::SINCE,
        )?;
        if self.payload_size() != writer.remaining_quota() {
            return Err(FdWriterError::InvalidProtocol);
        }
        writer.next_i32(self.zone).await?;
        Ok(())
    }
}
