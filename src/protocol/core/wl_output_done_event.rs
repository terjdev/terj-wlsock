use crate::protocol::ClientState;
use crate::protocol::{CommandWriter, CommandWriterExt};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Server event `wl_output.done`.
///
/// Opcode 2; available since version 2. The structure contains the receiver and arguments in XML order.
/// Encoding checks wire values and receiver metadata; compositor semantics are not validated.
/// The caller handles compositor policy and message contents.
/// ```xml
/// <event name="done" since="2">
///       <description summary="sent all information about output">
///         This event is sent after all other properties have been
///         sent after binding to the output object and after any
///         other property changes done after that. This allows
///         changes to the output properties to be seen as
///         atomic, even if they happen via multiple events.
///       </description>
///     </event>
/// ```
#[derive(Debug)]
pub struct WlOutputDoneEvent {
    /// Wire identifier of the object receiving the message.
    pub object_id: u32,
}
impl WlOutputDoneEvent {
    /// Opcode in the event namespace of interface wl_output.
    pub const OPCODE: u16 = 2;
    /// Minimum interface version supporting this message.
    pub const SINCE: u32 = 2;
}
impl CommandWriter for WlOutputDoneEvent {
    fn object_id(&self) -> u32 {
        self.object_id
    }
    fn opcode(&self) -> u16 {
        Self::OPCODE
    }
    fn payload_size(&self) -> usize {
        Self::total_payload_size(&[])
    }
    async fn write(
        self,
        writer: &mut FdPayloadWriter<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<(), FdWriterError> {
        Self::event_interface(state, self.object_id, "wl_output", Self::SINCE)?;
        if self.payload_size() != writer.remaining_quota() {
            return Err(FdWriterError::InvalidProtocol);
        }
        Ok(())
    }
}
