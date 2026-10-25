use super::ObjectInteface as XxImageCaptureColorManagementObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, CommandWriterExt, InterfaceReader,
    ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Client request `xx_image_capture_color_manager_v1.set_frame_image_description`.
///
/// Framing, receiver protocol and version are checked; semantics belong to the caller.
/// ```xml
/// <request name="set_frame_image_description">
///       <description summary="set the image description of a frame">
///         If the frame is successfully captured, it will be encoded according to
///         this image description.
///
///         This request must not be used if the image description is not ready.
///         Otherwise the image_description_not_ready error is emitted.
///
///         This request must not be used if the frame has already been captured.
///         Otherwise the already_captured error is emitted.
///       </description>
///       <arg name="frame" type="object" interface="ext_image_copy_capture_frame_v1" />
///       <arg name="image_description" type="object" interface="wp_image_description_v1" />
///     </request>
/// ```
#[derive(Debug)]
pub struct XxImageCaptureColorManagerV1SetFrameImageDescriptionRequest {
    /// Wire identifier of the receiver.
    pub object_id: u32,
    /// Wire argument `frame`.
    pub frame: u32,
    /// Wire argument `image_description`.
    pub image_description: u32,
}
impl XxImageCaptureColorManagerV1SetFrameImageDescriptionRequest {
    /// Opcode in the request namespace.
    pub const OPCODE: u16 = 2;
    /// Minimum interface version.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for XxImageCaptureColorManagerV1SetFrameImageDescriptionRequest {
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
impl InterfaceReader for XxImageCaptureColorManagerV1SetFrameImageDescriptionRequest {
    async fn read_with_interface(
        object_id: u32,
        opcode: u16,
        interface: ObjectInteface,
        reader: &mut FdPayloadReader<'_>,
        _state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        if !matches!(
            interface,
            ObjectInteface::XxImageCaptureColorManagement(
                XxImageCaptureColorManagementObjectInteface::XxImageCaptureColorManagerV1 { .. }
            )
        ) || opcode != Self::OPCODE
            || interface.version() < Self::SINCE
        {
            return Err(FdReaderError::InvalidProtocol);
        }
        let frame = reader.next_u32().await?;
        let image_description = reader.next_u32().await?;
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        Ok(Self {
            object_id,
            frame,
            image_description,
        })
    }
}
impl CommandWriter for XxImageCaptureColorManagerV1SetFrameImageDescriptionRequest {
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
        let interface = Self::writer_request_interface(
            state,
            self.object_id,
            "xx_image_capture_color_manager_v1",
            Self::SINCE,
        )?;
        if !matches!(
            interface,
            ObjectInteface::XxImageCaptureColorManagement(
                XxImageCaptureColorManagementObjectInteface::XxImageCaptureColorManagerV1 { .. }
            )
        ) || self.payload_size() != writer.remaining_quota()
        {
            return Err(FdWriterError::InvalidProtocol);
        }
        writer.next_u32(self.frame).await?;
        writer.next_u32(self.image_description).await?;
        Ok(())
    }
}
