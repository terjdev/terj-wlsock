use crate::protocol::ClientState;
use crate::protocol::{CommandReader, CommandReaderExt};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use std::sync::{Arc, Mutex};

/// Client request `wl_shell_surface.set_title`.
///
/// Opcode 8; available since version 1. The structure contains the receiver and arguments in XML order.
/// Decoding checks wire framing and receiver metadata; object arguments belong to the compositor.
/// The caller handles compositor policy and message contents.
/// ```xml
/// <request name="set_title">
///       <description summary="set surface title">
///         Set a short title for the surface.
///
///         This string may be used to identify the surface in a task bar,
///         window list, or other user interface elements provided by the
///         compositor.
///
///         The string must be encoded in UTF-8.
///       </description>
///       <arg name="title" type="string" summary="surface title" />
///     </request>
/// ```
#[derive(Debug)]
pub struct WlShellSurfaceSetTitleRequest {
    /// Wire identifier of the object receiving the message.
    pub object_id: u32,
    /// Argument `title`: surface title.
    pub title: String,
}
impl WlShellSurfaceSetTitleRequest {
    /// Opcode in the request namespace of interface wl_shell_surface.
    pub const OPCODE: u16 = 8;
    /// Minimum interface version supporting this message.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for WlShellSurfaceSetTitleRequest {
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
        let title = reader.next_string().await?;
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        Ok(Self { object_id, title })
    }
}
