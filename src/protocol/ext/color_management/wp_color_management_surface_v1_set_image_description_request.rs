use super::WpColorManagerV1RenderIntent;
use crate::protocol::ext::color_management::ObjectInteface as ColorManagementObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, CommandWriterExt, InterfaceReader,
    ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Client request `wp_color_management_surface_v1.set_image_description`.
///
/// Opcode 1; available since version 1.
/// Wire framing and receiver metadata are checked; protocol semantics belong to the caller.
/// ```xml
/// <request name="set_image_description">
///       <description summary="set the surface image description">
///         If this protocol object is inert, the protocol error inert is raised.
///
///         Set the image description of the underlying surface. The image
///         description and rendering intent are double-buffered state, see
///         wl_surface.commit.
///
///         It is the client's responsibility to understand the image description
///         it sets on a surface, and to provide content that matches that image
///         description. Compositors might convert images to match their own or any
///         other image descriptions.
///
///         Image descriptions which are not ready (see wp_image_description_v1)
///         are forbidden in this request, and in such case the protocol error
///         image_description is raised.
///
///         All image descriptions which are ready (see wp_image_description_v1)
///         are allowed and must always be accepted by the compositor.
///
///         When an image description is set on a surface, it establishes an
///         explicit link between surface pixel values and surface colorimetry.
///         This link may be undefined for some pixel values, see the image
///         description creator interfaces for the conditions. Non-finite
///         floating-point values (NaN, Inf) always have an undefined colorimetry.
///
///         A rendering intent provides the client's preference on how surface
///         colorimetry should be mapped to each output. The render_intent value
///         must be one advertised by the compositor with
///         wp_color_manager_v1.render_intent event, otherwise the protocol error
///         render_intent is raised.
///
///         By default, a surface does not have an associated image description
///         nor a rendering intent. The handling of color on such surfaces is
///         compositor implementation defined. Compositors should handle such
///         surfaces as sRGB, but may handle them differently if they have specific
///         requirements.
///
///         Setting the image description has copy semantics; after this request,
///         the image description can be immediately destroyed without affecting
///         the pending state of the surface.
///       </description>
///
///       <arg name="image_description" type="object" interface="wp_image_description_v1" />
///       <arg name="render_intent" type="uint" enum="wp_color_manager_v1.render_intent" summary="rendering intent" />
///     </request>
/// ```
#[derive(Debug)]
pub struct WpColorManagementSurfaceV1SetImageDescriptionRequest {
    /// Wire identifier of the receiver.
    pub object_id: u32,
    /// Wire argument `image_description`.
    pub image_description: u32,
    /// rendering intent.
    pub render_intent: WpColorManagerV1RenderIntent,
}
impl WpColorManagementSurfaceV1SetImageDescriptionRequest {
    /// Opcode in the request namespace.
    pub const OPCODE: u16 = 1;
    /// Minimum supported interface version.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for WpColorManagementSurfaceV1SetImageDescriptionRequest {
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
impl InterfaceReader for WpColorManagementSurfaceV1SetImageDescriptionRequest {
    async fn read_with_interface(
        object_id: u32,
        opcode: u16,
        interface: ObjectInteface,
        reader: &mut FdPayloadReader<'_>,
        _state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        if !matches!(
            interface,
            ObjectInteface::ColorManagement(
                ColorManagementObjectInteface::WpColorManagementSurfaceV1 { .. }
            )
        ) || opcode != Self::OPCODE
            || interface.version() < Self::SINCE
        {
            return Err(FdReaderError::InvalidProtocol);
        }
        let image_description = reader.next_u32().await?;
        let render_intent = WpColorManagerV1RenderIntent(reader.next_u32().await?);
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        Ok(Self {
            object_id,
            image_description,
            render_intent,
        })
    }
}
impl CommandWriter for WpColorManagementSurfaceV1SetImageDescriptionRequest {
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
        Self::writer_request_interface(
            state,
            self.object_id,
            "wp_color_management_surface_v1",
            Self::SINCE,
        )?;
        if self.payload_size() != writer.remaining_quota() {
            return Err(FdWriterError::InvalidProtocol);
        }
        writer.next_u32(self.image_description).await?;
        writer.next_u32(self.render_intent.0).await?;
        Ok(())
    }
}
