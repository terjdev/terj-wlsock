use crate::protocol::ClientState;
use crate::protocol::{CommandReader, CommandReaderExt};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use std::sync::{Arc, Mutex};

/// Client request `wl_shell_surface.pong`.
///
/// Opcode 0; available since version 1. The structure contains the receiver and arguments in XML order.
/// Decoding checks wire framing and receiver metadata; object arguments belong to the compositor.
/// The caller handles compositor policy and message contents.
/// ```xml
/// <request name="pong">
///       <description summary="respond to a ping event">
///         A client must respond to a ping event with a pong request or
///         the client may be deemed unresponsive.
///       </description>
///       <arg name="serial" type="uint" summary="serial number of the ping event" />
///     </request>
/// ```
#[derive(Debug)]
pub struct WlShellSurfacePongRequest {
    /// Wire identifier of the object receiving the message.
    pub object_id: u32,
    /// Argument `serial`: serial number of the ping event.
    pub serial: u32,
}
impl WlShellSurfacePongRequest {
    /// Opcode in the request namespace of interface wl_shell_surface.
    pub const OPCODE: u16 = 0;
    /// Minimum interface version supporting this message.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for WlShellSurfacePongRequest {
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
            "wl_shell_surface",
            Self::SINCE,
        )?;
        let serial = reader.next_u32().await?;
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        Ok(Self { object_id, serial })
    }
}
