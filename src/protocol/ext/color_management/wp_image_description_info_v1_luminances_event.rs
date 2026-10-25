use crate::protocol::ext::color_management::ObjectInteface as ColorManagementObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, CommandWriterExt, InterfaceReader,
    ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Server event `wp_image_description_info_v1.luminances`.
///
/// Opcode 6; available since version 1.
/// Wire framing and receiver metadata are checked; protocol semantics belong to the caller.
/// ```xml
/// <event name="luminances">
///       <description summary="primary color volume luminance range and reference white">
///         Delivers the primary color volume luminance range and the reference
///         white luminance level. These values include the minimum display emission
///         and ambient flare luminances, assumed to be optically additive and have
///         the chromaticity of the primary color volume white point.
///
///         The minimum luminance is multiplied by 10000 to get the argument
///         'min_lum' value and carries precision of 4 decimals. The maximum
///         luminance and reference white luminance values are unscaled.
///       </description>
///
///       <arg name="min_lum" type="uint" summary="minimum luminance (cd/m²) * 10000" />
///       <arg name="max_lum" type="uint" summary="maximum luminance (cd/m²)" />
///       <arg name="reference_lum" type="uint" summary="reference white luminance (cd/m²)" />
///     </event>
/// ```
#[derive(Debug)]
pub struct WpImageDescriptionInfoV1LuminancesEvent {
    /// Wire identifier of the receiver.
    pub object_id: u32,
    /// minimum luminance (cd/m²) * 10000.
    pub min_lum: u32,
    /// maximum luminance (cd/m²).
    pub max_lum: u32,
    /// reference white luminance (cd/m²).
    pub reference_lum: u32,
}
impl WpImageDescriptionInfoV1LuminancesEvent {
    /// Opcode in the event namespace.
    pub const OPCODE: u16 = 6;
    /// Minimum supported interface version.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for WpImageDescriptionInfoV1LuminancesEvent {
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
impl InterfaceReader for WpImageDescriptionInfoV1LuminancesEvent {
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
impl CommandWriter for WpImageDescriptionInfoV1LuminancesEvent {
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
        writer.next_u32(self.reference_lum).await?;
        Ok(())
    }
}
