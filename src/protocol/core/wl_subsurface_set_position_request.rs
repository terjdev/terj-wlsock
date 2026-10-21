use crate::protocol::ClientState;
use crate::protocol::{CommandReader, CommandReaderExt};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use std::sync::{Arc, Mutex};

/// Client request `wl_subsurface.set_position`.
///
/// Opcode 1; available since version 1. The structure contains the receiver and arguments in XML order.
/// Decoding checks wire framing and receiver metadata; object arguments belong to the compositor.
/// The caller handles compositor policy and message contents.
/// ```xml
/// <request name="set_position">
///       <description summary="reposition the sub-surface">
///         This sets the position of the sub-surface, relative to the parent
///         surface.
///
///         The sub-surface will be moved so that its origin (top left
///         corner pixel) will be at the location x, y of the parent surface
///         coordinate system. The coordinates are not restricted to the parent
///         surface area. Negative values are allowed.
///
///         The initial position is 0, 0.
///
///         Position is double-buffered state on the parent surface, see
///         wl_subsurface and wl_surface.commit for more information.
///       </description>
///       <arg name="x" type="int" summary="x coordinate in the parent surface" />
///       <arg name="y" type="int" summary="y coordinate in the parent surface" />
///     </request>
/// ```
#[derive(Debug)]
pub struct WlSubsurfaceSetPositionRequest {
    /// Wire identifier of the object receiving the message.
    pub object_id: u32,
    /// Argument `x`: x coordinate in the parent surface.
    pub x: i32,
    /// Argument `y`: y coordinate in the parent surface.
    pub y: i32,
}
impl WlSubsurfaceSetPositionRequest {
    /// Opcode in the request namespace of interface wl_subsurface.
    pub const OPCODE: u16 = 1;
    /// Minimum interface version supporting this message.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for WlSubsurfaceSetPositionRequest {
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
            "wl_subsurface",
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
