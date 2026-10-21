use crate::protocol::ClientState;
use crate::protocol::{CommandReader, CommandReaderExt};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use std::sync::{Arc, Mutex};

/// Client request `wl_subsurface.set_sync`.
///
/// Opcode 4; available since version 1. The structure contains the receiver and arguments in XML order.
/// Decoding checks wire framing and receiver metadata; object arguments belong to the compositor.
/// The caller handles compositor policy and message contents.
/// ```xml
/// <request name="set_sync">
///       <description summary="set sub-surface to synchronized mode">
///         Change the commit behaviour of the sub-surface to synchronized
///         mode.
///
///         See wl_subsurface and wl_surface.commit for more information.
///       </description>
///     </request>
/// ```
#[derive(Debug)]
pub struct WlSubsurfaceSetSyncRequest {
    /// Wire identifier of the object receiving the message.
    pub object_id: u32,
}
impl WlSubsurfaceSetSyncRequest {
    /// Opcode in the request namespace of interface wl_subsurface.
    pub const OPCODE: u16 = 4;
    /// Minimum interface version supporting this message.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for WlSubsurfaceSetSyncRequest {
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
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        Ok(Self { object_id })
    }
}
