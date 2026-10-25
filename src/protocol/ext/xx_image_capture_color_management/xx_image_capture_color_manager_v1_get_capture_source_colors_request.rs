use super::ObjectInteface as XxImageCaptureColorManagementObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, CommandWriterExt, InterfaceReader,
    ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Client request `xx_image_capture_color_manager_v1.get_capture_source_colors`.
///
/// Framing, receiver protocol and version are checked; semantics belong to the caller.
/// New IDs are registered before writes; close the connection after a failed or cancelled write.
/// ```xml
/// <request name="get_capture_source_colors">
///       <description summary="create a color management interface for an ext_image_capture_source_v1">
///         This creates a new xx_image_capture_source_colors_v1 for a given
///         ext_image_capture_source_v1.
///
///         See xx_image_capture_source_colors_v1 for more details.
///       </description>
///       <arg name="colors" type="new_id" interface="xx_image_capture_source_colors_v1" />
///       <arg name="source" type="object" interface="ext_image_capture_source_v1" />
///     </request>
/// ```
#[derive(Debug)]
pub struct XxImageCaptureColorManagerV1GetCaptureSourceColorsRequest {
    /// Wire identifier of the receiver.
    pub object_id: u32,
    /// Wire argument `colors`.
    pub colors: u32,
    /// Wire argument `source`.
    pub source: u32,
}
impl XxImageCaptureColorManagerV1GetCaptureSourceColorsRequest {
    /// Opcode in the request namespace.
    pub const OPCODE: u16 = 1;
    /// Minimum interface version.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for XxImageCaptureColorManagerV1GetCaptureSourceColorsRequest {
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
impl InterfaceReader for XxImageCaptureColorManagerV1GetCaptureSourceColorsRequest {
    async fn read_with_interface(
        object_id: u32,
        opcode: u16,
        interface: ObjectInteface,
        reader: &mut FdPayloadReader<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        if !matches!(
            interface,
            ObjectInteface::XxImageCaptureColorManagement(
                XxImageCaptureColorManagementObjectInteface::XxImageCaptureColorManagerV1 { .. }
            )
        ) || opcode != Self::OPCODE
            || interface.version() < Self::SINCE
        {
            return Err(FdReaderError::InvalidProtocol);
        }
        let colors = reader.next_u32().await?;
        let source = reader.next_u32().await?;
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        {
            let new_interface = ObjectInteface::XxImageCaptureColorManagement(
                XxImageCaptureColorManagementObjectInteface::XxImageCaptureSourceColorsV1 {
                    version: interface.version().min(1),
                },
            );
            Self::reader_state(state)?
                .register_object(colors, new_interface, false)
                .map_err(|_| FdReaderError::InvalidProtocol)?;
        }
        Ok(Self {
            object_id,
            colors,
            source,
        })
    }
}
impl CommandWriter for XxImageCaptureColorManagerV1GetCaptureSourceColorsRequest {
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
        let interface = Self::writer_request_interface(
            state,
            self.object_id,
            "xx_image_capture_color_manager_v1",
            Self::SINCE,
        )?;
        if !matches!(
            interface,
            ObjectInteface::XxImageCaptureColorManagement(
                XxImageCaptureColorManagementObjectInteface::XxImageCaptureColorManagerV1 { .. }
            )
        ) || self.payload_size() != writer.remaining_quota()
        {
            return Err(FdWriterError::InvalidProtocol);
        }
        {
            let new_interface = ObjectInteface::XxImageCaptureColorManagement(
                XxImageCaptureColorManagementObjectInteface::XxImageCaptureSourceColorsV1 {
                    version: interface.version().min(1),
                },
            );
            Self::writer_state(state)?
                .register_object(self.colors, new_interface, false)
                .map_err(|_| FdWriterError::InvalidProtocol)?;
        }
        writer.next_u32(self.colors).await?;
        writer.next_u32(self.source).await?;
        Ok(())
    }
}
