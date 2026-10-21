use crate::protocol::ClientState;
use crate::protocol::{CommandReader, CommandReaderExt};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use std::sync::{Arc, Mutex};

/// Client request `wl_subsurface.place_above`.
///
/// Opcode 2; available since version 1. The structure contains the receiver and arguments in XML order.
/// Decoding checks wire framing and receiver metadata; object arguments belong to the compositor.
/// The caller handles compositor policy and message contents.
/// ```xml
/// <request name="place_above">
///       <description summary="restack the sub-surface">
///         This sub-surface is taken from the stack, and put back just
///         above the reference surface, changing the z-order of the sub-surfaces.
///         The reference surface must be one of the sibling surfaces, or the
///         parent surface. Using any other surface, including this sub-surface,
///         will cause a protocol error.
///
///         A new sub-surface is initially added as the top-most in the stack
///         of its siblings and parent.
///
///         Z-order is double-buffered state on the parent surface, see
///         wl_subsurface and wl_surface.commit for more information.
///       </description>
///       <arg name="sibling" type="object" interface="wl_surface" summary="the reference surface" />
///     </request>
/// ```
#[derive(Debug)]
pub struct WlSubsurfacePlaceAboveRequest {
    /// Wire identifier of the object receiving the message.
    pub object_id: u32,
    /// Argument `sibling`: the reference surface.
    /// Object implementing interface wl_surface.
    pub sibling: u32,
}
impl WlSubsurfacePlaceAboveRequest {
    /// Opcode in the request namespace of interface wl_subsurface.
    pub const OPCODE: u16 = 2;
    /// Minimum interface version supporting this message.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for WlSubsurfacePlaceAboveRequest {
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
        let sibling = reader.next_u32().await?;
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        Ok(Self { object_id, sibling })
    }
}
