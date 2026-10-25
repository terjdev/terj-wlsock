use crate::protocol::ext::color_management::ObjectInteface as ColorManagementObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, CommandWriterExt, InterfaceReader,
    ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Client request `wp_image_description_creator_icc_v1.create`.
///
/// Opcode 0; available since version 1.
/// Wire framing and receiver metadata are checked; protocol semantics belong to the caller.
/// New IDs are registered before writes; close the connection after a failed or cancelled write.
/// ```xml
/// <request name="create" type="destructor">
///       <description summary="Create the image description object from ICC data">
///         Create an image description object based on the ICC information
///         previously set on this object. A compositor must parse the ICC data in
///         some undefined but finite amount of time.
///
///         The completeness of the parameter set is verified. If the set is not
///         complete, the protocol error incomplete_set is raised. For the
///         definition of a complete set, see the description of this interface.
///
///         If the particular combination of the information is not supported
///         by the compositor, the resulting image description object shall
///         immediately deliver the wp_image_description_v1.failed event with the
///         'unsupported' cause. If a valid image description was created from the
///         information, the wp_image_description_v1.ready event will eventually
///         be sent instead.
///
///         This request destroys the wp_image_description_creator_icc_v1 object.
///
///         The resulting image description object does not allow get_information
///         request.
///       </description>
///
///       <arg name="image_description" type="new_id" interface="wp_image_description_v1" />
///     </request>
/// ```
#[derive(Debug)]
pub struct WpImageDescriptionCreatorIccV1CreateRequest {
    /// Wire identifier of the receiver.
    pub object_id: u32,
    /// Wire argument `image_description`.
    pub image_description: u32,
}
impl WpImageDescriptionCreatorIccV1CreateRequest {
    /// Opcode in the request namespace.
    pub const OPCODE: u16 = 0;
    /// Minimum supported interface version.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for WpImageDescriptionCreatorIccV1CreateRequest {
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
impl InterfaceReader for WpImageDescriptionCreatorIccV1CreateRequest {
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
                ColorManagementObjectInteface::WpImageDescriptionCreatorIccV1 { .. }
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
        Self::reader_state(state)?.release_server_id(object_id);
        Ok(Self {
            object_id,
            image_description,
        })
    }
}
impl CommandWriter for WpImageDescriptionCreatorIccV1CreateRequest {
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
            "wp_image_description_creator_icc_v1",
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
        Self::writer_state(state)?.release_server_id(self.object_id);
        Ok(())
    }
}
