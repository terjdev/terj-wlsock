use super::WpContentTypeV1Type;
use crate::protocol::ext::content_type::ObjectInteface as ContentTypeObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, CommandWriterExt, InterfaceReader,
    ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Client request `wp_content_type_v1.set_content_type`.
///
/// Opcode 1; available since version 1.
/// Wire framing and receiver metadata are checked; protocol semantics belong to the caller.
/// ```xml
/// <request name="set_content_type">
///       <description summary="specify the content type">
///         Set the surface content type. This informs the compositor that the
///         client believes it is displaying buffers matching this content type.
///
///         This is purely a hint for the compositor, which can be used to adjust
///         its behavior or hardware settings to fit the presented content best.
///
///         The content type is double-buffered state, see wl_surface.commit for
///         details.
///       </description>
///       <arg name="content_type" type="uint" enum="type" summary="the content type" />
///     </request>
/// ```
#[derive(Debug)]
pub struct WpContentTypeV1SetContentTypeRequest {
    /// Wire identifier of the receiver.
    pub object_id: u32,
    /// the content type.
    pub content_type: WpContentTypeV1Type,
}
impl WpContentTypeV1SetContentTypeRequest {
    /// Opcode in the request namespace.
    pub const OPCODE: u16 = 1;
    /// Minimum supported interface version.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for WpContentTypeV1SetContentTypeRequest {
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
impl InterfaceReader for WpContentTypeV1SetContentTypeRequest {
    async fn read_with_interface(
        object_id: u32,
        opcode: u16,
        interface: ObjectInteface,
        reader: &mut FdPayloadReader<'_>,
        _state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        if !matches!(
            interface,
            ObjectInteface::ContentType(ContentTypeObjectInteface::WpContentTypeV1 { .. })
        ) || opcode != Self::OPCODE
            || interface.version() < Self::SINCE
        {
            return Err(FdReaderError::InvalidProtocol);
        }
        let content_type = WpContentTypeV1Type(reader.next_u32().await?);
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        Ok(Self {
            object_id,
            content_type,
        })
    }
}
impl CommandWriter for WpContentTypeV1SetContentTypeRequest {
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
        Self::writer_request_interface(state, self.object_id, "wp_content_type_v1", Self::SINCE)?;
        if self.payload_size() != writer.remaining_quota() {
            return Err(FdWriterError::InvalidProtocol);
        }
        writer.next_u32(self.content_type.0).await?;
        Ok(())
    }
}
