use super::WlShmFormat;
use crate::protocol::ClientState;
use crate::protocol::InterfaceReader;
use crate::protocol::core::ObjectInteface as CoreObjectInteface;
use crate::protocol::{CommandReader, CommandReaderExt, ObjectInteface};
use crate::protocol::{CommandWriter, CommandWriterExt};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Client request `wl_shm_pool.create_buffer`.
///
/// Opcode 0; available since version 1. The structure contains the receiver and arguments in XML order.
/// Encoding and decoding check wire framing and receiver metadata; object semantics belong to the compositor.
/// The caller handles compositor policy and message contents.
/// Client encoding checks the complete payload quota before writing arguments.
/// New object IDs are registered before payload I/O; client IDs remain reserved
/// after destruction until the server acknowledges them with delete_id.
/// ```xml
/// <request name="create_buffer">
///       <description summary="create a buffer from the pool">
///         Create a wl_buffer object from the pool.
///
///         The buffer is created offset bytes into the pool and has
///         width and height as specified.  The stride argument specifies
///         the number of bytes from the beginning of one row to the beginning
///         of the next.  The format is the pixel format of the buffer and
///         must be one of those advertised through the wl_shm.format event.
///
///         A buffer will keep a reference to the pool it was created from
///         so it is valid to destroy the pool immediately after creating
///         a buffer from it.
///       </description>
///       <arg name="id" type="new_id" interface="wl_buffer" summary="buffer to create" />
///       <arg name="offset" type="int" summary="buffer byte offset within the pool" />
///       <arg name="width" type="int" summary="buffer width, in pixels" />
///       <arg name="height" type="int" summary="buffer height, in pixels" />
///       <arg name="stride" type="int" summary="number of bytes from the beginning of one row to the beginning of the next row" />
///       <arg name="format" type="uint" enum="wl_shm.format" summary="buffer pixel format" />
///     </request>
/// ```
#[derive(Debug)]
pub struct WlShmPoolCreateBufferRequest {
    /// Wire identifier of the object receiving the message.
    pub object_id: u32,
    /// Argument `id`: buffer to create.
    /// Object implementing interface wl_buffer.
    pub id: u32,
    /// Argument `offset`: buffer byte offset within the pool.
    pub offset: i32,
    /// Argument `width`: buffer width, in pixels.
    pub width: i32,
    /// Argument `height`: buffer height, in pixels.
    pub height: i32,
    /// Argument `stride`: number of bytes from the beginning of one row to the beginning of the next row.
    pub stride: i32,
    /// Argument `format`: buffer pixel format.
    pub format: WlShmFormat,
}
impl WlShmPoolCreateBufferRequest {
    /// Opcode in the request namespace of interface wl_shm_pool.
    pub const OPCODE: u16 = 0;
    /// Minimum interface version supporting this message.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for WlShmPoolCreateBufferRequest {
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

impl InterfaceReader for WlShmPoolCreateBufferRequest {
    async fn read_with_interface(
        object_id: u32,
        opcode: u16,
        interface: ObjectInteface,
        reader: &mut FdPayloadReader<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        if !matches!(
            interface,
            ObjectInteface::Core(CoreObjectInteface::WlShmPool { .. })
        ) || opcode != Self::OPCODE
            || interface.version() < Self::SINCE
        {
            return Err(FdReaderError::InvalidProtocol);
        }
        let receiver_interface = interface;
        let id = reader.next_u32().await?;
        let offset = reader.next_i32().await?;
        let width = reader.next_i32().await?;
        let height = reader.next_i32().await?;
        let stride = reader.next_i32().await?;
        let format = WlShmFormat(reader.next_u32().await?);
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        {
            let mut state = Self::reader_state(state)?;
            let new_interface =
                ObjectInteface::from_name("wl_buffer", receiver_interface.version().min(1))
                    .ok_or(FdReaderError::InvalidProtocol)?;
            state
                .register_object(id, new_interface, false)
                .map_err(|_| FdReaderError::InvalidProtocol)?;
        }
        Ok(Self {
            object_id,
            id,
            offset,
            width,
            height,
            stride,
            format,
        })
    }
}

impl CommandWriter for WlShmPoolCreateBufferRequest {
    fn object_id(&self) -> u32 {
        self.object_id
    }
    fn opcode(&self) -> u16 {
        Self::OPCODE
    }
    fn payload_size(&self) -> usize {
        Self::total_payload_size(&[Some(4), Some(4), Some(4), Some(4), Some(4), Some(4)])
    }
    async fn write(
        self,
        writer: &mut FdPayloadWriter<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<(), FdWriterError> {
        let receiver_interface =
            Self::writer_request_interface(state, self.object_id, "wl_shm_pool", Self::SINCE)?;
        if self.payload_size() != writer.remaining_quota() {
            return Err(FdWriterError::InvalidProtocol);
        }
        {
            let mut state = Self::writer_state(state)?;
            let new_interface =
                ObjectInteface::from_name("wl_buffer", receiver_interface.version().min(1))
                    .ok_or(FdWriterError::InvalidProtocol)?;
            state
                .register_object(self.id, new_interface, false)
                .map_err(|_| FdWriterError::InvalidProtocol)?;
        }
        writer.next_u32(self.id).await?;
        writer.next_i32(self.offset).await?;
        writer.next_i32(self.width).await?;
        writer.next_i32(self.height).await?;
        writer.next_i32(self.stride).await?;
        writer.next_u32(self.format.0).await?;
        Ok(())
    }
}
