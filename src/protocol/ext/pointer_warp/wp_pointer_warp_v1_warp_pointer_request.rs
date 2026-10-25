use crate::protocol::ext::pointer_warp::ObjectInteface as PointerWarpObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, CommandWriterExt, InterfaceReader,
    ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::FdWriter;
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Client request `wp_pointer_warp_v1.warp_pointer`.
///
/// Opcode 1; available since version 1.
/// Wire framing and receiver metadata are checked; protocol semantics belong to the caller.
/// ```xml
/// <request name="warp_pointer">
///       <description summary="reposition the pointer">
///         Request the compositor to move the pointer to a surface-local position.
///         Whether or not the compositor honors the request is implementation defined,
///         but it should
///         - honor it if the surface has pointer focus, including
///           when it has an implicit pointer grab
///         - reject it if the enter serial is incorrect
///         - reject it if the requested position is outside of the surface
///
///         Note that the enter serial is valid for any surface of the client,
///         and does not have to be from the surface the pointer is warped to.
///
///       </description>
///       <arg name="surface" type="object" interface="wl_surface" summary="surface to position the pointer on" />
///       <arg name="pointer" type="object" interface="wl_pointer" summary="the pointer that should be repositioned" />
///       <arg name="x" type="fixed" />
///       <arg name="y" type="fixed" />
///       <arg name="serial" type="uint" summary="serial number of the enter event" />
///     </request>
/// ```
#[derive(Debug)]
pub struct WpPointerWarpV1WarpPointerRequest {
    /// Wire identifier of the receiver.
    pub object_id: u32,
    /// surface to position the pointer on.
    pub surface: u32,
    /// the pointer that should be repositioned.
    pub pointer: u32,
    /// Wire argument `x`.
    pub x: f64,
    /// Wire argument `y`.
    pub y: f64,
    /// serial number of the enter event.
    pub serial: u32,
}
impl WpPointerWarpV1WarpPointerRequest {
    /// Opcode in the request namespace.
    pub const OPCODE: u16 = 1;
    /// Minimum supported interface version.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for WpPointerWarpV1WarpPointerRequest {
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
impl InterfaceReader for WpPointerWarpV1WarpPointerRequest {
    async fn read_with_interface(
        object_id: u32,
        opcode: u16,
        interface: ObjectInteface,
        reader: &mut FdPayloadReader<'_>,
        _state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        if !matches!(
            interface,
            ObjectInteface::PointerWarp(PointerWarpObjectInteface::WpPointerWarpV1 { .. })
        ) || opcode != Self::OPCODE
            || interface.version() < Self::SINCE
        {
            return Err(FdReaderError::InvalidProtocol);
        }
        let surface = reader.next_u32().await?;
        let pointer = reader.next_u32().await?;
        let x = reader.next_fixed().await?;
        let y = reader.next_fixed().await?;
        let serial = reader.next_u32().await?;
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        Ok(Self {
            object_id,
            surface,
            pointer,
            x,
            y,
            serial,
        })
    }
}
impl CommandWriter for WpPointerWarpV1WarpPointerRequest {
    fn object_id(&self) -> u32 {
        self.object_id
    }
    fn opcode(&self) -> u16 {
        Self::OPCODE
    }
    fn payload_size(&self) -> usize {
        Self::total_payload_size(&[Some(4), Some(4), Some(4), Some(4), Some(4)])
    }
    async fn write(
        self,
        writer: &mut FdPayloadWriter<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<(), FdWriterError> {
        Self::writer_request_interface(state, self.object_id, "wp_pointer_warp_v1", Self::SINCE)?;
        if self.payload_size() != writer.remaining_quota() {
            return Err(FdWriterError::InvalidProtocol);
        }
        if !FdWriter::is_valid_fixed(self.x) {
            return Err(FdWriterError::InvalidProtocol);
        }
        if !FdWriter::is_valid_fixed(self.y) {
            return Err(FdWriterError::InvalidProtocol);
        }
        writer.next_u32(self.surface).await?;
        writer.next_u32(self.pointer).await?;
        writer.next_fixed(self.x).await?;
        writer.next_fixed(self.y).await?;
        writer.next_u32(self.serial).await?;
        Ok(())
    }
}
