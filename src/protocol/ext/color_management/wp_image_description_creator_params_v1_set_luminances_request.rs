use crate::protocol::ext::color_management::ObjectInteface as ColorManagementObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, CommandWriterExt, InterfaceReader,
    ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Client request `wp_image_description_creator_params_v1.set_luminances`.
///
/// Opcode 5; available since version 1.
/// Wire framing and receiver metadata are checked; protocol semantics belong to the caller.
/// ```xml
/// <request name="set_luminances">
///       <description summary="primary color volume luminance range and reference white">
///         Sets the primary color volume luminance range and the reference white
///         luminance level. These values include the minimum display emission, but
///         not external flare. The minimum display emission is assumed to have
///         the chromaticity of the primary color volume white point.
///
///         The default luminances from
///         https://www.color.org/chardata/rgb/srgb.xalter are
///         - primary color volume minimum: 0.2 cd/m²
///         - primary color volume maximum: 80 cd/m²
///         - reference white: 80 cd/m²
///
///         Setting a named transfer characteristic can imply other default
///         luminances.
///
///         The default luminances get overwritten when this request is used.
///         With transfer_function.st2084_pq the given 'max_lum' value is ignored,
///         and 'max_lum' is taken as 'min_lum' + 10000 cd/m².
///
///         'min_lum' and 'max_lum' specify the minimum and maximum luminances of
///         the primary color volume as reproduced by the targeted display.
///
///         'reference_lum' specifies the luminance of the reference white as
///         reproduced by the targeted display, and reflects the targeted viewing
///         environment.
///
///         Compositors should make sure that all content is anchored, meaning that
///         an input signal level of 'reference_lum' on one image description and
///         another input signal level of 'reference_lum' on another image
///         description should produce the same output level, even though the
///         'reference_lum' on both image representations can be different.
///
///         'reference_lum' may be higher than 'max_lum'. In that case reaching
///         the reference white output level in image content requires the
///         'extended_target_volume' feature support.
///
///         If 'max_lum' or 'reference_lum' are less than or equal to 'min_lum',
///         the protocol error invalid_luminance is raised.
///
///         The minimum luminance is multiplied by 10000 to get the argument
///         'min_lum' value and carries precision of 4 decimals. The maximum
///         luminance and reference white luminance values are unscaled.
///
///         If the primary color volume luminance range and the reference white
///         luminance level have already been set on this object, the protocol error
///         already_set is raised.
///
///         This request can be used if the compositor advertises
///         wp_color_manager_v1.feature.set_luminances. Otherwise this request
///         raises the protocol error unsupported_feature.
///       </description>
///
///       <arg name="min_lum" type="uint" summary="minimum luminance (cd/m²) * 10000" />
///       <arg name="max_lum" type="uint" summary="maximum luminance (cd/m²)" />
///       <arg name="reference_lum" type="uint" summary="reference white luminance (cd/m²)" />
///     </request>
/// ```
#[derive(Debug)]
pub struct WpImageDescriptionCreatorParamsV1SetLuminancesRequest {
    /// Wire identifier of the receiver.
    pub object_id: u32,
    /// minimum luminance (cd/m²) * 10000.
    pub min_lum: u32,
    /// maximum luminance (cd/m²).
    pub max_lum: u32,
    /// reference white luminance (cd/m²).
    pub reference_lum: u32,
}
impl WpImageDescriptionCreatorParamsV1SetLuminancesRequest {
    /// Opcode in the request namespace.
    pub const OPCODE: u16 = 5;
    /// Minimum supported interface version.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for WpImageDescriptionCreatorParamsV1SetLuminancesRequest {
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
impl InterfaceReader for WpImageDescriptionCreatorParamsV1SetLuminancesRequest {
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
                ColorManagementObjectInteface::WpImageDescriptionCreatorParamsV1 { .. }
            )
        ) || opcode != Self::OPCODE
            || interface.version() < Self::SINCE
        {
            return Err(FdReaderError::InvalidProtocol);
        }
        let min_lum = reader.next_u32().await?;
        let max_lum = reader.next_u32().await?;
        let reference_lum = reader.next_u32().await?;
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        Ok(Self {
            object_id,
            min_lum,
            max_lum,
            reference_lum,
        })
    }
}
impl CommandWriter for WpImageDescriptionCreatorParamsV1SetLuminancesRequest {
    fn object_id(&self) -> u32 {
        self.object_id
    }
    fn opcode(&self) -> u16 {
        Self::OPCODE
    }
    fn payload_size(&self) -> usize {
        Self::total_payload_size(&[Some(4), Some(4), Some(4)])
    }
    async fn write(
        self,
        writer: &mut FdPayloadWriter<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<(), FdWriterError> {
        Self::writer_request_interface(
            state,
            self.object_id,
            "wp_image_description_creator_params_v1",
            Self::SINCE,
        )?;
        if self.payload_size() != writer.remaining_quota() {
            return Err(FdWriterError::InvalidProtocol);
        }
        writer.next_u32(self.min_lum).await?;
        writer.next_u32(self.max_lum).await?;
        writer.next_u32(self.reference_lum).await?;
        Ok(())
    }
}
