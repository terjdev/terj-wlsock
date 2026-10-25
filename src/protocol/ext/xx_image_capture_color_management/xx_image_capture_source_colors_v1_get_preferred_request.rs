use super::ObjectInteface as XxImageCaptureColorManagementObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, CommandWriterExt, InterfaceReader,
    ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Client request `xx_image_capture_source_colors_v1.get_preferred`.
///
/// Framing, receiver protocol and version are checked; semantics belong to the caller.
/// New IDs are registered before writes; close the connection after a failed or cancelled write.
/// ```xml
/// <request name="get_preferred">
///       <description summary="retrieve the preferred description">
///         The preferred image description represents the compositor's preferred
///         color encoding for this capture source at the current time. There might
///         be performance and power advantages, as well as improved color
///         reproduction, if the image description of a capture frame matches the
///         preferred image description.
///
///         This creates a new wp_image_description_reference_v1 object for the
///         currently preferred image description for the capture source. The client
///         should stop using and destroy the image descriptions created by earlier
///         invocations of this request for the associated capture source. This
///         request is usually sent as a reaction to the
///         xx_image_capture_source_colors_v1.preferred_changed event or when
///         creating a xx_image_capture_source_colors_v1 object if the client is
///         capable of adapting to image descriptions.
///
///         The created wp_image_description_reference_v1 object preserves the
///         preferred image description of the capture source from the time the
///         object was created.
///
///         The resulting image description object allows the
///         wp_image_description_v1.get_information request.
///       </description>
///       <arg name="id" type="new_id" interface="wp_image_description_reference_v1" summary="the preferred description" />
///     </request>
/// ```
#[derive(Debug)]
pub struct XxImageCaptureSourceColorsV1GetPreferredRequest {
    /// Wire identifier of the receiver.
    pub object_id: u32,
    /// the preferred description.
    pub id: u32,
}
impl XxImageCaptureSourceColorsV1GetPreferredRequest {
    /// Opcode in the request namespace.
    pub const OPCODE: u16 = 1;
    /// Minimum interface version.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for XxImageCaptureSourceColorsV1GetPreferredRequest {
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
impl InterfaceReader for XxImageCaptureSourceColorsV1GetPreferredRequest {
    async fn read_with_interface(
        object_id: u32,
        opcode: u16,
        interface: ObjectInteface,
        reader: &mut FdPayloadReader<'_>,
        state: &Arc<Mutex<ClientState>>,
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
        let id = reader.next_u32().await?;
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        {
            let new_interface = ObjectInteface::from_name("wp_image_description_reference_v1", 1)
                .ok_or(FdReaderError::InvalidProtocol)?;
            Self::reader_state(state)?
                .register_object(id, new_interface, false)
                .map_err(|_| FdReaderError::InvalidProtocol)?;
        }
        Ok(Self { object_id, id })
    }
}
impl CommandWriter for XxImageCaptureSourceColorsV1GetPreferredRequest {
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
        {
            let new_interface = ObjectInteface::from_name("wp_image_description_reference_v1", 1)
                .ok_or(FdWriterError::InvalidProtocol)?;
            Self::writer_state(state)?
                .register_object(self.id, new_interface, false)
                .map_err(|_| FdWriterError::InvalidProtocol)?;
        }
        writer.next_u32(self.id).await?;
        Ok(())
    }
}
