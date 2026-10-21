use super::WlShmFormat;
use crate::protocol::ClientState;
use crate::protocol::{CommandReader, CommandReaderExt, ObjectInteface};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use std::sync::{Arc, Mutex};

/// Client request `wl_shm_pool.create_buffer`.
///
/// Opcode 0; available since version 1. The structure contains the receiver and arguments in XML order.
/// Decoding checks wire framing and receiver metadata; object arguments belong to the compositor.
/// The caller handles compositor policy and message contents.
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
        let receiver_interface = Self::request_interface(
            state,
            object_id,
            opcode,
            Self::OPCODE,
            "wl_shm_pool",
            Self::SINCE,
        )?;
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
            let mut state = Self::read_state(state)?;
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
