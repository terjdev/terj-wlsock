use crate::protocol::ext::color_management::ObjectInteface as ColorManagementObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, CommandWriterExt, InterfaceReader,
    ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Server event `wp_image_description_info_v1.target_luminance`.
///
/// Opcode 8; available since version 1.
/// Wire framing and receiver metadata are checked; protocol semantics belong to the caller.
/// ```xml
/// <event name="target_luminance">
///       <description summary="target luminance range">
///         Provides the luminance range that the image description is targeting as
///         the minimum and maximum absolute luminance L. These values include the
///         minimum display emission and ambient flare luminances, assumed to be
///         optically additive and have the chromaticity of the primary color
///         volume white point. This should be compatible with the SMPTE ST 2086
///         definition of HDR static metadata.
///
///         This luminance range is only theoretical and may not correspond to the
///         luminance of light emitted on an actual display.
///
///         Min L value is multiplied by 10000 to get the argument min_lum value and
///         carry precision of 4 decimals. Max L value is unscaled for max_lum.
///       </description>
///
///       <arg name="min_lum" type="uint" summary="min L (cd/m²) * 10000" />
///       <arg name="max_lum" type="uint" summary="max L (cd/m²)" />
///     </event>
/// ```
#[derive(Debug)]
pub struct WpImageDescriptionInfoV1TargetLuminanceEvent {
    /// Wire identifier of the receiver.
    pub object_id: u32,
    /// min L (cd/m²) * 10000.
    pub min_lum: u32,
    /// max L (cd/m²).
    pub max_lum: u32,
}
impl WpImageDescriptionInfoV1TargetLuminanceEvent {
    /// Opcode in the event namespace.
    pub const OPCODE: u16 = 8;
    /// Minimum supported interface version.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for WpImageDescriptionInfoV1TargetLuminanceEvent {
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
impl InterfaceReader for WpImageDescriptionInfoV1TargetLuminanceEvent {
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
                ColorManagementObjectInteface::WpImageDescriptionInfoV1 { .. }
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
impl CommandWriter for WpImageDescriptionInfoV1TargetLuminanceEvent {
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
        Self::writer_event_interface(
            state,
            self.object_id,
            "wp_image_description_info_v1",
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
