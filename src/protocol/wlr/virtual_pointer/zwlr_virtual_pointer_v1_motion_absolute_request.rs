use crate::protocol::wlr::virtual_pointer::ObjectInteface as WlrVirtualPointerObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, CommandWriterExt, InterfaceReader,
    ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Client request `zwlr_virtual_pointer_v1.motion_absolute`.
///
/// Opcode 1; available since version 1.
/// Wire framing and receiver metadata are checked; protocol semantics belong to the caller.
/// ```xml
/// <request name="motion_absolute">
///       <description summary="pointer absolute motion event">
///         The pointer has moved in an absolute coordinate frame.
///
///         Value of x can range from 0 to x_extent, value of y can range from 0
///         to y_extent.
///       </description>
///       <arg name="time" type="uint" summary="timestamp with millisecond granularity" />
///       <arg name="x" type="uint" summary="position on the x-axis" />
///       <arg name="y" type="uint" summary="position on the y-axis" />
///       <arg name="x_extent" type="uint" summary="extent of the x-axis" />
///       <arg name="y_extent" type="uint" summary="extent of the y-axis" />
///     </request>
/// ```
#[derive(Debug)]
pub struct ZwlrVirtualPointerV1MotionAbsoluteRequest {
    /// Wire identifier of the receiver.
    pub object_id: u32,
    /// timestamp with millisecond granularity.
    pub time: u32,
    /// position on the x-axis.
    pub x: u32,
    /// position on the y-axis.
    pub y: u32,
    /// extent of the x-axis.
    pub x_extent: u32,
    /// extent of the y-axis.
    pub y_extent: u32,
}
impl ZwlrVirtualPointerV1MotionAbsoluteRequest {
    /// Opcode in the request namespace.
    pub const OPCODE: u16 = 1;
    /// Minimum supported interface version.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for ZwlrVirtualPointerV1MotionAbsoluteRequest {
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
impl InterfaceReader for ZwlrVirtualPointerV1MotionAbsoluteRequest {
    async fn read_with_interface(
        object_id: u32,
        opcode: u16,
        interface: ObjectInteface,
        reader: &mut FdPayloadReader<'_>,
        _state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        if !matches!(
            interface,
            ObjectInteface::WlrVirtualPointer(
                WlrVirtualPointerObjectInteface::ZwlrVirtualPointerV1 { .. }
            )
        ) || opcode != Self::OPCODE
            || interface.version() < Self::SINCE
        {
            return Err(FdReaderError::InvalidProtocol);
        }
        let time = reader.next_u32().await?;
        let x = reader.next_u32().await?;
        let y = reader.next_u32().await?;
        let x_extent = reader.next_u32().await?;
        let y_extent = reader.next_u32().await?;
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        Ok(Self {
            object_id,
            time,
            x,
            y,
            x_extent,
            y_extent,
        })
    }
}
impl CommandWriter for ZwlrVirtualPointerV1MotionAbsoluteRequest {
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
        Self::writer_request_interface(
            state,
            self.object_id,
            "zwlr_virtual_pointer_v1",
            Self::SINCE,
        )?;
        if self.payload_size() != writer.remaining_quota() {
            return Err(FdWriterError::InvalidProtocol);
        }
        writer.next_u32(self.time).await?;
        writer.next_u32(self.x).await?;
        writer.next_u32(self.y).await?;
        writer.next_u32(self.x_extent).await?;
        writer.next_u32(self.y_extent).await?;
        Ok(())
    }
}
