use crate::protocol::ClientState;
use crate::protocol::{CommandReader, CommandReaderExt, ObjectInteface};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use std::sync::{Arc, Mutex};

/// Client request `wl_shm.create_pool`.
///
/// Opcode 0; available since version 1. The structure contains the receiver and arguments in XML order.
/// Decoding checks wire framing and receiver metadata; object arguments belong to the compositor.
/// The caller handles compositor policy and message contents.
/// ```xml
/// <request name="create_pool">
///       <description summary="create a shm pool">
///         Create a new wl_shm_pool object.
///
///         The pool can be used to create shared memory based buffer
///         objects.  The server will mmap size bytes of the passed file
///         descriptor, to use as backing memory for the pool.
///       </description>
///       <arg name="id" type="new_id" interface="wl_shm_pool" summary="pool to create" />
///       <arg name="fd" type="fd" summary="file descriptor for the pool" />
///       <arg name="size" type="int" summary="pool size, in bytes" />
///     </request>
/// ```
#[derive(Debug)]
pub struct WlShmCreatePoolRequest {
    /// Wire identifier of the object receiving the message.
    pub object_id: u32,
    /// Argument `id`: pool to create.
    /// Object implementing interface wl_shm_pool.
    pub id: u32,
    /// Argument `fd`: file descriptor for the pool.
    pub fd: std::os::fd::OwnedFd,
    /// Argument `size`: pool size, in bytes.
    pub size: i32,
}
impl WlShmCreatePoolRequest {
    /// Opcode in the request namespace of interface wl_shm.
    pub const OPCODE: u16 = 0;
    /// Minimum interface version supporting this message.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for WlShmCreatePoolRequest {
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
            "wl_shm",
            Self::SINCE,
        )?;
        let id = reader.next_u32().await?;
        let size = reader.next_i32().await?;
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        let fd = reader.next_fd().ok_or(FdReaderError::InvalidProtocol)?;
        {
            let mut state = Self::read_state(state)?;
            let new_interface =
                ObjectInteface::from_name("wl_shm_pool", receiver_interface.version().min(3))
                    .ok_or(FdReaderError::InvalidProtocol)?;
            state
                .register_object(id, new_interface, false)
                .map_err(|_| FdReaderError::InvalidProtocol)?;
        }
        Ok(Self {
            object_id,
            id,
            fd,
            size,
        })
    }
}
