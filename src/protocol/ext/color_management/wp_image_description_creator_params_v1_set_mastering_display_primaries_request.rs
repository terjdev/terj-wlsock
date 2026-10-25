use crate::protocol::ext::color_management::ObjectInteface as ColorManagementObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, CommandWriterExt, InterfaceReader,
    ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Client request `wp_image_description_creator_params_v1.set_mastering_display_primaries`.
///
/// Opcode 6; available since version 1.
/// Wire framing and receiver metadata are checked; protocol semantics belong to the caller.
/// ```xml
/// <request name="set_mastering_display_primaries">
///       <description summary="mastering display primaries as chromaticity coordinates">
///         Provides the color primaries and white point of the mastering display
///         using CIE 1931 xy chromaticity coordinates. This is compatible with the
///         SMPTE ST 2086 definition of HDR static metadata.
///
///         The mastering display primaries and mastering display luminances define
///         the target color volume.
///
///         If mastering display primaries are not explicitly set, the target color
///         volume is assumed to have the same primaries as the primary color volume.
///
///         The target color volume is defined by all tristimulus values between 0.0
///         and 1.0 (inclusive) of the color space defined by the given mastering
///         display primaries and white point. The colorimetry is identical between
///         the container color space and the mastering display color space,
///         including that no chromatic adaptation is applied even if the white
///         points differ.
///
///         The target color volume can exceed the primary color volume to allow for
///         a greater color volume with an existing color space definition (for
///         example scRGB). It can be smaller than the primary color volume to
///         minimize gamut and tone mapping distances for big color spaces (HDR
///         metadata).
///
///         To make use of the entire target color volume a suitable pixel format
///         has to be chosen (e.g. floating point to exceed the primary color
///         volume, or abusing limited quantization range as with xvYCC).
///
///         Each coordinate value is multiplied by 1 million to get the argument
///         value to carry precision of 6 decimals.
///
///         If mastering display primaries have already been set on this object, the
///         protocol error already_set is raised.
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
///       <arg name="r_x" type="int" summary="Red x * 1M" />
///       <arg name="r_y" type="int" summary="Red y * 1M" />
///       <arg name="g_x" type="int" summary="Green x * 1M" />
///       <arg name="g_y" type="int" summary="Green y * 1M" />
///       <arg name="b_x" type="int" summary="Blue x * 1M" />
///       <arg name="b_y" type="int" summary="Blue y * 1M" />
///       <arg name="w_x" type="int" summary="White x * 1M" />
///       <arg name="w_y" type="int" summary="White y * 1M" />
///     </request>
/// ```
#[derive(Debug)]
pub struct WpImageDescriptionCreatorParamsV1SetMasteringDisplayPrimariesRequest {
    /// Wire identifier of the receiver.
    pub object_id: u32,
    /// Red x * 1M.
    pub r_x: i32,
    /// Red y * 1M.
    pub r_y: i32,
    /// Green x * 1M.
    pub g_x: i32,
    /// Green y * 1M.
    pub g_y: i32,
    /// Blue x * 1M.
    pub b_x: i32,
    /// Blue y * 1M.
    pub b_y: i32,
    /// White x * 1M.
    pub w_x: i32,
    /// White y * 1M.
    pub w_y: i32,
}
impl WpImageDescriptionCreatorParamsV1SetMasteringDisplayPrimariesRequest {
    /// Opcode in the request namespace.
    pub const OPCODE: u16 = 6;
    /// Minimum supported interface version.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for WpImageDescriptionCreatorParamsV1SetMasteringDisplayPrimariesRequest {
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
impl InterfaceReader for WpImageDescriptionCreatorParamsV1SetMasteringDisplayPrimariesRequest {
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
        let r_x = reader.next_i32().await?;
        let r_y = reader.next_i32().await?;
        let g_x = reader.next_i32().await?;
        let g_y = reader.next_i32().await?;
        let b_x = reader.next_i32().await?;
        let b_y = reader.next_i32().await?;
        let w_x = reader.next_i32().await?;
        let w_y = reader.next_i32().await?;
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        Ok(Self {
            object_id,
            r_x,
            r_y,
            g_x,
            g_y,
            b_x,
            b_y,
            w_x,
            w_y,
        })
    }
}
impl CommandWriter for WpImageDescriptionCreatorParamsV1SetMasteringDisplayPrimariesRequest {
    fn object_id(&self) -> u32 {
        self.object_id
    }
    fn opcode(&self) -> u16 {
        Self::OPCODE
    }
    fn payload_size(&self) -> usize {
        Self::total_payload_size(&[
            Some(4),
            Some(4),
            Some(4),
            Some(4),
            Some(4),
            Some(4),
            Some(4),
            Some(4),
        ])
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
        writer.next_i32(self.r_x).await?;
        writer.next_i32(self.r_y).await?;
        writer.next_i32(self.g_x).await?;
        writer.next_i32(self.g_y).await?;
        writer.next_i32(self.b_x).await?;
        writer.next_i32(self.b_y).await?;
        writer.next_i32(self.w_x).await?;
        writer.next_i32(self.w_y).await?;
        Ok(())
    }
}
