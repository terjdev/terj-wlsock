use crate::protocol::ext::image_copy_capture::ObjectInteface as ImageCopyCaptureObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, CommandWriterExt, InterfaceReader,
    ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Client request `ext_image_copy_capture_session_v1.create_frame`.
///
/// Opcode 0; available since version 1.
/// Wire framing and receiver metadata are checked; protocol semantics belong to the caller.
/// New IDs are registered before writes; close the connection after a failed or cancelled write.
/// ```xml
/// <request name="create_frame">
///       <description summary="create a frame">
///         Create a capture frame for this session.
///
///         At most one frame object can exist for a given session at any time. If
///         a client sends a create_frame request before a previous frame object
///         has been destroyed, the duplicate_frame protocol error is raised.
///       </description>
///       <arg name="frame" type="new_id" interface="ext_image_copy_capture_frame_v1" />
///     </request>
/// ```
#[derive(Debug)]
pub struct ExtImageCopyCaptureSessionV1CreateFrameRequest {
    /// Wire identifier of the receiver.
    pub object_id: u32,
    /// Wire argument `frame`.
    pub frame: u32,
}
impl ExtImageCopyCaptureSessionV1CreateFrameRequest {
    /// Opcode in the request namespace.
    pub const OPCODE: u16 = 0;
    /// Minimum supported interface version.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for ExtImageCopyCaptureSessionV1CreateFrameRequest {
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
impl InterfaceReader for ExtImageCopyCaptureSessionV1CreateFrameRequest {
    async fn read_with_interface(
        object_id: u32,
        opcode: u16,
        interface: ObjectInteface,
        reader: &mut FdPayloadReader<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        if !matches!(
            interface,
            ObjectInteface::ImageCopyCapture(
                ImageCopyCaptureObjectInteface::ExtImageCopyCaptureSessionV1 { .. }
            )
        ) || opcode != Self::OPCODE
            || interface.version() < Self::SINCE
        {
            return Err(FdReaderError::InvalidProtocol);
        }
        let frame = reader.next_u32().await?;
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        {
            let new_interface = ObjectInteface::from_name(
                "ext_image_copy_capture_frame_v1",
                interface.version().min(1),
            )
            .ok_or(FdReaderError::InvalidProtocol)?;
            Self::reader_state(state)?
                .register_object(frame, new_interface, false)
                .map_err(|_| FdReaderError::InvalidProtocol)?;
        }
        Ok(Self { object_id, frame })
    }
}
impl CommandWriter for ExtImageCopyCaptureSessionV1CreateFrameRequest {
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
        let interface = Self::writer_request_interface(
            state,
            self.object_id,
            "ext_image_copy_capture_session_v1",
            Self::SINCE,
        )?;
        if self.payload_size() != writer.remaining_quota() {
            return Err(FdWriterError::InvalidProtocol);
        }
        {
            let new_interface = ObjectInteface::from_name(
                "ext_image_copy_capture_frame_v1",
                interface.version().min(1),
            )
            .ok_or(FdWriterError::InvalidProtocol)?;
            Self::writer_state(state)?
                .register_object(self.frame, new_interface, false)
                .map_err(|_| FdWriterError::InvalidProtocol)?;
        }
        writer.next_u32(self.frame).await?;
        Ok(())
    }
}
