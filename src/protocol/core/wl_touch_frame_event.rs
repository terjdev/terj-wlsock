use crate::protocol::ClientState;
use crate::protocol::{CommandWriter, CommandWriterExt};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Server event `wl_touch.frame`.
///
/// Opcode 3; available since version 1. The structure contains the receiver and arguments in XML order.
/// Encoding checks wire values and receiver metadata; compositor semantics are not validated.
/// The caller handles compositor policy and message contents.
/// ```xml
/// <event name="frame">
///       <description summary="end of touch frame event">
///         Indicates the end of a set of events that logically belong together.
///         A client is expected to accumulate the data in all events within the
///         frame before proceeding.
///
///         A wl_touch.frame terminates at least one event but otherwise no
///         guarantee is provided about the set of events within a frame. A client
///         must assume that any state not updated in a frame is unchanged from the
///         previously known state.
///       </description>
///     </event>
/// ```
#[derive(Debug)]
pub struct WlTouchFrameEvent {
    /// Wire identifier of the object receiving the message.
    pub object_id: u32,
}
impl WlTouchFrameEvent {
    /// Opcode in the event namespace of interface wl_touch.
    pub const OPCODE: u16 = 3;
    /// Minimum interface version supporting this message.
    pub const SINCE: u32 = 1;
}
impl CommandWriter for WlTouchFrameEvent {
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
        Self::event_interface(state, self.object_id, "wl_touch", Self::SINCE)?;
        if self.payload_size() != writer.remaining_quota() {
            return Err(FdWriterError::InvalidProtocol);
        }
        Ok(())
    }
}
