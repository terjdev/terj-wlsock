use crate::protocol::ClientState;
use crate::protocol::{CommandReader, CommandReaderExt};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use std::sync::{Arc, Mutex};

/// Client request `wl_buffer.destroy`.
///
/// Opcode 0; available since version 1. The message ends the receiver's lifetime.
/// The structure contains the receiver and arguments in XML order.
/// Decoding checks wire framing and receiver metadata; object arguments belong to the compositor.
/// The caller handles compositor policy and message contents.
/// ```xml
/// <request name="destroy" type="destructor">
///       <description summary="destroy a buffer">
///         Destroy a buffer. If and how you need to release the backing
///         storage is defined by the buffer factory interface.
///
///         For possible side-effects to a surface, see wl_surface.attach.
///       </description>
///     </request>
/// ```
#[derive(Debug)]
pub struct WlBufferDestroyRequest {
    /// Wire identifier of the object receiving the message.
    pub object_id: u32,
}
impl WlBufferDestroyRequest {
    /// Opcode in the request namespace of interface wl_buffer.
    pub const OPCODE: u16 = 0;
    /// Minimum interface version supporting this message.
    pub const SINCE: u32 = 1;
}
impl CommandReader<Self> for WlBufferDestroyRequest {
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
            "wl_buffer",
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
