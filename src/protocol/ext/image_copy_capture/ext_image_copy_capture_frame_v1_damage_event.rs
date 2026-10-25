use crate::protocol::ext::image_copy_capture::ObjectInteface as ImageCopyCaptureObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, CommandWriterExt, InterfaceReader,
    ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Server event `ext_image_copy_capture_frame_v1.damage`.
///
/// Opcode 1; available since version 1.
/// Wire framing and receiver metadata are checked; protocol semantics belong to the caller.
/// ```xml
/// <event name="damage">
///       <description summary="buffer damaged region">
///         This event is sent before the ready event. It may be generated multiple
///         times to describe a region.
///
///         The first captured frame in a session will always carry full damage.
///         Subsequent frames' damaged regions describe which parts of the buffer
///         have changed since the last ready event.
///
///         These coordinates originate in the upper left corner of the buffer.
///       </description>
///       <arg name="x" type="int" summary="damage x coordinate" />
///       <arg name="y" type="int" summary="damage y coordinate" />
///       <arg name="width" type="int" summary="damage width" />
///       <arg name="height" type="int" summary="damage height" />
///     </event>
/// ```
#[derive(Debug)]
pub struct ExtImageCopyCaptureFrameV1DamageEvent {
    /// Wire identifier of the receiver.
    pub object_id: u32,
    /// damage x coordinate.
    pub x: i32,
    /// damage y coordinate.
    pub y: i32,
    /// damage width.
    pub width: i32,
    /// damage height.
    pub height: i32,
}
impl ExtImageCopyCaptureFrameV1DamageEvent {
    /// Opcode in the event namespace.
    pub const OPCODE: u16 = 1;
    /// Minimum supported interface version.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for ExtImageCopyCaptureFrameV1DamageEvent {
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
impl InterfaceReader for ExtImageCopyCaptureFrameV1DamageEvent {
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
impl CommandWriter for ExtImageCopyCaptureFrameV1DamageEvent {
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
        Self::writer_event_interface(
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
