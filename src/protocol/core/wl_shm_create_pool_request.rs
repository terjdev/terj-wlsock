use crate::protocol::ClientState;
use crate::protocol::InterfaceReader;
use crate::protocol::{CommandReader, CommandReaderExt, ObjectInteface};
use crate::protocol::{CommandWriter, CommandWriterExt};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Client request `wl_shm.create_pool`.
///
/// Opcode 0; available since version 1. The structure contains the receiver and arguments in XML order.
/// Encoding and decoding check wire framing and receiver metadata; object semantics belong to the compositor.
/// The caller handles compositor policy and message contents.
/// Client encoding checks the complete payload quota before writing arguments.
/// New object IDs are registered before payload I/O; client IDs remain reserved
/// after destruction until the server acknowledges them with delete_id.
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
        let interface = Self::reader_state(state)?
            .object_interface(object_id)
            .ok_or(FdReaderError::InvalidProtocol)?;
        Self::read_with_interface(object_id, opcode, interface, reader, state).await
    }
}

impl InterfaceReader for WlShmCreatePoolRequest {
    async fn read_with_interface(
        object_id: u32,
        opcode: u16,
        interface: ObjectInteface,
        reader: &mut FdPayloadReader<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        if !matches!(interface, ObjectInteface::WlShm { .. })
            || opcode != Self::OPCODE
            || interface.version() < Self::SINCE
        {
            return Err(FdReaderError::InvalidProtocol);
        }
        let receiver_interface = interface;
        let id = reader.next_u32().await?;
        let size = reader.next_i32().await?;
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        let fd = reader.next_fd().ok_or(FdReaderError::InvalidProtocol)?;
        {
            let mut state = Self::reader_state(state)?;
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

impl CommandWriter for WlShmCreatePoolRequest {
    fn object_id(&self) -> u32 {
        self.object_id
    }
    fn opcode(&self) -> u16 {
        Self::OPCODE
    }
    fn payload_size(&self) -> usize {
        Self::total_payload_size(&[Some(4), Some(0), Some(4)])
    }
    async fn write(
        self,
        writer: &mut FdPayloadWriter<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<(), FdWriterError> {
        let receiver_interface =
            Self::writer_request_interface(state, self.object_id, "wl_shm", Self::SINCE)?;
        if self.payload_size() != writer.remaining_quota() {
            return Err(FdWriterError::InvalidProtocol);
        }
        {
            let mut state = Self::writer_state(state)?;
            let new_interface =
                ObjectInteface::from_name("wl_shm_pool", receiver_interface.version().min(3))
                    .ok_or(FdWriterError::InvalidProtocol)?;
            state
                .register_object(self.id, new_interface, false)
                .map_err(|_| FdWriterError::InvalidProtocol)?;
        }
        writer.next_u32(self.id).await?;
        writer.next_fd(self.fd).await?;
        writer.next_i32(self.size).await?;
        Ok(())
    }
}
