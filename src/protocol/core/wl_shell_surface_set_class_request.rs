use crate::protocol::ClientState;
use crate::protocol::{CommandReader, CommandReaderExt};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use std::sync::{Arc, Mutex};

/// Client request `wl_shell_surface.set_class`.
///
/// Opcode 9; available since version 1. The structure contains the receiver and arguments in XML order.
/// Decoding checks wire framing and receiver metadata; object arguments belong to the compositor.
/// The caller handles compositor policy and message contents.
/// ```xml
/// <request name="set_class">
///       <description summary="set surface class">
///         Set a class for the surface.
///
///         The surface class identifies the general class of applications
///         to which the surface belongs. A common convention is to use the
///         file name (or the full path if it is a non-standard location) of
///         the application's .desktop file as the class.
///       </description>
///       <arg name="class_" type="string" summary="surface class" />
///     </request>
/// ```
#[derive(Debug)]
pub struct WlShellSurfaceSetClassRequest {
    /// Wire identifier of the object receiving the message.
    pub object_id: u32,
    /// Argument `class_`: surface class.
    pub class_: String,
}
impl WlShellSurfaceSetClassRequest {
    /// Opcode in the request namespace of interface wl_shell_surface.
    pub const OPCODE: u16 = 9;
    /// Minimum interface version supporting this message.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for WlShellSurfaceSetClassRequest {
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
        let class_ = reader.next_string().await?;
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        Ok(Self { object_id, class_ })
    }
}
