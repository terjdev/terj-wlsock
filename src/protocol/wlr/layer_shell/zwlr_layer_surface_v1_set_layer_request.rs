use super::ZwlrLayerShellV1Layer;
use crate::protocol::wlr::layer_shell::ObjectInteface as WlrLayerShellObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, CommandWriterExt, InterfaceReader,
    ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Client request `zwlr_layer_surface_v1.set_layer`.
///
/// Opcode 8; available since version 2.
/// Wire framing and receiver metadata are checked; protocol semantics belong to the caller.
/// ```xml
/// <request name="set_layer" since="2">
///       <description summary="change the layer of the surface">
///         Change the layer that the surface is rendered on.
///
///         Layer is double-buffered, see wl_surface.commit.
///       </description>
///       <arg name="layer" type="uint" enum="zwlr_layer_shell_v1.layer" summary="layer to move this surface to" />
///     </request>
/// ```
#[derive(Debug)]
pub struct ZwlrLayerSurfaceV1SetLayerRequest {
    /// Wire identifier of the receiver.
    pub object_id: u32,
    /// layer to move this surface to.
    pub layer: ZwlrLayerShellV1Layer,
}
impl ZwlrLayerSurfaceV1SetLayerRequest {
    /// Opcode in the request namespace.
    pub const OPCODE: u16 = 8;
    /// Minimum supported interface version.
    pub const SINCE: u32 = 2;
}
impl CommandReader<Self> for ZwlrLayerSurfaceV1SetLayerRequest {
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
impl InterfaceReader for ZwlrLayerSurfaceV1SetLayerRequest {
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
        let layer = ZwlrLayerShellV1Layer(reader.next_u32().await?);
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        Ok(Self { object_id, layer })
    }
}
impl CommandWriter for ZwlrLayerSurfaceV1SetLayerRequest {
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
        writer.next_u32(self.layer.0).await?;
        Ok(())
    }
}
