use crate::protocol::ext::image_capture_source::ObjectInteface as ImageCaptureSourceObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, CommandWriterExt, InterfaceReader,
    ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Client request `ext_foreign_toplevel_image_capture_source_manager_v1.create_source`.
///
/// Opcode 0; available since version 1.
/// Wire framing and receiver metadata are checked; protocol semantics belong to the caller.
/// New IDs are registered before writes; close the connection after a failed or cancelled write.
/// ```xml
/// <request name="create_source">
///       <description summary="create source object for foreign toplevel">
///         Creates a source object for a foreign toplevel handle. Images captured
///         from this source will show the same content as the toplevel.
///       </description>
///       <arg name="source" type="new_id" interface="ext_image_capture_source_v1" />
///       <arg name="toplevel_handle" type="object" interface="ext_foreign_toplevel_handle_v1" />
///     </request>
/// ```
#[derive(Debug)]
pub struct ExtForeignToplevelImageCaptureSourceManagerV1CreateSourceRequest {
    /// Wire identifier of the receiver.
    pub object_id: u32,
    /// Wire argument `source`.
    pub source: u32,
    /// Wire argument `toplevel_handle`.
    pub toplevel_handle: u32,
}
impl ExtForeignToplevelImageCaptureSourceManagerV1CreateSourceRequest {
    /// Opcode in the request namespace.
    pub const OPCODE: u16 = 0;
    /// Minimum supported interface version.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for ExtForeignToplevelImageCaptureSourceManagerV1CreateSourceRequest {
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
impl InterfaceReader for ExtForeignToplevelImageCaptureSourceManagerV1CreateSourceRequest {
    async fn read_with_interface(
        object_id: u32,
        opcode: u16,
        interface: ObjectInteface,
        reader: &mut FdPayloadReader<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        if !matches!(
            interface,
            ObjectInteface::ImageCaptureSource(
                ImageCaptureSourceObjectInteface::ExtForeignToplevelImageCaptureSourceManagerV1 { .. }
            )
        ) || opcode != Self::OPCODE
            || interface.version() < Self::SINCE
        {
            return Err(FdReaderError::InvalidProtocol);
        }
        let source = reader.next_u32().await?;
        let toplevel_handle = reader.next_u32().await?;
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        {
            let new_interface = ObjectInteface::from_name(
                "ext_image_capture_source_v1",
                interface.version().min(1),
            )
            .ok_or(FdReaderError::InvalidProtocol)?;
            Self::reader_state(state)?
                .register_object(source, new_interface, false)
                .map_err(|_| FdReaderError::InvalidProtocol)?;
        }
        Ok(Self {
            object_id,
            source,
            toplevel_handle,
        })
    }
}
impl CommandWriter for ExtForeignToplevelImageCaptureSourceManagerV1CreateSourceRequest {
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
            "ext_foreign_toplevel_image_capture_source_manager_v1",
            Self::SINCE,
        )?;
        if self.payload_size() != writer.remaining_quota() {
            return Err(FdWriterError::InvalidProtocol);
        }
        {
            let new_interface = ObjectInteface::from_name(
                "ext_image_capture_source_v1",
                interface.version().min(1),
            )
            .ok_or(FdWriterError::InvalidProtocol)?;
            Self::writer_state(state)?
                .register_object(self.source, new_interface, false)
                .map_err(|_| FdWriterError::InvalidProtocol)?;
        }
        writer.next_u32(self.source).await?;
        writer.next_u32(self.toplevel_handle).await?;
        Ok(())
    }
}
