use super::ZwlrLayerSurfaceV1KeyboardInteractivity;
use crate::protocol::wlr::layer_shell::ObjectInteface as WlrLayerShellObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, CommandWriterExt, InterfaceReader,
    ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Client request `zwlr_layer_surface_v1.set_keyboard_interactivity`.
///
/// Opcode 4; available since version 1.
/// Wire framing and receiver metadata are checked; protocol semantics belong to the caller.
/// ```xml
/// <request name="set_keyboard_interactivity">
///       <description summary="requests keyboard events">
///         Set how keyboard events are delivered to this surface. By default,
///         layer shell surfaces do not receive keyboard events; this request can
///         be used to change this.
///
///         This setting is inherited by child surfaces set by the get_popup
///         request.
///
///         Layer surfaces receive pointer, touch, and tablet events normally. If
///         you do not want to receive them, set the input region on your surface
///         to an empty region.
///
///         Keyboard interactivity is double-buffered, see wl_surface.commit.
///       </description>
///       <arg name="keyboard_interactivity" type="uint" enum="keyboard_interactivity" />
///     </request>
/// ```
#[derive(Debug)]
pub struct ZwlrLayerSurfaceV1SetKeyboardInteractivityRequest {
    /// Wire identifier of the receiver.
    pub object_id: u32,
    /// Wire argument `keyboard_interactivity`.
    pub keyboard_interactivity: ZwlrLayerSurfaceV1KeyboardInteractivity,
}
impl ZwlrLayerSurfaceV1SetKeyboardInteractivityRequest {
    /// Opcode in the request namespace.
    pub const OPCODE: u16 = 4;
    /// Minimum supported interface version.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for ZwlrLayerSurfaceV1SetKeyboardInteractivityRequest {
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
impl InterfaceReader for ZwlrLayerSurfaceV1SetKeyboardInteractivityRequest {
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
        let keyboard_interactivity =
            ZwlrLayerSurfaceV1KeyboardInteractivity(reader.next_u32().await?);
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        Ok(Self {
            object_id,
            keyboard_interactivity,
        })
    }
}
impl CommandWriter for ZwlrLayerSurfaceV1SetKeyboardInteractivityRequest {
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
        writer.next_u32(self.keyboard_interactivity.0).await?;
        Ok(())
    }
}
