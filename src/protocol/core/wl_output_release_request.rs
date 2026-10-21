use crate::protocol::ClientState;
use crate::protocol::{CommandReader, CommandReaderExt};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use std::sync::{Arc, Mutex};

/// Client request `wl_output.release`.
///
/// Opcode 0; available since version 3. The message ends the receiver's lifetime.
/// The structure contains the receiver and arguments in XML order.
/// Decoding checks wire framing and receiver metadata; object arguments belong to the compositor.
/// The caller handles compositor policy and message contents.
/// ```xml
/// <request name="release" type="destructor" since="3">
///       <description summary="release the output object">
///         Using this request a client can tell the server that it is not going to
///         use the output object anymore.
///       </description>
///     </request>
/// ```
#[derive(Debug)]
pub struct WlOutputReleaseRequest {
    /// Wire identifier of the object receiving the message.
    pub object_id: u32,
}
impl WlOutputReleaseRequest {
    /// Opcode in the request namespace of interface wl_output.
    pub const OPCODE: u16 = 0;
    /// Minimum interface version supporting this message.
    pub const SINCE: u32 = 3;
}
impl CommandReader<Self> for WlOutputReleaseRequest {
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
            "wl_output",
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
