use crate::protocol::ext::image_copy_capture::ObjectInteface as ImageCopyCaptureObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, CommandWriterExt, InterfaceReader,
    ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Client request `ext_image_copy_capture_frame_v1.damage_buffer`.
///
/// Opcode 2; available since version 1.
/// Wire framing and receiver metadata are checked; protocol semantics belong to the caller.
/// ```xml
/// <request name="damage_buffer">
///       <description summary="damage buffer">
///         Apply damage to the buffer which is to be captured next. This request
///         may be sent multiple times to describe a region.
///
///         The client indicates the accumulated damage since this wl_buffer was
///         last captured. During capture, the compositor will update the buffer
///         with at least the union of the region passed by the client and the
///         region advertised by ext_image_copy_capture_frame_v1.damage.
///
///         When a wl_buffer is captured for the first time, or when the client
///         doesn't track damage, the client must damage the whole buffer.
///
///         This is for optimisation purposes. The compositor may use this
///         information to reduce copying.
///
///         These coordinates originate from the upper left corner of the buffer.
///
///         If x or y are strictly negative, or if width or height are negative or
///         zero, the invalid_buffer_damage protocol error is raised.
///
///         This request must not be sent after capture, or else the
///         already_captured protocol error is raised.
///       </description>
///       <arg name="x" type="int" summary="region x coordinate" />
///       <arg name="y" type="int" summary="region y coordinate" />
///       <arg name="width" type="int" summary="region width" />
///       <arg name="height" type="int" summary="region height" />
///     </request>
/// ```
#[derive(Debug)]
pub struct ExtImageCopyCaptureFrameV1DamageBufferRequest {
    /// Wire identifier of the receiver.
    pub object_id: u32,
    /// region x coordinate.
    pub x: i32,
    /// region y coordinate.
    pub y: i32,
    /// region width.
    pub width: i32,
    /// region height.
    pub height: i32,
}
impl ExtImageCopyCaptureFrameV1DamageBufferRequest {
    /// Opcode in the request namespace.
    pub const OPCODE: u16 = 2;
    /// Minimum supported interface version.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for ExtImageCopyCaptureFrameV1DamageBufferRequest {
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
impl InterfaceReader for ExtImageCopyCaptureFrameV1DamageBufferRequest {
    async fn read_with_interface(
        object_id: u32,
        opcode: u16,
        interface: ObjectInteface,
        reader: &mut FdPayloadReader<'_>,
        _state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        if !matches!(
            interface,
            ObjectInteface::ImageCopyCapture(
                ImageCopyCaptureObjectInteface::ExtImageCopyCaptureFrameV1 { .. }
            )
        ) || opcode != Self::OPCODE
            || interface.version() < Self::SINCE
        {
            return Err(FdReaderError::InvalidProtocol);
        }
        let x = reader.next_i32().await?;
        let y = reader.next_i32().await?;
        let width = reader.next_i32().await?;
        let height = reader.next_i32().await?;
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        Ok(Self {
            object_id,
            x,
            y,
            width,
            height,
        })
    }
}
impl CommandWriter for ExtImageCopyCaptureFrameV1DamageBufferRequest {
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
            "ext_image_copy_capture_frame_v1",
            Self::SINCE,
        )?;
        if self.payload_size() != writer.remaining_quota() {
            return Err(FdWriterError::InvalidProtocol);
        }
        writer.next_i32(self.x).await?;
        writer.next_i32(self.y).await?;
        writer.next_i32(self.width).await?;
        writer.next_i32(self.height).await?;
        Ok(())
    }
}
