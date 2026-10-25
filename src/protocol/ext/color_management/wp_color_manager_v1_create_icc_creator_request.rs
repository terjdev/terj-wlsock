use crate::protocol::ext::color_management::ObjectInteface as ColorManagementObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, CommandWriterExt, InterfaceReader,
    ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Client request `wp_color_manager_v1.create_icc_creator`.
///
/// Opcode 4; available since version 1.
/// Wire framing and receiver metadata are checked; protocol semantics belong to the caller.
/// New IDs are registered before writes; close the connection after a failed or cancelled write.
/// ```xml
/// <request name="create_icc_creator">
///       <description summary="make a new ICC-based image description creator object">
///         Makes a new ICC-based image description creator object with all
///         properties initially unset. The client can then use the object's
///         interface to define all the required properties for an image description
///         and finally create a wp_image_description_v1 object.
///
///         This request can be used when the compositor advertises
///         wp_color_manager_v1.feature.icc_v2_v4.
///         Otherwise this request raises the protocol error unsupported_feature.
///       </description>
///
///       <arg name="obj" type="new_id" interface="wp_image_description_creator_icc_v1" summary="the new creator object" />
///     </request>
/// ```
#[derive(Debug)]
pub struct WpColorManagerV1CreateIccCreatorRequest {
    /// Wire identifier of the receiver.
    pub object_id: u32,
    /// the new creator object.
    pub obj: u32,
}
impl WpColorManagerV1CreateIccCreatorRequest {
    /// Opcode in the request namespace.
    pub const OPCODE: u16 = 4;
    /// Minimum supported interface version.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for WpColorManagerV1CreateIccCreatorRequest {
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
impl InterfaceReader for WpColorManagerV1CreateIccCreatorRequest {
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
        let obj = reader.next_u32().await?;
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        {
            let new_interface = ObjectInteface::from_name(
                "wp_image_description_creator_icc_v1",
                interface.version().min(3),
            )
            .ok_or(FdReaderError::InvalidProtocol)?;
            Self::reader_state(state)?
                .register_object(obj, new_interface, false)
                .map_err(|_| FdReaderError::InvalidProtocol)?;
        }
        Ok(Self { object_id, obj })
    }
}
impl CommandWriter for WpColorManagerV1CreateIccCreatorRequest {
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
            let new_interface = ObjectInteface::from_name(
                "wp_image_description_creator_icc_v1",
                interface.version().min(3),
            )
            .ok_or(FdWriterError::InvalidProtocol)?;
            Self::writer_state(state)?
                .register_object(self.obj, new_interface, false)
                .map_err(|_| FdWriterError::InvalidProtocol)?;
        }
        writer.next_u32(self.obj).await?;
        Ok(())
    }
}
