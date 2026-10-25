use crate::protocol::wlr::layer_shell::ObjectInteface as WlrLayerShellObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, CommandWriterExt, InterfaceReader,
    ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Client request `zwlr_layer_surface_v1.get_popup`.
///
/// Opcode 5; available since version 1.
/// Wire framing and receiver metadata are checked; protocol semantics belong to the caller.
/// ```xml
/// <request name="get_popup">
///       <description summary="assign this layer_surface as an xdg_popup parent">
///         This assigns an xdg_popup's parent to this layer_surface.  This popup
///         should have been created via xdg_surface::get_popup with the parent set
///         to NULL, and this request must be invoked before committing the popup's
///         initial state.
///
///         See the documentation of xdg_popup for more details about what an
///         xdg_popup is and how it is used.
///       </description>
///       <arg name="popup" type="object" interface="xdg_popup" />
///     </request>
/// ```
#[derive(Debug)]
pub struct ZwlrLayerSurfaceV1GetPopupRequest {
    /// Wire identifier of the receiver.
    pub object_id: u32,
    /// Wire argument `popup`.
    pub popup: u32,
}
impl ZwlrLayerSurfaceV1GetPopupRequest {
    /// Opcode in the request namespace.
    pub const OPCODE: u16 = 5;
    /// Minimum supported interface version.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for ZwlrLayerSurfaceV1GetPopupRequest {
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
impl InterfaceReader for ZwlrLayerSurfaceV1GetPopupRequest {
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
        let popup = reader.next_u32().await?;
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        Ok(Self { object_id, popup })
    }
}
impl CommandWriter for ZwlrLayerSurfaceV1GetPopupRequest {
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
        writer.next_u32(self.popup).await?;
        Ok(())
    }
}
