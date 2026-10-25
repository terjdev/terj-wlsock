use super::ObjectInteface as WlrForeignToplevelManagementObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, CommandWriterExt, InterfaceReader,
    ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Client request `zwlr_foreign_toplevel_handle_v1.set_rectangle`.
///
/// Framing, receiver protocol and version are checked; semantics belong to the caller.
/// ```xml
/// <request name="set_rectangle">
///       <description summary="the rectangle which represents the toplevel">
///         The rectangle of the surface specified in this request corresponds to
///         the place where the app using this protocol represents the given toplevel.
///         It can be used by the compositor as a hint for some operations, e.g
///         minimizing. The client is however not required to set this, in which
///         case the compositor is free to decide some default value.
///
///         If the client specifies more than one rectangle, only the last one is
///         considered.
///
///         The dimensions are given in surface-local coordinates.
///         Setting width=height=0 removes the already-set rectangle.
///       </description>
///
///       <arg name="surface" type="object" interface="wl_surface" />
///       <arg name="x" type="int" />
///       <arg name="y" type="int" />
///       <arg name="width" type="int" />
///       <arg name="height" type="int" />
///     </request>
/// ```
#[derive(Debug)]
pub struct ZwlrForeignToplevelHandleV1SetRectangleRequest {
    /// Wire identifier of the receiver.
    pub object_id: u32,
    /// Wire argument `surface`.
    pub surface: u32,
    /// Wire argument `x`.
    pub x: i32,
    /// Wire argument `y`.
    pub y: i32,
    /// Wire argument `width`.
    pub width: i32,
    /// Wire argument `height`.
    pub height: i32,
}
impl ZwlrForeignToplevelHandleV1SetRectangleRequest {
    /// Opcode in the request namespace.
    pub const OPCODE: u16 = 6;
    /// Minimum interface version.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for ZwlrForeignToplevelHandleV1SetRectangleRequest {
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
impl InterfaceReader for ZwlrForeignToplevelHandleV1SetRectangleRequest {
    async fn read_with_interface(
        object_id: u32,
        opcode: u16,
        interface: ObjectInteface,
        reader: &mut FdPayloadReader<'_>,
        _state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        if !matches!(
            interface,
            ObjectInteface::WlrForeignToplevelManagement(
                WlrForeignToplevelManagementObjectInteface::ZwlrForeignToplevelHandleV1 { .. }
            )
        ) || opcode != Self::OPCODE
            || interface.version() < Self::SINCE
        {
            return Err(FdReaderError::InvalidProtocol);
        }
        let surface = reader.next_u32().await?;
        let x = reader.next_i32().await?;
        let y = reader.next_i32().await?;
        let width = reader.next_i32().await?;
        let height = reader.next_i32().await?;
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        Ok(Self {
            object_id,
            surface,
            x,
            y,
            width,
            height,
        })
    }
}
impl CommandWriter for ZwlrForeignToplevelHandleV1SetRectangleRequest {
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
        let interface = Self::writer_request_interface(
            state,
            self.object_id,
            "zwlr_foreign_toplevel_handle_v1",
            Self::SINCE,
        )?;
        if !matches!(
            interface,
            ObjectInteface::WlrForeignToplevelManagement(
                WlrForeignToplevelManagementObjectInteface::ZwlrForeignToplevelHandleV1 { .. }
            )
        ) || self.payload_size() != writer.remaining_quota()
        {
            return Err(FdWriterError::InvalidProtocol);
        }
        writer.next_u32(self.surface).await?;
        writer.next_i32(self.x).await?;
        writer.next_i32(self.y).await?;
        writer.next_i32(self.width).await?;
        writer.next_i32(self.height).await?;
        Ok(())
    }
}
