use crate::protocol::ext::color_management::ObjectInteface as ColorManagementObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, CommandWriterExt, InterfaceReader,
    ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Client request `wp_color_management_surface_feedback_v1.get_preferred`.
///
/// Opcode 1; available since version 1.
/// Wire framing and receiver metadata are checked; protocol semantics belong to the caller.
/// New IDs are registered before writes; close the connection after a failed or cancelled write.
/// ```xml
/// <request name="get_preferred">
///       <description summary="get the preferred image description">
///         If this protocol object is inert, the protocol error inert is raised.
///
///         The preferred image description represents the compositor's preferred
///         color encoding for this wl_surface at the current time. There might be
///         performance and power advantages, as well as improved color
///         reproduction, if the image description of a content update matches the
///         preferred image description.
///
///         This creates a new wp_image_description_v1 object for the currently
///         preferred image description for the wl_surface. The client should
///         stop using and destroy the image descriptions created by earlier
///         invocations of this request for the associated wl_surface.
///         This request is usually sent as a reaction to the preferred_changed
///         event or when creating a wp_color_management_surface_feedback_v1 object
///         if the client is capable of adapting to image descriptions.
///
///         The created wp_image_description_v1 object preserves the preferred image
///         description of the wl_surface from the time the object was created.
///
///         The resulting image description object allows get_information request.
///
///         If the image description is parametric, the client should set it on its
///         wl_surface only if the image description is an exact match with the
///         client content. Particularly if everything else matches, but the target
///         color volume is greater than what the client needs, the client should
///         create its own parametric image description with its exact parameters.
///
///         If the interface version is inadequate for the preferred image
///         description, meaning that the client does not support all the
///         events needed to deliver the crucial information, the resulting image
///         description object shall immediately deliver the
///         wp_image_description_v1.failed event with the low_version cause,
///         otherwise the object shall immediately deliver the ready event.
///       </description>
///
///       <arg name="image_description" type="new_id" interface="wp_image_description_v1" />
///     </request>
/// ```
#[derive(Debug)]
pub struct WpColorManagementSurfaceFeedbackV1GetPreferredRequest {
    /// Wire identifier of the receiver.
    pub object_id: u32,
    /// Wire argument `image_description`.
    pub image_description: u32,
}
impl WpColorManagementSurfaceFeedbackV1GetPreferredRequest {
    /// Opcode in the request namespace.
    pub const OPCODE: u16 = 1;
    /// Minimum supported interface version.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for WpColorManagementSurfaceFeedbackV1GetPreferredRequest {
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
impl InterfaceReader for WpColorManagementSurfaceFeedbackV1GetPreferredRequest {
    async fn read_with_interface(
        object_id: u32,
        opcode: u16,
        interface: ObjectInteface,
        reader: &mut FdPayloadReader<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        if !matches!(
            interface,
            ObjectInteface::ColorManagement(
                ColorManagementObjectInteface::WpColorManagementSurfaceFeedbackV1 { .. }
            )
        ) || opcode != Self::OPCODE
            || interface.version() < Self::SINCE
        {
            return Err(FdReaderError::InvalidProtocol);
        }
        let image_description = reader.next_u32().await?;
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        {
            let new_interface =
                ObjectInteface::from_name("wp_image_description_v1", interface.version().min(3))
                    .ok_or(FdReaderError::InvalidProtocol)?;
            Self::reader_state(state)?
                .register_object(image_description, new_interface, false)
                .map_err(|_| FdReaderError::InvalidProtocol)?;
        }
        Ok(Self {
            object_id,
            image_description,
        })
    }
}
impl CommandWriter for WpColorManagementSurfaceFeedbackV1GetPreferredRequest {
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
            "wp_color_management_surface_feedback_v1",
            Self::SINCE,
        )?;
        if self.payload_size() != writer.remaining_quota() {
            return Err(FdWriterError::InvalidProtocol);
        }
        {
            let new_interface =
                ObjectInteface::from_name("wp_image_description_v1", interface.version().min(3))
                    .ok_or(FdWriterError::InvalidProtocol)?;
            Self::writer_state(state)?
                .register_object(self.image_description, new_interface, false)
                .map_err(|_| FdWriterError::InvalidProtocol)?;
        }
        writer.next_u32(self.image_description).await?;
        Ok(())
    }
}
