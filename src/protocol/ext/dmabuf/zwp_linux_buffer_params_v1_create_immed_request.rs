use super::ZwpLinuxBufferParamsV1Flags;
use crate::protocol::ext::dmabuf::ObjectInteface as DmabufObjectInteface;
use crate::protocol::{
    ClientState, CommandReader, CommandReaderExt, CommandWriter, CommandWriterExt, InterfaceReader,
    ObjectInteface,
};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Client request `zwp_linux_buffer_params_v1.create_immed`.
///
/// Opcode 3; available since version 2.
/// Wire framing and receiver metadata are checked; protocol semantics belong to the caller.
/// New IDs are registered before writes; close the connection after a failed or cancelled write.
/// ```xml
/// <request name="create_immed" since="2">
///       <description summary="immediately create a wl_buffer from the given                      dmabufs">
///         This asks for immediate creation of a wl_buffer by importing the
///         added dmabufs.
///
///         In case of import success, no event is sent from the server, and the
///         wl_buffer is ready to be used by the client.
///
///         Upon import failure, either of the following may happen, as seen fit
///         by the implementation:
///         - the client is terminated with one of the following fatal protocol
///           errors:
///           - INCOMPLETE, INVALID_FORMAT, INVALID_DIMENSIONS, OUT_OF_BOUNDS,
///             in case of argument errors such as mismatch between the number
///             of planes and the format, bad format, non-positive width or
///             height, or bad offset or stride.
///           - INVALID_WL_BUFFER, in case the cause for failure is unknown or
///             platform specific.
///         - the server creates an invalid wl_buffer, marks it as failed and
///           sends a 'failed' event to the client. The result of using this
///           invalid wl_buffer as an argument in any request by the client is
///           defined by the compositor implementation.
///
///         This takes the same arguments as a 'create' request, and obeys the
///         same restrictions.
///       </description>
///       <arg name="buffer_id" type="new_id" interface="wl_buffer" summary="id for the newly created wl_buffer" />
///       <arg name="width" type="int" summary="base plane width in pixels" />
///       <arg name="height" type="int" summary="base plane height in pixels" />
///       <arg name="format" type="uint" summary="DRM_FORMAT code" />
///       <arg name="flags" type="uint" enum="flags" summary="see enum flags" />
///     </request>
/// ```
#[derive(Debug)]
pub struct ZwpLinuxBufferParamsV1CreateImmedRequest {
    /// Wire identifier of the receiver.
    pub object_id: u32,
    /// id for the newly created wl_buffer.
    pub buffer_id: u32,
    /// base plane width in pixels.
    pub width: i32,
    /// base plane height in pixels.
    pub height: i32,
    /// DRM_FORMAT code.
    pub format: u32,
    /// see enum flags.
    pub flags: ZwpLinuxBufferParamsV1Flags,
}
impl ZwpLinuxBufferParamsV1CreateImmedRequest {
    /// Opcode in the request namespace.
    pub const OPCODE: u16 = 3;
    /// Minimum supported interface version.
    pub const SINCE: u32 = 2;
}
impl CommandReader<Self> for ZwpLinuxBufferParamsV1CreateImmedRequest {
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
impl InterfaceReader for ZwpLinuxBufferParamsV1CreateImmedRequest {
    async fn read_with_interface(
        object_id: u32,
        opcode: u16,
        interface: ObjectInteface,
        reader: &mut FdPayloadReader<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        if !matches!(
            interface,
            ObjectInteface::Dmabuf(DmabufObjectInteface::ZwpLinuxBufferParamsV1 { .. })
        ) || opcode != Self::OPCODE
            || interface.version() < Self::SINCE
        {
            return Err(FdReaderError::InvalidProtocol);
        }
        let buffer_id = reader.next_u32().await?;
        let width = reader.next_i32().await?;
        let height = reader.next_i32().await?;
        let format = reader.next_u32().await?;
        let flags = ZwpLinuxBufferParamsV1Flags(reader.next_u32().await?);
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        {
            let new_interface =
                ObjectInteface::from_name("wl_buffer", 1).ok_or(FdReaderError::InvalidProtocol)?;
            Self::reader_state(state)?
                .register_object(buffer_id, new_interface, false)
                .map_err(|_| FdReaderError::InvalidProtocol)?;
        }
        Ok(Self {
            object_id,
            buffer_id,
            width,
            height,
            format,
            flags,
        })
    }
}
impl CommandWriter for ZwpLinuxBufferParamsV1CreateImmedRequest {
    fn object_id(&self) -> u32 {
        self.object_id
    }
    fn opcode(&self) -> u16 {
        Self::OPCODE
    }
    fn payload_size(&self) -> usize {
        Self::total_payload_size(&[Some(4), Some(4), Some(4), Some(4), Some(4)])
    }
    async fn write(
        self,
        writer: &mut FdPayloadWriter<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<(), FdWriterError> {
        Self::writer_request_interface(
            state,
            self.object_id,
            "zwp_linux_buffer_params_v1",
            Self::SINCE,
        )?;
        if self.payload_size() != writer.remaining_quota() {
            return Err(FdWriterError::InvalidProtocol);
        }
        {
            let new_interface =
                ObjectInteface::from_name("wl_buffer", 1).ok_or(FdWriterError::InvalidProtocol)?;
            Self::writer_state(state)?
                .register_object(self.buffer_id, new_interface, false)
                .map_err(|_| FdWriterError::InvalidProtocol)?;
        }
        writer.next_u32(self.buffer_id).await?;
        writer.next_i32(self.width).await?;
        writer.next_i32(self.height).await?;
        writer.next_u32(self.format).await?;
        writer.next_u32(self.flags.0).await?;
        Ok(())
    }
}
