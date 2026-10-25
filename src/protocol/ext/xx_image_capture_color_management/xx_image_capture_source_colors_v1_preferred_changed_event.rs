use super::ObjectInteface as XxImageCaptureColorManagementObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, CommandWriterExt, InterfaceReader,
    ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Server event `xx_image_capture_source_colors_v1.preferred_changed`.
///
/// Framing, receiver protocol and version are checked; semantics belong to the caller.
/// ```xml
/// <event name="preferred_changed">
///       <description summary="the preferred image description changed">
///         The preferred image description is the one which likely has the most
///         performance and/or quality benefits for the compositor if used by the
///         client for its capture frames. This event is sent whenever
///         the compositor changes the capture source's preferred image description.
///
///         This event sends the identity of the new preferred state as the
///         argument, so clients who are aware of the image description already can
///         reuse it. Otherwise, if the client wants to know what the preferred
///         image description is, it shall use the
///         xx_image_capture_source_colors_v1.get_preferred request.
///
///         The preferred image description is not automatically used for anything.
///         It is only a hint, and clients may set any valid image description with
///         xx_image_capture_color_manager_v1.set_frame_image_description, but there
///         might be performance and color accuracy improvements by providing the
///         capture frame's contents in the preferred image description. Therefore
///         clients that can, should capture frames in the preferred image
///         description.
///       </description>
///       <arg name="identity_hi" type="uint" summary="image description id number (high 32 bits)" />
///       <arg name="identity_lo" type="uint" summary="image description id number (low 32 bits)" />
///     </event>
/// ```
#[derive(Debug)]
pub struct XxImageCaptureSourceColorsV1PreferredChangedEvent {
    /// Wire identifier of the receiver.
    pub object_id: u32,
    /// image description id number (high 32 bits).
    pub identity_hi: u32,
    /// image description id number (low 32 bits).
    pub identity_lo: u32,
}
impl XxImageCaptureSourceColorsV1PreferredChangedEvent {
    /// Opcode in the event namespace.
    pub const OPCODE: u16 = 0;
    /// Minimum interface version.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for XxImageCaptureSourceColorsV1PreferredChangedEvent {
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
impl InterfaceReader for XxImageCaptureSourceColorsV1PreferredChangedEvent {
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
                XxImageCaptureColorManagementObjectInteface::XxImageCaptureSourceColorsV1 { .. }
            )
        ) || opcode != Self::OPCODE
            || interface.version() < Self::SINCE
        {
            return Err(FdReaderError::InvalidProtocol);
        }
        let identity_hi = reader.next_u32().await?;
        let identity_lo = reader.next_u32().await?;
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        Ok(Self {
            object_id,
            identity_hi,
            identity_lo,
        })
    }
}
impl CommandWriter for XxImageCaptureSourceColorsV1PreferredChangedEvent {
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
        let interface = Self::writer_event_interface(
            state,
            self.object_id,
            "xx_image_capture_source_colors_v1",
            Self::SINCE,
        )?;
        if !matches!(
            interface,
            ObjectInteface::XxImageCaptureColorManagement(
                XxImageCaptureColorManagementObjectInteface::XxImageCaptureSourceColorsV1 { .. }
            )
        ) || self.payload_size() != writer.remaining_quota()
        {
            return Err(FdWriterError::InvalidProtocol);
        }
        writer.next_u32(self.identity_hi).await?;
        writer.next_u32(self.identity_lo).await?;
        Ok(())
    }
}
