use crate::protocol::ClientState;
use crate::protocol::{CommandWriter, CommandWriterExt};
use crate::utils::writer::{FdPayloadWriter, FdWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Server event `wl_output.description`.
///
/// Opcode 5; available since version 4. The structure contains the receiver and arguments in XML order.
/// Encoding checks wire values and receiver metadata; compositor semantics are not validated.
/// The caller handles compositor policy and message contents.
/// ```xml
/// <event name="description" since="4">
///       <description summary="human-readable description of this output">
///         Many compositors can produce human-readable descriptions of their
///         outputs. The client may wish to know this description as well, e.g. for
///         output selection purposes.
///
///         The description is a UTF-8 string with no convention defined for its
///         contents. The description is not guaranteed to be unique among all
///         wl_output globals. Examples might include 'Foocorp 11" Display' or
///         'Virtual X11 output via :1'.
///
///         The description event is sent after binding the output object and
///         whenever the description changes. The description is optional, and may
///         not be sent at all.
///
///         The description event will be followed by a done event.
///       </description>
///       <arg name="description" type="string" summary="output description" />
///     </event>
/// ```
#[derive(Debug)]
pub struct WlOutputDescriptionEvent {
    /// Wire identifier of the object receiving the message.
    pub object_id: u32,
    /// Argument `description`: output description.
    pub description: String,
}
impl WlOutputDescriptionEvent {
    /// Opcode in the event namespace of interface wl_output.
    pub const OPCODE: u16 = 5;
    /// Minimum interface version supporting this message.
    pub const SINCE: u32 = 4;
}
impl CommandWriter for WlOutputDescriptionEvent {
    fn object_id(&self) -> u32 {
        self.object_id
    }
    fn opcode(&self) -> u16 {
        Self::OPCODE
    }
    fn payload_size(&self) -> usize {
        Self::total_payload_size(&[FdWriter::encoded_string_size(Some(&self.description))])
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
        if !FdWriter::is_valid_string(&self.description) {
            return Err(FdWriterError::InvalidProtocol);
        }
        writer.next_string(&self.description).await?;
        Ok(())
    }
}
