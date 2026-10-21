use crate::protocol::ClientState;
use crate::protocol::{CommandReader, CommandReaderExt};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use std::sync::{Arc, Mutex};

/// Client request `wl_surface.offset`.
///
/// Opcode 10; available since version 5. The structure contains the receiver and arguments in XML order.
/// Decoding checks wire framing and receiver metadata; object arguments belong to the compositor.
/// The caller handles compositor policy and message contents.
/// ```xml
/// <request name="offset" since="5">
///       <description summary="set the surface contents offset">
///         The x and y arguments specify the location of the new pending
///         buffer's upper left corner, relative to the current buffer's upper
///         left corner, in surface-local coordinates. In other words, the
///         x and y, combined with the new surface size define in which
///         directions the surface's size changes.
///
///         The exact semantics of wl_surface.offset are role-specific. Refer to
///         the documentation of specific roles for more information.
///
///         Surface location offset is double-buffered state, see
///         wl_surface.commit.
///
///         This request is semantically equivalent to and the replaces the x and y
///         arguments in the wl_surface.attach request in wl_surface versions prior
///         to 5. See wl_surface.attach for details.
///       </description>
///       <arg name="x" type="int" summary="surface-local x coordinate" />
///       <arg name="y" type="int" summary="surface-local y coordinate" />
///     </request>
/// ```
#[derive(Debug)]
pub struct WlSurfaceOffsetRequest {
    /// Wire identifier of the object receiving the message.
    pub object_id: u32,
    /// Argument `x`: surface-local x coordinate.
    pub x: i32,
    /// Argument `y`: surface-local y coordinate.
    pub y: i32,
}
impl WlSurfaceOffsetRequest {
    /// Opcode in the request namespace of interface wl_surface.
    pub const OPCODE: u16 = 10;
    /// Minimum interface version supporting this message.
    pub const SINCE: u32 = 5;
}
impl CommandReader<Self> for WlSurfaceOffsetRequest {
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
            "wl_surface",
            Self::SINCE,
        )?;
        let x = reader.next_i32().await?;
        let y = reader.next_i32().await?;
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        Ok(Self { object_id, x, y })
    }
}
