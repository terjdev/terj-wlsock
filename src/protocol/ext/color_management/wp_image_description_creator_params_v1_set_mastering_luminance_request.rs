use crate::protocol::ext::color_management::ObjectInteface as ColorManagementObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, CommandWriterExt, InterfaceReader,
    ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Client request `wp_image_description_creator_params_v1.set_mastering_luminance`.
///
/// Opcode 7; available since version 1.
/// Wire framing and receiver metadata are checked; protocol semantics belong to the caller.
/// ```xml
/// <request name="set_mastering_luminance">
///       <description summary="display mastering luminance range">
///         Sets the luminance range that was used during the content mastering
///         process as the minimum and maximum absolute luminance L. These values
///         include the minimum display emission and ambient flare luminances,
///         assumed to be optically additive and have the chromaticity of the
///         primary color volume white point. This should be
///         compatible with the SMPTE ST 2086 definition of HDR static metadata.
///
///         The mastering display primaries and mastering display luminances define
///         the target color volume.
///
///         If mastering luminances are not explicitly set, the target color volume
///         is assumed to have the same min and max luminances as the primary color
///         volume.
///
///         If max L is less than or equal to min L, the protocol error
///         invalid_luminance is raised.
///
///         Min L value is multiplied by 10000 to get the argument min_lum value
///         and carry precision of 4 decimals. Max L value is unscaled for max_lum.
///
///         This request can be used if the compositor advertises
///         wp_color_manager_v1.feature.set_mastering_display_primaries. Otherwise
///         this request raises the protocol error unsupported_feature. The
///         advertisement implies support only for target color volumes fully
///         contained within the primary color volume.
///
///         If a compositor additionally supports target color volume exceeding the
///         primary color volume, it must advertise
///         wp_color_manager_v1.feature.extended_target_volume. If a client uses
///         target color volume exceeding the primary color volume and the
///         compositor does not support it, the result is implementation defined.
///         Compositors are recommended to detect this case and fail the image
///         description gracefully, but it may as well result in color artifacts.
///       </description>
///
///       <arg name="min_lum" type="uint" summary="min L (cd/m²) * 10000" />
///       <arg name="max_lum" type="uint" summary="max L (cd/m²)" />
///     </request>
/// ```
#[derive(Debug)]
pub struct WpImageDescriptionCreatorParamsV1SetMasteringLuminanceRequest {
    /// Wire identifier of the receiver.
    pub object_id: u32,
    /// min L (cd/m²) * 10000.
    pub min_lum: u32,
    /// max L (cd/m²).
    pub max_lum: u32,
}
impl WpImageDescriptionCreatorParamsV1SetMasteringLuminanceRequest {
    /// Opcode in the request namespace.
    pub const OPCODE: u16 = 7;
    /// Minimum supported interface version.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for WpImageDescriptionCreatorParamsV1SetMasteringLuminanceRequest {
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
impl InterfaceReader for WpImageDescriptionCreatorParamsV1SetMasteringLuminanceRequest {
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
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        Ok(Self {
            object_id,
            min_lum,
            max_lum,
        })
    }
}
impl CommandWriter for WpImageDescriptionCreatorParamsV1SetMasteringLuminanceRequest {
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
            "wp_image_description_creator_params_v1",
            Self::SINCE,
        )?;
        if self.payload_size() != writer.remaining_quota() {
            return Err(FdWriterError::InvalidProtocol);
        }
        writer.next_u32(self.min_lum).await?;
        writer.next_u32(self.max_lum).await?;
        Ok(())
    }
}
