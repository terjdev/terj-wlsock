use crate::protocol::ext::image_copy_capture::ObjectInteface as ImageCopyCaptureObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, CommandWriterExt, InterfaceReader,
    ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Server event `ext_image_copy_capture_cursor_session_v1.hotspot`.
///
/// Opcode 3; available since version 1.
/// Wire framing and receiver metadata are checked; protocol semantics belong to the caller.
/// ```xml
/// <event name="hotspot">
///       <description summary="hotspot changed">
///         The hotspot describes the offset between the cursor image and the
///         position of the input device.
///
///         The given coordinates are the hotspot's offset from the origin in
///         buffer coordinates.
///
///         Clients should not apply the hotspot immediately: the hotspot becomes
///         effective when the next ext_image_copy_capture_frame_v1.ready event is received.
///
///         Compositors may delay this event until the client captures a new frame.
///       </description>
///       <arg name="x" type="int" summary="hotspot x coordinates" />
///       <arg name="y" type="int" summary="hotspot y coordinates" />
///     </event>
/// ```
#[derive(Debug)]
pub struct ExtImageCopyCaptureCursorSessionV1HotspotEvent {
    /// Wire identifier of the receiver.
    pub object_id: u32,
    /// hotspot x coordinates.
    pub x: i32,
    /// hotspot y coordinates.
    pub y: i32,
}
impl ExtImageCopyCaptureCursorSessionV1HotspotEvent {
    /// Opcode in the event namespace.
    pub const OPCODE: u16 = 3;
    /// Minimum supported interface version.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for ExtImageCopyCaptureCursorSessionV1HotspotEvent {
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
impl InterfaceReader for ExtImageCopyCaptureCursorSessionV1HotspotEvent {
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
                ImageCopyCaptureObjectInteface::ExtImageCopyCaptureCursorSessionV1 { .. }
            )
        ) || opcode != Self::OPCODE
            || interface.version() < Self::SINCE
        {
            return Err(FdReaderError::InvalidProtocol);
        }
        let x = reader.next_i32().await?;
        let y = reader.next_i32().await?;
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        Ok(Self { object_id, x, y })
    }
}
impl CommandWriter for ExtImageCopyCaptureCursorSessionV1HotspotEvent {
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
        Self::writer_event_interface(
            state,
            self.object_id,
            "ext_image_copy_capture_cursor_session_v1",
            Self::SINCE,
        )?;
        if self.payload_size() != writer.remaining_quota() {
            return Err(FdWriterError::InvalidProtocol);
        }
        writer.next_i32(self.x).await?;
        writer.next_i32(self.y).await?;
        Ok(())
    }
}
