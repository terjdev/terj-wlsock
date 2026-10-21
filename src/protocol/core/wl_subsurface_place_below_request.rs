use crate::protocol::ClientState;
use crate::protocol::{CommandReader, CommandReaderExt};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use std::sync::{Arc, Mutex};

/// Client request `wl_subsurface.place_below`.
///
/// Opcode 3; available since version 1. The structure contains the receiver and arguments in XML order.
/// Decoding checks wire framing and receiver metadata; object arguments belong to the compositor.
/// The caller handles compositor policy and message contents.
/// ```xml
/// <request name="place_below">
///       <description summary="restack the sub-surface">
///         The sub-surface is placed just below the reference surface.
///
///         See wl_subsurface.place_above.
///       </description>
///       <arg name="sibling" type="object" interface="wl_surface" summary="the reference surface" />
///     </request>
/// ```
#[derive(Debug)]
pub struct WlSubsurfacePlaceBelowRequest {
    /// Wire identifier of the object receiving the message.
    pub object_id: u32,
    /// Argument `sibling`: the reference surface.
    /// Object implementing interface wl_surface.
    pub sibling: u32,
}
impl WlSubsurfacePlaceBelowRequest {
    /// Opcode in the request namespace of interface wl_subsurface.
    pub const OPCODE: u16 = 3;
    /// Minimum interface version supporting this message.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for WlSubsurfacePlaceBelowRequest {
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
