use crate::protocol::wlr::layer_shell::ObjectInteface as WlrLayerShellObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, CommandWriterExt, InterfaceReader,
    ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Server event `zwlr_layer_surface_v1.configure`.
///
/// Opcode 0; available since version 1.
/// Wire framing and receiver metadata are checked; protocol semantics belong to the caller.
/// ```xml
/// <event name="configure">
///       <description summary="suggest a surface change">
///         The configure event asks the client to resize its surface.
///
///         Clients should arrange their surface for the new states, and then send
///         an ack_configure request with the serial sent in this configure event at
///         some point before committing the new surface.
///
///         The client is free to dismiss all but the last configure event it
///         received.
///
///         The width and height arguments specify the size of the window in
///         surface-local coordinates.
///
///         The size is a hint, in the sense that the client is free to ignore it if
///         it doesn't resize, pick a smaller size (to satisfy aspect ratio or
///         resize in steps of NxM pixels). If the client picks a smaller size and
///         is anchored to two opposite anchors (e.g. 'top' and 'bottom'), the
///         surface will be centered on this axis.
///
///         If the width or height arguments are zero, it means the client should
///         decide its own window dimension.
///       </description>
///       <arg name="serial" type="uint" />
///       <arg name="width" type="uint" />
///       <arg name="height" type="uint" />
///     </event>
/// ```
#[derive(Debug)]
pub struct ZwlrLayerSurfaceV1ConfigureEvent {
    /// Wire identifier of the receiver.
    pub object_id: u32,
    /// Wire argument `serial`.
    pub serial: u32,
    /// Wire argument `width`.
    pub width: u32,
    /// Wire argument `height`.
    pub height: u32,
}
impl ZwlrLayerSurfaceV1ConfigureEvent {
    /// Opcode in the event namespace.
    pub const OPCODE: u16 = 0;
    /// Minimum supported interface version.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for ZwlrLayerSurfaceV1ConfigureEvent {
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
impl InterfaceReader for ZwlrLayerSurfaceV1ConfigureEvent {
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
        let serial = reader.next_u32().await?;
        let width = reader.next_u32().await?;
        let height = reader.next_u32().await?;
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        Ok(Self {
            object_id,
            serial,
            width,
            height,
        })
    }
}
impl CommandWriter for ZwlrLayerSurfaceV1ConfigureEvent {
    fn object_id(&self) -> u32 {
        self.object_id
    }
    fn opcode(&self) -> u16 {
        Self::OPCODE
    }
    fn payload_size(&self) -> usize {
        Self::total_payload_size(&[Some(4), Some(4), Some(4)])
    }
    async fn write(
        self,
        writer: &mut FdPayloadWriter<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<(), FdWriterError> {
        Self::writer_event_interface(state, self.object_id, "zwlr_layer_surface_v1", Self::SINCE)?;
        if self.payload_size() != writer.remaining_quota() {
            return Err(FdWriterError::InvalidProtocol);
        }
        writer.next_u32(self.serial).await?;
        writer.next_u32(self.width).await?;
        writer.next_u32(self.height).await?;
        Ok(())
    }
}
