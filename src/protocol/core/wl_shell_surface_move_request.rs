use crate::protocol::ClientState;
use crate::protocol::{CommandReader, CommandReaderExt};
use crate::protocol::{CommandWriter, CommandWriterExt};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Client request `wl_shell_surface.move`.
///
/// Opcode 1; available since version 1. The structure contains the receiver and arguments in XML order.
/// Encoding and decoding check wire framing and receiver metadata; object semantics belong to the compositor.
/// The caller handles compositor policy and message contents.
/// Client encoding checks the complete payload quota before writing arguments.
/// New object IDs are registered before payload I/O; client IDs remain reserved
/// after destruction until the server acknowledges them with delete_id.
/// ```xml
/// <request name="move">
///       <description summary="start an interactive move">
///         Start a pointer-driven move of the surface.
///
///         This request must be used in response to a button press event.
///         The server may ignore move requests depending on the state of
///         the surface (e.g. fullscreen or maximized).
///       </description>
///       <arg name="seat" type="object" interface="wl_seat" summary="seat whose pointer is used" />
///       <arg name="serial" type="uint" summary="serial number of the implicit grab on the pointer" />
///     </request>
/// ```
#[derive(Debug)]
pub struct WlShellSurfaceMoveRequest {
    /// Wire identifier of the object receiving the message.
    pub object_id: u32,
    /// Argument `seat`: seat whose pointer is used.
    /// Object implementing interface wl_seat.
    pub seat: u32,
    /// Argument `serial`: serial number of the implicit grab on the pointer.
    pub serial: u32,
}
impl WlShellSurfaceMoveRequest {
    /// Opcode in the request namespace of interface wl_shell_surface.
    pub const OPCODE: u16 = 1;
    /// Minimum interface version supporting this message.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for WlShellSurfaceMoveRequest {
    async fn read(
        object_id: u32,
        opcode: u16,
        reader: &mut FdPayloadReader<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        Self::reader_request_interface(
            state,
            object_id,
            opcode,
            Self::OPCODE,
            "wl_shell_surface",
            Self::SINCE,
        )?;
        let seat = reader.next_u32().await?;
        let serial = reader.next_u32().await?;
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        Ok(Self {
            object_id,
            seat,
            serial,
        })
    }
}

impl CommandWriter for WlShellSurfaceMoveRequest {
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
        Self::writer_request_interface(state, self.object_id, "wl_shell_surface", Self::SINCE)?;
        if self.payload_size() != writer.remaining_quota() {
            return Err(FdWriterError::InvalidProtocol);
        }
        writer.next_u32(self.seat).await?;
        writer.next_u32(self.serial).await?;
        Ok(())
    }
}
