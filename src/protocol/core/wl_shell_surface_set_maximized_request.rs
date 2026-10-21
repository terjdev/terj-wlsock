use crate::protocol::ClientState;
use crate::protocol::{CommandReader, CommandReaderExt};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use std::sync::{Arc, Mutex};

/// Client request `wl_shell_surface.set_maximized`.
///
/// Opcode 7; available since version 1. The structure contains the receiver and arguments in XML order.
/// Decoding checks wire framing and receiver metadata; object arguments belong to the compositor.
/// The caller handles compositor policy and message contents.
/// ```xml
/// <request name="set_maximized">
///       <description summary="make the surface a maximized surface">
///         Map the surface as a maximized surface.
///
///         If an output parameter is given then the surface will be
///         maximized on that output. If the client does not specify the
///         output then the compositor will apply its policy - usually
///         choosing the output on which the surface has the biggest surface
///         area.
///
///         The compositor will reply with a configure event telling
///         the expected new surface size. The operation is completed
///         on the next buffer attach to this surface.
///
///         A maximized surface typically fills the entire output it is
///         bound to, except for desktop elements such as panels. This is
///         the main difference between a maximized shell surface and a
///         fullscreen shell surface.
///
///         The details depend on the compositor implementation.
///       </description>
///       <arg name="output" type="object" interface="wl_output" allow-null="true" summary="output on which the surface is to be maximized" />
///     </request>
/// ```
#[derive(Debug)]
pub struct WlShellSurfaceSetMaximizedRequest {
    /// Wire identifier of the object receiving the message.
    pub object_id: u32,
    /// Argument `output`: output on which the surface is to be maximized.
    /// The protocol permits NULL.
    /// Object implementing interface wl_output.
    pub output: Option<u32>,
}
impl WlShellSurfaceSetMaximizedRequest {
    /// Opcode in the request namespace of interface wl_shell_surface.
    pub const OPCODE: u16 = 7;
    /// Minimum interface version supporting this message.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for WlShellSurfaceSetMaximizedRequest {
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
        let output = {
            let id = reader.next_u32().await?;
            (id != 0).then_some(id)
        };
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        Ok(Self { object_id, output })
    }
}
