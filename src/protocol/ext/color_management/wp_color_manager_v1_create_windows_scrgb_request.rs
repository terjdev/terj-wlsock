use crate::protocol::ext::color_management::ObjectInteface as ColorManagementObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, CommandWriterExt, InterfaceReader,
    ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Client request `wp_color_manager_v1.create_windows_scrgb`.
///
/// Opcode 6; available since version 1.
/// Wire framing and receiver metadata are checked; protocol semantics belong to the caller.
/// New IDs are registered before writes; close the connection after a failed or cancelled write.
/// ```xml
/// <request name="create_windows_scrgb">
///       <description summary="create Windows-scRGB image description object">
///         This creates a pre-defined image description for the so-called
///         Windows-scRGB stimulus encoding. This comes from the Windows 10 handling
///         of its own definition of an scRGB color space for an HDR screen
///         driven in BT.2100/PQ signalling mode.
///
///         Windows-scRGB uses sRGB (BT.709) color primaries and white point.
///         The transfer characteristic is extended linear.
///
///         The nominal color channel value range is extended, meaning it includes
///         negative and greater than 1.0 values. Negative values are used to
///         escape the sRGB color gamut boundaries. To make use of the extended
///         range, the client needs to use a pixel format that can represent those
///         values, e.g. floating-point 16 bits per channel.
///
///         Nominal color value R=G=B=0.0 corresponds to BT.2100/PQ system
///         0 cd/m², and R=G=B=1.0 corresponds to BT.2100/PQ system 80 cd/m².
///         The maximum is R=G=B=125.0 corresponding to 10k cd/m².
///
///         Windows-scRGB is displayed by Windows 10 by converting it to
///         BT.2100/PQ, maintaining the CIE 1931 chromaticity and mapping the
///         luminance as above. No adjustment is made to the signal to account
///         for the viewing conditions.
///
///         The reference white level of Windows-scRGB is unknown. If a
///         reference white level must be assumed for compositor processing, it
///         should be R=G=B=2.5375 corresponding to 203 cd/m² of Report ITU-R
///         BT.2408-7.
///
///         The target color volume of Windows-scRGB is unknown. The color gamut
///         may be anything between sRGB and BT.2100.
///
///         Note: EGL_EXT_gl_colorspace_scrgb_linear definition differs from
///         Windows-scRGB by using R=G=B=1.0 as the reference white level, while
///         Windows-scRGB reference white level is unknown or varies. However,
///         it seems probable that Windows implements both
///         EGL_EXT_gl_colorspace_scrgb_linear and Vulkan
///         VK_COLOR_SPACE_EXTENDED_SRGB_LINEAR_EXT as Windows-scRGB.
///
///         This request can be used when the compositor advertises
///         wp_color_manager_v1.feature.windows_scrgb.
///         Otherwise this request raises the protocol error unsupported_feature.
///
///         The resulting image description object does not allow get_information
///         request. The wp_image_description_v1.ready event shall be sent.
///       </description>
///
///       <arg name="image_description" type="new_id" interface="wp_image_description_v1" />
///     </request>
/// ```
#[derive(Debug)]
pub struct WpColorManagerV1CreateWindowsScrgbRequest {
    /// Wire identifier of the receiver.
    pub object_id: u32,
    /// Wire argument `image_description`.
    pub image_description: u32,
}
impl WpColorManagerV1CreateWindowsScrgbRequest {
    /// Opcode in the request namespace.
    pub const OPCODE: u16 = 6;
    /// Minimum supported interface version.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for WpColorManagerV1CreateWindowsScrgbRequest {
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
impl InterfaceReader for WpColorManagerV1CreateWindowsScrgbRequest {
    async fn read_with_interface(
        object_id: u32,
        opcode: u16,
        interface: ObjectInteface,
        reader: &mut FdPayloadReader<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        if !matches!(
            interface,
            ObjectInteface::ColorManagement(ColorManagementObjectInteface::WpColorManagerV1 { .. })
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
impl CommandWriter for WpColorManagerV1CreateWindowsScrgbRequest {
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
            "wp_color_manager_v1",
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
