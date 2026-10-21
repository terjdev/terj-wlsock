use crate::protocol::ClientState;
use crate::protocol::{CommandReader, CommandReaderExt};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use std::sync::{Arc, Mutex};

/// Client request `wl_shell_surface.set_toplevel`.
///
/// Opcode 3; available since version 1. The structure contains the receiver and arguments in XML order.
/// Decoding checks wire framing and receiver metadata; object arguments belong to the compositor.
/// The caller handles compositor policy and message contents.
/// ```xml
/// <request name="set_toplevel">
///       <description summary="make the surface a toplevel surface">
///         Map the surface as a toplevel surface.
///
///         A toplevel surface is not fullscreen, maximized or transient.
///       </description>
///     </request>
/// ```
#[derive(Debug)]
pub struct WlShellSurfaceSetToplevelRequest {
    /// Wire identifier of the object receiving the message.
    pub object_id: u32,
}
impl WlShellSurfaceSetToplevelRequest {
    /// Opcode in the request namespace of interface wl_shell_surface.
    pub const OPCODE: u16 = 3;
    /// Minimum interface version supporting this message.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for WlShellSurfaceSetToplevelRequest {
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
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        Ok(Self { object_id })
    }
}
