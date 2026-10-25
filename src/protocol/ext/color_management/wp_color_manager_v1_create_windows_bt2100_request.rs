use crate::protocol::ext::color_management::ObjectInteface as ColorManagementObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, CommandWriterExt, InterfaceReader,
    ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Client request `wp_color_manager_v1.create_windows_bt2100`.
///
/// Opcode 8; available since version 3.
/// Wire framing and receiver metadata are checked; protocol semantics belong to the caller.
/// New IDs are registered before writes; close the connection after a failed or cancelled write.
/// ```xml
/// <request name="create_windows_bt2100" since="3">
///       <description summary="create Windows-BT.2100 image description object">
///         This creates a pre-defined image description for the so-called
///         Windows-BT.2100 stimulus encoding. This comes from the Windows 10
///         handling of its own definition of a BT.2100 color space for an HDR
///         screen driven in BT.2100/PQ signalling mode.
///
///         Windows-BT.2100 uses BT.2020 color primaries and white point.
///         The transfer characteristic is st2084_pq.
///
///         Windows-BT.2100 is generally displayed by Windows 10 without any
///         adjustments to the signal to account for viewing conditions.
///
///         The reference white level of Windows-BT.2100 is unknown. If a
///         reference white level must be assumed for compositor processing, it
///         should be 203 cd/m² of Report ITU-R BT.2408-7.
///
///         The target color volume of Windows-BT.2100 is unknown. The color gamut
///         may be anything up to BT.2100.
///
///         This request can be used when the compositor advertises
///         wp_color_manager_v1.feature.windows_bt2100.
///         Otherwise this request raises the protocol error unsupported_feature.
///
///         The resulting image description object does not allow get_information
///         request. The wp_image_description_v1.ready event shall be sent.
///       </description>
///
///       <arg name="image_description" type="new_id" interface="wp_image_description_v1" />
///       </request>
/// ```
#[derive(Debug)]
pub struct WpColorManagerV1CreateWindowsBt2100Request {
    /// Wire identifier of the receiver.
    pub object_id: u32,
    /// Wire argument `image_description`.
    pub image_description: u32,
}
impl WpColorManagerV1CreateWindowsBt2100Request {
    /// Opcode in the request namespace.
    pub const OPCODE: u16 = 8;
    /// Minimum supported interface version.
    pub const SINCE: u32 = 3;
}
impl CommandReader<Self> for WpColorManagerV1CreateWindowsBt2100Request {
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
impl InterfaceReader for WpColorManagerV1CreateWindowsBt2100Request {
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
        Ok(Self {
            object_id,
            image_description,
        })
    }
}
impl CommandWriter for WpColorManagerV1CreateWindowsBt2100Request {
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
            let new_interface =
                ObjectInteface::from_name("wp_image_description_v1", interface.version().min(3))
                    .ok_or(FdWriterError::InvalidProtocol)?;
            Self::writer_state(state)?
                .register_object(self.image_description, new_interface, false)
                .map_err(|_| FdWriterError::InvalidProtocol)?;
        }
        writer.next_u32(self.image_description).await?;
        Ok(())
    }
}
