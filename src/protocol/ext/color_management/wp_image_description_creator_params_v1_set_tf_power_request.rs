use crate::protocol::ext::color_management::ObjectInteface as ColorManagementObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, CommandWriterExt, InterfaceReader,
    ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Client request `wp_image_description_creator_params_v1.set_tf_power`.
///
/// Opcode 2; available since version 1.
/// Wire framing and receiver metadata are checked; protocol semantics belong to the caller.
/// ```xml
/// <request name="set_tf_power">
///       <description summary="transfer characteristic as a power curve">
///         Sets the color component transfer characteristic to a power curve with
///         the given exponent. Negative values are handled by mirroring the
///         positive half of the curve through the origin. The valid domain and
///         range of the curve are all finite real numbers. This curve represents
///         the conversion from electrical to optical color channel values.
///
///         The curve exponent shall be multiplied by 10000 to get the argument eexp
///         value to carry the precision of 4 decimals.
///
///         The curve exponent must be at least 1.0 and at most 10.0. Otherwise the
///         protocol error invalid_tf is raised.
///
///         If transfer characteristic has already been set on this object, the
///         protocol error already_set is raised.
///
///         This request can be used when the compositor advertises
///         wp_color_manager_v1.feature.set_tf_power. Otherwise this request raises
///         the protocol error unsupported_feature.
///       </description>
///
///       <arg name="eexp" type="uint" summary="the exponent * 10000" />
///     </request>
/// ```
#[derive(Debug)]
pub struct WpImageDescriptionCreatorParamsV1SetTfPowerRequest {
    /// Wire identifier of the receiver.
    pub object_id: u32,
    /// the exponent * 10000.
    pub eexp: u32,
}
impl WpImageDescriptionCreatorParamsV1SetTfPowerRequest {
    /// Opcode in the request namespace.
    pub const OPCODE: u16 = 2;
    /// Minimum supported interface version.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for WpImageDescriptionCreatorParamsV1SetTfPowerRequest {
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
impl InterfaceReader for WpImageDescriptionCreatorParamsV1SetTfPowerRequest {
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
        let eexp = reader.next_u32().await?;
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        Ok(Self { object_id, eexp })
    }
}
impl CommandWriter for WpImageDescriptionCreatorParamsV1SetTfPowerRequest {
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
        Self::writer_request_interface(
            state,
            self.object_id,
            "wp_image_description_creator_params_v1",
            Self::SINCE,
        )?;
        if self.payload_size() != writer.remaining_quota() {
            return Err(FdWriterError::InvalidProtocol);
        }
        writer.next_u32(self.eexp).await?;
        Ok(())
    }
}
