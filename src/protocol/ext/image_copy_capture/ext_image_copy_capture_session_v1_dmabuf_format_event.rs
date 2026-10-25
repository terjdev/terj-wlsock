use crate::protocol::ext::image_copy_capture::ObjectInteface as ImageCopyCaptureObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, CommandWriterExt, InterfaceReader,
    ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::FdWriter;
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Server event `ext_image_copy_capture_session_v1.dmabuf_format`.
///
/// Opcode 3; available since version 1.
/// Wire framing and receiver metadata are checked; protocol semantics belong to the caller.
/// ```xml
/// <event name="dmabuf_format">
///       <description summary="dma-buf format">
///         Provides the format that must be used for dma-buf buffers.
///
///         The client may choose any of the modifiers advertised in the array of
///         64-bit unsigned integers.
///
///         This event may be emitted multiple times, in which case the client may
///         choose any given format.
///       </description>
///       <arg name="format" type="uint" summary="drm format code" />
///       <arg name="modifiers" type="array" summary="drm format modifiers" />
///     </event>
/// ```
#[derive(Debug)]
pub struct ExtImageCopyCaptureSessionV1DmabufFormatEvent {
    /// Wire identifier of the receiver.
    pub object_id: u32,
    /// drm format code.
    pub format: u32,
    /// drm format modifiers.
    pub modifiers: Vec<u8>,
}
impl ExtImageCopyCaptureSessionV1DmabufFormatEvent {
    /// Opcode in the event namespace.
    pub const OPCODE: u16 = 3;
    /// Minimum supported interface version.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for ExtImageCopyCaptureSessionV1DmabufFormatEvent {
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
impl InterfaceReader for ExtImageCopyCaptureSessionV1DmabufFormatEvent {
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
                ImageCopyCaptureObjectInteface::ExtImageCopyCaptureSessionV1 { .. }
            )
        ) || opcode != Self::OPCODE
            || interface.version() < Self::SINCE
        {
            return Err(FdReaderError::InvalidProtocol);
        }
        let format = reader.next_u32().await?;
        let modifiers = reader.next_array().await?;
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        Ok(Self {
            object_id,
            format,
            modifiers,
        })
    }
}
impl CommandWriter for ExtImageCopyCaptureSessionV1DmabufFormatEvent {
    fn object_id(&self) -> u32 {
        self.object_id
    }
    fn opcode(&self) -> u16 {
        Self::OPCODE
    }
    fn payload_size(&self) -> usize {
        Self::total_payload_size(&[Some(4), FdWriter::encoded_array_size(self.modifiers.len())])
    }
    async fn write(
        self,
        writer: &mut FdPayloadWriter<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<(), FdWriterError> {
        Self::writer_event_interface(
            state,
            self.object_id,
            "ext_image_copy_capture_session_v1",
            Self::SINCE,
        )?;
        if self.payload_size() != writer.remaining_quota() {
            return Err(FdWriterError::InvalidProtocol);
        }
        writer.next_u32(self.format).await?;
        writer.next_array(&self.modifiers).await?;
        Ok(())
    }
}
