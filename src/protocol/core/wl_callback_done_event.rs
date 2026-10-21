use crate::protocol::ClientState;
use crate::protocol::{CommandReader, CommandReaderExt};
use crate::protocol::{CommandWriter, CommandWriterExt};
use crate::utils::reader::{FdPayloadReader, FdReaderError};
use crate::utils::writer::{FdPayloadWriter, FdWriterError};
use std::sync::{Arc, Mutex};

/// Server event `wl_callback.done`.
///
/// Opcode 0; available since version 1. The message ends the receiver's lifetime.
/// The structure contains the receiver and arguments in XML order.
/// Encoding and decoding check wire values and receiver metadata; compositor semantics are not validated.
/// Decoding consumes the complete payload and updates wire object metadata when needed.
/// The caller handles compositor policy and message contents.
/// ```xml
/// <event name="done" type="destructor">
///       <description summary="done event">
///         Notify the client when the related request is done.
///       </description>
///       <arg name="callback_data" type="uint" summary="request-specific data for the callback" />
///     </event>
/// ```
#[derive(Debug)]
pub struct WlCallbackDoneEvent {
    /// Wire identifier of the object receiving the message.
    pub object_id: u32,
    /// Argument `callback_data`: request-specific data for the callback.
    pub callback_data: u32,
}
impl WlCallbackDoneEvent {
    /// Opcode in the event namespace of interface wl_callback.
    pub const OPCODE: u16 = 0;
    /// Minimum interface version supporting this message.
    pub const SINCE: u32 = 1;
}
impl CommandWriter for WlCallbackDoneEvent {
    fn object_id(&self) -> u32 {
        self.object_id
    }
    fn opcode(&self) -> u16 {
        Self::OPCODE
    }
    fn payload_size(&self) -> usize {
        Self::total_payload_size(&[Some(4)])
    }
    async fn write(
        self,
        writer: &mut FdPayloadWriter<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<(), FdWriterError> {
        Self::event_interface(state, self.object_id, "wl_callback", Self::SINCE)?;
        if self.payload_size() != writer.remaining_quota() {
            return Err(FdWriterError::InvalidProtocol);
        }
        writer.next_u32(self.callback_data).await?;
        {
            let mut state = Self::write_state(state)?;
            state.release_server_id(self.object_id);
        }
        Ok(())
    }
}

impl CommandReader<Self> for WlCallbackDoneEvent {
    async fn read(
        object_id: u32,
        opcode: u16,
        reader: &mut FdPayloadReader<'_>,
        state: &Arc<Mutex<ClientState>>,
    ) -> Result<Self, FdReaderError> {
        Self::event_interface(
            state,
            object_id,
            opcode,
            Self::OPCODE,
            "wl_callback",
            Self::SINCE,
        )?;
        let callback_data = reader.next_u32().await?;
        if reader.remaining_quota() != 0 {
            return Err(FdReaderError::InvalidProtocol);
        }
        Self::read_state(state)?.release_server_id(object_id);
        Ok(Self {
            object_id,
            callback_data,
        })
    }
}
