use crate::protocol::ext::image_copy_capture::ObjectInteface as ImageCopyCaptureObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, CommandWriterExt, InterfaceReader,
    ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Client request `ext_image_copy_capture_cursor_session_v1.get_capture_session`.
///
/// Opcode 1; available since version 1.
/// Wire framing and receiver metadata are checked; protocol semantics belong to the caller.
/// New IDs are registered before writes; close the connection after a failed or cancelled write.
/// ```xml
/// <request name="get_capture_session">
///       <description summary="get image copy capturer session">
///         Gets the image copy capture session for this cursor session.
///
///         The session will produce frames of the cursor image. The compositor may
///         pause the session when the cursor leaves the captured area.
///
///         This request must not be sent more than once, or else the
///         duplicate_session protocol error is raised.
///       </description>
///       <arg name="session" type="new_id" interface="ext_image_copy_capture_session_v1" />
///     </request>
/// ```
#[derive(Debug)]
pub struct ExtImageCopyCaptureCursorSessionV1GetCaptureSessionRequest {
    /// Wire identifier of the receiver.
    pub object_id: u32,
    /// Wire argument `session`.
    pub session: u32,
}
impl ExtImageCopyCaptureCursorSessionV1GetCaptureSessionRequest {
    /// Opcode in the request namespace.
    pub const OPCODE: u16 = 1;
    /// Minimum supported interface version.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for ExtImageCopyCaptureCursorSessionV1GetCaptureSessionRequest {
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
impl InterfaceReader for ExtImageCopyCaptureCursorSessionV1GetCaptureSessionRequest {
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
                ImageCopyCaptureObjectInteface::ExtImageCopyCaptureCursorSessionV1 { .. }
            )
        ) || opcode != Self::OPCODE
            || interface.version() < Self::SINCE
        {
            return Err(FdReaderError::InvalidProtocol);
        }
        let session = reader.next_u32().await?;
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        {
            let new_interface = ObjectInteface::from_name(
                "ext_image_copy_capture_session_v1",
                interface.version().min(1),
            )
            .ok_or(FdReaderError::InvalidProtocol)?;
            Self::reader_state(state)?
                .register_object(session, new_interface, false)
                .map_err(|_| FdReaderError::InvalidProtocol)?;
        }
        Ok(Self { object_id, session })
    }
}
impl CommandWriter for ExtImageCopyCaptureCursorSessionV1GetCaptureSessionRequest {
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
            "ext_image_copy_capture_cursor_session_v1",
            Self::SINCE,
        )?;
        if self.payload_size() != writer.remaining_quota() {
            return Err(FdWriterError::InvalidProtocol);
        }
        {
            let new_interface = ObjectInteface::from_name(
                "ext_image_copy_capture_session_v1",
                interface.version().min(1),
            )
            .ok_or(FdWriterError::InvalidProtocol)?;
            Self::writer_state(state)?
                .register_object(self.session, new_interface, false)
                .map_err(|_| FdWriterError::InvalidProtocol)?;
        }
        writer.next_u32(self.session).await?;
        Ok(())
    }
}
