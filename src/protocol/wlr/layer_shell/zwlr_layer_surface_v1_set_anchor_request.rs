use super::ZwlrLayerSurfaceV1Anchor;
use crate::protocol::wlr::layer_shell::ObjectInteface as WlrLayerShellObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, CommandWriterExt, InterfaceReader,
    ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Client request `zwlr_layer_surface_v1.set_anchor`.
///
/// Opcode 1; available since version 1.
/// Wire framing and receiver metadata are checked; protocol semantics belong to the caller.
/// ```xml
/// <request name="set_anchor">
///       <description summary="configures the anchor point of the surface">
///         Requests that the compositor anchor the surface to the specified edges
///         and corners. If two orthogonal edges are specified (e.g. 'top' and
///         'left'), then the anchor point will be the intersection of the edges
///         (e.g. the top left corner of the output); otherwise the anchor point
///         will be centered on that edge, or in the center if none is specified.
///
///         Anchor is double-buffered, see wl_surface.commit.
///       </description>
///       <arg name="anchor" type="uint" enum="anchor" />
///     </request>
/// ```
#[derive(Debug)]
pub struct ZwlrLayerSurfaceV1SetAnchorRequest {
    /// Wire identifier of the receiver.
    pub object_id: u32,
    /// Wire argument `anchor`.
    pub anchor: ZwlrLayerSurfaceV1Anchor,
}
impl ZwlrLayerSurfaceV1SetAnchorRequest {
    /// Opcode in the request namespace.
    pub const OPCODE: u16 = 1;
    /// Minimum supported interface version.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for ZwlrLayerSurfaceV1SetAnchorRequest {
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
impl InterfaceReader for ZwlrLayerSurfaceV1SetAnchorRequest {
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
        let anchor = ZwlrLayerSurfaceV1Anchor(reader.next_u32().await?);
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        Ok(Self { object_id, anchor })
    }
}
impl CommandWriter for ZwlrLayerSurfaceV1SetAnchorRequest {
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
        writer.next_u32(self.anchor.0).await?;
        Ok(())
    }
}
