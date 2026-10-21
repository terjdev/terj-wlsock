use crate::protocol::ClientState;
use crate::protocol::{CommandReader, CommandReaderExt, ObjectInteface};
use crate::protocol::{CommandWriter, CommandWriterExt};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Client request `wl_shell.get_shell_surface`.
///
/// Opcode 0; available since version 1. The structure contains the receiver and arguments in XML order.
/// Encoding and decoding check wire framing and receiver metadata; object semantics belong to the compositor.
/// The caller handles compositor policy and message contents.
/// Client encoding checks the complete payload quota before writing arguments.
/// New object IDs are registered before payload I/O; client IDs remain reserved
/// after destruction until the server acknowledges them with delete_id.
/// ```xml
/// <request name="get_shell_surface">
///       <description summary="create a shell surface from a surface">
///         Create a shell surface for an existing surface. This gives
///         the wl_surface the role of a shell surface. If the wl_surface
///         already has another role, it raises a protocol error.
///
///         Only one shell surface can be associated with a given surface.
///       </description>
///       <arg name="id" type="new_id" interface="wl_shell_surface" summary="shell surface to create" />
///       <arg name="surface" type="object" interface="wl_surface" summary="surface to be given the shell surface role" />
///     </request>
/// ```
#[derive(Debug)]
pub struct WlShellGetShellSurfaceRequest {
    /// Wire identifier of the object receiving the message.
    pub object_id: u32,
    /// Argument `id`: shell surface to create.
    /// Object implementing interface wl_shell_surface.
    pub id: u32,
    /// Argument `surface`: surface to be given the shell surface role.
    /// Object implementing interface wl_surface.
    pub surface: u32,
}
impl WlShellGetShellSurfaceRequest {
    /// Opcode in the request namespace of interface wl_shell.
    pub const OPCODE: u16 = 0;
    /// Minimum interface version supporting this message.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for WlShellGetShellSurfaceRequest {
    async fn read(
        object_id: u32,
        opcode: u16,
        reader: &mut FdPayloadReader<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        let receiver_interface = Self::reader_request_interface(
            state,
            object_id,
            opcode,
            Self::OPCODE,
            "wl_shell",
            Self::SINCE,
        )?;
        let id = reader.next_u32().await?;
        let surface = reader.next_u32().await?;
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        {
            let mut state = Self::reader_state(state)?;
            let new_interface =
                ObjectInteface::from_name("wl_shell_surface", receiver_interface.version().min(1))
                    .ok_or(FdReaderError::InvalidProtocol)?;
            state
                .register_object(id, new_interface, false)
                .map_err(|_| FdReaderError::InvalidProtocol)?;
        }
        Ok(Self {
            object_id,
            id,
            surface,
        })
    }
}

impl CommandWriter for WlShellGetShellSurfaceRequest {
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
        let receiver_interface =
            Self::writer_request_interface(state, self.object_id, "wl_shell", Self::SINCE)?;
        if self.payload_size() != writer.remaining_quota() {
            return Err(FdWriterError::InvalidProtocol);
        }
        {
            let mut state = Self::writer_state(state)?;
            let new_interface =
                ObjectInteface::from_name("wl_shell_surface", receiver_interface.version().min(1))
                    .ok_or(FdWriterError::InvalidProtocol)?;
            state
                .register_object(self.id, new_interface, false)
                .map_err(|_| FdWriterError::InvalidProtocol)?;
        }
        writer.next_u32(self.id).await?;
        writer.next_u32(self.surface).await?;
        Ok(())
    }
}
