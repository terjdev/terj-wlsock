use crate::protocol::ClientState;
use crate::protocol::{CommandReader, CommandReaderExt};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use std::sync::{Arc, Mutex};

/// Client request `wl_shm_pool.resize`.
///
/// Opcode 2; available since version 1. The structure contains the receiver and arguments in XML order.
/// Decoding checks wire framing and receiver metadata; object arguments belong to the compositor.
/// The caller handles compositor policy and message contents.
/// ```xml
/// <request name="resize">
///       <description summary="change the size of the pool mapping">
///         This request will cause the server to remap the backing memory
///         for the pool from the file descriptor passed when the pool was
///         created, but using the new size.  This request can only be
///         used to make the pool bigger.
///
///         This request only changes the amount of bytes that are mmapped
///         by the server and does not touch the file corresponding to the
///         file descriptor passed at creation time. It is the client's
///         responsibility to ensure that the file is at least as big as
///         the new pool size.
///       </description>
///       <arg name="size" type="int" summary="new size of the pool, in bytes" />
///     </request>
/// ```
#[derive(Debug)]
pub struct WlShmPoolResizeRequest {
    /// Wire identifier of the object receiving the message.
    pub object_id: u32,
    /// Argument `size`: new size of the pool, in bytes.
    pub size: i32,
}
impl WlShmPoolResizeRequest {
    /// Opcode in the request namespace of interface wl_shm_pool.
    pub const OPCODE: u16 = 2;
    /// Minimum interface version supporting this message.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for WlShmPoolResizeRequest {
    async fn read(
        object_id: u32,
        opcode: u16,
        reader: &mut FdPayloadReader<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        Self::request_interface(
            state,
            object_id,
            opcode,
            Self::OPCODE,
            "wl_shm_pool",
            Self::SINCE,
        )?;
        let size = reader.next_i32().await?;
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        Ok(Self { object_id, size })
    }
}
