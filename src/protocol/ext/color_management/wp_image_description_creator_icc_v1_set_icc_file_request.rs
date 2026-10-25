use crate::protocol::ext::color_management::ObjectInteface as ColorManagementObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, CommandWriterExt, InterfaceReader,
    ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Client request `wp_image_description_creator_icc_v1.set_icc_file`.
///
/// Opcode 1; available since version 1.
/// Wire framing and receiver metadata are checked; protocol semantics belong to the caller.
/// ```xml
/// <request name="set_icc_file">
///       <description summary="set the ICC profile file">
///         Sets the ICC profile file to be used as the basis of the image
///         description.
///
///         The data shall be found through the given fd at the given offset, having
///         the given length. The fd must be seekable and readable. Violating these
///         requirements raises the bad_fd protocol error.
///
///         If reading the data fails due to an error independent of the client, the
///         compositor shall send the wp_image_description_v1.failed event on the
///         created wp_image_description_v1 with the 'operating_system' cause.
///
///         The maximum size of the ICC profile is 32 MB. If length is greater than
///         that or zero, the protocol error bad_size is raised. If offset + length
///         exceeds the file size, the protocol error out_of_file is raised.
///
///         A compositor may read the file at any time starting from this request
///         and only until whichever happens first:
///         - If create request was issued, the wp_image_description_v1 object
///           delivers either failed or ready event; or
///         - if create request was not issued, this
///           wp_image_description_creator_icc_v1 object is destroyed.
///
///         A compositor shall not modify the contents of the file, and the fd may
///         be sealed for writes and size changes. The client must ensure to its
///         best ability that the data does not change while the compositor is
///         reading it.
///
///         The data must represent a valid ICC profile. The ICC profile version
///         must be 2 or 4, it must be a 3 channel profile and the class must be
///         Display or ColorSpace. Violating these requirements will not result in a
///         protocol error, but will eventually send the
///         wp_image_description_v1.failed event on the created
///         wp_image_description_v1 with the 'unsupported' cause.
///
///         See the International Color Consortium specification ICC.1:2022 for more
///         details about ICC profiles.
///
///         If ICC file has already been set on this object, the protocol error
///         already_set is raised.
///       </description>
///
///       <arg name="icc_profile" type="fd" summary="ICC profile" />
///       <arg name="offset" type="uint" summary="byte offset in fd to start of ICC data" />
///       <arg name="length" type="uint" summary="length of ICC data in bytes" />
///     </request>
/// ```
#[derive(Debug)]
pub struct WpImageDescriptionCreatorIccV1SetIccFileRequest {
    /// Wire identifier of the receiver.
    pub object_id: u32,
    /// ICC profile.
    pub icc_profile: std::os::fd::OwnedFd,
    /// byte offset in fd to start of ICC data.
    pub offset: u32,
    /// length of ICC data in bytes.
    pub length: u32,
}
impl WpImageDescriptionCreatorIccV1SetIccFileRequest {
    /// Opcode in the request namespace.
    pub const OPCODE: u16 = 1;
    /// Minimum supported interface version.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for WpImageDescriptionCreatorIccV1SetIccFileRequest {
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
impl InterfaceReader for WpImageDescriptionCreatorIccV1SetIccFileRequest {
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
                ColorManagementObjectInteface::WpImageDescriptionCreatorIccV1 { .. }
            )
        ) || opcode != Self::OPCODE
            || interface.version() < Self::SINCE
        {
            return Err(FdReaderError::InvalidProtocol);
        }
        let offset = reader.next_u32().await?;
        let length = reader.next_u32().await?;
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        let icc_profile = reader.next_fd().ok_or(FdReaderError::InvalidProtocol)?;
        Ok(Self {
            object_id,
            icc_profile,
            offset,
            length,
        })
    }
}
impl CommandWriter for WpImageDescriptionCreatorIccV1SetIccFileRequest {
    fn object_id(&self) -> u32 {
        self.object_id
    }
    fn opcode(&self) -> u16 {
        Self::OPCODE
    }
    fn payload_size(&self) -> usize {
        Self::total_payload_size(&[Some(0), Some(4), Some(4)])
    }
    async fn write(
        self,
        writer: &mut FdPayloadWriter<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<(), FdWriterError> {
        Self::writer_request_interface(
            state,
            self.object_id,
            "wp_image_description_creator_icc_v1",
            Self::SINCE,
        )?;
        if self.payload_size() != writer.remaining_quota() {
            return Err(FdWriterError::InvalidProtocol);
        }
        writer.next_fd(self.icc_profile).await?;
        writer.next_u32(self.offset).await?;
        writer.next_u32(self.length).await?;
        Ok(())
    }
}
