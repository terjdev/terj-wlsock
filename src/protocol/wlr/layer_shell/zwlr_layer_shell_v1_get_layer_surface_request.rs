use super::ZwlrLayerShellV1Layer;
use crate::protocol::wlr::layer_shell::ObjectInteface as WlrLayerShellObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, CommandWriterExt, InterfaceReader,
    ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::FdWriter;
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Client request `zwlr_layer_shell_v1.get_layer_surface`.
///
/// Opcode 0; available since version 1.
/// Wire framing and receiver metadata are checked; protocol semantics belong to the caller.
/// New IDs are registered before writes; close the connection after a failed or cancelled write.
/// ```xml
/// <request name="get_layer_surface">
///       <description summary="create a layer_surface from a surface">
///         Create a layer surface for an existing surface. This assigns the role of
///         layer_surface, or raises a protocol error if another role is already
///         assigned.
///
///         Creating a layer surface from a wl_surface which has a buffer attached
///         or committed is a client error, and any attempts by a client to attach
///         or manipulate a buffer prior to the first layer_surface.configure call
///         must also be treated as errors.
///
///         After creating a layer_surface object and setting it up, the client
///         must perform an initial commit without any buffer attached.
///         The compositor will reply with a layer_surface.configure event.
///         The client must acknowledge it and is then allowed to attach a buffer
///         to map the surface.
///
///         You may pass NULL for output to allow the compositor to decide which
///         output to use. Generally this will be the one that the user most
///         recently interacted with.
///
///         Clients can specify a namespace that defines the purpose of the layer
///         surface.
///       </description>
///       <arg name="id" type="new_id" interface="zwlr_layer_surface_v1" />
///       <arg name="surface" type="object" interface="wl_surface" />
///       <arg name="output" type="object" interface="wl_output" allow-null="true" />
///       <arg name="layer" type="uint" enum="layer" summary="layer to add this surface to" />
///       <arg name="namespace" type="string" summary="namespace for the layer surface" />
///     </request>
/// ```
#[derive(Debug)]
pub struct ZwlrLayerShellV1GetLayerSurfaceRequest {
    /// Wire identifier of the receiver.
    pub object_id: u32,
    /// Wire argument `id`.
    pub id: u32,
    /// Wire argument `surface`.
    pub surface: u32,
    /// Wire argument `output`.
    pub output: Option<u32>,
    /// layer to add this surface to.
    pub layer: ZwlrLayerShellV1Layer,
    /// namespace for the layer surface.
    pub namespace: String,
}
impl ZwlrLayerShellV1GetLayerSurfaceRequest {
    /// Opcode in the request namespace.
    pub const OPCODE: u16 = 0;
    /// Minimum supported interface version.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for ZwlrLayerShellV1GetLayerSurfaceRequest {
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
impl InterfaceReader for ZwlrLayerShellV1GetLayerSurfaceRequest {
    async fn read_with_interface(
        object_id: u32,
        opcode: u16,
        interface: ObjectInteface,
        reader: &mut FdPayloadReader<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        if !matches!(
            interface,
            ObjectInteface::WlrLayerShell(WlrLayerShellObjectInteface::ZwlrLayerShellV1 { .. })
        ) || opcode != Self::OPCODE
            || interface.version() < Self::SINCE
        {
            return Err(FdReaderError::InvalidProtocol);
        }
        let id = reader.next_u32().await?;
        let surface = reader.next_u32().await?;
        let output = {
            let id = reader.next_u32().await?;
            (id != 0).then_some(id)
        };
        let layer = ZwlrLayerShellV1Layer(reader.next_u32().await?);
        let namespace = reader.next_string().await?;
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        {
            let new_interface =
                ObjectInteface::from_name("zwlr_layer_surface_v1", interface.version())
                    .ok_or(FdReaderError::InvalidProtocol)?;
            Self::reader_state(state)?
                .register_object(id, new_interface, false)
                .map_err(|_| FdReaderError::InvalidProtocol)?;
        }
        Ok(Self {
            object_id,
            id,
            surface,
            output,
            layer,
            namespace,
        })
    }
}
impl CommandWriter for ZwlrLayerShellV1GetLayerSurfaceRequest {
    fn object_id(&self) -> u32 {
        self.object_id
    }
    fn opcode(&self) -> u16 {
        Self::OPCODE
    }
    fn payload_size(&self) -> usize {
        Self::total_payload_size(&[
            Some(4),
            Some(4),
            Some(4),
            Some(4),
            FdWriter::encoded_string_size(Some(&self.namespace)),
        ])
    }
    async fn write(
        self,
        writer: &mut FdPayloadWriter<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<(), FdWriterError> {
        let interface = Self::writer_request_interface(
            state,
            self.object_id,
            "zwlr_layer_shell_v1",
            Self::SINCE,
        )?;
        if self.payload_size() != writer.remaining_quota() {
            return Err(FdWriterError::InvalidProtocol);
        }
        if !FdWriter::is_valid_string(&self.namespace) {
            return Err(FdWriterError::InvalidProtocol);
        }
        {
            let new_interface =
                ObjectInteface::from_name("zwlr_layer_surface_v1", interface.version())
                    .ok_or(FdWriterError::InvalidProtocol)?;
            Self::writer_state(state)?
                .register_object(self.id, new_interface, false)
                .map_err(|_| FdWriterError::InvalidProtocol)?;
        }
        writer.next_u32(self.id).await?;
        writer.next_u32(self.surface).await?;
        writer.next_u32(self.output.unwrap_or(0)).await?;
        writer.next_u32(self.layer.0).await?;
        writer.next_string(&self.namespace).await?;
        Ok(())
    }
}
