use crate::protocol::wlr::layer_shell::ObjectInteface as WlrLayerShellObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, CommandWriterExt, InterfaceReader,
    ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Client request `zwlr_layer_surface_v1.set_margin`.
///
/// Opcode 3; available since version 1.
/// Wire framing and receiver metadata are checked; protocol semantics belong to the caller.
/// ```xml
/// <request name="set_margin">
///       <description summary="sets a margin from the anchor point">
///         Requests that the surface be placed some distance away from the anchor
///         point on the output, in surface-local coordinates. Setting this value
///         for edges you are not anchored to has no effect.
///
///         The exclusive zone includes the margin.
///
///         Margin is double-buffered, see wl_surface.commit.
///       </description>
///       <arg name="top" type="int" />
///       <arg name="right" type="int" />
///       <arg name="bottom" type="int" />
///       <arg name="left" type="int" />
///     </request>
/// ```
#[derive(Debug)]
pub struct ZwlrLayerSurfaceV1SetMarginRequest {
    /// Wire identifier of the receiver.
    pub object_id: u32,
    /// Wire argument `top`.
    pub top: i32,
    /// Wire argument `right`.
    pub right: i32,
    /// Wire argument `bottom`.
    pub bottom: i32,
    /// Wire argument `left`.
    pub left: i32,
}
impl ZwlrLayerSurfaceV1SetMarginRequest {
    /// Opcode in the request namespace.
    pub const OPCODE: u16 = 3;
    /// Minimum supported interface version.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for ZwlrLayerSurfaceV1SetMarginRequest {
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
impl InterfaceReader for ZwlrLayerSurfaceV1SetMarginRequest {
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
        let top = reader.next_i32().await?;
        let right = reader.next_i32().await?;
        let bottom = reader.next_i32().await?;
        let left = reader.next_i32().await?;
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        Ok(Self {
            object_id,
            top,
            right,
            bottom,
            left,
        })
    }
}
impl CommandWriter for ZwlrLayerSurfaceV1SetMarginRequest {
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
        Self::writer_request_interface(
            state,
            self.object_id,
            "zwlr_layer_surface_v1",
            Self::SINCE,
        )?;
        if self.payload_size() != writer.remaining_quota() {
            return Err(FdWriterError::InvalidProtocol);
        }
        writer.next_i32(self.top).await?;
        writer.next_i32(self.right).await?;
        writer.next_i32(self.bottom).await?;
        writer.next_i32(self.left).await?;
        Ok(())
    }
}
