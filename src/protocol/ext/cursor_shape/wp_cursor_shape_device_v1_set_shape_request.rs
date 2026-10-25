use super::WpCursorShapeDeviceV1Shape;
use crate::protocol::ext::cursor_shape::ObjectInteface as CursorShapeObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, CommandWriterExt, InterfaceReader,
    ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Client request `wp_cursor_shape_device_v1.set_shape`.
///
/// Opcode 1; available since version 1.
/// Wire framing and receiver metadata are checked; protocol semantics belong to the caller.
/// ```xml
/// <request name="set_shape">
///       <description summary="set device cursor to the shape">
///         Sets the device cursor to the specified shape. The compositor will
///         change the cursor image based on the specified shape.
///
///         The cursor actually changes only if the input device focus is one of
///         the requesting client's surfaces. If any, the previous cursor image
///         (surface or shape) is replaced.
///
///         The "shape" argument must be a valid enum entry, otherwise the
///         invalid_shape protocol error is raised.
///
///         This is similar to the wl_pointer.set_cursor and
///         zwp_tablet_tool_v2.set_cursor requests, but this request accepts a
///         shape instead of contents in the form of a surface. Clients can mix
///         set_cursor and set_shape requests.
///
///         The serial parameter must match the latest wl_pointer.enter or
///         zwp_tablet_tool_v2.proximity_in serial number sent to the client.
///         Otherwise the request will be ignored.
///       </description>
///       <arg name="serial" type="uint" summary="serial number of the enter event" />
///       <arg name="shape" type="uint" enum="shape" />
///     </request>
/// ```
#[derive(Debug)]
pub struct WpCursorShapeDeviceV1SetShapeRequest {
    /// Wire identifier of the receiver.
    pub object_id: u32,
    /// serial number of the enter event.
    pub serial: u32,
    /// Wire argument `shape`.
    pub shape: WpCursorShapeDeviceV1Shape,
}
impl WpCursorShapeDeviceV1SetShapeRequest {
    /// Opcode in the request namespace.
    pub const OPCODE: u16 = 1;
    /// Minimum supported interface version.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for WpCursorShapeDeviceV1SetShapeRequest {
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
impl InterfaceReader for WpCursorShapeDeviceV1SetShapeRequest {
    async fn read_with_interface(
        object_id: u32,
        opcode: u16,
        interface: ObjectInteface,
        reader: &mut FdPayloadReader<'_>,
        _state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        if !matches!(
            interface,
            ObjectInteface::CursorShape(CursorShapeObjectInteface::WpCursorShapeDeviceV1 { .. })
        ) || opcode != Self::OPCODE
            || interface.version() < Self::SINCE
        {
            return Err(FdReaderError::InvalidProtocol);
        }
        let serial = reader.next_u32().await?;
        let shape = WpCursorShapeDeviceV1Shape(reader.next_u32().await?);
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        Ok(Self {
            object_id,
            serial,
            shape,
        })
    }
}
impl CommandWriter for WpCursorShapeDeviceV1SetShapeRequest {
    fn object_id(&self) -> u32 {
        self.object_id
    }
    fn opcode(&self) -> u16 {
        Self::OPCODE
    }
    fn payload_size(&self) -> usize {
        Self::total_payload_size(&[Some(4), Some(4)])
    }
    async fn write(
        self,
        writer: &mut FdPayloadWriter<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<(), FdWriterError> {
        Self::writer_request_interface(
            state,
            self.object_id,
            "wp_cursor_shape_device_v1",
            Self::SINCE,
        )?;
        if self.payload_size() != writer.remaining_quota() {
            return Err(FdWriterError::InvalidProtocol);
        }
        writer.next_u32(self.serial).await?;
        writer.next_u32(self.shape.0).await?;
        Ok(())
    }
}
