use crate::protocol::ClientState;
use crate::protocol::{CommandReader, CommandReaderExt};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use std::sync::{Arc, Mutex};

/// Client request `wl_subsurface.destroy`.
///
/// Opcode 0; available since version 1. The message ends the receiver's lifetime.
/// The structure contains the receiver and arguments in XML order.
/// Decoding checks wire framing and receiver metadata; object arguments belong to the compositor.
/// The caller handles compositor policy and message contents.
/// ```xml
/// <request name="destroy" type="destructor">
///       <description summary="remove sub-surface interface">
///         The sub-surface interface is removed from the wl_surface object
///         that was turned into a sub-surface with a
///         wl_subcompositor.get_subsurface request. The wl_surface's association
///         to the parent is deleted. The wl_surface is unmapped immediately.
///       </description>
///     </request>
/// ```
#[derive(Debug)]
pub struct WlSubsurfaceDestroyRequest {
    /// Wire identifier of the object receiving the message.
    pub object_id: u32,
}
impl WlSubsurfaceDestroyRequest {
    /// Opcode in the request namespace of interface wl_subsurface.
    pub const OPCODE: u16 = 0;
    /// Minimum interface version supporting this message.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for WlSubsurfaceDestroyRequest {
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
        {
            let mut state = Self::read_state(state)?;
            state.release_server_id(object_id);
        }
        Ok(Self { object_id })
    }
}
