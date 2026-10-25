use super::WpColorManagerV1TransferFunction;
use crate::protocol::ext::color_management::ObjectInteface as ColorManagementObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, CommandWriterExt, InterfaceReader,
    ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Client request `wp_image_description_creator_params_v1.set_tf_named`.
///
/// Opcode 1; available since version 1.
/// Wire framing and receiver metadata are checked; protocol semantics belong to the caller.
/// ```xml
/// <request name="set_tf_named">
///       <description summary="named transfer characteristic">
///         Sets the transfer characteristic using explicitly enumerated named
///         functions.
///
///         When the resulting image description is attached to an image, the
///         content should be decoded according to the industry standard
///         practices for the transfer characteristic.
///
///         Only names advertised with wp_color_manager_v1 event supported_tf_named
///         are allowed. Other values shall raise the protocol error invalid_tf.
///
///         If transfer characteristic has already been set on this object, the
///         protocol error already_set is raised.
///       </description>
///
///       <arg name="tf" type="uint" enum="wp_color_manager_v1.transfer_function" summary="named transfer function" />
///     </request>
/// ```
#[derive(Debug)]
pub struct WpImageDescriptionCreatorParamsV1SetTfNamedRequest {
    /// Wire identifier of the receiver.
    pub object_id: u32,
    /// named transfer function.
    pub tf: WpColorManagerV1TransferFunction,
}
impl WpImageDescriptionCreatorParamsV1SetTfNamedRequest {
    /// Opcode in the request namespace.
    pub const OPCODE: u16 = 1;
    /// Minimum supported interface version.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for WpImageDescriptionCreatorParamsV1SetTfNamedRequest {
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
impl InterfaceReader for WpImageDescriptionCreatorParamsV1SetTfNamedRequest {
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
        let tf = WpColorManagerV1TransferFunction(reader.next_u32().await?);
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        Ok(Self { object_id, tf })
    }
}
impl CommandWriter for WpImageDescriptionCreatorParamsV1SetTfNamedRequest {
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
        writer.next_u32(self.tf.0).await?;
        Ok(())
    }
}
